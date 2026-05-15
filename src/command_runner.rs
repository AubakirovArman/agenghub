#[cfg(test)]
mod tests;

use std::fs;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command: String,
    pub cwd: String,
    pub exit_code: Option<i32>,
    pub success: bool,
    pub timed_out: bool,
    pub duration_ms: u128,
    pub stdout: String,
    pub stderr: String,
    pub sandbox_level: u8,
}

#[derive(Debug)]
pub struct SupervisedChild {
    child: Child,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandSandbox {
    pub level: u8,
}

pub fn run_shell(command: &str, cwd: &Path, timeout: Duration) -> Result<CommandResult> {
    run_shell_with_sandbox(command, cwd, timeout, CommandSandbox::default())
}

pub fn run_shell_with_sandbox(
    command: &str,
    cwd: &Path,
    timeout: Duration,
    sandbox: CommandSandbox,
) -> Result<CommandResult> {
    let started = Instant::now();
    let mut process = Command::new("sh");
    process
        .arg("-lc")
        .arg(command)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure_sandbox(&mut process, cwd, sandbox)?;
    configure_process_group(&mut process);

    let mut child = process
        .spawn()
        .with_context(|| format!("spawn command `{command}` in {}", cwd.display()))?;

    let mut timed_out = false;
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if started.elapsed() >= timeout {
            timed_out = true;
            terminate_process_tree(&mut child);
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    let output = child
        .wait_with_output()
        .with_context(|| format!("wait for command `{command}`"))?;
    let exit_code = output.status.code();
    let success = output.status.success() && !timed_out;

    Ok(CommandResult {
        command: command.to_string(),
        cwd: cwd.display().to_string(),
        exit_code,
        success,
        timed_out,
        duration_ms: started.elapsed().as_millis(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        sandbox_level: sandbox.level,
    })
}

pub fn spawn_shell(command: &str, cwd: &Path) -> Result<SupervisedChild> {
    let mut process = Command::new("sh");
    process
        .arg("-lc")
        .arg(command)
        .current_dir(cwd)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    configure_process_group(&mut process);

    let child = process
        .spawn()
        .with_context(|| format!("spawn command `{command}` in {}", cwd.display()))?;
    Ok(SupervisedChild { child })
}

impl SupervisedChild {
    pub fn terminate(&mut self) {
        terminate_process_tree(&mut self.child);
    }
}

impl Drop for SupervisedChild {
    fn drop(&mut self) {
        self.terminate();
    }
}

#[cfg(unix)]
fn configure_process_group(command: &mut Command) {
    use std::os::unix::process::CommandExt;

    unsafe {
        command.pre_exec(|| {
            if libc::setpgid(0, 0) == 0 {
                Ok(())
            } else {
                Err(std::io::Error::last_os_error())
            }
        });
    }
}

#[cfg(not(unix))]
fn configure_process_group(_command: &mut Command) {}

fn configure_sandbox(process: &mut Command, cwd: &Path, sandbox: CommandSandbox) -> Result<()> {
    if sandbox.level == 0 {
        return Ok(());
    }
    if sandbox.level > 1 {
        return Err(anyhow!(
            "sandbox level {} requires an external runner",
            sandbox.level
        ));
    }
    let tmp = cwd.join(".agent-sandbox/tmp");
    fs::create_dir_all(&tmp).with_context(|| format!("create {}", tmp.display()))?;
    let path = std::env::var_os("PATH");
    process.env_clear();
    if let Some(path) = path {
        process.env("PATH", path);
    }
    process
        .env("HOME", cwd)
        .env("TMPDIR", &tmp)
        .env("AGENTHUB_SANDBOX_LEVEL", sandbox.level.to_string());
    Ok(())
}

#[cfg(unix)]
fn terminate_process_tree(child: &mut Child) {
    let pgid = -(child.id() as i32);
    unsafe {
        libc::kill(pgid, libc::SIGTERM);
    }

    let grace_started = Instant::now();
    while grace_started.elapsed() < Duration::from_secs(1) {
        if matches!(child.try_wait(), Ok(Some(_))) {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }

    unsafe {
        libc::kill(pgid, libc::SIGKILL);
    }
}

#[cfg(not(unix))]
fn terminate_process_tree(child: &mut Child) {
    let _ = child.kill();
}
