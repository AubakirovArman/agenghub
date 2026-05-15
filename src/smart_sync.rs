use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::git;
use crate::workspace::PreparedWorkspace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartSyncDecision {
    pub decision: String,
    pub base_head: String,
    pub current_head: String,
    pub tx_changed_files: Vec<String>,
    pub main_changed_files: Vec<String>,
    pub overlapping_files: Vec<String>,
    pub verifier_rerun_required: bool,
}

impl SmartSyncDecision {
    pub fn clean(prepared: &PreparedWorkspace, tx_changed_files: Vec<String>) -> Self {
        Self {
            decision: "clean".to_string(),
            base_head: prepared.base_head.clone(),
            current_head: prepared.base_head.clone(),
            tx_changed_files,
            main_changed_files: Vec::new(),
            overlapping_files: Vec::new(),
            verifier_rerun_required: false,
        }
    }
}

pub fn evaluate(
    project_root: &Path,
    prepared: &PreparedWorkspace,
    tx_changed_files: &[String],
) -> Result<SmartSyncDecision> {
    let current_head = git::head(project_root)?.unwrap_or_default();
    if current_head == prepared.base_head {
        return Ok(SmartSyncDecision::clean(
            prepared,
            tx_changed_files.to_vec(),
        ));
    }
    let main_changed_files =
        git::changed_files_between(project_root, &prepared.base_head, &current_head)?;
    let overlapping_files = overlap(tx_changed_files, &main_changed_files);
    let decision = if overlapping_files.is_empty() {
        "rebase_required"
    } else {
        "blocked_overlap"
    };
    Ok(SmartSyncDecision {
        decision: decision.to_string(),
        base_head: prepared.base_head.clone(),
        current_head,
        tx_changed_files: tx_changed_files.to_vec(),
        main_changed_files,
        overlapping_files,
        verifier_rerun_required: decision == "rebase_required",
    })
}

fn overlap(left: &[String], right: &[String]) -> Vec<String> {
    let right = right.iter().collect::<BTreeSet<_>>();
    left.iter()
        .filter(|file| right.contains(file))
        .cloned()
        .collect()
}
