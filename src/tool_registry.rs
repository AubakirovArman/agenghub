mod definitions;
mod policy;
#[cfg(test)]
mod tests;
mod types;

use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use serde_json::{json, Value};

use crate::command_runner::{run_shell, CommandResult};
use crate::llm_gateway::{classify_tool_call, ToolCall};
use crate::observability::redact_text;
use crate::tool_permissions::ToolPermissionProfile;

pub use definitions::builtin_tool_definitions;
use policy::{
    bound_text, candidate_files, relative_display, resolve_existing, shell_network_policy,
    shell_protected_path_policy,
};
use types::{
    error_outcome, ok_outcome, outcome, ToolRegistryOutcome, MAX_LIST_ENTRIES, MAX_READ_BYTES,
    MAX_RESULT_CHARS, MAX_SEARCH_FILES, MAX_SEARCH_MATCHES, SHELL_TIMEOUT_SECS,
};
pub use types::{
    ToolExecutionResult, ToolExecutionStatus, ToolNetworkPolicy, ToolOutputPolicy, ToolPathPolicy,
    ToolPolicyDecision, ToolPolicyLimits, MAX_TOOL_ROUNDS,
};

pub fn execute_tool_call(root: &Path, call: &ToolCall) -> ToolExecutionResult {
    let permission = classify_tool_call(call);
    if permission.approval_required || permission.profile != ToolPermissionProfile::ReadOnly {
        let mut policy = ToolPolicyDecision::new();
        policy.output.reason =
            Some("tool did not pass the read-only builtin auto-execution threshold".to_string());
        return ToolExecutionResult {
            call_id: call.id.clone(),
            name: call.name.clone(),
            status: ToolExecutionStatus::ApprovalRequired,
            permission,
            policy,
            content: None,
            error: Some("tool call requires approval or is not read-only".to_string()),
        };
    }

    let outcome = match call.name.trim().to_ascii_lowercase().as_str() {
        "read_file" => read_file(root, call),
        "list_dir" => list_dir(root, call),
        "search" | "grep" => search(root, call),
        "shell" | "bash" | "execute_command" | "run_command" => shell(root, call),
        _ => error_outcome(
            ToolPolicyDecision::new(),
            format!("unknown AgentHub builtin tool `{}`", call.name),
        ),
    };

    ToolExecutionResult {
        call_id: call.id.clone(),
        name: call.name.clone(),
        status: outcome.status,
        permission,
        policy: outcome.policy,
        content: outcome.content,
        error: outcome.error,
    }
}

pub fn result_needs_approval(result: &ToolExecutionResult) -> bool {
    matches!(result.status, ToolExecutionStatus::ApprovalRequired)
        || result.permission.approval_required
}

pub fn results_prompt(round: usize, results: &[ToolExecutionResult]) -> Result<String> {
    let value = serde_json::to_value(results)?;
    let rendered = serde_json::to_string_pretty(&value)?;
    Ok(format!(
        "\n\nAgentHub builtin tool results, round {round} (redacted JSON):\n```json\n{}\n```\nContinue the same turn. Use these results to either call another bounded read-only AgentHub tool or call `agenthub_command_plan` with the final non-interactive command plan. Do not repeat completed tool calls unless the result was an error.\n",
        bound_text(&rendered, MAX_RESULT_CHARS)
    ))
}

