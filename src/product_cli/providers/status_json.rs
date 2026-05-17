use std::path::Path;

use anyhow::Result;
use serde::Serialize;

use super::{
    api_key_for_status, kimi_auth_blocker_evidence_for_status, recovery::provider_next_commands,
    status_detail, statuses, ProviderStatus,
};

#[derive(Debug, Serialize)]
pub struct ProviderStatusJson {
    pub provider: String,
    pub check_id: String,
    pub state: String,
    pub available: bool,
    pub default: bool,
    pub detail: String,
    pub endpoint: Option<String>,
    pub model: Option<String>,
    pub profile_kind: Option<String>,
    pub api_key_env: Option<String>,
    pub api_key_file: Option<String>,
    pub credential_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_classification: Option<String>,
    pub blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocker_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key_sha256_12: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_warning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub next_commands: Vec<String>,
}

pub fn render_status_json(project_root: &Path) -> Result<String> {
    let status = statuses(project_root)?
        .into_iter()
        .map(|status| {
            let state = status_state(&status).to_string();
            let auth_evidence = kimi_auth_blocker_evidence_for_status(project_root, &status);
            let credential_classification =
                credential_classification(&status, auth_evidence.as_ref());
            ProviderStatusJson {
                provider: status.info.id.clone(),
                check_id: format!("provider_{}", status.info.id),
                state: state.clone(),
                available: status.available,
                default: status.is_default,
                detail: status_detail(&status),
                endpoint: status.endpoint.clone(),
                model: status.model.clone(),
                profile_kind: status.profile_kind.clone(),
                api_key_env: status.api_key_env.clone(),
                api_key_file: status
                    .api_key_file
                    .as_ref()
                    .map(|path| path.display().to_string()),
                credential_source: credential_source(&status),
                credential_classification,
                blocked: status.state.as_deref() == Some("blocked"),
                blocker_kind: blocker_kind(&status, &state).map(str::to_string),
                auth_status: auth_evidence
                    .as_ref()
                    .map(|evidence| evidence.status.clone()),
                auth_key_sha256_12: auth_evidence
                    .as_ref()
                    .map(|evidence| evidence.auth_key_sha256_12.clone()),
                auth_key_source: auth_evidence
                    .as_ref()
                    .and_then(|evidence| evidence.auth_key_source.clone()),
                credential_warning: auth_evidence
                    .as_ref()
                    .and_then(|evidence| evidence.credential_warning.clone()),
                next_action: auth_evidence.map(|evidence| evidence.next_action),
                next_commands: provider_next_commands(&status, &state),
            }
        })
        .collect::<Vec<_>>();
    Ok(format!("{}\n", serde_json::to_string_pretty(&status)?))
}

fn status_state(status: &ProviderStatus) -> &str {
    let fallback_state = if status.available { "ok" } else { "missing" };
    status.state.as_deref().unwrap_or(fallback_state)
}

fn blocker_kind(status: &ProviderStatus, state: &str) -> Option<&'static str> {
    if state == "ok" {
        return None;
    }
    if matches!(status.info.id.as_str(), "deepseek" | "kimi") && !status.available {
        return Some("external_credential");
    }
    if status.info.id == "kimi" && state == "blocked" {
        return Some("external_credential");
    }
    if state == "blocked" {
        return Some("external_provider");
    }
    None
}

fn credential_source(status: &ProviderStatus) -> Option<String> {
    if let Some(env_name) = &status.api_key_env {
        if std::env::var(env_name)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .is_some()
        {
            return Some(format!("env:{env_name}"));
        }
    }
    status
        .api_key_file
        .as_ref()
        .map(|path| format!("file:{}", path.display()))
}

fn credential_classification(
    status: &ProviderStatus,
    auth_evidence: Option<&super::auth_evidence::KimiAuthBlockerEvidence>,
) -> Option<String> {
    if status.info.id != "kimi" {
        return None;
    }
    if let Some(evidence) = auth_evidence {
        if evidence
            .credential_warning
            .as_deref()
            .is_some_and(|warning| warning.contains("Kimi Code CLI OAuth"))
        {
            return Some("kimi_code_cli_oauth_reported".to_string());
        }
        return Some("known_auth_blocker".to_string());
    }
    let key = api_key_for_status(status)?;
    let reason = super::key_rotation::unsupported_kimi_credential_reason(&key)?;
    if reason.contains("Kimi Code CLI OAuth") {
        Some("kimi_code_cli_oauth".to_string())
    } else {
        Some("json_object".to_string())
    }
}
