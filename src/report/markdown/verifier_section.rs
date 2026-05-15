use crate::report::TransactionReport;

pub(super) fn render(md: &mut String, report: &TransactionReport) {
    let Some(verifier) = &report.verifier else {
        return;
    };
    md.push_str("\n## Verifier\n\n");
    md.push_str(&format!("- Passed: `{}`\n", verifier.passed));
    md.push_str(&format!(
        "- Profile: `{}`\n",
        verifier.profile.as_deref().unwrap_or("<none>")
    ));
    for command in &verifier.commands {
        super::command_line(md, command);
    }
    if let Some(domain) = &verifier.domain {
        md.push_str(&format!("- Domain checks: `{}`\n", domain.passed));
        for check in &domain.checks {
            md.push_str(&format!(
                "- `{}` -> success `{}` detail `{}`\n",
                check.name, check.success, check.detail
            ));
        }
    }
    if let Some(runtime) = &verifier.runtime_smoke {
        md.push_str(&format!("- Runtime smoke: `{}`\n", runtime.passed));
        for check in &runtime.checks {
            md.push_str(&format!(
                "- `{}` expected `{}` actual `{:?}`\n",
                check.path, check.expected, check.actual
            ));
        }
    }
    if let Some(integration) = &report.verifier_integration {
        md.push_str(&format!(
            "- Structured checks: `{}`\n",
            integration.trend.total
        ));
        md.push_str(&format!(
            "- Failed checks: `{}`\n",
            integration.trend.failed
        ));
        if !integration.fingerprints.is_empty() {
            md.push_str("\nVerifier fingerprints:\n\n");
            for item in &integration.fingerprints {
                md.push_str(&format!(
                    "- `{}` for `{}`\n",
                    item.fingerprint, item.check_id
                ));
            }
        }
        md.push_str("\nVerifier artifacts:\n\n");
        md.push_str("- `verifier_integration.json`\n");
    }
}
