use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

use crate::llm_gateway::{complete_with_retry, HttpProvider, LlmRequest, RetryPolicy};

use super::config;
use super::env::find_executable;

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub id: &'static str,
    pub binary: Option<&'static str>,
    pub endpoint_env: Option<&'static str>,
    pub template: Option<&'static str>,
    pub note: &'static str,
}

#[derive(Debug, Clone)]
pub struct ProviderStatus {
    pub info: ProviderInfo,
    pub available: bool,
    pub path: Option<PathBuf>,
    pub endpoint: Option<String>,
    pub is_default: bool,
}

pub fn supported() -> Vec<ProviderInfo> {
    vec![
        ProviderInfo {
            id: "command",
            binary: None,
            endpoint_env: None,
            template: None,
            note: "built-in deterministic command runner",
        },
        ProviderInfo {
            id: "codex",
            binary: Some("codex"),
            endpoint_env: None,
            template: Some("codex exec --prompt-file {prompt}"),
            note: "install the Codex CLI and make `codex` available on PATH",
        },
        ProviderInfo {
            id: "gemini",
            binary: Some("gemini"),
            endpoint_env: None,
            template: Some("gemini --prompt-file {prompt}"),
            note: "install the Gemini CLI and make `gemini` available on PATH",
        },
        ProviderInfo {
            id: "kimi",
            binary: Some("kimi"),
            endpoint_env: None,
            template: Some("kimi --prompt-file {prompt}"),
            note: "install the Kimi CLI and make `kimi` available on PATH",
        },
        ProviderInfo {
            id: "openai-http",
            binary: None,
            endpoint_env: Some("AGENTHUB_OPENAI_COMPAT_BASE_URL"),
            template: None,
            note: "set AGENTHUB_OPENAI_COMPAT_BASE_URL for an OpenAI-compatible HTTP endpoint",
        },
    ]
}

pub fn render_list() -> String {
    supported()
        .into_iter()
        .map(|item| format!("{}\t{}\n", item.id, item.note))
        .collect()
}

pub fn statuses(project_root: &Path) -> Result<Vec<ProviderStatus>> {
    let default = config::default_provider(project_root)?;
    Ok(supported()
        .into_iter()
        .map(|info| {
            let path = info.binary.and_then(find_executable);
            let endpoint = info
                .endpoint_env
                .and_then(|key| std::env::var(key).ok())
                .filter(|value| !value.is_empty());
            let available = match (info.binary, info.endpoint_env) {
                (None, None) => true,
                (Some(_), _) => path.is_some(),
                (_, Some(_)) => endpoint.is_some(),
            };
            let is_default = info.id == default;
            ProviderStatus {
                info,
                available,
                path,
                endpoint,
                is_default,
            }
        })
        .collect())
}

pub fn render_status(project_root: &Path) -> Result<String> {
    let mut out = String::new();
    for status in statuses(project_root)? {
        let state = if status.available { "ok" } else { "missing" };
        let marker = if status.is_default { "default" } else { "-" };
        let path = status
            .path
            .map(|path| path.display().to_string())
            .or(status.endpoint)
            .unwrap_or_else(|| status.info.note.to_string());
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\n",
            status.info.id, state, marker, path
        ));
    }
    Ok(out)
}

pub fn setup_provider(project_root: &Path, provider: &str) -> Result<String> {
    let status = status_for(project_root, provider)?;
    if !status.available {
        return Ok(format!(
            "missing\t{}\t{}\n",
            status.info.id, status.info.note
        ));
    }
    config::set_value(project_root, "default_provider", status.info.id)?;
    if let Some(template) = status.info.template {
        config::set_value(
            project_root,
            &format!("provider.{}.template", status.info.id),
            template,
        )?;
    }
    Ok(format!("configured\t{}\n", status.info.id))
}

pub fn test_provider(project_root: &Path, provider: &str) -> Result<String> {
    let status = status_for(project_root, provider)?;
    if status.info.id == "openai-http" {
        return test_http_provider(status);
    }
    if status.available {
        let detail = status
            .path
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "built-in".to_string());
        return Ok(format!("ok\t{}\t{}\n", status.info.id, detail));
    }
    Ok(format!(
        "missing\t{}\t{}\n",
        status.info.id, status.info.note
    ))
}

fn test_http_provider(status: ProviderStatus) -> Result<String> {
    let Some(endpoint) = status.endpoint else {
        return Ok(format!(
            "missing\t{}\t{}\n",
            status.info.id, status.info.note
        ));
    };
    let provider = HttpProvider::new(
        endpoint,
        std::env::var("AGENTHUB_OPENAI_COMPAT_API_KEY").ok(),
        std::env::var("AGENTHUB_OPENAI_COMPAT_MODEL").ok(),
    );
    let request = LlmRequest {
        id: "provider-test".to_string(),
        role: "provider_test".to_string(),
        provider: status.info.id.to_string(),
        model: None,
        prompt: Some("AgentHub provider test".to_string()),
        context_pack_hash: "provider-test".to_string(),
        prompt_hash: "provider-test".to_string(),
        prompt_tokens: 5,
        response_format: None,
    };
    let policy = RetryPolicy {
        max_attempts: 1,
        backoff_ms: Vec::new(),
    };
    let response = complete_with_retry(&provider, request, &policy, None)?;
    Ok(format!(
        "ok\t{}\tcompletion_tokens:{}\n",
        status.info.id, response.completion_tokens
    ))
}

fn status_for(project_root: &Path, provider: &str) -> Result<ProviderStatus> {
    statuses(project_root)?
        .into_iter()
        .find(|status| status.info.id == provider)
        .ok_or_else(|| anyhow!("unknown provider `{provider}`"))
}
