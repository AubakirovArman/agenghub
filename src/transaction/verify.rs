use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
use serde_json::json;

use crate::agent_adapter::{self, AgentRoutes};
use crate::command_runner::RemoteRunner;
use crate::effects::EffectLedger;
use crate::journal::Journal;
use crate::spec::AgentSpec;
use crate::verifier::{self, VerifierResult};

use super::execution::run_repair_commands;
use super::guards::maybe_fail_at;
use super::{RunState, TransactionStatus};

pub(super) struct VerifyContext<'a> {
    pub(super) project_root: &'a Path,
    pub(super) spec: &'a AgentSpec,
    pub(super) tx_id: &'a str,
    pub(super) tx_dir: &'a Path,
    pub(super) journal: &'a Journal,
    pub(super) agent_routes: &'a AgentRoutes,
    pub(super) worktree: &'a Path,
}

pub(super) fn verify_transaction(ctx: VerifyContext<'_>, state: &mut RunState) -> Result<()> {
    ctx.journal
        .append("VERIFYING", "running verifier commands")?;
    maybe_fail_at("VERIFYING", ctx.tx_dir, ctx.journal)?;
    let verifier = run_verifier_with_repair(
        ctx.spec,
        ctx.worktree,
        ctx.tx_dir,
        ctx.journal,
        ctx.agent_routes,
        state.remote_runner.as_ref(),
        &ctx.tx_dir.join("verifier.log"),
    )?;
    fs::write(
        ctx.tx_dir.join("verifier.json"),
        serde_json::to_string_pretty(&verifier)?,
    )?;
    let integration = verifier::build_integration_artifact(ctx.project_root, &verifier)?;
    fs::write(
        ctx.tx_dir.join("verifier_integration.json"),
        serde_json::to_string_pretty(&integration)?,
    )?;
    if !verifier.passed {
        if verifier::detects_missing_env(&verifier) {
            state.status = Some(TransactionStatus::BlockedOnHuman);
            state.verifier = Some(verifier);
            state.verifier_integration = Some(integration);
            return Err(anyhow!(
                "verifier failed because required environment appears to be missing"
            ));
        }
        let fingerprint = integration
            .fingerprints
            .first()
            .map(|item| item.fingerprint.clone())
            .unwrap_or_else(|| "none".to_string());
        state.verifier = Some(verifier);
        state.verifier_integration = Some(integration);
        state.failure_reason = Some(format!(
            "verifier failed in transaction {}; fingerprint {fingerprint}",
            ctx.tx_id
        ));
        return Err(anyhow!(
            "verifier failed in transaction {}; fingerprint {fingerprint}",
            ctx.tx_id
        ));
    }
    if let Some(diff_guard) = &state.diff_guard {
        EffectLedger::for_tx_dir(ctx.tx_dir)
            .record_verified_files("verifier", &diff_guard.summary.changed_files)?;
    }
    state.verifier = Some(verifier);
    state.verifier_integration = Some(integration);
    Ok(())
}

pub(super) fn run_verifier_with_repair(
    spec: &AgentSpec,
    worktree: &Path,
    tx_dir: &Path,
    journal: &Journal,
    agent_routes: &AgentRoutes,
    remote_runner: Option<&RemoteRunner>,
    log_path: &Path,
) -> Result<VerifierResult> {
    let mut verifier = verifier::run(
        &spec.verify,
        &spec.execution.sandbox,
        remote_runner,
        worktree,
        log_path,
    )?;
    let mut repair_results = Vec::new();

    for attempt in 1..=spec.transaction.max_repair_attempts {
        if verifier.passed || spec.repair.commands.is_empty() {
            break;
        }
        journal.append_data(
            "REPAIRING",
            "running repair commands",
            json!({ "attempt": attempt }),
        )?;
        if let Some(route) = agent_routes.repair.as_ref() {
            agent_adapter::invoke_adapter(spec, tx_dir, worktree, route, remote_runner)?;
        }
        let results = run_repair_commands(spec, tx_dir, worktree, remote_runner)?;
        if let Some(route) = agent_routes.repair.as_ref() {
            agent_adapter::write_transcript(tx_dir, route, &results)?;
        }
        repair_results.push(json!({ "attempt": attempt, "commands": results }));
        verifier = verifier::run(
            &spec.verify,
            &spec.execution.sandbox,
            remote_runner,
            worktree,
            log_path,
        )?;
    }

    if !repair_results.is_empty() {
        fs::write(
            tx_dir.join("repair.json"),
            serde_json::to_string_pretty(&repair_results)?,
        )?;
    }
    Ok(verifier)
}
