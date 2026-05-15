#[cfg(test)]
mod tests;

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackReport {
    pub tx_id: String,
    pub created_at: DateTime<Utc>,
    pub effects: Vec<RollbackEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackEffect {
    pub path: String,
    pub handler: String,
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct HandlerSpec {
    pub name: &'static str,
    pub reason: Option<&'static str>,
}

pub fn handler_for_path(path: &str) -> HandlerSpec {
    if is_package_manifest(path) {
        return HandlerSpec {
            name: "package_manifest_restore",
            reason: Some("package manifests and lockfiles must be restored together"),
        };
    }
    if is_terraform_state(path) {
        return HandlerSpec {
            name: "terraform_state_restore",
            reason: Some("terraform state needs a state-aware restore path"),
        };
    }
    if is_environment_file(path) {
        return HandlerSpec {
            name: "manual_approval_required",
            reason: Some("environment and secret files require human review"),
        };
    }
    HandlerSpec {
        name: "file_restore",
        reason: None,
    }
}

pub fn write_report(
    tx_dir: &Path,
    tx_id: &str,
    files: &[String],
    status: &str,
) -> Result<RollbackReport> {
    let report = RollbackReport {
        tx_id: tx_id.to_string(),
        created_at: Utc::now(),
        effects: files
            .iter()
            .map(|path| {
                let handler = handler_for_path(path);
                RollbackEffect {
                    path: path.clone(),
                    handler: handler.name.to_string(),
                    status: status.to_string(),
                    reason: handler.reason.map(str::to_string),
                }
            })
            .collect(),
    };
    let path = tx_dir.join("rollback.json");
    fs::write(&path, serde_json::to_string_pretty(&report)?)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(report)
}

fn is_package_manifest(path: &str) -> bool {
    matches!(
        path,
        "package.json"
            | "package-lock.json"
            | "pnpm-lock.yaml"
            | "yarn.lock"
            | "Cargo.toml"
            | "Cargo.lock"
    )
}

fn is_terraform_state(path: &str) -> bool {
    path.ends_with(".tfstate") || path.ends_with(".tfstate.backup")
}

fn is_environment_file(path: &str) -> bool {
    path == ".env" || path.starts_with(".env.") || path.ends_with("/.env")
}
