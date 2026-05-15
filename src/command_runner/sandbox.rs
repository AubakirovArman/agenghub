use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

use super::CommandSandbox;

pub fn configure(process: &mut Command, cwd: &Path, sandbox: &CommandSandbox) -> Result<()> {
    if sandbox.level == 0 {
        return Ok(());
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
