use std::path::Path;

use anyhow::Result;

use super::{
    audit::build_report,
    types::{
        AuditOptions, AuditRenderResult, ReadinessBlocker, ReadinessBlockerReport, ReadinessCheck,
        ReadinessSources,
    },
};

pub fn render_blockers(project_root: &Path, options: AuditOptions) -> Result<AuditRenderResult> {
    let report = build_report(project_root, options.no_refresh)?;
    let blockers = report
        .checks
        .iter()
        .filter(|check| check.status != "passed")
        .map(blocker_from_check)
        .collect::<Vec<_>>();
    let failed = !blockers.is_empty();
    let blocker_report = ReadinessBlockerReport {
        objective: report.objective,
        status: if failed { "blocked" } else { "clear" }.to_string(),
        failed,
        sources: ReadinessSources {
            api_native_plan: report.sources.api_native_plan,
            post_1_0_plan: report.sources.post_1_0_plan,
            repo_roadmap: report.sources.repo_roadmap,
        },
        evidence: report.evidence,
        dogfood_history: report.dogfood_history,
        kimi_auth_report: report.kimi_auth_report,
        metrics: report.metrics,
        blockers,
        next: if failed { report.next } else { Vec::new() },
    };
    let output = if options.json {
        format!("{}\n", serde_json::to_string_pretty(&blocker_report)?)
    } else {
        render_blockers_text(&blocker_report)
    };
    Ok(AuditRenderResult { output, failed })
}

fn render_blockers_text(report: &ReadinessBlockerReport) -> String {
    let mut out = String::new();
    out.push_str("AgentHub readiness blockers\n");
    out.push_str(&format!("objective\t{}\n", report.objective));
    out.push_str(&format!("evidence\t{}\n", report.evidence));
    out.push_str(&format!("dogfood_history\t{}\n", report.dogfood_history));
    out.push_str(&format!("kimi_auth_report\t{}\n", report.kimi_auth_report));
    out.push_str(&format!(
        "metrics\treal_sessions\t{}/{}\n",
        report.metrics.real_sessions, report.metrics.required_sessions
    ));
    out.push_str(&format!(
        "metrics\tops_flows\t{}/{}\n",
        report.metrics.ops_flows, report.metrics.required_ops_flows
    ));
    out.push_str(&format!(
        "metrics\tproject_edit_flows\t{}/{}\n",
        report.metrics.project_edit_flows, report.metrics.required_project_edit_flows
    ));
    out.push_str(&format!(
        "metrics\tcost_receipts\t{}/{}\n",
        report.metrics.cost_receipts, report.metrics.required_cost_receipts
    ));
    out.push_str(&format!(
        "metrics\topen_blockers\t{}\n",
        report.metrics.open_blockers
    ));
    if report.blockers.is_empty() {
        out.push_str("blockers\tclear\n");
    } else {
        for blocker in &report.blockers {
            out.push_str(&format!(
                "blocker\t{}\t{}\t{}\n",
                blocker.id, blocker.status, blocker.detail
            ));
            for (index, command) in blocker.next_commands.iter().enumerate() {
                out.push_str(&format!(
                    "blocker_next\t{}\t{}\t{}\n",
                    blocker.id,
                    index + 1,
                    command
                ));
            }
        }
    }
    out.push_str(&format!("status\t{}\n", report.status));
    for (index, command) in report.next.iter().enumerate() {
        out.push_str(&format!("next\t{}\t{}\n", index + 1, command));
    }
    out
}

fn blocker_from_check(check: &ReadinessCheck) -> ReadinessBlocker {
    ReadinessBlocker {
        id: check.id.clone(),
        status: check.status.clone(),
        detail: check.detail.clone(),
        next_commands: blocker_next_commands(&check.id, &check.detail),
    }
}

fn blocker_next_commands(id: &str, detail: &str) -> Vec<String> {
    if id == "kimi_auth" {
        return vec![
            "agenthub providers preflight-key kimi --from-file <new-key-file>".to_string(),
            "agenthub providers rc-unblock kimi --from-file <new-key-file>".to_string(),
            "agenthub providers test kimi".to_string(),
            "scripts/kimi-auth-check.sh".to_string(),
        ];
    }
    if id == "provider_kimi" {
        return vec![
            "agenthub providers preflight-key kimi --from-file <new-key-file>".to_string(),
            "agenthub providers rc-unblock kimi --from-file <new-key-file>".to_string(),
            "AGENTHUB_PROVIDER_DOGFOOD_PROVIDER=kimi AGENTHUB_PROVIDER_DOGFOOD_LIVE=1 scripts/provider-dogfood.sh".to_string(),
        ];
    }
    if id == "open_blockers" {
        let mut commands = vec![
            "scripts/rc-evidence-collect.sh".to_string(),
            "agenthub readiness blockers --json --check".to_string(),
        ];
        if detail.contains("kimi-auth") {
            commands.insert(
                0,
                "agenthub providers rc-unblock kimi --from-file <new-key-file>".to_string(),
            );
        }
        return commands;
    }
    if id == "rc_dogfood_gate" {
        return vec![
            "agenthub readiness blockers --json --check".to_string(),
            "scripts/rc-evidence-collect.sh".to_string(),
            "scripts/rc-dogfood-gate.sh --check".to_string(),
        ];
    }
    if let Some(provider) = id.strip_prefix("provider_") {
        return vec![format!("agenthub providers test {provider}")];
    }
    if id.starts_with("rc_check_") {
        return vec![
            "scripts/rc-evidence-collect.sh".to_string(),
            "scripts/rc-dogfood-gate.sh --check".to_string(),
        ];
    }
    Vec::new()
}
