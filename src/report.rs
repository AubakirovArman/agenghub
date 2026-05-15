mod markdown;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::adaptive::AdaptiveDecision;
use crate::command_runner::RunnerMetadata;
use crate::diff_guard::DiffGuardResult;
use crate::observability::CostProfile;
use crate::reviewer::ReviewResult;
use crate::smart_sync::SmartSyncDecision;
use crate::verifier::VerifierResult;
use crate::workspace::WorkspaceRuntimeMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReport {
    pub tx_id: String,
    pub task_id: String,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub base_head: Option<String>,
    pub committed: bool,
    pub report_path: PathBuf,
    pub diff_guard: Option<DiffGuardResult>,
    pub review: Option<ReviewResult>,
    pub verifier: Option<VerifierResult>,
    pub sync: Option<SmartSyncDecision>,
    pub workspace_runtime: Option<WorkspaceRuntimeMetadata>,
    pub runner: Option<RunnerMetadata>,
    pub cost_profile: Option<CostProfile>,
    pub adaptive: Option<AdaptiveDecision>,
    pub error_fingerprint: Option<String>,
    pub failure_reason: Option<String>,
}

impl TransactionReport {
    pub fn write_markdown(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        }
        fs::write(path, self.to_markdown()).with_context(|| format!("write {}", path.display()))
    }

    pub fn to_markdown(&self) -> String {
        markdown::render(self)
    }
}
