use std::fs;
use std::path::Path;
use std::time::Duration;

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
    let log_path = ctx.tx_dir.join("verifier.log");
    let verifier = run_verifier_with_repair(VerifierRepairContext {
        spec: ctx.spec,
        worktree: ctx.worktree,
        tx_dir: ctx.tx_dir,
        journal: ctx.journal,
        agent_routes: ctx.agent_routes,
        remote_runner: state.remote_runner.as_ref(),
        command_timeout: state.command_timeout(),
        log_path: &log_path,
    })?;
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

struct VerifierRepairContext<'a> {
    spec: &'a AgentSpec,
    worktree: &'a Path,
    tx_dir: &'a Path,
    journal: &'a Journal,
    agent_routes: &'a AgentRoutes,
    remote_runner: Option<&'a RemoteRunner>,
    command_timeout: Duration,
    log_path: &'a Path,
}

fn run_verifier_with_repair(ctx: VerifierRepairContext<'_>) -> Result<VerifierResult> {
    let mut verifier = verifier::run(
        &ctx.spec.verify,
        &ctx.spec.execution.sandbox,
        ctx.remote_runner,
        ctx.worktree,
        ctx.log_path,
        ctx.command_timeout,
    )?;
    let mut repair_results = Vec::new();

    for attempt in 1..=ctx.spec.transaction.max_repair_attempts {
        if verifier.passed || ctx.spec.repair.commands.is_empty() {
            break;
        }
        ctx.journal.append_data(
            "REPAIRING",
            "running repair commands",
            json!({ "attempt": attempt }),
        )?;
        if let Some(route) = ctx.agent_routes.repair.as_ref() {
            agent_adapter::invoke_adapter(
                ctx.spec,
                ctx.tx_dir,
                ctx.worktree,
                route,
                ctx.remote_runner,
            )?;
        }
        let results = run_repair_commands(
            ctx.spec,
            ctx.tx_dir,
            ctx.worktree,
            ctx.remote_runner,
            ctx.command_timeout,
        )?;
        if let Some(route) = ctx.agent_routes.repair.as_ref() {
            agent_adapter::write_transcript(ctx.tx_dir, route, &results)?;
        }
        repair_results.push(json!({ "attempt": attempt, "commands": results }));
        verifier = verifier::run(
            &ctx.spec.verify,
            &ctx.spec.execution.sandbox,
            ctx.remote_runner,
            ctx.worktree,
            ctx.log_path,
            ctx.command_timeout,
        )?;
    }

    if !repair_results.is_empty() {
        fs::write(
            ctx.tx_dir.join("repair.json"),
            serde_json::to_string_pretty(&repair_results)?,
        )?;
    }
    Ok(verifier)
}
