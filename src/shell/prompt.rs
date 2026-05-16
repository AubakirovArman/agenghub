use std::path::Path;

use crate::product_cli::{config, providers};

use super::chat::ChatSession;

pub(super) fn render(root: &Path, chat: &ChatSession, tx: Option<&str>) -> String {
    let project = root
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("project");
    let provider = provider_label(root);
    let chat = short_chat(&chat.id);
    match tx {
        Some(tx) => format!("{project} {provider} {chat} {tx}> "),
        None => format!("{project} {provider} {chat}> "),
    }
}

fn provider_label(root: &Path) -> String {
    let default = config::default_provider(root).unwrap_or_else(|_| "command".to_string());
    let ready = providers::statuses(root)
        .ok()
        .and_then(|items| {
            items
                .into_iter()
                .find(|status| status.info.id == default)
                .map(|status| status.available)
        })
        .unwrap_or(false);
    if ready {
        format!("{default}✓")
    } else {
        format!("{default}?")
    }
}

fn short_chat(id: &str) -> String {
    let compact = id
        .strip_prefix("chat-")
        .unwrap_or(id)
        .chars()
        .take(18)
        .collect::<String>();
    format!("chat:{compact}")
}
