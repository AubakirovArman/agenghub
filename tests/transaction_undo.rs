use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use tempfile::TempDir;

use agenthub::agent_dir;
use agenthub::transaction::{self, TransactionStatus};
use agenthub::tx_undo;

#[test]
fn undo_last_reverts_committed_transaction() -> Result<()> {
    let repo = TestRepo::new()?;
    agent_dir::init_project(repo.path(), false)?;
    repo.commit_all("baseline")?;
    let spec = repo.write_spec(
        "create.yaml",
        r#"
task:
  id: undo_generated_file
  type: code.command
workspace:
  type: code.git
  isolation: git_worktree
execution:
  commands:
    - mkdir -p generated
    - printf 'undo me\n' > generated/undo.txt
scope:
  allow:
    - generated/**
verify:
  commands:
    - test -f generated/undo.txt
transaction:
  commit_on_success: true
"#,
    )?;
    let outcome = transaction::run(repo.path(), &spec, false)?;
    assert!(matches!(outcome.status, TransactionStatus::Committed));
    assert!(repo.path().join("generated/undo.txt").exists());

    let undo = tx_undo::undo(repo.path(), "last")?;

    assert_eq!(undo.tx_id, outcome.tx_id);
    assert!(!repo.path().join("generated/undo.txt").exists());
    assert!(outcome.report_path.with_file_name("undo.json").exists());
    let journal = fs::read_to_string(outcome.report_path.with_file_name("journal.jsonl"))?;
    assert!(journal.contains("UNDO_REVERTED"));
    Ok(())
}

struct TestRepo {
    dir: TempDir,
    specs: TempDir,
}

impl TestRepo {
    fn new() -> Result<Self> {
        let dir = tempfile::tempdir()?;
        let specs = tempfile::tempdir()?;
        run_git(dir.path(), &["init"])?;
        run_git(dir.path(), &["config", "user.email", "test@example.com"])?;
        run_git(dir.path(), &["config", "user.name", "AgentHub Test"])?;
        fs::write(
            dir.path().join(".gitignore"),
            ".agent/tx/\n.agent/workspaces/\n.agent/cache/\n.agent/memory/*.jsonl\n",
        )?;
        Ok(Self { dir, specs })
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }

    fn commit_all(&self, message: &str) -> Result<()> {
        run_git(self.path(), &["add", "-A"])?;
        run_git(self.path(), &["commit", "-m", message])
    }

    fn write_spec(&self, name: &str, content: &str) -> Result<std::path::PathBuf> {
        let path = self.specs.path().join(name);
        fs::write(&path, content.trim_start())?;
        Ok(path)
    }
}

fn run_git(root: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .with_context(|| format!("git {}", args.join(" ")))?;
    if !output.status.success() {
        anyhow::bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
