use std::path::Path;

use crate::spec::AgentSpec;

use super::format;

#[derive(Debug, Clone)]
pub(super) struct ApprovalRequest {
    pub description: String,
    pub provider_route: String,
    pub workspace: String,
    pub target_files: String,
    pub denied_files: String,
    pub verifier: String,
    pub commands: Vec<String>,
    pub action_scope: Vec<String>,
    pub protected_paths: Vec<String>,
    pub patch_preview: Vec<String>,
    pub verifier_plan: Vec<String>,
    pub rollback_receipts: Vec<String>,
    pub effects: String,
    pub estimated_cost: String,
    pub risk_level: String,
    pub risk_reason: String,
}

impl ApprovalRequest {
    pub(super) fn from_spec(
        spec: &AgentSpec,
        provider_route: String,
        verifier: String,
        commands: Vec<String>,
        effects: String,
        estimated_cost: String,
        risk: (&str, String),
    ) -> Self {
        Self {
            description: spec
                .task
                .title
                .clone()
                .unwrap_or_else(|| spec.task.id.clone()),
            provider_route,
            workspace: spec.workspace.kind.clone(),
            target_files: list_or_none(&spec.scope.allow),
            denied_files: list_or_none(&spec.scope.deny),
            verifier,
            commands,
            action_scope: action_scope(spec),
            protected_paths: protected_paths(spec),
            patch_preview: patch_preview(spec),
            verifier_plan: verifier_plan(spec),
            rollback_receipts: rollback_receipts(spec),
            effects,
            estimated_cost,
            risk_level: risk.0.to_string(),
            risk_reason: risk.1,
        }
    }
}

pub(super) fn render_card(request: &ApprovalRequest, spec_path: Option<&Path>) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}AgentHub Plan{}\n",
        format::bold_color(format::Color::Cyan),
        format::reset()
    ));
    out.push_str(&format!("provider route: {}\n\n", request.provider_route));
    out.push_str("Task\n");
    out.push_str(&format!("  {}\n\n", request.description));
    out.push_str("Plan\n");
    out.push_str(&format!("  workspace: {}\n", request.workspace));
    out.push_str(&format!("  target files: {}\n", request.target_files));
    out.push_str(&format!("  deny: {}\n", request.denied_files));
    out.push_str(&format!("  verify: {}\n", request.verifier));
    if !request.commands.is_empty() {
        out.push_str(&format!("  commands: {}\n", request.commands.join(" && ")));
    }
    out.push_str(&format!("  effects: {}\n", request.effects));
    out.push_str(&format!("  estimated cost: {}\n", request.estimated_cost));
    out.push_str(&format!(
        "  risk: {} - {}\n",
        format::status_label(&request.risk_level),
        request.risk_reason
    ));
    if let Some(path) = spec_path {
        out.push_str(&format!("  draft: {}\n", path.display()));
    }
    out.push('\n');
    out.push_str("Scope\n");
    for item in &request.action_scope {
        out.push_str(&format!("  - {item}\n"));
    }
    out.push('\n');
    out.push_str("Patch Preview\n");
    for item in &request.patch_preview {
        out.push_str(&format!("  {item}\n"));
    }
    out.push('\n');
    out.push_str("Verifier Plan\n");
    for item in &request.verifier_plan {
        out.push_str(&format!("  - {item}\n"));
    }
    out.push('\n');
    out.push_str("Rollback Receipts\n");
    for item in &request.rollback_receipts {
        out.push_str(&format!("  - {item}\n"));
    }
    out.push('\n');
    if !request.protected_paths.is_empty() {
        out.push_str("Protected Paths\n");
        for item in &request.protected_paths {
            out.push_str(&format!("  - {item}\n"));
        }
        out.push('\n');
    }
    out.push_str("Safe actions\n");
    out.push_str("  [ok] read workspace context\n");
    out.push_str("  [ok] build transaction draft\n\n");
    out.push_str("Needs approval\n");
    out.push_str(&format!("  [ ] {}\n\n", request.effects));
    out.push_str("Actions\n");
    out.push_str("  [Enter] approve once + run   [e] edit plan   [d] full draft\n");
    out.push_str(
        "  [v] verifier plan   [x] scope/diff preview   [r] rollback receipts   [q] reject\n",
    );
    out
}

pub(super) fn render_diff_preview(diff: &str) -> String {
    format::diff_from_str(diff)
}

fn list_or_none(items: &[String]) -> String {
    if items.is_empty() {
        "<none>".to_string()
    } else {
        items.join(", ")
    }
}

