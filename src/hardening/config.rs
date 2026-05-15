use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceLimitConfig {
    pub timeout_secs: u64,
    pub cpu_cores: Option<f32>,
    pub memory_mb: Option<u64>,
    pub disk_mb: Option<u64>,
    pub network: String,
    pub filesystem: String,
}

#[derive(Debug, Default, Deserialize)]
struct ResourcePolicyFile {
    #[serde(default)]
    resources: Option<ResourceLimitConfig>,
}

impl Default for ResourceLimitConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 300,
            cpu_cores: None,
            memory_mb: None,
            disk_mb: None,
            network: "inherit".to_string(),
            filesystem: "workspace".to_string(),
        }
    }
}

pub fn load_resource_limits(project_root: &Path) -> Result<ResourceLimitConfig> {
    let path = project_root.join(".agent/policies/resources.yaml");
    let mut limits = if path.exists() {
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        serde_yaml::from_str::<ResourcePolicyFile>(&text)
            .with_context(|| format!("parse {}", path.display()))?
            .resources
            .unwrap_or_default()
    } else {
        ResourceLimitConfig::default()
    };
    apply_env(&mut limits);
    Ok(limits)
}

fn apply_env(limits: &mut ResourceLimitConfig) {
    if let Some(value) = env_u64("AGENTHUB_TIMEOUT_SECS") {
        limits.timeout_secs = value;
    }
    limits.cpu_cores = env_f32("AGENTHUB_CPU_CORES").or(limits.cpu_cores);
    limits.memory_mb = env_u64("AGENTHUB_MEMORY_MB").or(limits.memory_mb);
    limits.disk_mb = env_u64("AGENTHUB_DISK_MB").or(limits.disk_mb);
    limits.network =
        std::env::var("AGENTHUB_NETWORK_MODE").unwrap_or_else(|_| limits.network.clone());
    limits.filesystem =
        std::env::var("AGENTHUB_FILESYSTEM_MODE").unwrap_or_else(|_| limits.filesystem.clone());
}

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

fn env_f32(name: &str) -> Option<f32> {
    std::env::var(name).ok()?.parse().ok()
}
