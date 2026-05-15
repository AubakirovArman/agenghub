use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::capabilities::{detect_capabilities, CapabilityStatus};
use super::config::{load_resource_limits, ResourceLimitConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxHardeningReport {
    pub platform: String,
    pub resource_limits: ResourceLimitConfig,
    pub capabilities: Vec<CapabilityStatus>,
    pub warnings: Vec<String>,
}

pub fn inspect(project_root: &Path) -> Result<SandboxHardeningReport> {
    let capabilities = detect_capabilities(project_root);
    let warnings = capabilities
        .iter()
        .filter(|capability| !capability.supported)
        .map(|capability| format!("{}: {}", capability.id, capability.detail))
        .collect();
    Ok(SandboxHardeningReport {
        platform: format!("{}/{}", std::env::consts::OS, std::env::consts::ARCH),
        resource_limits: load_resource_limits(project_root)?,
        capabilities,
        warnings,
    })
}
