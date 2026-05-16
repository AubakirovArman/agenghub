use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::{agent_dir::AgentPaths, diff_guard::DiffGuardResult, git};

pub fn diff(root: &Path, tx_id: &str) -> Result<String> {
    let subject = format!("AgentHub {tx_id}:");
    if let Ok(Some(commit)) = git::find_commit_by_subject(root, &subject) {
        return Ok(git::run(
            root,
            &["show", "--stat", "--patch", "--format=short", &commit],
        )?
        .stdout);
    }
    diff_guard_summary(root, tx_id)
}

pub fn logs(root: &Path, tx_id: &str, filter: Option<&str>, tail: usize) -> Result<String> {
    let logs_dir = AgentPaths::new(root).tx_dir(tx_id).join("logs");
    if !logs_dir.exists() {
        return Ok(format!("no logs found for {tx_id}\n"));
    }
    let mut files = log_files(&logs_dir)?;
    files.sort();
    let mut out = String::new();
    for path in files {
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if filter.is_some_and(|needle| !name.contains(needle)) {
            continue;
        }
        out.push_str(&format!("== {} ==\n", path.display()));
        out.push_str(&tail_text(&path, tail)?);
        out.push('\n');
    }
    if out.is_empty() {
        out.push_str("no matching logs\n");
    }
    Ok(out)
}

fn diff_guard_summary(root: &Path, tx_id: &str) -> Result<String> {
    let path = AgentPaths::new(root).tx_dir(tx_id).join("diff_guard.json");
    if !path.exists() {
        return Ok(format!(
            "no committed diff or diff_guard.json found for {tx_id}\n"
        ));
    }
    let guard: DiffGuardResult = serde_json::from_str(
        &fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?,
    )?;
    let mut out = format!(
        "diff summary for {tx_id}\npassed: {}\nfiles: {}\n+{}\n-{}\n",
        guard.passed,
        guard.summary.files_changed,
        guard.summary.lines_added,
        guard.summary.lines_deleted
    );
    for file in guard.summary.changed_files {
        out.push_str(&format!("- {file}\n"));
    }
    if !guard.violations.is_empty() {
        out.push_str("violations:\n");
        for violation in guard.violations {
            out.push_str(&format!("- {violation}\n"));
        }
    }
    Ok(out)
}

fn log_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("read {}", dir.display()))? {
        let path = entry?.path();
        if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}

fn tail_text(path: &Path, max_lines: usize) -> Result<String> {
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let lines = text.lines().collect::<Vec<_>>();
    let start = lines.len().saturating_sub(max_lines);
    Ok(lines[start..].join("\n"))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn logs_filter_and_tail_command_output() {
        let temp = tempdir().unwrap();
        let logs_dir = temp.path().join(".agent/tx/tx-1/logs");
        fs::create_dir_all(&logs_dir).unwrap();
        fs::write(logs_dir.join("execute.stdout.log"), "one\ntwo\nthree\n").unwrap();
        fs::write(logs_dir.join("verify.stdout.log"), "alpha\nbeta\n").unwrap();

        let output = logs(temp.path(), "tx-1", Some("execute"), 2).unwrap();

        assert!(output.contains("execute.stdout.log"));
        assert!(output.contains("two\nthree"));
        assert!(!output.contains("verify.stdout.log"));
        assert!(!output.contains("one\n"));
    }

    #[test]
    fn diff_falls_back_to_diff_guard_summary() {
        let temp = tempdir().unwrap();
        let tx_dir = temp.path().join(".agent/tx/tx-1");
        fs::create_dir_all(&tx_dir).unwrap();
        fs::write(
            tx_dir.join("diff_guard.json"),
            r#"{
              "passed": true,
              "summary": {
                "files_changed": 1,
                "lines_added": 2,
                "lines_deleted": 0,
                "changed_files": ["docs/demo.md"]
              },
              "violations": []
            }"#,
        )
        .unwrap();

        let output = diff(temp.path(), "tx-1").unwrap();

        assert!(output.contains("diff summary for tx-1"));
        assert!(output.contains("docs/demo.md"));
        assert!(output.contains("+2"));
    }
}
