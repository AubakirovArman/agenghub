use std::env;
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};

use crate::agent_dir::ensure_runtime_dirs;
use crate::enterprise::types::{ActorContext, EnterprisePolicy};

pub fn load_policy(project_root: &Path) -> Result<EnterprisePolicy> {
    let paths = ensure_runtime_dirs(project_root)?;
    let path = paths.enterprise.join("policy.yaml");
    if !path.exists() {
        return Ok(EnterprisePolicy::default());
    }
    let content = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    serde_yaml::from_str(&content).with_context(|| format!("parse {}", path.display()))
}

pub fn authorize(project_root: &Path, permission: &str) -> Result<ActorContext> {
    let policy = load_policy(project_root)?;
    let actor = actor_name();
    let role = env::var("AGENTHUB_ROLE").unwrap_or(policy.enterprise.default_role.clone());
    let permissions = policy
        .enterprise
        .roles
        .get(&role)
        .map(|role| role.permissions.clone())
        .ok_or_else(|| anyhow!("enterprise role `{role}` is not defined"))?;
    let context = ActorContext {
        actor,
        role,
        permissions,
    };
    if !policy.enterprise.enabled || context.allows(permission) {
        return Ok(context);
    }
    Err(anyhow!(
        "actor `{}` with role `{}` lacks permission `{permission}`",
        context.actor,
        context.role
    ))
}

impl ActorContext {
    pub fn allows(&self, permission: &str) -> bool {
        self.permissions
            .iter()
            .any(|item| item == "*" || item == permission)
    }
}

fn actor_name() -> String {
    env::var("AGENTHUB_ACTOR")
        .or_else(|_| env::var("USER"))
        .unwrap_or_else(|_| "local".to_string())
}
