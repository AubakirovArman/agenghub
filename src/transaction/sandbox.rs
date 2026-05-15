use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::journal::Journal;
use crate::sandbox as sandbox_policy;
use crate::spec::AgentSpec;

use super::{RunState, TransactionStatus};

pub(super) fn enforce(
    project_root: &Path,
    spec: &AgentSpec,
    tx_dir: &Path,
    journal: &Journal,
    state: &mut RunState,
) -> Result<()> {
    let report = sandbox_policy::evaluate(project_root, spec)?;
    state.remote_runner = report.runner.clone();
    fs::write(
        tx_dir.join("sandbox.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    journal.append_data(
        "SANDBOX",
        "evaluated execution sandbox",
        serde_json::to_value(&report)?,
    )?;
    if let Err(error) = report.enforce() {
        state.status = Some(TransactionStatus::BlockedOnHuman);
        return Err(error);
    }
    Ok(())
}
