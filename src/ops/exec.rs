use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::command_policy::{self, CommandPolicyCommand};
use crate::command_runner::{run_shell_with_sandbox_logged, CommandResult, CommandSandbox};
use crate::home;
use crate::tool_permissions::ToolPermissionDecision;

use super::{command_trust, record_explicit_command, OpsCommandReceipt, OpsHostTrust};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpsExecStatus {
    Completed,
    ApprovalRequired,
    Blocked,
}

impl OpsExecStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::ApprovalRequired => "approval_required",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsExecOutcome {
    pub status: OpsExecStatus,
    pub permission: ToolPermissionDecision,
    pub policy: CommandPolicyCommand,
    pub approval_reason: Option<String>,
    pub result: Option<CommandResult>,
    pub receipt: Option<OpsCommandReceipt>,
}

pub fn exec_command(root: &Path, command: &str) -> Result<OpsExecOutcome> {
    if command.trim().is_empty() {
        return Err(anyhow!("ops command is required"));
    }
    fs::create_dir_all(root).with_context(|| format!("create {}", root.display()))?;
    let permission = crate::tool_permissions::classify_shell_command(command);
    let policy = command_policy::classify_shell_command(root, command)?;
    let trust = command_trust(command).unwrap_or(OpsHostTrust::Unknown);
    let approval_reason = approval_reason(&permission, &policy, trust);
    if let Some(reason) = approval_reason {
        let status = if policy.classification == "restricted" {
            OpsExecStatus::Blocked
        } else {
            OpsExecStatus::ApprovalRequired
        };
        let receipt = record_explicit_command(root, command, &permission, None)?;
        return Ok(OpsExecOutcome {
            status,
            permission,
            policy,
            approval_reason: Some(reason),
            result: None,
            receipt,
        });
    }

    let logs = home::global_shell_commands_dir(root);
    let prefix = format!("ops-{}", Utc::now().format("%Y%m%d%H%M%S"));
    let result = run_shell_with_sandbox_logged(
        command,
        root,
        Duration::from_secs(900),
        CommandSandbox::default(),
        &logs,
        &prefix,
    )?;
    let receipt = record_explicit_command(root, command, &permission, Some(&result))?;
    Ok(OpsExecOutcome {
        status: OpsExecStatus::Completed,
        permission,
        policy,
        approval_reason: None,
        result: Some(result),
        receipt,
    })
}

fn approval_reason(
    decision: &ToolPermissionDecision,
    policy: &CommandPolicyCommand,
    trust: OpsHostTrust,
) -> Option<String> {
    if policy.classification == "restricted" {
        return Some(
            policy
                .matched_policy
                .as_deref()
                .unwrap_or("restricted command policy")
                .to_string(),
        );
    }
    if decision.approval_required {
        return Some(decision.reason.clone());
    }
    if policy.classification == "needs_approval" {
        return Some(
            policy
                .matched_policy
                .as_deref()
                .unwrap_or("matched command policy")
                .to_string(),
        );
    }
    if trust == OpsHostTrust::Untrusted {
        return Some("Ops target is marked untrusted".to_string());
    }
    None
}
