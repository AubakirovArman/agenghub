use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnterprisePolicy {
    pub enterprise: EnterpriseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_role")]
    pub default_role: String,
    #[serde(default)]
    pub roles: BTreeMap<String, RolePolicy>,
    #[serde(default)]
    pub secrets: SecretsPolicy,
    #[serde(default)]
    pub runners: RunnerPolicy,
    #[serde(default)]
    pub model_routing: ModelRoutingPolicy,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RolePolicy {
    #[serde(default)]
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsPolicy {
    #[serde(default = "default_secret_provider")]
    pub provider: String,
    #[serde(default)]
    pub allowed_prefixes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerPolicy {
    #[serde(default = "default_runner")]
    pub default: String,
    #[serde(default)]
    pub remote: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelRoutingPolicy {
    #[serde(default)]
    pub private_models: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ActorContext {
    pub actor: String,
    pub role: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub actor: String,
    pub role: String,
    pub action: String,
    pub permission: String,
    pub outcome: String,
    pub target: Option<String>,
    pub details: Value,
}

impl Default for EnterpriseConfig {
    fn default() -> Self {
        let mut roles = BTreeMap::new();
        roles.insert(
            "developer".to_string(),
            RolePolicy {
                permissions: vec![
                    "transaction.run".to_string(),
                    "transaction.read".to_string(),
                    "workspace.read".to_string(),
                    "memory.read".to_string(),
                    "skills.read".to_string(),
                    "plugins.read".to_string(),
                    "plugins.install".to_string(),
                ],
            },
        );
        roles.insert(
            "admin".to_string(),
            RolePolicy {
                permissions: vec!["*".to_string()],
            },
        );
        Self {
            enabled: true,
            default_role: default_role(),
            roles,
            secrets: SecretsPolicy::default(),
            runners: RunnerPolicy::default(),
            model_routing: ModelRoutingPolicy::default(),
        }
    }
}

impl Default for SecretsPolicy {
    fn default() -> Self {
        Self {
            provider: default_secret_provider(),
            allowed_prefixes: vec!["AGENTHUB_".to_string()],
        }
    }
}

impl Default for RunnerPolicy {
    fn default() -> Self {
        Self {
            default: default_runner(),
            remote: Vec::new(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_role() -> String {
    "developer".to_string()
}

fn default_secret_provider() -> String {
    "env".to_string()
}

fn default_runner() -> String {
    "local".to_string()
}
