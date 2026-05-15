use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use chrono::Utc;
use serde_json::json;

use crate::llm_gateway::provider::LlmProvider;
use crate::llm_gateway::types::{LlmRequest, LlmResponse, RetryPolicy};
use crate::observability::{redact_text, write_jsonl};

pub fn complete_with_retry<P: LlmProvider>(
    provider: &P,
    request: LlmRequest,
    policy: &RetryPolicy,
    transcript_path: Option<&Path>,
) -> Result<LlmResponse> {
    let max_attempts = policy.max_attempts.max(1);
    let mut last_error = None;
    for attempt in 1..=max_attempts {
        let started = Instant::now();
        let result = provider.complete(request.clone());
        let duration_ms = started.elapsed().as_millis();
        match result {
            Ok(response) if response.error.is_none() && response.status != "error" => {
                record(transcript_path, attempt, &request, "ok", None, duration_ms)?;
                return Ok(response);
            }
            Ok(response) => {
                let error = response.error.unwrap_or_else(|| response.status.clone());
                record(
                    transcript_path,
                    attempt,
                    &request,
                    "error",
                    Some(&error),
                    duration_ms,
                )?;
                last_error = Some(error);
            }
            Err(error) => {
                let text = error.to_string();
                record(
                    transcript_path,
                    attempt,
                    &request,
                    "error",
                    Some(&text),
                    duration_ms,
                )?;
                last_error = Some(text);
            }
        }
        if attempt < max_attempts {
            let wait = policy
                .backoff_ms
                .get((attempt - 1) as usize)
                .copied()
                .unwrap_or(0);
            if wait > 0 {
                thread::sleep(Duration::from_millis(wait));
            }
        }
    }
    Err(anyhow!(
        "{}",
        last_error.unwrap_or_else(|| "provider call failed".to_string())
    ))
}

fn record(
    path: Option<&Path>,
    attempt: u8,
    request: &LlmRequest,
    status: &str,
    error: Option<&str>,
    duration_ms: u128,
) -> Result<()> {
    if let Some(path) = path {
        write_jsonl(
            path,
            &json!({
                "ts": Utc::now(),
                "kind": "provider_attempt",
                "request_id": request.id,
                "provider": request.provider,
                "attempt": attempt,
                "status": status,
                "error": error.map(redact_text).transpose()?,
                "duration_ms": duration_ms,
            }),
        )?;
    }
    Ok(())
}
