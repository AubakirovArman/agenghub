use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};

use super::*;

#[test]
fn resolves_dependencies() -> Result<()> {
    let dir = tempfile::tempdir()?;
    write_skill(dir.path(), "base", "base.skill", &[])?;
    write_skill(dir.path(), "feature", "feature.skill", &["base.skill"])?;

    let loaded = load_requested(dir.path(), &["feature.skill".to_string()])?;
    let ids = loaded
        .into_iter()
        .map(|manifest| manifest.skill.id)
        .collect::<Vec<_>>();

    assert_eq!(ids, vec!["base.skill", "feature.skill"]);
    Ok(())
}

#[test]
fn scorecard_uses_analytics_history() -> Result<()> {
    let dir = tempfile::tempdir()?;
    write_skill(dir.path(), "feature", "feature.skill", &[])?;
    crate::analytics::record(dir.path(), &analytics_record("feature.skill"))?;

    let cards = scorecards(dir.path())?;
    let card = cards
        .iter()
        .find(|card| card.id == "feature.skill")
        .ok_or_else(|| anyhow!("missing feature.skill scorecard"))?;
    assert_eq!(card.runs, 1);
    assert_eq!(card.success_rate, 1.0);
    Ok(())
}

#[test]
fn bundled_skills_are_available_without_project_skills() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let skills = list_available(dir.path())?;

    assert!(skills
        .iter()
        .any(|manifest| manifest.skill.id == "code.nextjs.add_page"));
    assert!(skills
        .iter()
        .any(|manifest| manifest.skill.id == "core.file.create"));
    Ok(())
}

#[test]
fn standard_library_has_required_quality_gates() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let skills = list_available(root)?;
    for id in REQUIRED_STANDARD_SKILLS {
        skills
            .iter()
            .find(|skill| skill.skill.id == *id)
            .ok_or_else(|| anyhow!("missing standard skill {id}"))?;
    }
    for manifest in &skills {
        let path = manifest.source_path.as_ref().unwrap();
        let id = &manifest.skill.id;
        assert!(path.with_file_name("README.md").exists(), "{id} README");
        assert!(!manifest.verifiers.is_empty(), "{id} verifier profile");
        assert!(!manifest.common_errors.is_empty(), "{id} known errors");
        let raw: serde_yaml::Value = serde_yaml::from_str(&fs::read_to_string(path)?)?;
        assert!(raw.get("examples").is_some(), "{id} example AgentSpec");
        assert!(raw.get("fixtures").is_some(), "{id} fixture project");
        assert!(raw.get("tests").is_some(), "{id} success/failure tests");
    }
    Ok(())
}

fn analytics_record(skill: &str) -> crate::analytics::AnalyticsRecord {
    crate::analytics::AnalyticsRecord {
        version: "analytics.record.v1".to_string(),
        tx_id: "tx-1".to_string(),
        task_id: "task".to_string(),
        task_type: "code".to_string(),
        status: "COMMITTED".to_string(),
        started_at: chrono::Utc::now(),
        finished_at: chrono::Utc::now(),
        duration_ms: 100,
        success: true,
        rollback: false,
        repair: false,
        human_block: false,
        dangerous_diff: false,
        task_class: None,
        topology: None,
        model: None,
        verifier_profile: None,
        skills: vec![skill.to_string()],
        cost_usd: 0.02,
        estimated_tokens: 20,
    }
}

fn write_skill(root: &Path, dir_name: &str, id: &str, dependencies: &[&str]) -> Result<()> {
    let dir = root.join("skills").join(dir_name);
    fs::create_dir_all(&dir)?;
    let dependencies_block = dependency_block(dependencies);
    fs::write(
        dir.join("skill.yaml"),
        format!(
            "skill:\n  id: {id}\n  version: 1.0.0\n  description: test skill\n{dependencies_block}"
        ),
    )?;
    Ok(())
}

fn dependency_block(dependencies: &[&str]) -> String {
    if dependencies.is_empty() {
        return String::new();
    }
    let dependency_yaml = dependencies
        .iter()
        .map(|dependency| format!("  - {dependency}\n"))
        .collect::<String>();
    format!("dependencies:\n{dependency_yaml}")
}

const REQUIRED_STANDARD_SKILLS: &[&str] = &[
    "core.file.create",
    "core.file.edit",
    "core.docs.update",
    "core.fix_build",
    "code.rust.fix_clippy",
    "code.rust.add_test",
    "code.rust.refactor_module",
    "web.add_page",
    "web.runtime_smoke",
    "web.reuse_component",
    "python.data_artifact",
    "infra.terraform_plan",
    "content.article_outline",
];
