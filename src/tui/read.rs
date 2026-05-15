use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

pub fn tail_lines(path: &Path, limit: usize) -> Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let mut lines = text
        .lines()
        .rev()
        .take(limit)
        .map(str::to_string)
        .collect::<Vec<_>>();
    lines.reverse();
    Ok(lines)
}

pub fn latest_output_tail(tx_dir: &Path, limit: usize) -> Result<Vec<String>> {
    let logs = tx_dir.join("logs");
    if logs.exists() {
        let mut files = fs::read_dir(&logs)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .collect::<Vec<_>>();
        files.sort();
        if let Some(path) = files.last() {
            return tail_lines(path, limit);
        }
    }
    execution_output_tail(&tx_dir.join("execution.json"), limit)
}

pub fn provider_label(tx_dir: &Path) -> Option<String> {
    let trace = read_json(&tx_dir.join("agent_trace.json")).ok()?;
    let routes = trace.get("routes")?.as_object()?;
    let mut providers = routes
        .values()
        .filter_map(|route| route.get("selected_adapter").and_then(Value::as_str))
        .map(str::to_string)
        .collect::<Vec<_>>();
    providers.sort();
    providers.dedup();
    (!providers.is_empty()).then(|| providers.join(","))
}

pub fn read_jsonl<T>(path: &Path) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).with_context(|| format!("parse {}", path.display())))
        .collect()
}

pub fn read_latest_jsonl(path: &Path) -> Result<Option<Value>> {
    Ok(read_jsonl::<Value>(path)?.pop())
}

pub fn count_lines(path: &Path) -> Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    Ok(fs::read_to_string(path)?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count())
}

pub fn read_json(path: &Path) -> Result<Value> {
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parse {}", path.display()))
}

pub fn array_len(value: &Value, field: &str) -> usize {
    value
        .get(field)
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0)
}

fn execution_output_tail(path: &Path, limit: usize) -> Result<Vec<String>> {
    let value = read_json(path).unwrap_or(Value::Null);
    let mut lines = Vec::new();
    if let Some(items) = value.as_array() {
        for item in items.iter().rev().take(limit) {
            push_output(item, "stdout", &mut lines);
            push_output(item, "stderr", &mut lines);
        }
    }
    Ok(lines.into_iter().rev().take(limit).collect())
}

fn push_output(item: &Value, field: &str, lines: &mut Vec<String>) {
    if let Some(value) = item.get(field).and_then(Value::as_str) {
        lines.extend(value.lines().rev().take(2).map(str::to_string));
    }
}
