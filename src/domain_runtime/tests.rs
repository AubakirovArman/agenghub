use anyhow::Result;

use crate::domain_runtime::{detect, evaluate, write_artifact, RuntimePack};
use crate::spec::WorkspaceProfile;

#[test]
fn selects_rust_runtime_from_code_workspace() -> Result<()> {
    let dir = tempfile::tempdir()?;
    std::fs::write(dir.path().join("Cargo.toml"), "[package]\nname='demo'\n")?;

    let artifact = evaluate(dir.path(), WorkspaceProfile::Code, None);

    assert_eq!(
        artifact.selected.as_ref().map(|pack| pack.id.as_str()),
        Some("code.rust")
    );
    assert!(artifact
        .catalog
        .iter()
        .any(|pack| pack.id == "infra.terraform"));
    Ok(())
}

#[test]
fn selects_research_runtime_from_verifier_profile() {
    let dir = tempfile::tempdir().expect("tempdir");
    let artifact = evaluate(
        dir.path(),
        WorkspaceProfile::Research,
        Some("research_report"),
    );

    assert_eq!(
        artifact.selected.as_ref().map(|pack| pack.id.as_str()),
        Some("research.citations")
    );
}

#[test]
fn missing_tools_are_structured_warnings() {
    let pack = RuntimePack {
        id: "test.missing".to_string(),
        domain: "code".to_string(),
        name: "Missing Tool Test".to_string(),
        supported_workspaces: vec!["code.git".to_string()],
        verifier_profiles: Vec::new(),
        effects: Vec::new(),
        artifacts: Vec::new(),
        memory_schemas: Vec::new(),
        required_tools: vec!["agenthub-tool-that-should-not-exist".to_string()],
        warnings: Vec::new(),
    };
    let warnings = detect::tool_warnings(&pack);

    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("required tool"));
}

#[test]
fn writes_domain_runtime_artifact() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let tx_dir = dir.path().join(".agent/tx/tx-domain");
    let write = write_artifact(
        dir.path(),
        &tx_dir,
        WorkspaceProfile::Data,
        Some("data_quality"),
    )?;
    let text = std::fs::read_to_string(write.path)?;

    assert_eq!(write.artifact.selected.unwrap().id, "data.python");
    assert!(text.contains("domain.runtime.v1"));
    Ok(())
}
