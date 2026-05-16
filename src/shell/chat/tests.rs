use std::path::Path;

use anyhow::Result;

use super::*;

#[test]
fn persists_chat_messages_and_transactions() -> Result<()> {
    let dir = tempfile::tempdir()?;
    std::fs::create_dir_all(dir.path().join(".agent/shell"))?;
    let session = create(dir.path())?;
    append_user(&session, "plan", "add page")?;
    append_draft(&session, "add page", Path::new(".agent/drafts/demo.yaml"))?;
    append_tx(
        &session,
        "add page",
        "tx-1",
        Path::new(".agent/tx/tx-1/report.md"),
    )?;

    let rows = list(dir.path())?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].messages, 1);
    assert_eq!(rows[0].txs, 1);
    assert_eq!(open(dir.path(), &session.id)?.id, session.id);
    Ok(())
}
