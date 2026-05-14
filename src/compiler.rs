use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::spec::AgentSpec;
use crate::topology;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionDag {
    pub task_id: String,
    pub nodes: Vec<DagNode>,
    pub edges: Vec<DagEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagNode {
    pub id: String,
    pub kind: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagEdge {
    pub from: String,
    pub to: String,
}

pub fn compile(spec: &AgentSpec) -> Result<ExecutionDag> {
    validate_policy(spec)?;
    let topology_plan = topology::compile(spec)?;

    let mut nodes = vec![
        node("preflight", "policy", "Validate AgentSpec and policy"),
        node("baseline", "workspace", "Capture base revision"),
        node("workspace", "workspace", "Prepare isolated worktree"),
        node("context_pack", "context", "Build least-context pack"),
    ];

    if spec.topology.kind == "executor_reviewer_repair" {
        push_role_nodes(&mut nodes, &topology_plan.roles[..1]);
        nodes.push(node("diff_guard", "policy", "Check scope and diff limits"));
        push_role_nodes(&mut nodes, &topology_plan.roles[1..]);
    } else {
        push_role_nodes(&mut nodes, &topology_plan.roles);
        nodes.push(node("diff_guard", "policy", "Check scope and diff limits"));
    }
    nodes.extend([
        node("verify", "verifier", "Run verifier profile"),
        node(
            "sync_check",
            "workspace",
            "Check base revision did not move",
        ),
        node("commit", "transaction", "Commit or rollback"),
        node("report", "observability", "Write transaction report"),
    ]);
    let edges = nodes
        .windows(2)
        .map(|pair| DagEdge {
            from: pair[0].id.clone(),
            to: pair[1].id.clone(),
        })
        .collect();

    Ok(ExecutionDag {
        task_id: spec.task.id.clone(),
        nodes,
        edges,
    })
}

pub fn validate_policy(spec: &AgentSpec) -> Result<()> {
    for pattern in spec.scope.allow.iter().chain(spec.scope.deny.iter()) {
        validate_scope_pattern(pattern)?;
    }

    if spec.scope.allow.is_empty() && !spec.execution.commands.is_empty() {
        return Err(anyhow!(
            "scope.allow is required when execution commands can mutate the workspace"
        ));
    }
    if spec.topology.kind == "executor_reviewer_repair" && spec.review.commands.is_empty() {
        return Err(anyhow!(
            "topology executor_reviewer_repair requires review.commands"
        ));
    }

    Ok(())
}

fn validate_scope_pattern(pattern: &str) -> Result<()> {
    if pattern.trim().is_empty() {
        return Err(anyhow!("scope pattern cannot be empty"));
    }
    if pattern.starts_with('/') {
        return Err(anyhow!("scope pattern must be project-relative: {pattern}"));
    }
    if pattern
        .split('/')
        .any(|segment| segment == ".." || segment == ".")
    {
        return Err(anyhow!(
            "scope pattern cannot contain relative traversal: {pattern}"
        ));
    }
    Ok(())
}

fn node(id: &str, kind: &str, label: &str) -> DagNode {
    DagNode {
        id: id.to_string(),
        kind: kind.to_string(),
        label: label.to_string(),
    }
}

fn push_role_nodes(nodes: &mut Vec<DagNode>, roles: &[topology::TopologyRole]) {
    for role in roles {
        nodes.push(node(&role.id, &format!("agent.{}", role.role), &role.label));
    }
}
