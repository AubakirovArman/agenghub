use std::io::{self, Write};
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde_json::Value;

use crate::llm_gateway::{HttpProvider, LlmRequest};
use crate::product_cli::{config, providers};

use super::chat::{self, ChatSession};

type EventSink<'a> = &'a mut dyn FnMut(&Value) -> Result<()>;
type EventEmitter<'a> = Option<EventSink<'a>>;

#[derive(Debug, Clone)]
pub(super) struct AnswerOutcome {
    pub content: String,
}

pub(super) fn answer(root: &Path, session: &ChatSession, request: &str) -> Result<()> {
    let Some(provider) = select_provider(root)? else {
        println!("API provider is not configured.");
        println!("Set DEEPSEEK_API_KEY/KIMI_API_KEY or create .deepseek/.kimi, then run `/providers test deepseek` or `/providers test kimi`.");
        return Ok(());
    };
    let _ = answer_with_provider(root, session, request, provider, true, None)?;
    Ok(())
}

pub(super) fn answer_silent(
    root: &Path,
    session: &ChatSession,
    request: &str,
) -> Result<AnswerOutcome> {
    answer_silent_with_events(root, session, request, None)
}

pub(super) fn answer_silent_with_events(
    root: &Path,
    session: &ChatSession,
    request: &str,
    emit_event: EventEmitter<'_>,
) -> Result<AnswerOutcome> {
    let provider = select_provider(root)?.ok_or_else(|| {
        anyhow!(
            "API provider is not configured; set DEEPSEEK_API_KEY/KIMI_API_KEY or create .deepseek/.kimi"
        )
    })?;
    answer_with_provider(root, session, request, provider, false, emit_event)
}

fn answer_with_provider(
    _root: &Path,
    session: &ChatSession,
    request: &str,
    provider: providers::ProviderStatus,
    print_terminal: bool,
    mut emit_event: EventEmitter<'_>,
) -> Result<AnswerOutcome> {
    let api = HttpProvider::new(
        provider
            .endpoint
            .clone()
            .ok_or_else(|| anyhow!("provider endpoint missing"))?,
        providers::api_key_for_status(&provider),
        provider.model.clone(),
    );
    let prompt = prompt_for(session, request)?;
    let request_id = format!("chat-{}", Utc::now().timestamp_millis());
    let prompt_tokens = estimate_tokens(request);
    let event = chat::append_provider_requested(
        session,
        &request_id,
        &provider.info.id,
        provider.model.as_deref(),
        prompt_tokens,
    )?;
    emit(&mut emit_event, &event)?;
    let mut stream_event_error = None;
    let response = match api.complete_streaming(
        LlmRequest {
            id: request_id.clone(),
            role: "chat".to_string(),
            provider: provider.info.id.clone(),
            model: provider.model.clone(),
            prompt: Some(prompt),
            context_pack_hash: "chat".to_string(),
            prompt_hash: "chat".to_string(),
            prompt_tokens,
            response_format: None,
        },
        |delta| {
            if print_terminal {
                print!("{delta}");
                let _ = io::stdout().flush();
            }
            if stream_event_error.is_none() {
                match chat::append_assistant_delta(session, &provider.info.id, delta) {
                    Ok(event) => {
                        if let Err(error) = emit(&mut emit_event, &event) {
                            stream_event_error = Some(error);
                        }
                    }
                    Err(error) => {
                        stream_event_error = Some(error);
                    }
                }
            }
        },
    ) {
        Ok(response) => {
            let event = chat::append_provider_finished(
                session,
                &request_id,
                &provider.info.id,
                &response.status,
                prompt_tokens,
                response.completion_tokens,
                None,
            )?;
            emit(&mut emit_event, &event)?;
            response
        }
        Err(error) => {
            let reason = error.to_string();
            let event = chat::append_provider_finished(
                session,
                &request_id,
                &provider.info.id,
                "error",
                prompt_tokens,
                0,
                Some(&reason),
            )?;
            emit(&mut emit_event, &event)?;
            let event =
                chat::append_turn_finished(session, &provider.info.id, "failed", prompt_tokens, 0)?;
            emit(&mut emit_event, &event)?;
            return Err(error);
        }
    };
    if let Some(error) = stream_event_error {
        let event = chat::append_turn_finished(
            session,
            &provider.info.id,
            "failed",
            prompt_tokens,
            response.completion_tokens,
        )?;
        emit(&mut emit_event, &event)?;
        return Err(error).context("write assistant stream event");
    }
    let content = response
        .content
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "<empty response>".to_string());
    if print_terminal {
        println!();
    }
    let event = chat::append_assistant(session, &provider.info.id, &content)?;
    emit(&mut emit_event, &event)?;
    let event = chat::append_turn_finished(
        session,
        &provider.info.id,
        "succeeded",
        prompt_tokens,
        response.completion_tokens,
    )?;
    emit(&mut emit_event, &event)?;
    Ok(AnswerOutcome { content })
}

fn emit(emit_event: &mut EventEmitter<'_>, event: &Value) -> Result<()> {
    if let Some(sink) = emit_event.as_deref_mut() {
        sink(event)?;
    }
    Ok(())
}

fn select_provider(root: &Path) -> Result<Option<providers::ProviderStatus>> {
    let default = config::default_provider(root)?;
    let statuses = providers::statuses(root)?;
    let preferred = statuses
        .iter()
        .find(|status| status.info.id == default && is_api_provider(status) && status.available)
        .cloned();
    Ok(preferred.or_else(|| {
        statuses
            .into_iter()
            .find(|status| is_api_provider(status) && status.available)
    }))
}

fn is_api_provider(status: &providers::ProviderStatus) -> bool {
    matches!(status.info.id.as_str(), "deepseek" | "kimi")
}

fn prompt_for(session: &ChatSession, request: &str) -> Result<String> {
    let recent = chat::read_events(&session.path)?
        .into_iter()
        .rev()
        .filter_map(event_text)
        .take(8)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n");
    Ok(format!(
        "You are AgentHub, an API-native terminal assistant. Answer directly unless the user explicitly asks to modify files or run commands.\n\nRecent conversation:\n{recent}\n\nUser:\n{request}"
    ))
}

fn event_text(event: Value) -> Option<String> {
    let kind = event.get("kind")?.as_str()?;
    let text = event.get("text")?.as_str()?;
    match kind {
        "user_message" => Some(format!("User: {text}")),
        "assistant_message" => Some(format!("Assistant: {text}")),
        _ => None,
    }
}

fn estimate_tokens(value: &str) -> usize {
    (value.len() / 4).max(1)
}
