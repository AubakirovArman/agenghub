use std::path::Path;

use anyhow::{anyhow, Result};
use serde_json::json;

use crate::agent_adapter::AgentRoutes;
use crate::git;
use crate::journal::Journal;
use crate::memory;
use crate::smart_sync::{self, SmartSyncDecision};
use crate::spec::AgentSpec;
use crate::workspace;

use super::guards::{check_diff_guard, maybe_fail_at};
use super::verify::{verify_transaction, VerifyContext};
use super::{RunState, TransactionStatus};

pub(super) struct CommitContext<'a> {
    pub(super) project_root: &'a Path,
    pub(super) spec: &'a AgentSpec,
    pub(super) tx_id: &'a str,
    pub(super) tx_dir: &'a Path,
    pub(super) journal: &'a Journal,
    pub(super) agent_routes: &'a AgentRoutes,
    pub(super) no_commit: bool,
}

pub(super) fn sync_and_commit(ctx: CommitContext<'_>, state: &mut RunState) -> Result<()> {
    let prepared = state
        .prepared
        .as_ref()
        .expect("prepared workspace exists")
        .clone();
    maybe_fail_at("BEFORE_COMMIT", ctx.tx_dir, ctx.journal)?;
    ctx.journal
        .append("SYNC_CHECK", "checking project HEAD and file overlap")?;
    let sync = evaluate_sync(ctx.project_root, &prepared, state)?;
    std::fs::write(
        ctx.tx_dir.join("sync.json"),
        serde_json::to_string_pretty(&sync)?,
    )?;
    ctx.journal
        .append_data("SYNC_DECISION", "smart sync decision", json!(&sync))?;
    state.sync = Some(sync.clone());
    if sync.decision == "blocked_overlap" {
        state.status = Some(TransactionStatus::BlockedOnHuman);
        return Err(anyhow!("sync check blocked on overlapping files"));
    }
    if sync.decision == "rebase_required" {
        git::rebase_onto(&prepared.worktree_path, &sync.current_head)?;
        ctx.journal
            .append_data("SYNC_REBASED", "rebased transaction worktree", json!(&sync))?;
        rerun_guards_and_verifier(&ctx, state)?;
    }
    if ctx.no_commit || !ctx.spec.transaction.commit_on_success {
        state.status = Some(TransactionStatus::Noop);
        return ctx
            .journal
            .append("CLOSED", "transaction passed without committing");
    }
    ctx.journal.append(
        "COMMITTING",
        "committing and fast-forward merging transaction branch",
    )?;
    maybe_fail_at("COMMITTING", ctx.tx_dir, ctx.journal)?;
    let runtime = workspace::runtime_for_prepared(&prepared);
    state.committed = runtime
        .commit(
            &prepared,
            &format!("AgentHub {}: {}", ctx.tx_id, ctx.spec.task.id),
        )?
        .committed;
    if ctx.spec.transaction.memory_promotion == "on_success" {
        if let Err(error) = maybe_fail_at("MEMORY_PROMOTION", ctx.tx_dir, ctx.journal)
            .and_then(|_| memory::promote_staging(ctx.project_root, ctx.tx_dir))
        {
            record_post_commit_warning(&ctx, state, "MEMORY_PROMOTION_FAILED", error)?;
        }
    }
    if let Err(error) =
        maybe_fail_at("AUTO_MEMORY_EXTRACTION", ctx.tx_dir, ctx.journal).and_then(|_| {
            let receipt = extract_project_memory_candidate(&ctx, state)?;
            ctx.journal.append_data(
                "AUTO_MEMORY_EXTRACTED",
                "auto memory candidates written to inbox",
                json!(&receipt),
            )
        })
    {
        record_post_commit_warning(&ctx, state, "AUTO_MEMORY_EXTRACTION_FAILED", error)?;
    }
    if let Err(error) =
        maybe_fail_at("CLEANUP", ctx.tx_dir, ctx.journal).and_then(|_| runtime.cleanup(&prepared))
    {
        record_post_commit_warning(&ctx, state, "CLEANUP_FAILED", error)?;
    }
    state.status = Some(TransactionStatus::Committed);
    ctx.journal.append("COMMITTED", "transaction committed")
}

fn extract_project_memory_candidate(
    ctx: &CommitContext<'_>,
    state: &RunState,
) -> Result<memory::AutoMemoryExtractionReceipt> {
    let changed_files = state
        .diff_guard
        .as_ref()
        .map(|guard| guard.summary.changed_files.clone())
        .unwrap_or_default();
    let profile = ctx.spec.workspace.profile()?;
    let title = ctx.spec.task.title.as_deref().unwrap_or(&ctx.spec.task.id);
    memory::extract_to_inbox(
        ctx.project_root,
        memory::AutoMemoryExtractionInput {
            source: "project_transaction".to_string(),
            mode: "project".to_string(),
            domain: profile.domain().to_string(),
            request: Some(format!("{}: {title}", ctx.spec.task.kind)),
            response: Some(format!(
                "Transaction {} committed with {} changed file(s).",
                ctx.tx_id,
                changed_files.len()
            )),
            task_id: Some(ctx.spec.task.id.clone()),
            artifacts: changed_files,
        },
    )
}

fn record_post_commit_warning(
    ctx: &CommitContext<'_>,
    state: &mut RunState,
    event: &str,
    error: anyhow::Error,
) -> Result<()> {
    let message = format!("{event}: {error}");
    state.failure_reason = Some(message.clone());
    ctx.journal.append_data(
        event,
        "post-commit operation failed",
        json!({ "error": message }),
    )
}

fn evaluate_sync(
    project_root: &Path,
    prepared: &crate::workspace::PreparedWorkspace,
    state: &RunState,
) -> Result<SmartSyncDecision> {
    let files = state
        .diff_guard
        .as_ref()
        .map(|guard| guard.summary.changed_files.clone())
        .unwrap_or_default();
    smart_sync::evaluate(project_root, prepared, &files)
}

fn rerun_guards_and_verifier(ctx: &CommitContext<'_>, state: &mut RunState) -> Result<()> {
    let prepared = state
        .prepared
        .as_ref()
        .expect("prepared workspace exists")
        .clone();
    let diff_guard = check_diff_guard(ctx.spec, &prepared.worktree_path, ctx.tx_dir)?;
    if !diff_guard.passed {
        state.diff_guard = Some(diff_guard);
        return Err(anyhow!("diff guard failed after smart sync rebase"));
    }
    state.diff_guard = Some(diff_guard);
    verify_transaction(
        VerifyContext {
            project_root: ctx.project_root,
            spec: ctx.spec,
            tx_id: ctx.tx_id,
            tx_dir: ctx.tx_dir,
            journal: ctx.journal,
            agent_routes: ctx.agent_routes,
            worktree: &prepared.worktree_path,
        },
        state,
    )
    .map_err(|_| anyhow!("verifier failed after smart sync rebase"))
}
