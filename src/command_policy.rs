use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::spec::AgentSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPolicyReport {
    pub passed: bool,
    pub commands: Vec<CommandPolicyCommand>,
    pub violations: Vec<CommandPolicyViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPolicyCommand {
    pub stage: String,
    pub command: String,
    pub classification: String,
    pub matched_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPolicyViolation {
    pub stage: String,
    pub command: String,
    pub kind: String,
    pub matched_policy: String,
}

#[derive(Debug)]
pub struct CommandPolicyError {
    violations: Vec<CommandPolicyViolation>,
}

#[derive(Debug, Default, Deserialize)]
struct PolicyFile {
    #[serde(default)]
    commands: PolicyCommands,
}

#[derive(Debug, Default, Deserialize)]
struct PolicyCommands {
    #[serde(default)]
    safe: Vec<String>,
    #[serde(default)]
    needs_approval: Vec<String>,
    #[serde(default)]
    restricted: Vec<String>,
}

pub fn evaluate(project_root: &Path, spec: &AgentSpec) -> Result<CommandPolicyReport> {
    let policy = load_policy(project_root)?;
    let mut commands = Vec::new();
    let mut violations = Vec::new();
    for (stage, command) in collect_commands(spec) {
        let classified = classify(&policy.commands, command);
        if classified.name == "restricted"
            || (classified.name == "needs_approval" && !spec.transaction.approval_required)
        {
            violations.push(CommandPolicyViolation {
                stage: stage.to_string(),
                command: command.to_string(),
                kind: classified.name.to_string(),
                matched_policy: classified.matched.clone().unwrap_or_default(),
            });
        }
        commands.push(CommandPolicyCommand {
            stage: stage.to_string(),
            command: command.to_string(),
            classification: classified.name.to_string(),
            matched_policy: classified.matched,
        });
    }
    Ok(CommandPolicyReport {
        passed: violations.is_empty(),
        commands,
        violations,
    })
}

impl CommandPolicyReport {
    pub fn enforce(&self) -> Result<()> {
        if self.violations.is_empty() {
            return Ok(());
        }
        Err(CommandPolicyError {
            violations: self.violations.clone(),
        }
        .into())
    }
}

impl CommandPolicyError {
    pub fn requires_human(&self) -> bool {
        self.violations
            .iter()
            .all(|item| item.kind == "needs_approval")
    }
}

impl fmt::Display for CommandPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first = self.violations.first().expect("policy violation exists");
        write!(
            formatter,
            "command policy violation: {} command `{}` matched `{}`",
            first.kind, first.command, first.matched_policy
        )
    }
}

impl Error for CommandPolicyError {}

struct Classified {
    name: &'static str,
    matched: Option<String>,
}

fn load_policy(project_root: &Path) -> Result<PolicyFile> {
    let path = project_root.join(".agent/policies/core.yaml");
    if !path.is_file() {
        return Ok(PolicyFile::default());
    }
    let content = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    serde_yaml::from_str(&content).with_context(|| format!("parse {}", path.display()))
}

fn collect_commands(spec: &AgentSpec) -> Vec<(&'static str, &str)> {
    let mut commands = Vec::new();
    push_stage(&mut commands, "execution", &spec.execution.commands);
    push_stage(&mut commands, "review", &spec.review.commands);
    push_stage(&mut commands, "repair", &spec.repair.commands);
    push_stage(&mut commands, "verify", &spec.verify.commands);
    commands
}

fn push_stage<'a>(
    out: &mut Vec<(&'static str, &'a str)>,
    stage: &'static str,
    commands: &'a [String],
) {
    out.extend(commands.iter().map(|command| (stage, command.as_str())));
}

fn classify(policy: &PolicyCommands, command: &str) -> Classified {
    if let Some(pattern) = find_match(command, &policy.restricted) {
        return Classified {
            name: "restricted",
            matched: Some(pattern),
        };
    }
    if let Some(pattern) = find_match(command, &policy.needs_approval) {
        return Classified {
            name: "needs_approval",
            matched: Some(pattern),
        };
    }
    if let Some(pattern) = find_match(command, &policy.safe) {
        return Classified {
            name: "safe",
            matched: Some(pattern),
        };
    }
    Classified {
        name: "unclassified",
        matched: None,
    }
}

fn find_match(command: &str, patterns: &[String]) -> Option<String> {
    let command = command.trim();
    patterns
        .iter()
        .find(|pattern| command_matches(command, pattern))
        .cloned()
}

fn command_matches(command: &str, pattern: &str) -> bool {
    let pattern = pattern.trim();
    !pattern.is_empty()
        && (command == pattern
            || command
                .strip_prefix(pattern)
                .is_some_and(|rest| rest.starts_with(' ')))
}
