use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::agent_dir::{self, AgentPaths};
use crate::effects::EffectLedger;
use crate::git;
use crate::journal::Journal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoReport {
    pub tx_id: String,
    pub reverted_commit: String,
    pub revert_head: String,
    pub created_at: DateTime<Utc>,
}

pub fn undo(root: &Path, target: &str) -> Result<UndoReport> {
    let tx_id = resolve_target(root, target)?;
    ensure_committed(root, &tx_id)?;
    let blockers = git::dirty_blockers(root)?;
    if !blockers.is_empty() {
        return Err(anyhow!(
            "cannot undo with uncommitted changes: {}",
            blockers.join(", ")
        ));
    }
    let needle = format!("AgentHub {tx_id}:");
    let commit = git::find_commit_by_subject(root, &needle)?
        .ok_or_else(|| anyhow!("could not find commit for transaction {tx_id}"))?;
    git::revert_no_edit(root, &commit)?;
    let revert_head = git::head(root)?.unwrap_or_else(|| "<none>".to_string());
    let report = UndoReport {
        tx_id: tx_id.clone(),
        reverted_commit: commit,
        revert_head,
        created_at: Utc::now(),
    };
    write_report(root, &tx_id, &report)?;
    Ok(report)
}

fn resolve_target(root: &Path, target: &str) -> Result<String> {
    if target == "last" || target == "latest" {
        return latest_committed(root);
    }
    Ok(target.to_string())
}

fn latest_committed(root: &Path) -> Result<String> {
    let mut rows = agent_dir::list_transactions(root)?;
    rows.reverse();
    rows.into_iter()
        .find(|row| report_status(&row.report_path).as_deref() == Some("COMMITTED"))
        .map(|row| row.id)
        .ok_or_else(|| anyhow!("no committed transaction found"))
}

fn ensure_committed(root: &Path, tx_id: &str) -> Result<()> {
    let report = AgentPaths::new(root).tx_dir(tx_id).join("report.md");
    if report_status(&report).as_deref() == Some("COMMITTED") {
        return Ok(());
    }
    Err(anyhow!("transaction is not committed: {tx_id}"))
}

fn report_status(path: &Path) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    text.lines()
        .find_map(|line| line.strip_prefix("- Status: `"))
        .and_then(|rest| rest.split('`').next())
        .map(str::to_string)
}

fn write_report(root: &Path, tx_id: &str, report: &UndoReport) -> Result<()> {
    let tx_dir = AgentPaths::new(root).tx_dir(tx_id);
    fs::write(
        tx_dir.join("undo.json"),
        serde_json::to_string_pretty(report)?,
    )
    .with_context(|| format!("write undo report for {tx_id}"))?;
    Journal::new(tx_id, tx_dir.join("journal.jsonl")).append_data(
        "UNDO_REVERTED",
        "transaction reverted",
        json!(report),
    )?;
    EffectLedger::for_tx_dir(&tx_dir).record_control("undo", "verified", json!(report))
}
