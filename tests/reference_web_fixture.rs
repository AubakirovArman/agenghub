use std::fs;
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use tempfile::TempDir;

use agenthub::agent_dir;
use agenthub::transaction::{self, TransactionStatus};

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/reference-web-app");
const SPEC: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/reference-web-add-courses.yaml"
);

#[test]
fn reference_web_fixture_adds_courses_page_end_to_end() -> Result<()> {
    if !has_web_smoke_tools() {
        return Ok(());
    }
    let repo = ReferenceRepo::new()?;
    let spec = repo.example_spec(free_port()?)?;

    let outcome = transaction::run(repo.path(), &spec, false)?;

    assert!(matches!(outcome.status, TransactionStatus::Committed));
    assert!(repo.path().join("src/app/courses/page.html").exists());
    assert!(outcome.report_path.exists());
    assert!(outcome.report_path.with_file_name("cost.json").exists());
    assert!(outcome
        .report_path
        .with_file_name("wal_replay.json")
        .exists());

    let verifier = fs::read_to_string(outcome.report_path.with_file_name("verifier.json"))?;
    assert!(verifier.contains("\"path\": \"/courses\""));
    assert!(verifier.contains("\"actual\": 200"));
    assert!(verifier.contains("\"success\": true"));

    let report = fs::read_to_string(&outcome.report_path)?;
    assert!(report.contains("Runtime smoke: `true`"));
    assert!(report.contains("Total cost"));

    let committed = fs::read_to_string(repo.path().join(".agent/memory/committed.jsonl"))?;
    assert!(committed.contains("reference_web_add_courses"));
    let failed = repo.read_failed_attempts()?;
    assert!(!failed.contains("reference_web_add_courses"));
    Ok(())
}

#[test]
fn reference_web_fixture_blocks_out_of_scope_edit() -> Result<()> {
    let repo = ReferenceRepo::new()?;
    let spec = repo.out_of_scope_spec()?;

    let outcome = transaction::run(repo.path(), &spec, false)?;

    assert!(matches!(outcome.status, TransactionStatus::RolledBack));
    assert!(!repo.path().join("src/app/courses/page.html").exists());
    let dashboard = fs::read_to_string(repo.path().join("src/app/dashboard/page.html"))?;
    assert!(dashboard.contains("Operations Dashboard"));

    let diff_guard = fs::read_to_string(outcome.report_path.with_file_name("report.md"))?;
    assert!(diff_guard.contains("out-of-policy denied path changed"));
    let failed = repo.read_failed_attempts()?;
    assert!(failed.contains("reference_web_scope_violation"));
    Ok(())
}

struct ReferenceRepo {
    dir: TempDir,
    specs: TempDir,
}

impl ReferenceRepo {
    fn new() -> Result<Self> {
        let dir = tempfile::tempdir()?;
        let specs = tempfile::tempdir()?;
        copy_dir(Path::new(FIXTURE), dir.path())?;
        run_git(dir.path(), &["init"])?;
        run_git(dir.path(), &["config", "user.email", "test@example.com"])?;
        run_git(dir.path(), &["config", "user.name", "AgentHub Test"])?;
        agent_dir::init_project(dir.path(), false)?;
        run_git(dir.path(), &["add", "-A"])?;
        run_git(dir.path(), &["commit", "-m", "reference web baseline"])?;
        Ok(Self { dir, specs })
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }

    fn example_spec(&self, port: u16) -> Result<std::path::PathBuf> {
        let content = fs::read_to_string(SPEC)?.replace("49231", &port.to_string());
        self.write_spec("reference-web-add-courses.yaml", &content)
    }

    fn out_of_scope_spec(&self) -> Result<std::path::PathBuf> {
        self.write_spec(
            "reference-web-scope-violation.yaml",
            r#"
task:
  id: reference_web_scope_violation
  type: code.add_page
workspace:
  type: code.git
  isolation: git_worktree
execution:
  commands:
    - mkdir -p src/app/courses
    - printf '<main><h1>Courses</h1></main>\n' > src/app/courses/page.html
    - printf '<main><h1>Mutated dashboard</h1></main>\n' > src/app/dashboard/page.html
scope:
  allow:
    - src/app/courses/**
  deny:
    - src/app/dashboard/**
verify:
  commands:
    - test -f src/app/courses/page.html
transaction:
  commit_on_success: true
  memory_promotion: on_success
  diff_limits:
    max_files_changed: 2
    max_lines_added: 10
    max_lines_deleted: 30
"#,
        )
    }

    fn write_spec(&self, name: &str, content: &str) -> Result<std::path::PathBuf> {
        let path = self.specs.path().join(name);
        fs::write(&path, content.trim_start())?;
        Ok(path)
    }

    fn read_failed_attempts(&self) -> Result<String> {
        let path = self.path().join(".agent/memory/failed_attempts.jsonl");
        Ok(fs::read_to_string(path).unwrap_or_default())
    }
}

fn copy_dir(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source).with_context(|| format!("read {}", source.display()))? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&source_path, &target_path)?;
        } else {
            fs::copy(&source_path, &target_path)
                .with_context(|| format!("copy {}", source_path.display()))?;
        }
    }
    Ok(())
}

fn has_web_smoke_tools() -> bool {
    command_exists("npm") && command_exists("node") && command_exists("curl")
}

fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .arg("-lc")
        .arg(format!("command -v {command}"))
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn free_port() -> Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    Ok(listener.local_addr()?.port())
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
