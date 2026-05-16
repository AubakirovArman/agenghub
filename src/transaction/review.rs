use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use serde_json::json;

use crate::agent_adapter::{self, AgentRoutes};
use crate::command_runner::RemoteRunner;
use crate::diff_guard::DiffGuardResult;
use crate::journal::Journal;
use crate::reviewer::{self, ReviewResult};
use crate::spec::AgentSpec;

use super::execution::run_repair_commands;
use super::guards::check_diff_guard;

pub(super) struct ReviewRepairContext<'a> {
    pub(super) spec: &'a AgentSpec,
    pub(super) worktree: &'a Path,
    pub(super) tx_dir: &'a Path,
    pub(super) journal: &'a Journal,
    pub(super) agent_routes: &'a AgentRoutes,
    pub(super) remote_runner: Option<&'a RemoteRunner>,
    pub(super) command_timeout: Duration,
    pub(super) diff_guard: DiffGuardResult,
}

pub(super) fn run_review_with_repair(
    ctx: ReviewRepairContext<'_>,
) -> Result<(ReviewResult, DiffGuardResult)> {
    let mut diff_guard = ctx.diff_guard;
    let mut review = run_review(
        ctx.spec,
        ctx.worktree,
        ctx.tx_dir,
        ctx.agent_routes,
        ctx.remote_runner,
        ctx.command_timeout,
    )?;
    let mut repair_results = Vec::new();

    for attempt in 1..=ctx.spec.transaction.max_repair_attempts {
        if review.passed || ctx.spec.repair.commands.is_empty() {
            break;
        }
        ctx.journal.append_data(
            "REPAIRING",
            "running reviewer repair commands",
            json!({ "attempt": attempt, "phase": "review" }),
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
        repair_results.push(json!({ "attempt": attempt, "phase": "review", "commands": results }));

        diff_guard = check_diff_guard(ctx.spec, ctx.worktree, ctx.tx_dir)?;
        if !diff_guard.passed {
            break;
        }
        review = run_review(
            ctx.spec,
            ctx.worktree,
            ctx.tx_dir,
            ctx.agent_routes,
            ctx.remote_runner,
            ctx.command_timeout,
        )?;
    }

    if !repair_results.is_empty() {
        fs::write(
            ctx.tx_dir.join("review_repair.json"),
            serde_json::to_string_pretty(&repair_results)?,
        )?;
    }
    Ok((review, diff_guard))
}

fn run_review(
    spec: &AgentSpec,
    worktree: &Path,
    tx_dir: &Path,
    agent_routes: &AgentRoutes,
    remote_runner: Option<&RemoteRunner>,
    command_timeout: Duration,
) -> Result<ReviewResult> {
    if let Some(route) = agent_routes.reviewer.as_ref() {
        agent_adapter::invoke_adapter(spec, tx_dir, worktree, route, remote_runner)?;
    }
    let review = reviewer::run(
        &spec.review,
        &spec.execution.sandbox,
        remote_runner,
        worktree,
        &tx_dir.join("reviewer.log"),
        command_timeout,
    )?;
    if let Some(route) = agent_routes.reviewer.as_ref() {
        agent_adapter::write_transcript(tx_dir, route, &review.commands)?;
    }
    Ok(review)
}
