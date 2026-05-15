use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Result};

use crate::agent_adapter::{self, AgentRoutes};
use crate::command_runner::{run_shell_with_sandbox, CommandResult, CommandSandbox};
use crate::spec::AgentSpec;

pub(super) fn execute(
    spec: &AgentSpec,
    tx_dir: &Path,
    worktree: &Path,
    agent_routes: &AgentRoutes,
) -> Result<()> {
    agent_adapter::invoke_adapter(spec, tx_dir, worktree, &agent_routes.executor)?;
    let results = run_execution_commands(spec, worktree)?;
    fs::write(
        tx_dir.join("execution.json"),
        serde_json::to_string_pretty(&results)?,
    )?;
    agent_adapter::write_transcript(tx_dir, &agent_routes.executor, &results)?;
    if let Some(failed) = results.iter().find(|result| !result.success) {
        return Err(anyhow!(
            "execution command failed: `{}` exit {:?}",
            failed.command,
            failed.exit_code
        ));
    }
    Ok(())
}

pub(super) fn run_execution_commands(
    spec: &AgentSpec,
    worktree: &Path,
) -> Result<Vec<CommandResult>> {
    run_commands(
        &spec.execution.commands,
        worktree,
        spec.execution.sandbox.level,
    )
}

pub(super) fn run_repair_commands(spec: &AgentSpec, worktree: &Path) -> Result<Vec<CommandResult>> {
    run_commands(
        &spec.repair.commands,
        worktree,
        spec.execution.sandbox.level,
    )
}

fn run_commands(
    commands: &[String],
    worktree: &Path,
    sandbox_level: u8,
) -> Result<Vec<CommandResult>> {
    let mut results = Vec::new();
    for command in commands {
        let result = run_shell_with_sandbox(
            command,
            worktree,
            Duration::from_secs(300),
            CommandSandbox {
                level: sandbox_level,
            },
        )?;
        let success = result.success;
        results.push(result);
        if !success {
            break;
        }
    }
    Ok(results)
}
