use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_json::{json, Value};

use super::{inspect_provider_key, KeyInspectOptions};

#[derive(Debug, Default)]
pub struct KimiUnblockRehearsalOptions {
    pub json: bool,
    pub from_file: Option<PathBuf>,
    pub from_env: Option<String>,
    pub stdin_value: Option<String>,
}

#[derive(Debug)]
pub struct KimiUnblockRehearsalResult {
    pub output: String,
    pub failed: bool,
}

#[derive(Debug, Serialize)]
struct KimiUnblockRehearsalReport {
    provider: String,
    objective: String,
    status: String,
    failed: bool,
    writes_key: bool,
    network: bool,
    detail: String,
    current_credential: Value,
    candidate: Option<Value>,
    command_plan: Vec<RehearsalCommand>,
    safety_guards: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RehearsalCommand {
    step: usize,
    command: String,
    writes_key: bool,
    network: bool,
    required: bool,
    purpose: String,
}

pub fn rehearse_provider_unblock(
    project_root: &Path,
    provider: &str,
    options: KimiUnblockRehearsalOptions,
) -> Result<KimiUnblockRehearsalResult> {
    if provider != "kimi" {
        return Err(anyhow!(
            "provider unblock rehearsal is only supported for `kimi` right now"
        ));
    }

    let source_count = usize::from(options.from_file.is_some())
        + usize::from(options.from_env.is_some())
        + usize::from(options.stdin_value.is_some());
    if source_count > 1 {
        return Err(anyhow!("choose at most one replacement key source"));
    }

    let render_json = options.json;
    let source_args = source_args(&options);
    let current_credential = inspect_current_credential(project_root);
    let candidate = if source_count == 0 {
        None
    } else {
        Some(inspect_candidate(project_root, options)?)
    };

    let candidate_failed = candidate
        .as_ref()
        .and_then(|value| value.get("failed"))
        .and_then(Value::as_bool)
        .unwrap_or(true);
    let status = if candidate.is_none() {
        "needs_candidate"
    } else if candidate_failed {
        "blocked"
    } else {
        "ready_to_preflight"
    };
    let failed = status != "ready_to_preflight";
    let detail = match status {
        "ready_to_preflight" => {
            "replacement key candidate has a plain API-key shape; run live preflight before any write"
        }
        "blocked" => {
            "replacement key candidate is not safe to preflight; create a plain Moonshot OpenAI-compatible API key"
        }
        _ => "provide a replacement key source to rehearse the Kimi unblock path",
    }
    .to_string();

    let command_plan = command_plan(source_args.as_deref());
    let next_commands = next_commands(status, source_args.as_deref());
    let report = KimiUnblockRehearsalReport {
        provider: "kimi".to_string(),
        objective: "kimi_replacement_key_unblock_rehearsal".to_string(),
        status: status.to_string(),
        failed,
        writes_key: false,
        network: false,
        detail,
        current_credential,
        candidate,
        command_plan,
        safety_guards: vec![
            "offline rehearsal does not call provider APIs".to_string(),
            "offline rehearsal does not write or rotate .kimi".to_string(),
            "secret material is never printed; only length and sha256 prefix are shown".to_string(),
            "Kimi Code CLI OAuth JSON is rejected before live preflight or install".to_string(),
            "live preflight must pass before rc-unblock installs a replacement key".to_string(),
            "successful preflight endpoint is carried into provider test, auth check, dogfood, and RC gate".to_string(),
        ],
        next_commands,
    };

    let output = if render_json {
        format!("{}\n", serde_json::to_string_pretty(&report)?)
    } else {
        render_text_report(&report)
    };

    Ok(KimiUnblockRehearsalResult { output, failed })
}

fn inspect_current_credential(project_root: &Path) -> Value {
    match inspect_provider_key(
        project_root,
        "kimi",
        KeyInspectOptions {
            json: true,
            ..Default::default()
        },
    ) {
        Ok(result) => parse_json_report(&result.output),
        Err(error) => json!({
            "provider": "kimi",
            "source": "active",
            "key_sha256_12": "none",
            "key_chars": 0,
            "trimmed_for_request": false,
            "writes_key": false,
            "network": false,
            "classification": "missing",
            "status": "missing",
            "detail": error.to_string(),
            "failed": true,
            "next_commands": [
                "agenthub providers inspect-key kimi --from-file <new-key-file>",
                "agenthub providers rehearse-unblock kimi --from-file <new-key-file>"
            ]
        }),
    }
}

fn inspect_candidate(project_root: &Path, options: KimiUnblockRehearsalOptions) -> Result<Value> {
    let result = inspect_provider_key(
        project_root,
        "kimi",
        KeyInspectOptions {
            json: true,
            from_file: options.from_file,
            from_env: options.from_env,
            stdin_value: options.stdin_value,
        },
    )?;
    Ok(parse_json_report(&result.output))
}

fn parse_json_report(output: &str) -> Value {
    serde_json::from_str(output).unwrap_or_else(|error| {
        json!({
            "provider": "kimi",
            "source": "unknown",
            "classification": "parse_error",
            "status": "invalid",
            "detail": error.to_string(),
            "failed": true
        })
    })
}

fn source_args(options: &KimiUnblockRehearsalOptions) -> Option<String> {
    options
        .from_file
        .as_ref()
        .map(|path| format!("--from-file {}", path.display()))
        .or_else(|| {
            options
                .from_env
                .as_ref()
                .map(|env_name| format!("--from-env {env_name}"))
        })
        .or_else(|| options.stdin_value.as_ref().map(|_| "--stdin".to_string()))
}

fn command_plan(source_args: Option<&str>) -> Vec<RehearsalCommand> {
    let candidate_args = source_args.unwrap_or("--from-file <new-key-file>");
    vec![
        RehearsalCommand {
            step: 1,
            command: "agenthub providers inspect-key kimi".to_string(),
            writes_key: false,
            network: false,
            required: true,
            purpose: "classify the active Kimi credential and current blocker safely".to_string(),
        },
        RehearsalCommand {
            step: 2,
            command: format!("agenthub providers inspect-key kimi {candidate_args}"),
            writes_key: false,
            network: false,
            required: true,
            purpose: "verify replacement key shape before any live API call".to_string(),
        },
        RehearsalCommand {
            step: 3,
            command: format!("agenthub providers preflight-key kimi {candidate_args}"),
            writes_key: false,
            network: true,
            required: true,
            purpose: "perform live auth check without writing the candidate key".to_string(),
        },
        RehearsalCommand {
            step: 4,
            command: format!("agenthub providers rc-unblock kimi {candidate_args}"),
            writes_key: true,
            network: true,
            required: true,
            purpose:
                "install only a preflight-passed key, then run provider dogfood and RC evidence"
                    .to_string(),
        },
        RehearsalCommand {
            step: 5,
            command: "agenthub readiness completion --json --check".to_string(),
            writes_key: false,
            network: false,
            required: true,
            purpose: "verify the Kimi unblock cleared provider and RC blockers".to_string(),
        },
    ]
}

fn next_commands(status: &str, source_args: Option<&str>) -> Vec<String> {
    let candidate_args = source_args.unwrap_or("--from-file <new-key-file>");
    match status {
        "ready_to_preflight" => vec![
            format!("agenthub providers preflight-key kimi {candidate_args}"),
            format!("agenthub providers rc-unblock kimi {candidate_args}"),
            "agenthub readiness completion --json --check".to_string(),
        ],
        "blocked" => vec![
            "create a plain Moonshot OpenAI-compatible API key".to_string(),
            format!("agenthub providers inspect-key kimi {candidate_args}"),
            format!("agenthub providers rehearse-unblock kimi {candidate_args}"),
        ],
        _ => vec![
            "agenthub providers inspect-key kimi".to_string(),
            "agenthub providers inspect-key kimi --from-file <new-key-file>".to_string(),
            "agenthub providers rehearse-unblock kimi --from-file <new-key-file>".to_string(),
            "agenthub providers preflight-key kimi --from-file <new-key-file>".to_string(),
            "agenthub providers rc-unblock kimi --from-file <new-key-file>".to_string(),
        ],
    }
}

fn render_text_report(report: &KimiUnblockRehearsalReport) -> String {
    let mut out = String::from("AgentHub Kimi unblock rehearsal\n");
    out.push_str("provider\tkimi\n");
    out.push_str(&format!("status\t{}\n", report.status));
    out.push_str(&format!("detail\t{}\n", report.detail));
    out.push_str("writes_key\tfalse\n");
    out.push_str("network\tfalse\n");
    append_credential_summary(&mut out, "current", &report.current_credential);
    if let Some(candidate) = &report.candidate {
        append_credential_summary(&mut out, "candidate", candidate);
    } else {
        out.push_str("candidate_status\tmissing\n");
    }
    for command in &report.command_plan {
        out.push_str(&format!(
            "plan\t{}\t{}\twrites_key:{}\tnetwork:{}\trequired:{}\t{}\n",
            command.step,
            command.command,
            command.writes_key,
            command.network,
            command.required,
            command.purpose
        ));
    }
    for (index, guard) in report.safety_guards.iter().enumerate() {
        out.push_str(&format!("guard\t{}\t{}\n", index + 1, guard));
    }
    for (index, command) in report.next_commands.iter().enumerate() {
        out.push_str(&format!("next\t{}\t{}\n", index + 1, command));
    }
    out
}

fn append_credential_summary(out: &mut String, prefix: &str, value: &Value) {
    for key in [
        "source",
        "key_sha256_12",
        "key_chars",
        "trimmed_for_request",
        "writes_key",
        "network",
        "classification",
        "status",
        "detail",
    ] {
        if let Some(rendered) = value_field(value, key) {
            out.push_str(&format!("{prefix}_{key}\t{rendered}\n"));
        }
    }
}

fn value_field(value: &Value, key: &str) -> Option<String> {
    let field = value.get(key)?;
    match field {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}
