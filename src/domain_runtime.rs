mod catalog;
pub(crate) mod detect;
#[cfg(test)]
mod tests;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::spec::WorkspaceProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRuntimeArtifact {
    pub version: String,
    pub selected: Option<RuntimePack>,
    pub catalog: Vec<RuntimePack>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimePack {
    pub id: String,
    pub domain: String,
    pub name: String,
    pub supported_workspaces: Vec<String>,
    pub verifier_profiles: Vec<String>,
    pub effects: Vec<String>,
    pub artifacts: Vec<String>,
    pub memory_schemas: Vec<String>,
    pub required_tools: Vec<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DomainRuntimeWrite {
    pub path: PathBuf,
    pub artifact: DomainRuntimeArtifact,
}

pub fn evaluate(
    project_root: &Path,
    workspace: WorkspaceProfile,
    verifier_profile: Option<&str>,
) -> DomainRuntimeArtifact {
    let catalog = catalog::packs();
    let selected = catalog
        .iter()
        .find(|pack| detect::matches(project_root, workspace, verifier_profile, pack))
        .cloned()
        .map(|mut pack| {
            pack.warnings = detect::tool_warnings(&pack);
            pack
        });
    let warnings = selected
        .as_ref()
        .map(|pack| pack.warnings.clone())
        .unwrap_or_default();
    DomainRuntimeArtifact {
        version: "domain.runtime.v1".to_string(),
        selected,
        catalog,
        warnings,
    }
}

pub fn write_artifact(
    project_root: &Path,
    tx_dir: &Path,
    workspace: WorkspaceProfile,
    verifier_profile: Option<&str>,
) -> Result<DomainRuntimeWrite> {
    let artifact = evaluate(project_root, workspace, verifier_profile);
    let path = tx_dir.join("domain_runtime.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(&path, serde_json::to_string_pretty(&artifact)?)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(DomainRuntimeWrite { path, artifact })
}
