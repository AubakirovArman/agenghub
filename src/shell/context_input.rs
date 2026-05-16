use std::fs;
use std::path::Path;

use anyhow::Result;

use super::actions;

#[derive(Debug, Clone)]
pub(super) struct EnrichedRequest {
    pub text: String,
    pub mentions: Vec<String>,
}

pub(super) fn enrich(root: &Path, request: &str) -> Result<EnrichedRequest> {
    let mut mentions = Vec::new();
    let mut clean = Vec::new();
    for token in request.split_whitespace() {
        if let Some(raw) = token.strip_prefix('@') {
            mentions.push(resolve_mention(root, raw)?);
        } else {
            clean.push(token);
        }
    }
    let text = if mentions.is_empty() {
        request.to_string()
    } else {
        format!(
            "{}\n\nExplicit context:\n{}",
            clean.join(" "),
            mentions.join("\n")
        )
    };
    Ok(EnrichedRequest { text, mentions })
}

fn resolve_mention(root: &Path, raw: &str) -> Result<String> {
    let raw = raw.trim_end_matches([',', '.', ';']);
    if matches!(raw, "last" | "latest") {
        return actions::latest_tx(root)
            .map(|tx| format!("- @last transaction {tx}"))
            .or_else(|_| Ok("- @last transaction <none>".to_string()));
    }
    let path = root.join(raw);
    if path.is_file() {
        let lines = fs::read_to_string(&path)
            .map(|text| text.lines().count())
            .unwrap_or(0);
        return Ok(format!("- file `{raw}` ({lines} lines)"));
    }
    if path.is_dir() {
        return Ok(format!(
            "- folder `{raw}` ({} files)",
            count_files(&path, 40)?
        ));
    }
    Ok(format!("- missing `{raw}`"))
}

fn count_files(path: &Path, limit: usize) -> Result<usize> {
    let mut count = 0;
    let mut stack = vec![path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() && count < limit {
                stack.push(path);
            } else if path.is_file() {
                count += 1;
                if count >= limit {
                    return Ok(count);
                }
            }
        }
    }
    Ok(count)
}
