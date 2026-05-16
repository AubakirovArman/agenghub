use std::io::{self, IsTerminal, Write};
use std::path::Path;

use anyhow::Result;
use serde_json::json;

use crate::memory::{self, TypedMemoryInput};

pub(super) fn add(root: &Path, note: &str) -> Result<()> {
    if note.trim().is_empty() {
        println!("memory note is empty");
        return Ok(());
    }
    let kind = infer_kind(note);
    println!("Add to project memory?");
    println!("Type: {kind}");
    println!("Value: {}", note.trim());
    if !confirm("Save?", true)? {
        println!("memory note skipped");
        return Ok(());
    }
    let record = memory::write_typed_fact(
        root,
        TypedMemoryInput {
            kind: kind.to_string(),
            domain: "code".to_string(),
            content: json!({ "note": note.trim(), "source": "shell" }),
            task_id: Some("manual_memory_note".to_string()),
            supersedes: None,
            confidence: Some(0.85),
        },
    )?;
    println!("memory saved {}", record.id);
    Ok(())
}

fn infer_kind(note: &str) -> &'static str {
    let lower = note.to_ascii_lowercase();
    if lower.contains("axios") || lower.contains("dependency") || lower.contains("library") {
        "dependency_policy"
    } else if lower.contains("style") || lower.contains("ui") || lower.contains("tailwind") {
        "style_rule"
    } else {
        "architecture_decision"
    }
}

fn confirm(question: &str, default_yes: bool) -> Result<bool> {
    if !io::stdin().is_terminal() {
        return Ok(default_yes);
    }
    let suffix = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("{question} {suffix} ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let answer = line.trim().to_ascii_lowercase();
    Ok(matches!(answer.as_str(), "" | "y" | "yes" | "д" | "да"))
}
