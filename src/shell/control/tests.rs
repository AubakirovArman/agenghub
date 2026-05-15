use std::fs;

use anyhow::Result;

use super::*;

#[test]
fn approve_uses_current_transaction_when_no_selector_is_given() -> Result<()> {
    let dir = tempfile::tempdir()?;
    fs::create_dir_all(dir.path().join(".agent/tx/tx-current"))?;

    let tx_id = approve_tx(dir.path(), Some("tx-current"), "approved by human")?;

    assert_eq!(tx_id, "tx-current");
    let resolution = fs::read_to_string(dir.path().join(".agent/tx/tx-current/resolutions.jsonl"))?;
    assert!(resolution.contains("approved by human"));
    assert!(
        fs::read_to_string(dir.path().join(".agent/tx/tx-current/effects.jsonl"))?
            .contains("control:resolve")
    );
    Ok(())
}

#[test]
fn approve_can_select_latest_transaction() -> Result<()> {
    let dir = tempfile::tempdir()?;
    fs::create_dir_all(dir.path().join(".agent/tx/tx-1"))?;
    fs::create_dir_all(dir.path().join(".agent/tx/tx-2"))?;

    let tx_id = approve_tx(dir.path(), None, "latest approved by human")?;

    assert_eq!(tx_id, "tx-2");
    let resolution = fs::read_to_string(dir.path().join(".agent/tx/tx-2/resolutions.jsonl"))?;
    assert!(resolution.contains("approved by human"));
    Ok(())
}
