use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone)]
pub struct ScaffoldOptions {
    pub package_id: String,
    pub skill_id: String,
    pub description: String,
    pub author: Option<String>,
    pub force: bool,
}

pub fn scaffold_package(output: &Path, options: ScaffoldOptions) -> Result<PathBuf> {
    validate_id("package_id", &options.package_id)?;
    validate_id("skill_id", &options.skill_id)?;
    if options.description.trim().is_empty() {
        return Err(anyhow!("description is required"));
    }

    let skill_dir = output.join("skills").join(&options.skill_id);
    fs::create_dir_all(&skill_dir).with_context(|| format!("create {}", skill_dir.display()))?;
    let manifest_path = output.join("agenthub-plugin.yaml");
    let skill_path = skill_dir.join("skill.yaml");

    write_file(&manifest_path, &manifest_yaml(&options), options.force)?;
    write_file(&skill_path, &skill_yaml(&options), options.force)?;
    Ok(manifest_path)
}

fn write_file(path: &Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        return Err(anyhow!(
            "{} already exists; use --force to replace it",
            path.display()
        ));
    }
    fs::write(path, content).with_context(|| format!("write {}", path.display()))
}

fn manifest_yaml(options: &ScaffoldOptions) -> String {
    let author = options
        .author
        .as_ref()
        .map(|value| format!("  author: {value}\n"))
        .unwrap_or_default();
    format!(
        "package:\n  id: {}\n  version: 0.1.0\n  description: {}\n{}skills:\n  - path: skills/{}/skill.yaml\n\nworkspace_plugins: []\nverifier_plugins: []\nsignature: null\n",
        options.package_id, options.description, author, options.skill_id
    )
}

fn skill_yaml(options: &ScaffoldOptions) -> String {
    format!(
        "skill:\n  id: {}\n  version: 0.1.0\n  description: {}\n\ninputs: {{}}\nrequires: {{}}\nprovides:\n  actions: []\npolicies:\n  require_scope: true\n",
        options.skill_id, options.description
    )
}

fn validate_id(field: &str, value: &str) -> Result<()> {
    let valid = !value.trim().is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'));
    if !valid {
        return Err(anyhow!(
            "{field} must contain only letters, digits, dot, dash, or underscore"
        ));
    }
    Ok(())
}
