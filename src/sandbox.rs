use std::error::Error;
use std::fmt;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::spec::AgentSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxReport {
    pub passed: bool,
    pub requested_level: u8,
    pub effective_level: u8,
    pub mode: String,
    pub reason: String,
}

#[derive(Debug)]
pub struct SandboxError {
    level: u8,
    reason: String,
}

pub fn evaluate(spec: &AgentSpec) -> SandboxReport {
    let level = spec.execution.sandbox.level;
    match level {
        0 => report(
            true,
            0,
            0,
            "local_controlled",
            "process groups and timeouts",
        ),
        1 => report(true, 1, 1, "local_sandbox", "sanitized command environment"),
        2 => report(
            false,
            2,
            0,
            "strong_isolation_required",
            "requires container, namespace, microVM, or remote runner",
        ),
        _ => report(
            false,
            level,
            0,
            "enterprise_runner_required",
            "requires enterprise isolated runner",
        ),
    }
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
) -> SandboxReport {
    SandboxReport {
        passed,
        requested_level,
        effective_level,
        mode: mode.to_string(),
        reason: reason.to_string(),
    }
}
