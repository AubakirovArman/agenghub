use std::fs;

use anyhow::Result;
use serde_json::json;

use super::*;

#[test]
fn appends_replays_and_writes_summary() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("wal.jsonl");
    let wal = Wal::new("tx-demo", &path);

    wal.append("CREATED", "created", &json!({}))?;
    wal.append("CLOSED", "closed", &json!({ "ok": true }))?;
    let replay = write_replay(&path, &dir.path().join("wal_replay.json"))?;

    assert_eq!(replay.record_count, 2);
    assert_eq!(replay.latest_state.as_deref(), Some("CLOSED"));
    assert!(dir.path().join("wal_replay.json").exists());
    Ok(())
}

#[test]
fn replay_detects_checksum_mismatch() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("wal.jsonl");
    let wal = Wal::new("tx-demo", &path);
    wal.append("CREATED", "created", &json!({ "value": "original" }))?;

    let tampered = fs::read_to_string(&path)?.replace("original", "tampered");
    fs::write(&path, tampered)?;

    let err = replay(&path).expect_err("tampered WAL should fail replay");

    assert!(err.to_string().contains("checksum mismatch"));
    Ok(())
}
