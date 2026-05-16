use std::path::Path;

use anyhow::Result;

use agenthub::{enterprise, memory};
use serde_json::json;

use crate::cli::{MemoryCommands, MemoryInboxCommands};

pub fn handle_memory(project_root: &Path, command: MemoryCommands) -> Result<()> {
    enterprise::authorize(project_root, "memory.read")?;
    match command {
        MemoryCommands::Inspect => {
            let stats = memory::inspect(project_root)?;
            println!("committed: {}", stats.committed);
            println!("failed_attempts: {}", stats.failed_attempts);
        }
        MemoryCommands::Summary => print_summary(project_root)?,
        MemoryCommands::Audit => print_audit(project_root)?,
        MemoryCommands::Inbox { command } => handle_inbox(project_root, command)?,
    }
    Ok(())
}

fn print_summary(project_root: &Path) -> Result<()> {
    let summary = memory::build_summary(project_root)?;
    println!("Stack:");
    print_items(&summary.stack);
    println!("\nActive decisions:");
    print_items(&summary.active_decisions);
    println!("\nKnown failures:");
    print_items(&summary.known_failures);
    Ok(())
}

fn print_audit(project_root: &Path) -> Result<()> {
    let audit = memory::run_audit(project_root)?;
    println!("active: {}", audit.active);
    println!("stale: {}", audit.stale);
    println!("failed_attempts: {}", audit.failed_attempts);
    println!("low_confidence: {}", audit.low_confidence);
    println!(
        "missing_last_verified_commit: {}",
        audit.missing_last_verified_commit
    );
    println!(
        "conflicting_decisions: {}",
        audit.conflicting_decisions.len()
    );
    print_named("warnings", &audit.warnings);
    print_named("conflicts", &audit.conflicting_decisions);
    Ok(())
}

fn handle_inbox(project_root: &Path, command: Option<MemoryInboxCommands>) -> Result<()> {
    match command.unwrap_or(MemoryInboxCommands::List { all: false }) {
        MemoryInboxCommands::List { all } => print_inbox(project_root, all),
        MemoryInboxCommands::Add { note, domain, kind } => {
            let item = memory::add_inbox_candidate(
                project_root,
                memory::MemoryInboxInput {
                    kind,
                    domain,
                    content: json!({ "note": note, "source": "memory_inbox" }),
                    source: "cli".to_string(),
                    reason: Some("manual candidate".to_string()),
                },
            )?;
            println!("candidate: {}", item.id);
            println!("status: {}", item.status);
            Ok(())
        }
        MemoryInboxCommands::Approve { id } => {
            let item = memory::review_inbox(project_root, &id, memory::InboxDecision::Approve)?;
            println!("approved: {}", item.id);
            if let Some(memory_id) = item.memory_id {
                println!("memory: {memory_id}");
            }
            Ok(())
        }
        MemoryInboxCommands::Reject { id } => {
            let item = memory::review_inbox(project_root, &id, memory::InboxDecision::Reject)?;
            println!("rejected: {}", item.id);
            Ok(())
        }
    }
}

fn print_inbox(project_root: &Path, all: bool) -> Result<()> {
    let items = memory::list_inbox(project_root, all)?;
    println!("Memory Inbox");
    println!("items: {}", items.len());
    for item in items {
        println!(
            "{}\t{}\t{}\t{}\t{}",
            item.id,
            item.status,
            item.domain,
            item.kind,
            inbox_note(&item.content)
        );
    }
    Ok(())
}

fn inbox_note(content: &serde_json::Value) -> String {
    content
        .get("note")
        .and_then(serde_json::Value::as_str)
        .or_else(|| content.get("summary").and_then(serde_json::Value::as_str))
        .unwrap_or("")
        .replace('\n', " ")
}

fn print_named(name: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }
    println!("{name}:");
    print_items(items);
}

fn print_items(items: &[String]) {
    for item in items {
        println!("- {item}");
    }
}
