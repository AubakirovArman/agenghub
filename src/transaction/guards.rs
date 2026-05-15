use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Result};
use serde_json::json;

use crate::diff_guard::{self, DiffGuardResult};
use crate::journal::Journal;
use crate::spec::AgentSpec;

pub(super) fn check_diff_guard(
    spec: &AgentSpec,
    worktree: &Path,
    tx_dir: &Path,
) -> Result<DiffGuardResult> {
    let diff_guard = diff_guard::check(worktree, &spec.scope, &spec.transaction.diff_limits)?;
    fs::write(
        tx_dir.join("diff_guard.json"),
        serde_json::to_string_pretty(&diff_guard)?,
    )?;
    Ok(diff_guard)
}

pub(super) fn maybe_fail_at(stage: &str, tx_dir: &Path, journal: &Journal) -> Result<()> {
    if !enabled() || std::env::var("AGENTHUB_FAIL_AT").ok().as_deref() != Some(stage) {
        return Ok(());
    }
    let record = json!({
        "stage": stage,
        "source": "AGENTHUB_FAIL_AT",
        "enabled_by": "AGENTHUB_FAULT_INJECTION",
    });
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(tx_dir.join("fault_injection.jsonl"))?;
    writeln!(file, "{}", serde_json::to_string(&record)?)?;
    journal.append_data("FAULT_INJECTION", "controlled test fault injected", record)?;
    Err(anyhow!("injected fault at {stage}"))
}

fn enabled() -> bool {
    std::env::var("AGENTHUB_FAULT_INJECTION").ok().as_deref() == Some("1")
}
