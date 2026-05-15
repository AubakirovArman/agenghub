use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::verifier::domain::common::{check, collect_files};
use crate::verifier::domain::DomainCheckResult;

pub fn research_checks(root: &Path) -> Result<Vec<DomainCheckResult>> {
    let dir = root.join("research");
    let sources = read_json(&dir.join("sources.json")).unwrap_or(Value::Null);
    let claims = read_json(&dir.join("claims.json")).unwrap_or(Value::Null);
    let graph = read_json(&dir.join("graph.json")).unwrap_or(Value::Null);
    let source_ids = source_ids(&sources);

    Ok(vec![
        sources_present(&sources, &source_ids),
        claims_cited(&claims, &source_ids),
        graph_valid(&graph),
        markdown_present("research_report_present", &dir.join("report.md")),
        markdown_present("research_critic_present", &dir.join("critic.md")),
        artifacts_present(root)?,
    ])
}

fn sources_present(sources: &Value, ids: &BTreeSet<String>) -> DomainCheckResult {
    check(
        "research_sources_present",
        sources.as_array().is_some_and(|items| !items.is_empty()) && !ids.is_empty(),
        format!("{} source id(s)", ids.len()),
    )
}

fn claims_cited(claims: &Value, source_ids: &BTreeSet<String>) -> DomainCheckResult {
    let Some(items) = claims.as_array() else {
        return check("research_claims_cited", false, "claims.json missing".into());
    };
    let mut invalid = 0;
    for claim in items {
        let cites = claim
            .get("citations")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if cites.is_empty()
            || cites
                .iter()
                .filter_map(Value::as_str)
                .any(|id| !source_ids.contains(id))
        {
            invalid += 1;
        }
    }
    check(
        "research_claims_cited",
        invalid == 0 && !items.is_empty(),
        format!("{invalid} invalid"),
    )
}

fn graph_valid(graph: &Value) -> DomainCheckResult {
    let nodes = graph
        .get("nodes")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    let edges = graph
        .get("edges")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    check(
        "research_graph_valid",
        nodes > 0,
        format!("{nodes} node(s), {edges} edge(s)"),
    )
}

fn markdown_present(name: &str, path: &Path) -> DomainCheckResult {
    let len = fs::metadata(path).map(|meta| meta.len()).unwrap_or(0);
    check(name, len > 0, format!("{len} bytes"))
}

fn artifacts_present(root: &Path) -> Result<DomainCheckResult> {
    let files = collect_files(&root.join("research"), &["json", "md", "txt"])?;
    Ok(check(
        "research_artifacts_present",
        files.len() >= 4,
        format!("{} artifact(s)", files.len()),
    ))
}

fn source_ids(sources: &Value) -> BTreeSet<String> {
    sources
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|source| source.get("id").and_then(Value::as_str))
        .map(str::to_string)
        .collect()
}

fn read_json(path: &Path) -> Result<Value> {
    let content = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("parse {}", path.display()))
}