fn action_scope(spec: &AgentSpec) -> Vec<String> {
    let mut scope = Vec::new();
    scope.push(format!("workspace {}", spec.workspace.kind));
    scope.push(format!("allow {}", list_or_none(&spec.scope.allow)));
    scope.push(format!("deny {}", list_or_none(&spec.scope.deny)));
    scope.push(format!(
        "diff limits files:{} added:{} deleted:{}",
        spec.transaction.diff_limits.max_files_changed,
        spec.transaction.diff_limits.max_lines_added,
        spec.transaction.diff_limits.max_lines_deleted
    ));
    if spec.transaction.commit_on_success {
        scope.push("commit on success enabled".to_string());
    } else {
        scope.push("commit on success disabled".to_string());
    }
    if spec.transaction.memory_promotion != "none" {
        scope.push(format!(
            "memory promotion {}",
            spec.transaction.memory_promotion
        ));
    }
    scope
}

fn protected_paths(spec: &AgentSpec) -> Vec<String> {
    let mut paths = spec
        .scope
        .deny
        .iter()
        .filter(|item| is_protected_path(item))
        .map(|item| format!("{item} protected by scope deny"))
        .collect::<Vec<_>>();
    if spec
        .scope
        .allow
        .iter()
        .any(|item| matches!(item.as_str(), "*" | "**"))
    {
        paths.push("broad allow scope requires extra review".to_string());
    }
    paths
}

fn is_protected_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.contains(".env")
        || lower.contains(".git")
        || lower.contains("secret")
        || lower.contains("token")
        || lower.contains("credential")
        || lower.contains("key")
        || lower.contains("node_modules")
        || lower.contains("target/")
}

fn patch_preview(spec: &AgentSpec) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("# planned scope only; generated patch is available after execution".to_string());
    if spec.scope.allow.is_empty() {
        lines.push("+ no file allowlist declared".to_string());
    } else {
        for item in spec.scope.allow.iter().take(6) {
            lines.push(format!("+ may edit {item}"));
        }
    }
    if !spec.scope.deny.is_empty() {
        for item in spec.scope.deny.iter().take(6) {
            lines.push(format!("- must not edit {item}"));
        }
    }
    lines.push("after run: use /diff, /report, or /explain for actual receipts".to_string());
    lines
}

fn verifier_plan(spec: &AgentSpec) -> Vec<String> {
    let mut plan = Vec::new();
    plan.push(format!(
        "profile {}",
        spec.verify.profile.as_deref().unwrap_or("default")
    ));
    if spec.verify.commands.is_empty() {
        plan.push("commands <none>".to_string());
    } else {
        plan.extend(
            spec.verify
                .commands
                .iter()
                .map(|command| format!("command {command}")),
        );
    }
    if let Some(runtime) = &spec.verify.runtime {
        plan.push(format!(
            "runtime smoke {} timeout {}s",
            runtime.base_url, runtime.timeout_secs
        ));
        for route in &spec.verify.routes {
            plan.push(format!("route {} expects {}", route.path, route.expect));
        }
    }
    if !spec.review.commands.is_empty() {
        plan.push(format!("review commands {}", spec.review.commands.len()));
    }
    if !spec.repair.commands.is_empty() {
        plan.push(format!(
            "repair attempts up to {} with {} command(s)",
            spec.transaction.max_repair_attempts,
            spec.repair.commands.len()
        ));
    }
    plan
}

fn rollback_receipts(spec: &AgentSpec) -> Vec<String> {
    let mut receipts = Vec::new();
    if spec.transaction.rollback_on_failure {
        receipts.push("rollback on failure enabled".to_string());
    } else {
        receipts.push("rollback on failure disabled by spec".to_string());
    }
    receipts.push(".agent/tx/<tx-id>/report.md".to_string());
    receipts.push(".agent/tx/<tx-id>/diff_guard.json".to_string());
    receipts.push(".agent/tx/<tx-id>/effects.jsonl".to_string());
    receipts.push(".agent/tx/<tx-id>/journal.jsonl".to_string());
    receipts.push(
        "commands: /diff <tx-id>, /logs <tx-id>, /report <tx-id>, /explain <tx-id>".to_string(),
    );
    receipts
}

#[cfg(test)]
mod tests {
    use crate::intent;

    use super::*;

    #[test]
    fn renders_inline_approval_card() {
        let yaml = intent::normalize_to_spec("add a generated health file").agent_spec_yaml;
        let spec: AgentSpec = serde_yaml::from_str(&yaml).unwrap();
        let request = ApprovalRequest::from_spec(
            &spec,
            "command".to_string(),
            "default".to_string(),
            Vec::new(),
            "file edits".to_string(),
            "unknown".to_string(),
            ("low", "bounded".to_string()),
        );

        let output = render_card(&request, None);

        assert!(output.contains("AgentHub Plan"));
        assert!(output.contains("Scope"));
        assert!(output.contains("Patch Preview"));
        assert!(output.contains("Verifier Plan"));
        assert!(output.contains("Rollback Receipts"));
        assert!(output.contains("Needs approval"));
        assert!(output.contains("[Enter] approve once + run"));
    }
}
