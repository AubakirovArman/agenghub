use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;

use crate::agent_dir::{ensure_runtime_dirs, list_transactions};
use crate::enterprise::{list_audit, load_policy};
use crate::plugin_registry;

#[derive(Debug, Clone)]
pub struct ComplianceReportResult {
    pub path: PathBuf,
}

pub fn generate_compliance_report(
    project_root: &Path,
    output: Option<&Path>,
) -> Result<ComplianceReportResult> {
    let paths = ensure_runtime_dirs(project_root)?;
    let policy = load_policy(project_root)?;
    let plugins = plugin_registry::list_installed(project_root)?;
    let transactions = list_transactions(project_root)?;
    let audits = list_audit(project_root, 100)?;
    let path = output.map(Path::to_path_buf).unwrap_or_else(|| {
        paths.enterprise.join(format!(
            "compliance-{}.md",
            Utc::now().format("%Y%m%d%H%M%S")
        ))
    });

    let report = render_report(&policy, plugins.len(), transactions.len(), audits.len());
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(&path, report).with_context(|| format!("write {}", path.display()))?;
    Ok(ComplianceReportResult { path })
}

fn render_report(
    policy: &crate::enterprise::EnterprisePolicy,
    plugin_count: usize,
    tx_count: usize,
    audit_count: usize,
) -> String {
    let roles = policy
        .enterprise
        .roles
        .keys()
        .map(|role| format!("- {role}\n"))
        .collect::<String>();
    format!(
        "# AgentHub Compliance Report\n\nGenerated: {}\n\n## Policy\n\n- enabled: {}\n- default_role: {}\n- secrets_provider: {}\n- runner_default: {}\n\n## Roles\n\n{}## Inventory\n\n- installed_plugins: {}\n- transactions: {}\n- recent_audit_events: {}\n",
        Utc::now(),
        policy.enterprise.enabled,
        policy.enterprise.default_role,
        policy.enterprise.secrets.provider,
        policy.enterprise.runners.default,
        roles,
        plugin_count,
        tx_count,
        audit_count
    )
}
