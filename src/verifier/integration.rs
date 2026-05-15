use crate::observability::sha256_short;
use crate::plugin_registry;
use crate::verifier::VerifierResult;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierIntegrationArtifact {
    pub version: String,
    pub profile: Option<String>,
    pub passed: bool,
    pub checks: Vec<VerifierCheck>,
    pub fingerprints: Vec<VerifierFingerprint>,
    pub trend: VerifierTrend,
    pub plugin_compatibility: Vec<VerifierPluginCompatibility>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierCheck {
    pub id: String,
    pub category: String,
    pub name: String,
    pub status: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierFingerprint {
    pub check_id: String,
    pub fingerprint: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierTrend {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub categories: Vec<VerifierTrendCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierTrendCategory {
    pub category: String,
    pub total: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierPluginCompatibility {
    pub package: String,
    pub verifier_id: String,
    pub profiles: Vec<String>,
    pub command: String,
}

pub fn build_integration_artifact(
    project_root: &Path,
    result: &VerifierResult,
) -> Result<VerifierIntegrationArtifact> {
    let checks = checks(result);
    let fingerprints = checks
        .iter()
        .filter(|check| check.status == "failed")
        .map(fingerprint)
        .collect();
    Ok(VerifierIntegrationArtifact {
        version: "verifier.integration.v1".to_string(),
        profile: result.profile.clone(),
        passed: result.passed,
        trend: trend(&checks),
        plugin_compatibility: plugin_compatibility(project_root, result)?,
        checks,
        fingerprints,
    })
}

fn checks(result: &VerifierResult) -> Vec<VerifierCheck> {
    let mut checks = Vec::new();
    for (index, command) in result.commands.iter().enumerate() {
        checks.push(VerifierCheck {
            id: format!("command-{index}"),
            category: "command".to_string(),
            name: command.command.clone(),
            status: status(command.success),
            detail: format!(
                "exit {:?}, timeout {}",
                command.exit_code, command.timed_out
            ),
            command: Some(command.command.clone()),
        });
    }
    if let Some(domain) = &result.domain {
        for check in &domain.checks {
            checks.push(VerifierCheck {
                id: format!("domain-{}-{}", domain.profile, sanitize(&check.name)),
                category: domain.profile.clone(),
                name: check.name.clone(),
                status: status(check.success),
                detail: check.detail.clone(),
                command: None,
            });
        }
    }
    if let Some(runtime) = &result.runtime_smoke {
        for route in &runtime.checks {
            checks.push(VerifierCheck {
                id: format!("runtime-{}", sanitize(&route.path)),
                category: "runtime_smoke".to_string(),
                name: route.path.clone(),
                status: status(route.success),
                detail: format!("expected {}, actual {:?}", route.expected, route.actual),
                command: Some(runtime.start_command.clone()),
            });
        }
    }
    checks
}

fn fingerprint(check: &VerifierCheck) -> VerifierFingerprint {
    let reason = format!("{}:{}:{}", check.category, check.name, check.detail);
    VerifierFingerprint {
        check_id: check.id.clone(),
        fingerprint: sha256_short(reason.as_bytes()),
        reason,
    }
}

fn trend(checks: &[VerifierCheck]) -> VerifierTrend {
    let mut categories = Vec::new();
    for check in checks {
        if !categories
            .iter()
            .any(|item: &VerifierTrendCategory| item.category == check.category)
        {
            let total = checks
                .iter()
                .filter(|item| item.category == check.category)
                .count();
            let failed = checks
                .iter()
                .filter(|item| item.category == check.category && item.status == "failed")
                .count();
            categories.push(VerifierTrendCategory {
                category: check.category.clone(),
                total,
                failed,
            });
        }
    }
    VerifierTrend {
        total: checks.len(),
        passed: checks
            .iter()
            .filter(|check| check.status == "passed")
            .count(),
        failed: checks
            .iter()
            .filter(|check| check.status == "failed")
            .count(),
        categories,
    }
}

fn plugin_compatibility(
    project_root: &Path,
    result: &VerifierResult,
) -> Result<Vec<VerifierPluginCompatibility>> {
    let Some(profile) = result.profile.as_deref() else {
        return Ok(Vec::new());
    };
    let mut out = Vec::new();
    for plugin in plugin_registry::list_installed(project_root)? {
        for verifier in plugin.verifier_plugin_metadata {
            if verifier.profiles.iter().any(|item| item == profile) {
                out.push(VerifierPluginCompatibility {
                    package: plugin.id.clone(),
                    verifier_id: verifier.id,
                    profiles: verifier.profiles,
                    command: verifier.command,
                });
            }
        }
    }
    Ok(out)
}

fn status(success: bool) -> String {
    if success { "passed" } else { "failed" }.to_string()
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}
