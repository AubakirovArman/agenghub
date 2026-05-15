use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::verifier::domain::common::{
    all_json, all_non_empty, check, collect_files, json_invalid, present, yaml_invalid,
};
use crate::verifier::domain::DomainCheckResult;

pub fn content_checks(root: &Path) -> Result<Vec<DomainCheckResult>> {
    let files = collect_files(&root.join("content"), &["md", "txt"])?;
    Ok(vec![
        present("content_files_present", &files),
        all_non_empty("content_files_non_empty", &files)?,
    ])
}

pub fn data_checks(root: &Path) -> Result<Vec<DomainCheckResult>> {
    let files = collect_files(&root.join("data"), &["json"])?;
    Ok(vec![
        present("data_json_present", &files),
        all_json("data_json_valid", &files)?,
    ])
}

pub fn infra_checks(root: &Path) -> Result<Vec<DomainCheckResult>> {
    let files = collect_files(&root.join("infra"), &["yaml", "yml", "tf"])?;
    Ok(vec![
        present("infra_artifacts_present", &files),
        manifests_valid("infra_artifacts_valid", &files, true)?,
    ])
}

pub fn media_checks(root: &Path) -> Result<Vec<DomainCheckResult>> {
    let files = collect_files(&root.join("media"), media_extensions())?;
    Ok(vec![
        present("media_assets_present", &files),
        all_non_empty("media_assets_non_empty", &files)?,
        manifests_valid("media_manifests_valid", &files, false)?,
    ])
}

fn media_extensions() -> &'static [&'static str] {
    &[
        "md", "txt", "json", "yaml", "yml", "mp3", "wav", "mp4", "mov", "png", "jpg", "jpeg",
        "webp",
    ]
}

fn manifests_valid(
    name: &str,
    files: &[PathBuf],
    require_text_read: bool,
) -> Result<DomainCheckResult> {
    let mut invalid = 0;
    for file in files {
        let content = read_maybe_binary(file, require_text_read)?;
        if content.trim().is_empty() || json_invalid(file, &content) || yaml_invalid(file, &content)
        {
            invalid += 1;
        }
    }
    Ok(check(
        name,
        invalid == 0 && !files.is_empty(),
        format!("{invalid} invalid"),
    ))
}

fn read_maybe_binary(file: &Path, require_text_read: bool) -> Result<String> {
    if require_text_read {
        return fs::read_to_string(file).with_context(|| format!("read {}", file.display()));
    }
    Ok(fs::read_to_string(file).unwrap_or_else(|_| String::from("binary asset")))
}
