use std::path::Path;

use anyhow::Result;

use super::run;

pub fn find_commit_by_subject(root: &Path, needle: &str) -> Result<Option<String>> {
    let output = run(root, &["log", "--format=%H%x09%s", "-n", "300"])?;
    Ok(output.stdout.lines().find_map(|line| {
        let (hash, subject) = line.split_once('\t')?;
        subject.contains(needle).then(|| hash.to_string())
    }))
}

pub fn revert_no_edit(root: &Path, commit: &str) -> Result<()> {
    run(root, &["revert", "--no-edit", commit]).map(|_| ())
}
