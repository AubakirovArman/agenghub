use std::path::Path;

use anyhow::{anyhow, Result};

use super::actions;
use crate::{enterprise, tx_control};

#[cfg(test)]
mod tests;

pub(super) fn approve_tx(root: &Path, current_tx: Option<&str>, args: &str) -> Result<String> {
    enterprise::authorize(root, "transaction.run")?;
    let (tx_id, note) = parse_approval(root, current_tx, args)?;
    let record = tx_control::resolve(root, &tx_id, note)?;
    println!("approved {} {}", record.tx_id, record.ts);
    Ok(record.tx_id)
}

pub(super) fn resume_tx(root: &Path, tx_id: &str) -> Result<String> {
    enterprise::authorize(root, "transaction.run")?;
    let report = tx_control::resume(root, tx_id)?;
    println!(
        "resumed {} {} {}",
        report.tx_id, report.resumed_tx_id, report.status
    );
    Ok(report.resumed_tx_id)
}

fn parse_approval<'a>(
    root: &Path,
    current_tx: Option<&str>,
    args: &'a str,
) -> Result<(String, &'a str)> {
    let args = args.trim();
    if args.is_empty() {
        return Err(anyhow!("approval note is required"));
    }
    let (tx_id, note) = match args.split_once(' ') {
        Some((first, rest)) if is_tx_selector(first) => (
            actions::resolve_tx(root, Some(first), current_tx)?,
            rest.trim(),
        ),
        _ => (actions::resolve_tx(root, None, current_tx)?, args),
    };
    if note.is_empty() {
        return Err(anyhow!("approval note is required"));
    }
    Ok((tx_id, note))
}

fn is_tx_selector(value: &str) -> bool {
    matches!(value, "latest" | "last") || value.starts_with("tx-")
}
