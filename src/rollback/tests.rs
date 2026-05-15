use anyhow::Result;

use super::{handler_for_path, write_report};

#[test]
fn selects_concrete_handlers_for_known_paths() {
    assert_eq!(
        handler_for_path("package.json").name,
        "package_manifest_restore"
    );
    assert_eq!(
        handler_for_path("infra/default.tfstate").name,
        "terraform_state_restore"
    );
    assert_eq!(
        handler_for_path(".env.local").name,
        "manual_approval_required"
    );
    assert_eq!(handler_for_path("src/lib.rs").name, "file_restore");
}

#[test]
fn writes_rollback_report() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let files = vec!["package.json".to_string(), "src/lib.rs".to_string()];
    let report = write_report(dir.path(), "tx-test", &files, "rolled_back")?;

    assert_eq!(report.effects.len(), 2);
    assert!(dir.path().join("rollback.json").exists());
    assert!(report
        .effects
        .iter()
        .any(|item| item.handler == "package_manifest_restore"));
    Ok(())
}
