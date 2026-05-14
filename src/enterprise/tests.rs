use anyhow::Result;
use serde_json::json;

use super::{authorize, list_audit, record_event};
use crate::agent_dir::init_project;

#[test]
fn default_policy_allows_transaction_run() -> Result<()> {
    let dir = tempfile::tempdir()?;
    init_project(dir.path(), false)?;

    let actor = authorize(dir.path(), "transaction.run")?;

    assert!(actor.allows("transaction.run"));
    Ok(())
}

#[test]
fn audit_events_are_append_only() -> Result<()> {
    let dir = tempfile::tempdir()?;
    init_project(dir.path(), false)?;
    let actor = authorize(dir.path(), "transaction.run")?;

    record_event(
        dir.path(),
        &actor,
        "agenthub.run",
        "transaction.run",
        "ok",
        Some("demo".to_string()),
        json!({ "tx_id": "tx-demo" }),
    )?;

    let events = list_audit(dir.path(), 10)?;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].action, "agenthub.run");
    Ok(())
}
