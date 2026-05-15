use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::adaptive::{AdaptiveDecision, TaskClass};
use crate::observability::CostProfile;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrchestrationScoreboard {
    pub entries: Vec<ScoreboardEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreboardEntry {
    pub task_class: TaskClass,
    pub topology: String,
    pub model: String,
    pub runs: u64,
    pub success: u64,
    pub repair: u64,
    pub rollback: u64,
    pub human_block: u64,
    pub total_cost_usd: f64,
    pub total_latency_ms: u64,
}

pub fn record(
    project_root: &Path,
    decision: &AdaptiveDecision,
    status: &str,
    cost: Option<&CostProfile>,
    latency_ms: u64,
    repaired: bool,
) -> Result<PathBuf> {
    let path = path(project_root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let mut board = read(&path)?;
    let entry = find_or_insert(&mut board, decision);
    entry.runs += 1;
    entry.success += u64::from(status == "COMMITTED");
    entry.rollback += u64::from(status == "ROLLED_BACK");
    entry.human_block += u64::from(status == "BLOCKED_ON_HUMAN");
    entry.repair += u64::from(repaired);
    entry.total_cost_usd += cost.map(|item| item.total_usd).unwrap_or_default();
    entry.total_latency_ms += latency_ms;
    fs::write(&path, serde_json::to_string_pretty(&board)?)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(path)
}

fn read(path: &Path) -> Result<OrchestrationScoreboard> {
    if !path.exists() {
        return Ok(OrchestrationScoreboard::default());
    }
    let content = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("parse {}", path.display()))
}

fn find_or_insert<'a>(
    board: &'a mut OrchestrationScoreboard,
    decision: &AdaptiveDecision,
) -> &'a mut ScoreboardEntry {
    let model = decision
        .model
        .clone()
        .unwrap_or_else(|| "command".to_string());
    if let Some(index) = board.entries.iter().position(|entry| {
        entry.task_class == decision.task_class
            && entry.topology == decision.selected_topology
            && entry.model == model
    }) {
        return &mut board.entries[index];
    }
    board.entries.push(ScoreboardEntry {
        task_class: decision.task_class,
        topology: decision.selected_topology.clone(),
        model,
        runs: 0,
        success: 0,
        repair: 0,
        rollback: 0,
        human_block: 0,
        total_cost_usd: 0.0,
        total_latency_ms: 0,
    });
    board.entries.last_mut().expect("entry just inserted")
}

fn path(project_root: &Path) -> PathBuf {
    project_root
        .join(crate::agent_dir::AGENT_DIR)
        .join("metrics")
        .join("orchestration_scoreboard.json")
}
