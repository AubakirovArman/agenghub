use anyhow::Result;

use super::chat::{self, ChatSession};

pub(super) fn print_chats(root: &std::path::Path) -> Result<()> {
    for row in chat::list(root)?.into_iter().rev().take(25) {
        println!(
            "{}\tmessages:{}\ttx:{}\t{}",
            row.id, row.messages, row.txs, row.updated_at
        );
    }
    Ok(())
}

pub(super) fn print_summary(session: &ChatSession) -> Result<()> {
    let summary = chat::summarize(&session.path)?;
    println!(
        "chat {}\tmessages:{}\ttx:{}\t{}",
        summary.id, summary.messages, summary.txs, summary.updated_at
    );
    println!("transcript {}", summary.path.display());
    Ok(())
}

pub(super) fn print_messages(session: &ChatSession) -> Result<()> {
    for event in chat::read_events(&session.path)? {
        let kind = event["kind"].as_str().unwrap_or("event");
        let at = event["at"].as_str().unwrap_or("<unknown>");
        let text = event["text"].as_str().unwrap_or("");
        let tx_id = event["tx_id"].as_str().unwrap_or("");
        let path = event["path"].as_str().unwrap_or("");
        println!("{at}\t{kind}\t{text}");
        if !tx_id.is_empty() {
            println!("  tx {tx_id}");
        }
        if !path.is_empty() {
            println!("  path {path}");
        }
    }
    Ok(())
}
