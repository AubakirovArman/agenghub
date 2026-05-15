use anyhow::Result;

use super::{inspect, load_resource_limits};

#[test]
fn hardening_report_degrades_unsupported_features_to_warnings() -> Result<()> {
    let dir = tempfile::tempdir()?;

    let report = inspect(dir.path())?;

    assert!(!report.capabilities.is_empty());
    assert!(report
        .capabilities
        .iter()
        .any(|capability| capability.id == "network.policy"));
    Ok(())
}

#[test]
fn resource_limits_load_from_policy_file() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let policy_dir = dir.path().join(".agent/policies");
    std::fs::create_dir_all(&policy_dir)?;
    std::fs::write(
        policy_dir.join("resources.yaml"),
        "resources:\n  timeout_secs: 42\n  memory_mb: 512\n  network: deny\n  filesystem: workspace\n",
    )?;

    let limits = load_resource_limits(dir.path())?;

    assert_eq!(limits.timeout_secs, 42);
    assert_eq!(limits.memory_mb, Some(512));
    assert_eq!(limits.network, "deny");
    Ok(())
}
