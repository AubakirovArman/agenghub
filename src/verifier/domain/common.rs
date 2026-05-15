use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::verifier::domain::DomainCheckResult;

pub fn present(name: &str, files: &[PathBuf]) -> DomainCheckResult {
    check(name, !files.is_empty(), format!("{} file(s)", files.len()))
}

pub fn all_non_empty(name: &str, files: &[PathBuf]) -> Result<DomainCheckResult> {
    let empty = files
        .iter()
        .filter(|file| {
            fs::metadata(file)
                .map(|meta| meta.len() == 0)
                .unwrap_or(true)
        })
        .count();
    Ok(check(
        name,
        empty == 0 && !files.is_empty(),
        format!("{empty} empty"),
    ))
}

pub fn all_json(name: &str, files: &[PathBuf]) -> Result<DomainCheckResult> {
    let mut invalid = 0;
    for file in files {
        let content =
            fs::read_to_string(file).with_context(|| format!("read {}", file.display()))?;
        if json_invalid(file, &content) {
            invalid += 1;
        }
    }
    Ok(check(
        name,
        invalid == 0 && !files.is_empty(),
        format!("{invalid} invalid"),
    ))
}

pub fn json_invalid(file: &Path, content: &str) -> bool {
    file.extension().and_then(|ext| ext.to_str()) == Some("json")
        && serde_json::from_str::<serde_json::Value>(content).is_err()
}

pub fn yaml_invalid(file: &Path, content: &str) -> bool {
    matches!(
        file.extension().and_then(|ext| ext.to_str()),
        Some("yaml" | "yml")
    ) && serde_yaml::from_str::<serde_yaml::Value>(content).is_err()
}

pub fn collect_files(root: &Path, extensions: &[&str]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    visit(root, extensions, &mut files)?;
    files.sort();
    Ok(files)
}

fn visit(dir: &Path, extensions: &[&str], files: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).with_context(|| format!("read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            visit(&path, extensions, files)?;
        } else if has_extension(&path, extensions) {
            files.push(path);
        }
    }
    Ok(())
}

fn has_extension(path: &Path, extensions: &[&str]) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| extensions.contains(&ext))
}

pub fn check(name: &str, success: bool, detail: String) -> DomainCheckResult {
    DomainCheckResult {
        name: name.to_string(),
        success,
        detail,
    }
}
