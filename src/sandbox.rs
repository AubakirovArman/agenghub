use std::error::Error;
use std::fmt;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::command_runner::RemoteRunner;
use crate::enterprise;
use crate::hardening::SandboxHardeningReport;
use crate::spec::AgentSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxReport {
    pub passed: bool,
    pub requested_level: u8,
    pub effective_level: u8,
    pub mode: String,
    pub reason: String,
    pub runner: Option<RemoteRunner>,
    pub hardening: SandboxHardeningReport,
}

#[derive(Debug)]
pub struct SandboxError {
    level: u8,
    reason: String,
}

pub fn evaluate(project_root: &Path, spec: &AgentSpec) -> Result<SandboxReport> {
    let level = spec.execution.sandbox.level;
    let hardening = crate::hardening::inspect(project_root)?;
    let report = match level {
        0 => report(
            true,
            0,
            0,
            "local_controlled",
            "process groups and timeouts",
            None,
            hardening.clone(),
        ),
        1 => report(
            true,
            1,
            1,
            "local_sandbox",
            "sanitized command environment",
            None,
            hardening.clone(),
        ),
        2 => match select_runner(project_root, false)? {
            Some(runner) => report(
                true,
                2,
                2,
                "remote_runner",
                "remote dispatch",
                Some(runner),
                hardening.clone(),
            ),
            None => report(
                false,
                2,
                0,
                "remote_runner_required",
                "no remote runner configured",
                None,
                hardening.clone(),
            ),
        },
        _ => match select_runner(project_root, true)? {
            Some(runner) => report(
                true,
                level,
                level,
                "enterprise_runner",
                "enterprise remote dispatch",
                Some(runner),
                hardening.clone(),
            ),
            None => report(
                false,
                level,
                0,
                "enterprise_runner_required",
                "no enterprise runner configured",
                None,
                hardening,
            ),
        },
    };
    Ok(report)
}

impl SandboxReport {
    pub fn enforce(&self) -> Result<()> {
        if self.passed {
            return Ok(());
        }
        Err(SandboxError {
            level: self.requested_level,
            reason: self.reason.clone(),
        }
        .into())
    }
}

impl fmt::Display for SandboxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "sandbox level {} is not available: {}",
            self.level, self.reason
        )
    }
}

impl Error for SandboxError {}

fn report(
    passed: bool,
    requested_level: u8,
    effective_level: u8,
    mode: &str,
    reason: &str,
    runner: Option<RemoteRunner>,
    hardening: SandboxHardeningReport,
) -> SandboxReport {
    SandboxReport {
        passed,
        requested_level,
        effective_level,
        mode: mode.to_string(),
        reason: reason.to_string(),
        runner,
        hardening,
    }
}

fn select_runner(project_root: &Path, enterprise_only: bool) -> Result<Option<RemoteRunner>> {
    let inventory = enterprise::runner_inventory(project_root)?;
    let selected = inventory.remote.into_iter().find(|runner| {
        !enterprise_only
            || runner
                .labels
                .iter()
                .any(|label| matches!(label.as_str(), "enterprise" | "isolated"))
    });
    Ok(selected.map(|runner| RemoteRunner {
        id: runner.id,
        endpoint: runner.endpoint,
    }))
}
