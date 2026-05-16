use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;

use crate::chat_index;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChatUsageSummary {
    pub turns: usize,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub estimated_cost_usd: f64,
    pub providers: Vec<ProviderUsage>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProviderUsage {
    pub provider: String,
    pub turns: usize,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub estimated_cost_usd: f64,
}

impl ProviderUsage {
    fn new(provider: String) -> Self {
        Self {
            provider,
            turns: 0,
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
            estimated_cost_usd: 0.0,
        }
    }
}

pub fn summarize(root: &Path) -> Result<ChatUsageSummary> {
    let mut summary = ChatUsageSummary::default();
    let mut providers = BTreeMap::<String, ProviderUsage>::new();
    for row in chat_index::recent_events(root, 100_000)? {
        if row.event.kind != "turn_finished" {
            continue;
        }
        let provider = row
            .event
            .provider
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let prompt_tokens = row.event.prompt_tokens.unwrap_or_default();
        let completion_tokens = row.event.completion_tokens.unwrap_or_default();
        let total_tokens = row
            .event
            .total_tokens
            .unwrap_or(prompt_tokens + completion_tokens);
        let estimated_cost_usd = row.event.estimated_cost_usd.unwrap_or_default();

        summary.turns += 1;
        summary.prompt_tokens += prompt_tokens;
        summary.completion_tokens += completion_tokens;
        summary.total_tokens += total_tokens;
        summary.estimated_cost_usd += estimated_cost_usd;

        let entry = providers
            .entry(provider.clone())
            .or_insert_with(|| ProviderUsage::new(provider));
        entry.turns += 1;
        entry.prompt_tokens += prompt_tokens;
        entry.completion_tokens += completion_tokens;
        entry.total_tokens += total_tokens;
        entry.estimated_cost_usd += estimated_cost_usd;
    }
    summary.providers = providers.into_values().collect();
    Ok(summary)
}

pub fn render(root: &Path) -> Result<String> {
    Ok(render_summary(&summarize(root)?))
}

pub fn render_summary(summary: &ChatUsageSummary) -> String {
    let mut out = String::new();
    out.push_str("Chat Usage\n");
    out.push_str(&format!("turns\t{}\n", summary.turns));
    out.push_str(&format!("prompt_tokens\t{}\n", summary.prompt_tokens));
    out.push_str(&format!(
        "completion_tokens\t{}\n",
        summary.completion_tokens
    ));
    out.push_str(&format!("total_tokens\t{}\n", summary.total_tokens));
    out.push_str(&format!(
        "estimated_cost_usd\t{:.8}\n",
        summary.estimated_cost_usd
    ));
    for provider in &summary.providers {
        out.push_str(&format!(
            "provider\t{}\tturns\t{}\tprompt_tokens\t{}\tcompletion_tokens\t{}\ttotal_tokens\t{}\testimated_cost_usd\t{:.8}\n",
            provider.provider,
            provider.turns,
            provider.prompt_tokens,
            provider.completion_tokens,
            provider.total_tokens,
            provider.estimated_cost_usd
        ));
    }
    out
}

#[cfg(test)]
mod tests;
