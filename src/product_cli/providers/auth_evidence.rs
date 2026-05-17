use std::path::{Path, PathBuf};

use serde_json::Value;

use super::{api_key_for_status, sha256_prefix, ProviderStatus};

#[derive(Debug, Clone)]
pub(crate) struct KimiAuthBlockerEvidence {
    pub status: String,
    pub auth_key_sha256_12: String,
    pub auth_key_source: Option<String>,
    pub credential_warning: Option<String>,
    pub next_action: String,
}

pub(crate) fn kimi_auth_blocker_note(project_root: &Path, current_key: &str) -> Option<String> {
    matching_kimi_auth_blocker_evidence(project_root, current_key).map(|evidence| {
        let warning = evidence
            .credential_warning
            .as_deref()
            .filter(|value| !value.is_empty())
            .map(|value| format!("; warning:{value}"))
            .unwrap_or_default();
        let source = evidence
            .auth_key_source
            .as_deref()
            .filter(|value| !value.is_empty())
            .map(|value| format!("; source:{value}"))
            .unwrap_or_default();
        format!(
            "latest Kimi auth check {}: key:{}{}{}; {}",
            evidence.status, evidence.auth_key_sha256_12, source, warning, evidence.next_action
        )
    })
}

pub(crate) fn kimi_auth_blocker_evidence_for_status(
    project_root: &Path,
    status: &ProviderStatus,
) -> Option<KimiAuthBlockerEvidence> {
    if status.info.id != "kimi" {
        return None;
    }
    api_key_for_status(status)
        .as_deref()
        .and_then(|key| matching_kimi_auth_blocker_evidence(project_root, key))
}

fn matching_kimi_auth_blocker_evidence(
    project_root: &Path,
    current_key: &str,
) -> Option<KimiAuthBlockerEvidence> {
    let report = read_kimi_auth_report(project_root)?;
    if report.get("provider").and_then(Value::as_str) != Some("kimi") {
        return None;
    }
    let status = report
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    if status == "passed" {
        return None;
    }
    let report_key = report
        .get("auth_key_sha256_12")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())?;
    let current_key = sha256_prefix(current_key.as_bytes());
    if report_key != current_key {
        return None;
    }
    let next_action = report
        .get("next_action")
        .and_then(Value::as_str)
        .unwrap_or("run scripts/kimi-auth-check.sh");
    let auth_key_source = report
        .get("auth_key_source")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let credential_warning = report
        .get("credential_warning")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    Some(KimiAuthBlockerEvidence {
        status: status.to_string(),
        auth_key_sha256_12: report_key.to_string(),
        auth_key_source,
        credential_warning,
        next_action: next_action.to_string(),
    })
}

fn read_kimi_auth_report(project_root: &Path) -> Option<Value> {
    let path = kimi_auth_report_path(project_root);
    std::fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
}

fn kimi_auth_report_path(project_root: &Path) -> PathBuf {
    std::env::var_os("AGENTHUB_KIMI_AUTH_REPORT")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| project_root.join("target/dogfood/kimi-auth-report.json"))
}
