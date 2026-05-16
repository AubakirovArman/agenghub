use std::fs;

use anyhow::Result;
use serde_json::json;
use tempfile::TempDir;

use crate::llm_gateway::ToolCall;
use crate::tool_registry::{execute_tool_call, ToolExecutionStatus, MAX_TOOL_ROUNDS};

#[test]
fn read_file_blocks_path_escape() -> Result<()> {
    let dir = TempDir::new()?;
    let outside = dir.path().join("../outside.txt");
    let call = ToolCall {
        id: "call-1".to_string(),
        name: "read_file".to_string(),
        arguments: json!({ "path": outside.display().to_string() }),
        raw_arguments: "{}".to_string(),
    };

    let result = execute_tool_call(dir.path(), &call);

    assert!(matches!(result.status, ToolExecutionStatus::Error));
    assert!(result.error.unwrap().contains("escapes AgentHub worktree"));
    Ok(())
}

#[test]
fn read_and_search_workspace_text() -> Result<()> {
    let dir = TempDir::new()?;
    fs::write(dir.path().join("README.md"), "hello AgentHub\nsecond\n")?;
    let read = ToolCall {
        id: "call-read".to_string(),
        name: "read_file".to_string(),
        arguments: json!({ "path": "README.md" }),
        raw_arguments: "{}".to_string(),
    };
    let search = ToolCall {
        id: "call-search".to_string(),
        name: "search".to_string(),
        arguments: json!({ "query": "AgentHub", "path": "." }),
        raw_arguments: "{}".to_string(),
    };

    let read_result = execute_tool_call(dir.path(), &read);
    let search_result = execute_tool_call(dir.path(), &search);

    assert!(matches!(read_result.status, ToolExecutionStatus::Ok));
    assert!(serde_json::to_string(&read_result.content)?.contains("hello AgentHub"));
    assert_eq!(read_result.policy.limits.max_tool_rounds, MAX_TOOL_ROUNDS);
    assert!(read_result
        .policy
        .path
        .as_ref()
        .is_some_and(|path| path.decision == "allowed"));
    assert!(matches!(search_result.status, ToolExecutionStatus::Ok));
    assert!(serde_json::to_string(&search_result.content)?.contains("README.md"));
    Ok(())
}

#[test]
fn protected_paths_require_approval_with_policy_reason() -> Result<()> {
    let dir = TempDir::new()?;
    fs::write(dir.path().join(".env"), "TOKEN=secret\n")?;
    let call = ToolCall {
        id: "call-env".to_string(),
        name: "read_file".to_string(),
        arguments: json!({ "path": ".env" }),
        raw_arguments: "{}".to_string(),
    };

    let result = execute_tool_call(dir.path(), &call);

    assert!(matches!(
        result.status,
        ToolExecutionStatus::ApprovalRequired
    ));
    let path = result.policy.path.expect("path policy");
    assert!(path.protected);
    assert_eq!(path.decision, "denied");
    assert!(path.reason.contains("protected"));
    Ok(())
}

#[test]
fn binary_files_are_not_reinjected() -> Result<()> {
    let dir = TempDir::new()?;
    fs::write(dir.path().join("image.bin"), [0, 159, 146, 150])?;
    let call = ToolCall {
        id: "call-bin".to_string(),
        name: "read_file".to_string(),
        arguments: json!({ "path": "image.bin" }),
        raw_arguments: "{}".to_string(),
    };

    let result = execute_tool_call(dir.path(), &call);

    assert!(matches!(result.status, ToolExecutionStatus::Error));
    assert!(result.policy.output.skipped_binary);
    assert!(result.content.is_none());
    Ok(())
}

#[cfg(unix)]
#[test]
fn symlink_paths_require_approval() -> Result<()> {
    let dir = TempDir::new()?;
    fs::write(dir.path().join("target.txt"), "ok\n")?;
    std::os::unix::fs::symlink(dir.path().join("target.txt"), dir.path().join("link.txt"))?;
    let call = ToolCall {
        id: "call-link".to_string(),
        name: "read_file".to_string(),
        arguments: json!({ "path": "link.txt" }),
        raw_arguments: "{}".to_string(),
    };

    let result = execute_tool_call(dir.path(), &call);

    assert!(matches!(
        result.status,
        ToolExecutionStatus::ApprovalRequired
    ));
    let path = result.policy.path.expect("path policy");
    assert!(path.symlink);
    assert!(!path.symlink_allowed);
    Ok(())
}

#[test]
fn shell_tool_runs_only_read_only_commands() {
    let dir = TempDir::new().expect("temp dir");
    let read_only = ToolCall {
        id: "call-shell".to_string(),
        name: "shell".to_string(),
        arguments: json!({ "command": "pwd" }),
        raw_arguments: "{}".to_string(),
    };
    let write = ToolCall {
        id: "call-write".to_string(),
        name: "shell".to_string(),
        arguments: json!({ "command": "touch x.txt" }),
        raw_arguments: "{}".to_string(),
    };

    let read_result = execute_tool_call(dir.path(), &read_only);
    let write_result = execute_tool_call(dir.path(), &write);

    assert!(matches!(read_result.status, ToolExecutionStatus::Ok));
    assert!(matches!(
        write_result.status,
        ToolExecutionStatus::ApprovalRequired
    ));
}

#[test]
fn shell_network_and_secret_paths_require_approval() {
    let dir = TempDir::new().expect("temp dir");
    let network = ToolCall {
        id: "call-network".to_string(),
        name: "shell".to_string(),
        arguments: json!({ "command": "curl https://example.com" }),
        raw_arguments: "{}".to_string(),
    };
    let secret = ToolCall {
        id: "call-secret".to_string(),
        name: "shell".to_string(),
        arguments: json!({ "command": "cat .env" }),
        raw_arguments: "{}".to_string(),
    };

    let network_result = execute_tool_call(dir.path(), &network);
    let secret_result = execute_tool_call(dir.path(), &secret);

    assert!(matches!(
        network_result.status,
        ToolExecutionStatus::ApprovalRequired
    ));
    assert!(network_result
        .policy
        .network
        .as_ref()
        .is_some_and(|network| !network.allowed));
    assert!(matches!(
        secret_result.status,
        ToolExecutionStatus::ApprovalRequired
    ));
    assert!(secret_result
        .policy
        .path
        .as_ref()
        .is_some_and(|path| path.protected));
}
