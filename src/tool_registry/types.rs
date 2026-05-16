use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tool_permissions::ToolPermissionDecision;

pub const MAX_TOOL_ROUNDS: usize = 3;
pub(super) const MAX_READ_BYTES: u64 = 64 * 1024;
pub(super) const MAX_LIST_ENTRIES: usize = 200;
pub(super) const MAX_SEARCH_FILES: usize = 500;
pub(super) const MAX_SEARCH_MATCHES: usize = 80;
pub(super) const SHELL_TIMEOUT_SECS: u64 = 30;
pub(super) const MAX_RESULT_CHARS: usize = 16_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionStatus {
    Ok,
    Error,
    ApprovalRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPolicyLimits {
    pub max_tool_rounds: usize,
    pub max_read_bytes: u64,
    pub max_list_entries: usize,
    pub max_search_files: usize,
    pub max_search_matches: usize,
    pub max_result_chars: usize,
    pub shell_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPathPolicy {
    pub requested: String,
    pub resolved: Option<String>,
    pub target_kind: Option<String>,
    pub within_workspace: bool,
    pub protected: bool,
    pub symlink: bool,
    pub symlink_allowed: bool,
    pub decision: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutputPolicy {
    pub max_chars: usize,
    pub max_bytes: Option<u64>,
    pub bytes_read: Option<u64>,
    pub chars_returned: Option<usize>,
    pub truncated: bool,
    pub skipped_binary: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolNetworkPolicy {
    pub allowed: bool,
    pub decision: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPolicyDecision {
    pub version: String,
    pub approval_threshold: String,
    pub limits: ToolPolicyLimits,
    pub path: Option<ToolPathPolicy>,
    pub output: ToolOutputPolicy,
    pub network: Option<ToolNetworkPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    pub call_id: String,
    pub name: String,
    pub status: ToolExecutionStatus,
    pub permission: ToolPermissionDecision,
    pub policy: ToolPolicyDecision,
    pub content: Option<Value>,
    pub error: Option<String>,
}

pub(super) struct ToolRegistryOutcome {
    pub(super) status: ToolExecutionStatus,
    pub(super) policy: ToolPolicyDecision,
    pub(super) content: Option<Value>,
    pub(super) error: Option<String>,
}

impl ToolPolicyDecision {
    pub(super) fn new() -> Self {
        Self {
            version: "tool-registry-policy.v1".to_string(),
            approval_threshold:
                "auto-execute only builtin read-only tools; protected, network, symlink, and mutating actions require approval"
                    .to_string(),
            limits: ToolPolicyLimits {
                max_tool_rounds: MAX_TOOL_ROUNDS,
                max_read_bytes: MAX_READ_BYTES,
                max_list_entries: MAX_LIST_ENTRIES,
                max_search_files: MAX_SEARCH_FILES,
                max_search_matches: MAX_SEARCH_MATCHES,
                max_result_chars: MAX_RESULT_CHARS,
                shell_timeout_secs: SHELL_TIMEOUT_SECS,
            },
            path: None,
            output: ToolOutputPolicy {
                max_chars: MAX_RESULT_CHARS,
                max_bytes: Some(MAX_READ_BYTES),
                bytes_read: None,
                chars_returned: None,
                truncated: false,
                skipped_binary: false,
                reason: None,
            },
            network: None,
        }
    }
}

pub(super) fn outcome(
    status: ToolExecutionStatus,
    policy: ToolPolicyDecision,
    content: Option<Value>,
    error: Option<String>,
) -> ToolRegistryOutcome {
    ToolRegistryOutcome {
        status,
        policy,
        content,
        error,
    }
}

pub(super) fn ok_outcome(policy: ToolPolicyDecision, content: Value) -> ToolRegistryOutcome {
    outcome(ToolExecutionStatus::Ok, policy, Some(content), None)
}

pub(super) fn error_outcome(
    policy: ToolPolicyDecision,
    error: impl ToString,
) -> ToolRegistryOutcome {
    outcome(
        ToolExecutionStatus::Error,
        policy,
        None,
        Some(error.to_string()),
    )
}
