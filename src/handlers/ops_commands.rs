use std::path::Path;

use anyhow::{anyhow, Result};
use serde_json::json;

use agenthub::{enterprise, ops};

use crate::cli::{OpsCommands, OpsHostCommands, OpsRunbookCommands};

pub fn handle_ops(project_root: &Path, command: OpsCommands) -> Result<()> {
    enterprise::authorize(project_root, "memory.read")?;
    match command {
        OpsCommands::Exec { command, jsonl } => exec(project_root, &command, jsonl)?,
        OpsCommands::Hosts { command } => handle_hosts(project_root, command)?,
        OpsCommands::Runbooks { command } => handle_runbooks(project_root, command)?,
        OpsCommands::Receipts { host, limit } => {
            print_receipts(project_root, host.as_deref(), limit)?
        }
    }
    Ok(())
}

fn exec(project_root: &Path, command: &str, jsonl: bool) -> Result<()> {
    enterprise::authorize(project_root, "transaction.run")?;
    let outcome = ops::exec_command(project_root, command)?;
    if jsonl {
        println!(
            "{}",
            serde_json::to_string(&json!({
                "kind": "tool_permission",
                "tool": outcome.permission.tool,
                "action": outcome.permission.action,
                "profile": outcome.permission.profile.as_str(),
                "risk": outcome.permission.risk.as_str(),
                "approval_required": outcome.permission.approval_required,
                "reason": outcome.permission.reason,
            }))?
        );
        println!(
            "{}",
            serde_json::to_string(&json!({
                "kind": "ops_command_receipt",
                "status": outcome.status.as_str(),
                "approval_required": outcome.status == ops::OpsExecStatus::ApprovalRequired,
                "receipt": outcome.receipt,
            }))?
        );
        if let Some(result) = &outcome.result {
            println!(
                "{}",
                serde_json::to_string(&json!({
                    "kind": "tool_finished",
                    "tool": "shell",
                    "status": if result.success { "succeeded" } else { "failed" },
                    "exit_code": result.exit_code,
                    "duration_ms": result.duration_ms,
                    "stdout_log": result.stdout_path,
                    "stderr_log": result.stderr_path,
                    "stdout_tail": result.stdout_tail,
                    "stderr_tail": result.stderr_tail,
                }))?
            );
        }
    } else {
        println!(
            "tool_permission\tprofile:{}\trisk:{}\tapproval:{}",
            outcome.permission.profile.as_str(),
            outcome.permission.risk.as_str(),
            outcome.permission.approval_required
        );
        if let Some(receipt) = &outcome.receipt {
            println!(
                "ops_receipt\t{}\t{}\ttrust:{}\tsuccess:{}",
                receipt.id,
                receipt.target,
                receipt.trust.as_str(),
                receipt
                    .success
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "n/a".to_string())
            );
        }
        if let Some(result) = &outcome.result {
            println!(
                "command\t{}\texit:{:?}\tduration_ms:{}",
                if result.success {
                    "succeeded"
                } else {
                    "failed"
                },
                result.exit_code,
                result.duration_ms
            );
            if !result.stdout_tail.trim().is_empty() {
                println!("stdout:\n{}", result.stdout_tail.trim());
            }
            if !result.stderr_tail.trim().is_empty() {
                println!("stderr:\n{}", result.stderr_tail.trim());
            }
        }
    }

    match outcome.status {
        ops::OpsExecStatus::Completed => Ok(()),
        ops::OpsExecStatus::ApprovalRequired => Err(anyhow!(
            "approval required for ops command: {}",
            outcome
                .approval_reason
                .unwrap_or_else(|| "policy".to_string())
        )),
        ops::OpsExecStatus::Blocked => Err(anyhow!(
            "blocked ops command: {}",
            outcome
                .approval_reason
                .unwrap_or_else(|| "policy".to_string())
        )),
    }
}

fn handle_hosts(project_root: &Path, command: Option<OpsHostCommands>) -> Result<()> {
    match command.unwrap_or(OpsHostCommands::List) {
        OpsHostCommands::List => print_hosts(project_root),
        OpsHostCommands::Add {
            target,
            alias,
            trust,
            note,
        } => {
            let profile = ops::upsert_host(
                project_root,
                ops::OpsHostInput {
                    target,
                    alias,
                    trust: ops::OpsHostTrust::parse(&trust)?,
                    note,
                    source: "cli".to_string(),
                },
            )?;
            println!(
                "host\t{}\t{}\ttrust:{}\tcommands:{}",
                profile.id,
                profile.target,
                profile.trust.as_str(),
                profile.command_count
            );
            Ok(())
        }
        OpsHostCommands::Trust { target, trust } => {
            let profile = ops::upsert_host(
                project_root,
                ops::OpsHostInput {
                    target,
                    alias: None,
                    trust: ops::OpsHostTrust::parse(&trust)?,
                    note: None,
                    source: "cli".to_string(),
                },
            )?;
            println!("host\t{}\ttrust:{}", profile.target, profile.trust.as_str());
            Ok(())
        }
    }
}

fn handle_runbooks(project_root: &Path, command: Option<OpsRunbookCommands>) -> Result<()> {
    match command.unwrap_or(OpsRunbookCommands::List { host: None }) {
        OpsRunbookCommands::List { host } => print_runbooks(project_root, host.as_deref()),
        OpsRunbookCommands::Add {
            title,
            host,
            command,
            note,
        } => {
            let card = ops::add_runbook_card(
                project_root,
                ops::OpsRunbookInput {
                    title,
                    host,
                    command,
                    note,
                },
            )?;
            println!(
                "runbook\t{}\t{}\t{}",
                card.id,
                card.host.as_deref().unwrap_or("<any-host>"),
                card.title
            );
            Ok(())
        }
    }
}

fn print_hosts(project_root: &Path) -> Result<()> {
    println!("Ops Hosts");
    for host in ops::list_hosts(project_root)? {
        println!(
            "{}\t{}\ttrust:{}\talias:{}\tcommands:{}\tlast_seen:{}",
            host.id,
            host.target,
            host.trust.as_str(),
            host.alias.as_deref().unwrap_or(""),
            host.command_count,
            host.last_seen_at
                .map(|value| value.to_rfc3339())
                .unwrap_or_else(|| "".to_string())
        );
    }
    Ok(())
}

fn print_runbooks(project_root: &Path, host: Option<&str>) -> Result<()> {
    println!("Ops Runbooks");
    for card in ops::list_runbook_cards(project_root, host)? {
        println!(
            "{}\t{}\tconfidence:{}\tcommand:{}\t{}",
            card.id,
            card.host.as_deref().unwrap_or("<any-host>"),
            card.confidence
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "n/a".to_string()),
            card.command.as_deref().unwrap_or(""),
            card.title
        );
    }
    Ok(())
}

fn print_receipts(project_root: &Path, host: Option<&str>, limit: usize) -> Result<()> {
    println!("Ops Receipts");
    for receipt in ops::list_receipts(project_root, limit, host)? {
        println!(
            "{}\t{}\ttrust:{}\trisk:{}\tapproval:{}\tsuccess:{}\t{}",
            receipt.id,
            receipt.target,
            receipt.trust.as_str(),
            receipt.risk,
            receipt.approval_required,
            receipt
                .success
                .map(|value| value.to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            receipt.command
        );
    }
    Ok(())
}
