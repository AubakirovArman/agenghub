use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use anyhow::{Context, Result};
use tempfile::TempDir;

use agenthub::agent_dir;
use agenthub::transaction::{self, TransactionOutcome, TransactionStatus};

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn injected_pre_commit_faults_roll_back_cleanly() -> Result<()> {
    for stage in [
        "WORKSPACE_READY",
        "EXECUTING",
        "AFTER_DIFF_GUARD",
        "VERIFYING",
        "BEFORE_COMMIT",
        "COMMITTING",
    ] {
        let repo = TestRepo::new()?;
        agent_dir::init_project(repo.path(), false)?;
        repo.commit_all("agenthub baseline")?;
        let spec = repo.write_spec(stage)?;

        let outcome = run_with_fault(repo.path(), &spec, stage)?;

        assert!(matches!(outcome.status, TransactionStatus::RolledBack));
        assert!(!repo.path().join("generated/chaos.txt").exists());
        assert_fault_recorded(&outcome, stage)?;
        let memory = fs::read_to_string(repo.path().join(".agent/memory/committed.jsonl"))?;
        assert!(!memory.contains(&task_id(stage)));
    }
    Ok(())
}

#[test]
fn injected_post_commit_faults_are_reported_without_false_rollback() -> Result<()> {
    for stage in ["MEMORY_PROMOTION", "CLEANUP"] {
        let repo = TestRepo::new()?;
        agent_dir::init_project(repo.path(), false)?;
        repo.commit_all("agenthub baseline")?;
        let spec = repo.write_spec(stage)?;

        let outcome = run_with_fault(repo.path(), &spec, stage)?;

        assert!(matches!(outcome.status, TransactionStatus::Committed));
        assert!(repo.path().join("generated/chaos.txt").exists());
        assert_fault_recorded(&outcome, stage)?;
        let journal = tx_text(&outcome, "journal.jsonl")?;
        assert!(journal.contains(&format!("{stage}_FAILED")));
    }
    Ok(())
}

fn run_with_fault(root: &Path, spec: &Path, stage: &str) -> Result<TransactionOutcome> {
    let _guard = ENV_LOCK.lock().expect("fault env lock");
    std::env::set_var("AGENTHUB_FAULT_INJECTION", "1");
    std::env::set_var("AGENTHUB_FAIL_AT", stage);
    let outcome = transaction::run(root, spec, false);
    std::env::remove_var("AGENTHUB_FAIL_AT");
    std::env::remove_var("AGENTHUB_FAULT_INJECTION");
    outcome
}

fn assert_fault_recorded(outcome: &TransactionOutcome, stage: &str) -> Result<()> {
    assert!(outcome.report_path.exists());
    let fault = tx_text(outcome, "fault_injection.jsonl")?;
    assert!(fault.contains(stage));
    let journal = tx_text(outcome, "journal.jsonl")?;
    assert!(journal.contains("FAULT_INJECTION"));
    Ok(())
}

fn tx_text(outcome: &TransactionOutcome, name: &str) -> Result<String> {
    Ok(fs::read_to_string(
        outcome.report_path.with_file_name(name),
    )?)
}

fn task_id(stage: &str) -> String {
    format!("chaos_{}", stage.to_ascii_lowercase())
}

struct TestRepo {
    dir: TempDir,
    specs: TempDir,
}

impl TestRepo {
    fn new() -> Result<Self> {
        let dir = tempfile::tempdir()?;
        let specs = tempfile::tempdir()?;
        git(dir.path(), &["init"])?;
        git(dir.path(), &["config", "user.email", "test@example.com"])?;
        git(dir.path(), &["config", "user.name", "AgentHub Test"])?;
        fs::write(
            dir.path().join(".gitignore"),
            ".agent/tx/\n.agent/workspaces/\n.agent/memory/*.jsonl\n",
        )?;
        fs::write(dir.path().join("README.md"), "test project\n")?;
        Ok(Self { dir, specs })
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }

    fn commit_all(&self, message: &str) -> Result<()> {
        git(self.path(), &["add", "-A"])?;
        git(self.path(), &["commit", "-m", message])
    }

    fn write_spec(&self, stage: &str) -> Result<PathBuf> {
        let path = self.specs.path().join(format!("{}.yaml", task_id(stage)));
        fs::write(&path, spec_yaml(&task_id(stage)))?;
        Ok(path)
    }
}

fn spec_yaml(task_id: &str) -> String {
    format!(
        r#"
task:
  id: {task_id}
  type: code.command
workspace:
  type: code.git
  isolation: git_worktree
execution:
  commands:
    - mkdir -p generated
    - printf 'chaos\n' > generated/chaos.txt
scope:
  allow:
    - generated/**
verify:
  commands:
    - test -f generated/chaos.txt
transaction:
  commit_on_success: true
  memory_promotion: on_success
  diff_limits:
    max_files_changed: 1
    max_lines_added: 1
    max_lines_deleted: 0
"#
    )
}

fn git(root: &Path, args: &[&str]) -> Result<()> {
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
