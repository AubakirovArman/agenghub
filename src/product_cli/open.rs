use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};

use crate::{agent_dir::AgentPaths, team, web_dashboard};

#[derive(Debug, Clone)]
pub struct OpenResult {
    pub kind: String,
    pub path: PathBuf,
    pub launched: bool,
}

pub fn dashboard(project_root: &Path) -> Result<OpenResult> {
    let output = project_root.join(".agent/reports/dashboard");
    let result = web_dashboard::write_dashboard(project_root, &output)?;
    let projects = vec![project_root.to_path_buf()];
    team::write_export(&projects, &project_root.join(".agent/reports/team"))?;
    open_path("dashboard", result.index_path)
}

pub fn report(project_root: &Path, tx_id: &str) -> Result<OpenResult> {
    let path = AgentPaths::new(project_root)
        .tx_dir(tx_id)
        .join("report.md");
    if !path.exists() {
        return Err(anyhow!("report not found for transaction `{tx_id}`"));
    }
    open_path("report", path)
}

fn open_path(kind: &str, path: PathBuf) -> Result<OpenResult> {
    let launched = launch(&path).with_context(|| format!("open {}", path.display()))?;
    Ok(OpenResult {
        kind: kind.to_string(),
        path,
        launched,
    })
}

fn launch(path: &Path) -> Result<bool> {
    if dry_run() {
        return Ok(false);
    }
    let Some(mut command) = opener(path) else {
        return Ok(false);
    };
    Ok(command
        .status()
        .map(|status| status.success())
        .unwrap_or(false))
}

fn dry_run() -> bool {
    matches!(std::env::var("AGENTHUB_OPEN_DRY_RUN").as_deref(), Ok("1"))
        || matches!(std::env::var("CI").as_deref(), Ok("true" | "1"))
}

#[cfg(target_os = "macos")]
fn opener(path: &Path) -> Option<Command> {
    let mut command = Command::new("open");
    command.arg(path);
    Some(command)
}

#[cfg(target_os = "windows")]
fn opener(path: &Path) -> Option<Command> {
    let mut command = Command::new("cmd");
    command.args(["/C", "start", ""]).arg(path);
    Some(command)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn opener(path: &Path) -> Option<Command> {
    let mut command = Command::new("xdg-open");
    command.arg(path);
    Some(command)
}
