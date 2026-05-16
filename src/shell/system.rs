use std::io::{self, IsTerminal, Write};
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use chrono::Utc;

use crate::command_policy;
use crate::command_runner::{run_shell_with_sandbox_logged, CommandSandbox};

pub(super) fn run(root: &Path, command: &str) -> Result<()> {
    if command.trim().is_empty() {
        println!("shell command is empty");
        return Ok(());
    }
    let policy = command_policy::classify_shell_command(root, command)?;
    match policy.classification.as_str() {
        "restricted" => {
            println!("blocked restricted command: {command}");
            return Ok(());
        }
        "needs_approval" if !confirm(&format!("Approve command `{command}`?"), false)? => {
            println!("command skipped");
            return Ok(());
        }
        _ => {}
    }
    let logs = root.join(".agent/shell/commands");
    let prefix = format!("shell-{}", Utc::now().format("%Y%m%d%H%M%S"));
    let result = run_shell_with_sandbox_logged(
        command,
        root,
        Duration::from_secs(900),
        CommandSandbox::default(),
        &logs,
        &prefix,
    )?;
    let status = if result.success {
        "completed"
    } else {
        "failed"
    };
    println!("command {status} in {} ms", result.duration_ms);
    if !result.stdout.trim().is_empty() {
        println!("stdout:\n{}", result.stdout.trim());
    }
    if !result.stderr.trim().is_empty() {
        println!("stderr:\n{}", result.stderr.trim());
    }
    if let Some(path) = result.stdout_path {
        println!("stdout_log {path}");
    }
    if let Some(path) = result.stderr_path {
        println!("stderr_log {path}");
    }
    Ok(())
}

fn confirm(question: &str, default_yes: bool) -> Result<bool> {
    if !io::stdin().is_terminal() {
        return Ok(default_yes);
    }
    let suffix = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("{question} {suffix} ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let answer = line.trim().to_ascii_lowercase();
    Ok(match answer.as_str() {
        "" => default_yes,
        "y" | "yes" | "д" | "да" => true,
        _ => false,
    })
}