fn read_file(root: &Path, call: &ToolCall) -> ToolRegistryOutcome {
    let mut policy = ToolPolicyDecision::new();
    let path = match required_string(call, "path") {
        Ok(path) => path,
        Err(error) => return error_outcome(policy, error),
    };
    let resolved = match resolve_existing(root, path) {
        Ok(resolved) => resolved,
        Err(error) => {
            policy.path = Some(error.policy);
            return outcome(error.status, policy, None, Some(error.reason));
        }
    };
    policy.path = Some(resolved.policy.clone());
    let meta = match fs::metadata(&resolved.path) {
        Ok(meta) => meta,
        Err(error) => {
            return error_outcome(policy, format!("stat {}: {error}", resolved.path.display()))
        }
    };
    if !meta.is_file() {
        return error_outcome(policy, format!("{} is not a file", path));
    }
    let bytes = match fs::read(&resolved.path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return error_outcome(policy, format!("read {}: {error}", resolved.path.display()))
        }
    };
    let truncated = bytes.len() as u64 > MAX_READ_BYTES;
    let slice = if truncated {
        &bytes[..MAX_READ_BYTES as usize]
    } else {
        &bytes
    };
    policy.output.bytes_read = Some(slice.len() as u64);
    policy.output.truncated = truncated;
    let text = match std::str::from_utf8(slice) {
        Ok(text) => text,
        Err(_) => {
            policy.output.skipped_binary = true;
            policy.output.reason = Some("binary or non-UTF-8 file content was not returned".into());
            return error_outcome(policy, format!("{path} is not valid UTF-8 text"));
        }
    };
    let text = match redact_text(text) {
        Ok(text) => bound_text(&text, MAX_RESULT_CHARS),
        Err(error) => return error_outcome(policy, error.to_string()),
    };
    policy.output.chars_returned = Some(text.chars().count());
    ok_outcome(
        policy,
        json!({
            "path": relative_display(root, &resolved.path),
            "bytes_read": slice.len(),
            "truncated": truncated,
            "text": text,
        }),
    )
}

fn list_dir(root: &Path, call: &ToolCall) -> ToolRegistryOutcome {
    let mut policy = ToolPolicyDecision::new();
    let path = match required_string(call, "path") {
        Ok(path) => path,
        Err(error) => return error_outcome(policy, error),
    };
    let resolved = match resolve_existing(root, path) {
        Ok(resolved) => resolved,
        Err(error) => {
            policy.path = Some(error.policy);
            return outcome(error.status, policy, None, Some(error.reason));
        }
    };
    policy.path = Some(resolved.policy.clone());
    if !resolved.path.is_dir() {
        return error_outcome(policy, format!("{} is not a directory", path));
    }
    let mut entries = Vec::new();
    let read_dir = match fs::read_dir(&resolved.path) {
        Ok(read_dir) => read_dir,
        Err(error) => {
            return error_outcome(policy, format!("read {}: {error}", resolved.path.display()))
        }
    };
    for entry in read_dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => return error_outcome(policy, error.to_string()),
        };
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => return error_outcome(policy, error.to_string()),
        };
        entries.push(json!({
            "name": entry.file_name().to_string_lossy(),
            "kind": if file_type.is_dir() {
                "dir"
            } else if file_type.is_file() {
                "file"
            } else {
                "other"
            },
        }));
        if entries.len() >= MAX_LIST_ENTRIES {
            break;
        }
    }
    policy.output.truncated = entries.len() >= MAX_LIST_ENTRIES;
    policy.output.chars_returned = Some(serde_json::to_string(&entries).unwrap_or_default().len());
    ok_outcome(
        policy,
        json!({
            "path": relative_display(root, &resolved.path),
            "truncated": entries.len() >= MAX_LIST_ENTRIES,
            "entries": entries,
        }),
    )
}

