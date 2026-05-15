use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::command_policy::{self, CommandPolicyError};
use crate::journal::Journal;
use crate::spec::AgentSpec;

use super::{RunState, TransactionStatus};

pub(super) fn enforce(
    project_root: &Path,
    spec: &AgentSpec,
    tx_dir: &Path,
    journal: &Journal,
    state: &mut RunState,
) -> Result<()> {
    let report = command_policy::evaluate(project_root, spec)?;
    fs::write(
        tx_dir.join("command_policy.json"),
        serde_json::to_string_pretty(&report)?,
    )?;
    journal.append_data(
        "COMMAND_POLICY",
        "evaluated command policy",
        serde_json::to_value(&report)?,
    )?;
    if let Err(error) = report.enforce() {
        if error
            .downcast_ref::<CommandPolicyError>()
            .is_some_and(CommandPolicyError::requires_human)
        {
            state.status = Some(TransactionStatus::BlockedOnHuman);
        }
        return Err(error);
    }
    Ok(())
}
