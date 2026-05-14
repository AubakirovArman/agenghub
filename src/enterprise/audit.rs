use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use anyhow::{Context, Result};
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use crate::agent_dir::ensure_runtime_dirs;
use crate::enterprise::types::{ActorContext, AuditEvent};

pub fn record_event(
    project_root: &Path,
    actor: &ActorContext,
    action: &str,
    permission: &str,
    outcome: &str,
    target: Option<String>,
    details: Value,
) -> Result<AuditEvent> {
    let paths = ensure_runtime_dirs(project_root)?;
    let event = AuditEvent {
        id: format!("audit-{}", &Uuid::new_v4().to_string()[..8]),
        created_at: Utc::now(),
        actor: actor.actor.clone(),
        role: actor.role.clone(),
        action: action.to_string(),
        permission: permission.to_string(),
        outcome: outcome.to_string(),
        target,
        details,
    };
    append_jsonl(&paths.enterprise.join("audit.jsonl"), &event)?;
    Ok(event)
}

pub fn list_audit(project_root: &Path, limit: usize) -> Result<Vec<AuditEvent>> {
    let paths = ensure_runtime_dirs(project_root)?;
    let path = paths.enterprise.join("audit.jsonl");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = fs::File::open(&path).with_context(|| format!("open {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        events.push(
            serde_json::from_str(&line).with_context(|| format!("parse {}", path.display()))?,
        );
    }
    events.reverse();
    events.truncate(limit);
    Ok(events)
}

fn append_jsonl<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("open {}", path.display()))?;
    writeln!(file, "{}", serde_json::to_string(value)?)?;
    Ok(())
}