fn search(root: &Path, call: &ToolCall) -> ToolRegistryOutcome {
    let mut policy = ToolPolicyDecision::new();
    let query = match required_string(call, "query") {
        Ok(query) => query,
        Err(error) => return error_outcome(policy, error),
    };
    if query.trim().is_empty() {
        return error_outcome(policy, "search query is empty");
    }
    let start = call
        .arguments
        .get("path")
        .and_then(Value::as_str)
        .unwrap_or(".");
    let resolved = match resolve_existing(root, start) {
        Ok(resolved) => resolved,
        Err(error) => {
            policy.path = Some(error.policy);
            return outcome(error.status, policy, None, Some(error.reason));
        }
    };
    policy.path = Some(resolved.policy.clone());
    let candidates = match candidate_files(&resolved.path) {
        Ok(candidates) => candidates,
        Err(error) => return error_outcome(policy, error),
    };
    let mut matches = Vec::new();
    let mut visited_files = 0usize;
    for file in candidates {
        if visited_files >= MAX_SEARCH_FILES || matches.len() >= MAX_SEARCH_MATCHES {
            break;
        }
        visited_files += 1;
        let Ok(bytes) = fs::read(&file) else {
            continue;
        };
        if bytes.len() as u64 > MAX_READ_BYTES {
            continue;
        }
        let Ok(text) = std::str::from_utf8(&bytes) else {
            continue;
        };
        for (line_index, line) in text.lines().enumerate() {
            if line.contains(query) {
                let line = match redact_text(line) {
                    Ok(line) => bound_text(&line, 800),
                    Err(error) => return error_outcome(policy, error.to_string()),
                };
                matches.push(json!({
                    "path": relative_display(root, &file),
                    "line": line_index + 1,
                    "text": line,
                }));
                if matches.len() >= MAX_SEARCH_MATCHES {
                    break;
                }
            }
        }
    }
    let truncated = visited_files >= MAX_SEARCH_FILES || matches.len() >= MAX_SEARCH_MATCHES;
    policy.output.truncated = truncated;
    policy.output.chars_returned = Some(serde_json::to_string(&matches).unwrap_or_default().len());
    ok_outcome(
        policy,
        json!({
            "query": query,
            "path": relative_display(root, &resolved.path),
            "visited_files": visited_files,
            "truncated": truncated,
            "matches": matches,
        }),
    )
}

fn shell(root: &Path, call: &ToolCall) -> ToolRegistryOutcome {
    let mut policy = ToolPolicyDecision::new();
    let command = match required_string(call, "command") {
        Ok(command) => command,
        Err(error) => return error_outcome(policy, error),
    };
    let network = shell_network_policy(command);
    let protected = shell_protected_path_policy(command);
    policy.network = Some(network.clone());
    if !network.allowed {
        return outcome(
            ToolExecutionStatus::ApprovalRequired,
            policy,
            None,
            Some(network.reason),
        );
    }
    if let Some(path_policy) = protected {
        let reason = path_policy.reason.clone();
        policy.path = Some(path_policy);
        return outcome(
            ToolExecutionStatus::ApprovalRequired,
            policy,
            None,
            Some(reason),
        );
    }
    let result = match run_shell(command, root, Duration::from_secs(SHELL_TIMEOUT_SECS)) {
        Ok(result) => result,
        Err(error) => return error_outcome(policy, error.to_string()),
    };
    shell_result_json(policy, &result)
}

fn shell_result_json(
    mut policy: ToolPolicyDecision,
    result: &CommandResult,
) -> ToolRegistryOutcome {
    let stdout = match redact_text(&result.stdout) {
        Ok(stdout) => bound_text(&stdout, MAX_RESULT_CHARS / 2),
        Err(error) => return error_outcome(policy, error.to_string()),
    };
    let stderr = match redact_text(&result.stderr) {
        Ok(stderr) => bound_text(&stderr, MAX_RESULT_CHARS / 2),
        Err(error) => return error_outcome(policy, error.to_string()),
    };
    policy.output.truncated = result.stdout_truncated
        || result.stderr_truncated
        || result.stdout.chars().count() > MAX_RESULT_CHARS / 2
        || result.stderr.chars().count() > MAX_RESULT_CHARS / 2;
    policy.output.chars_returned = Some(stdout.chars().count() + stderr.chars().count());
    ok_outcome(
        policy,
        json!({
            "command": result.command,
            "success": result.success,
            "exit_code": result.exit_code,
            "timed_out": result.timed_out,
            "duration_ms": result.duration_ms,
            "stdout": stdout,
            "stderr": stderr,
        }),
    )
}

fn required_string<'a>(call: &'a ToolCall, key: &str) -> std::result::Result<&'a str, String> {
    call.arguments
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("tool `{}` missing required string `{}`", call.name, key))
}
