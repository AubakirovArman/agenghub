use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRoute {
    pub requested_adapter: String,
    pub selected_adapter: String,
    pub role: String,
    pub model: Option<String>,
    pub command_template: Option<String>,
    pub dry_run: bool,
    pub routing_policy: Vec<String>,
    pub fallback_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRoutes {
    pub executor: AgentRoute,
    pub reviewer: Option<AgentRoute>,
    pub repair: Option<AgentRoute>,
}

impl AgentRoute {
    pub fn selected(
        requested: String,
        role: String,
        model: Option<String>,
        fallback_reason: Option<String>,
        dry_run: bool,
    ) -> Self {
        Self {
            requested_adapter: requested,
            selected_adapter: "command".to_string(),
            role,
            model,
            command_template: None,
            dry_run,
            routing_policy: routing_policy(),
            fallback_reason,
        }
    }

    pub fn external(
        adapter: String,
        role: String,
        model: Option<String>,
        command_template: Option<String>,
        dry_run: bool,
    ) -> Self {
        Self {
            requested_adapter: adapter.clone(),
            selected_adapter: adapter,
            role,
            model,
            command_template,
            dry_run,
            routing_policy: routing_policy(),
            fallback_reason: None,
        }
    }

    pub fn uses_external_cli(&self) -> bool {
        self.selected_adapter != "command"
    }
}

fn routing_policy() -> Vec<String> {
    vec![
        "user_preference".to_string(),
        "private_mode".to_string(),
        "executable_available".to_string(),
        "dry_run".to_string(),
    ]
}
