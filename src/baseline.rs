use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::code_maps;
use crate::git;
use crate::spec::AgentSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineSnapshot {
    pub base_head: String,
    pub scoped_files: BTreeMap<String, String>,
    pub relevant_files: BTreeMap<String, String>,
}

pub fn capture(project_root: &Path, spec: &AgentSpec, base_head: &str) -> Result<BaselineSnapshot> {
    Ok(BaselineSnapshot {
        base_head: base_head.to_string(),
        scoped_files: scoped_file_hashes(project_root, spec)?,
        relevant_files: relevant_file_hashes(project_root, spec)?,
    })
}

pub fn write(tx_dir: &Path, snapshot: &BaselineSnapshot) -> Result<()> {
    fs::write(
        tx_dir.join("baseline.json"),
        serde_json::to_string_pretty(snapshot)?,
    )
    .with_context(|| format!("write {}", tx_dir.join("baseline.json").display()))
}

fn scoped_file_hashes(project_root: &Path, spec: &AgentSpec) -> Result<BTreeMap<String, String>> {
    let scope = compile_scope(&spec.scope.allow)?;
    let files = git::tracked_files(project_root)?;
    hash_matching(project_root, files, scope.as_ref())
}

fn relevant_file_hashes(project_root: &Path, spec: &AgentSpec) -> Result<BTreeMap<String, String>> {
    let mut files = BTreeSet::new();
    if let Ok(selection) = code_maps::select_context(project_root, spec) {
        for route in selection.routes {
            files.insert(route.file);
        }
        for component in selection.components {
            files.insert(component.file);
        }
        for export in selection.exports {
            files.insert(export.file);
        }
    }
    hash_matching(project_root, files.into_iter().collect(), None)
}

fn hash_matching(
    project_root: &Path,
    files: Vec<String>,
    scope: Option<&GlobSet>,
) -> Result<BTreeMap<String, String>> {
    let mut hashes = BTreeMap::new();
    for file in files {
        if scope.map(|scope| scope.is_match(&file)).unwrap_or(true) {
            hashes.insert(file.clone(), hash_file(&project_root.join(&file))?);
        }
    }
    Ok(hashes)
}

fn compile_scope(patterns: &[String]) -> Result<Option<GlobSet>> {
    if patterns.is_empty() {
        return Ok(None);
    }
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }
    Ok(Some(builder.build()?))
}

fn hash_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("hash {}", path.display()))?;
    let digest = Sha256::digest(bytes);
    Ok(format!("{digest:x}"))
}
