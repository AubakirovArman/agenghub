use std::path::Path;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::storage::append_jsonl;
use super::{add_inbox_candidate, memory_paths, MemoryInboxInput};
use crate::observability::redact_text;

const RECEIPT_FILE: &str = "auto_extract_receipts.jsonl";
const MAX_CANDIDATES: usize = 3;
const MIN_SIGNAL_CHARS: usize = 40;
const REVIEW_REASON: &str = "auto extracted candidate; pending inbox review required";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoMemoryExtractionInput {
    pub source: String,
    pub mode: String,
    pub domain: String,
    pub request: Option<String>,
    pub response: Option<String>,
    pub task_id: Option<String>,
    #[serde(default)]
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoMemoryExtractionReceipt {
    pub created_at: DateTime<Utc>,
    pub source: String,
    pub mode: String,
    pub domain: String,
    pub candidates_considered: usize,
    pub candidates_added: usize,
    pub inbox_ids: Vec<String>,
    pub skipped_reason: Option<String>,
    pub candidates: Vec<AutoMemoryCandidateReceipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoMemoryCandidateReceipt {
    pub inbox_id: String,
    pub kind: String,
    pub domain: String,
    pub scope: String,
    pub confidence: f32,
    pub summary: String,
}

impl AutoMemoryExtractionReceipt {
    pub fn failed(input: &AutoMemoryExtractionInput, reason: impl Into<String>) -> Self {
        Self {
            created_at: Utc::now(),
            source: input.source.clone(),
            mode: input.mode.clone(),
            domain: input.domain.clone(),
            candidates_considered: 0,
            candidates_added: 0,
            inbox_ids: Vec::new(),
            skipped_reason: Some(reason.into()),
            candidates: Vec::new(),
        }
    }
}

pub fn extract_to_inbox(
    root: &Path,
    input: AutoMemoryExtractionInput,
) -> Result<AutoMemoryExtractionReceipt> {
    let candidates = build_candidates(&input);
    let considered = candidates.len();
    let mut inbox_ids = Vec::new();
    let mut candidate_receipts = Vec::new();

    for candidate in candidates.into_iter().take(MAX_CANDIDATES) {
        let item = add_inbox_candidate(
            root,
            MemoryInboxInput {
                kind: candidate.kind.clone(),
                domain: candidate.domain.clone(),
                content: candidate.content,
                source: input.source.clone(),
                reason: Some(REVIEW_REASON.to_string()),
            },
        )?;
        inbox_ids.push(item.id.clone());
        candidate_receipts.push(AutoMemoryCandidateReceipt {
            inbox_id: item.id,
            kind: candidate.kind,
            domain: candidate.domain,
            scope: candidate.scope,
            confidence: candidate.confidence,
            summary: candidate.summary,
        });
    }

    let receipt = AutoMemoryExtractionReceipt {
        created_at: Utc::now(),
        source: input.source,
        mode: input.mode,
        domain: input.domain,
        candidates_considered: considered,
        candidates_added: inbox_ids.len(),
        inbox_ids,
        skipped_reason: (candidate_receipts.is_empty()).then(|| skipped_reason(&considered)),
        candidates: candidate_receipts,
    };
    let paths = memory_paths(root)?;
    append_jsonl(&paths.memory.join(RECEIPT_FILE), &receipt)?;
    Ok(receipt)
}

struct Candidate {
    kind: String,
    domain: String,
    scope: String,
    confidence: f32,
    summary: String,
    content: Value,
}

fn build_candidates(input: &AutoMemoryExtractionInput) -> Vec<Candidate> {
    let request = cleaned(input.request.as_deref().unwrap_or_default());
    let response = cleaned(input.response.as_deref().unwrap_or_default());
    let combined = format!("{request}\n{response}");
    if combined.trim().chars().count() < MIN_SIGNAL_CHARS {
        return Vec::new();
    }

    let lower = combined.to_lowercase();
    let mut candidates = Vec::new();
    if contains_any(
        &lower,
        &[
            "remember",
            "prefer",
            "always",
            "never",
            "use ",
            "запомни",
            "предпоч",
            "используй",
            "всегда",
            "никогда",
        ],
    ) {
        let domain = normalized_domain(input);
        candidates.push(candidate(
            input,
            "style_rule",
            domain,
            0.62,
            "durable preference or instruction",
            &request,
            &response,
        ));
    }
    if contains_any(
        &lower,
        &[
            "kubectl",
            "systemctl",
            "docker",
            "ssh",
            "nginx",
            "journalctl",
            "ufw",
            "pm2",
            "postgres",
            "server",
            "сервер",
            "деплой",
            "нагруз",
        ],
    ) {
        candidates.push(candidate(
            input,
            "runbook_step",
            "ops",
            0.58,
            "operations runbook hint",
            &request,
            &response,
        ));
    }
    if input.mode == "project"
        || !input.artifacts.is_empty()
        || contains_any(
            &lower,
            &[
                "cargo",
                "npm",
                "pytest",
                "build",
                "verifier",
                "release",
                "git",
                "tests",
                "тест",
                "релиз",
            ],
        )
    {
        let domain = if input.domain == "core" {
            "code"
        } else {
            input.domain.as_str()
        };
        candidates.push(candidate(
            input,
            "execution_learning",
            domain,
            0.56,
            "completed workflow learning",
            &request,
            &response,
        ));
    }
    candidates
}

fn candidate(
    input: &AutoMemoryExtractionInput,
    kind: &str,
    domain: &str,
    confidence: f32,
    label: &str,
    request: &str,
    response: &str,
) -> Candidate {
    let scope = format!("{}:{}", input.mode, domain);
    let summary = summarize(label, request, response);
    let content = json!({
        "summary": summary,
        "source": input.source.clone(),
        "scope": {
            "mode": input.mode.clone(),
            "domain": domain,
            "task_id": input.task_id.clone(),
        },
        "confidence": confidence,
        "evidence": {
            "request_excerpt": excerpt(request, 360),
            "response_excerpt": excerpt(response, 360),
            "artifacts": input.artifacts.clone(),
        },
        "diff": {
            "type": if input.artifacts.is_empty() { "conversation" } else { "workspace_artifacts" },
            "changed_files": input.artifacts.clone(),
        },
    });
    Candidate {
        kind: kind.to_string(),
        domain: domain.to_string(),
        scope,
        confidence,
        summary,
        content,
    }
}

fn summarize(label: &str, request: &str, response: &str) -> String {
    let text = if !request.trim().is_empty() {
        request
    } else {
        response
    };
    format!("{label}: {}", excerpt(text, 120))
}

fn normalized_domain(input: &AutoMemoryExtractionInput) -> &str {
    match input.mode.as_str() {
        "ops" => "ops",
        "project" if input.domain == "core" => "code",
        _ => input.domain.as_str(),
    }
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn cleaned(value: &str) -> String {
    let redacted = redact_text(value).unwrap_or_else(|_| value.to_string());
    redacted.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn excerpt(value: &str, max_chars: usize) -> String {
    let text = value.trim();
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut output = text.chars().take(max_chars).collect::<String>();
    output.push_str("...");
    output
}

fn skipped_reason(considered: &usize) -> String {
    if *considered == 0 {
        "no durable memory signal detected".to_string()
    } else {
        "no memory candidate added".to_string()
    }
}
