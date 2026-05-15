use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::adaptive::{self, AdaptiveDecision};
use crate::agent_adapter::AgentRoutes;
use crate::spec::AgentSpec;

use super::RunState;

pub(super) fn apply(spec: &mut AgentSpec, tx_dir: &Path) -> Result<AdaptiveDecision> {
    let decision = adaptive::apply(spec);
    fs::write(
        tx_dir.join("effective_plan.yaml"),
        serde_yaml::to_string(spec)?,
    )
    .with_context(|| format!("write {}", tx_dir.join("effective_plan.yaml").display()))?;
    Ok(decision)
}

pub(super) fn finish_decision(
    tx_dir: &Path,
    routes: &AgentRoutes,
    decision: &mut AdaptiveDecision,
) -> Result<Value> {
    decision.model = Some(
        routes
            .executor
            .model
            .clone()
            .unwrap_or_else(|| routes.executor.selected_adapter.clone()),
    );
    fs::write(
        tx_dir.join("adaptive.json"),
        serde_json::to_string_pretty(decision)?,
    )
    .with_context(|| format!("write {}", tx_dir.join("adaptive.json").display()))?;
    Ok(serde_json::json!(decision))
}

pub(super) fn record_scoreboard(
    project_root: &Path,
    tx_dir: &Path,
    state: &RunState,
    status: &str,
    started_at: DateTime<Utc>,
    finished_at: DateTime<Utc>,
) -> Result<()> {
    let Some(decision) = &state.adaptive else {
        return Ok(());
    };
    let latency_ms = (finished_at - started_at).num_milliseconds().max(0) as u64;
    adaptive::scoreboard::record(
        project_root,
        decision,
        status,
        state.cost_profile.as_ref(),
        latency_ms,
        tx_dir.join("review_repair.json").exists(),
    )?;
    Ok(())
}
