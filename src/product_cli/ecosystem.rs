use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EcosystemSurface {
    pub protocol: &'static str,
    pub priority: &'static str,
    pub status: &'static str,
    pub scope: &'static str,
    pub transports: &'static str,
    pub policy: &'static str,
    pub gate: &'static str,
    pub next_files: &'static str,
}

pub fn surfaces() -> Vec<EcosystemSurface> {
    vec![
        EcosystemSurface {
            protocol: "mcp",
            priority: "P0",
            status: "planned_api_native_surface",
            scope: "tools,resources,prompts",
            transports: "stdio,streamable-http",
            policy: "disabled_until_explicit_registry_approval",
            gate: "after_api_native_1_0_rc",
            next_files: "src/mcp/client.rs,src/mcp/server_registry.rs,src/mcp/tool_executor.rs",
        },
        EcosystemSurface {
            protocol: "a2a",
            priority: "P0",
            status: "planned_api_native_surface",
            scope: "agent_cards,tasks,messages,artifacts",
            transports: "https,json-rpc-compatible-events",
            policy: "disabled_until_trusted_agent_card_approval",
            gate: "after_api_native_1_0_rc",
            next_files: "src/a2a/client.rs,src/a2a/agent_card.rs,src/a2a/task_manager.rs,src/a2a/discovery.rs",
        },
    ]
}

pub fn render_status(json: bool) -> String {
    let surfaces = surfaces();
    if json {
        return format!(
            "{}\n",
            serde_json::to_string_pretty(&surfaces).expect("serialize ecosystem status")
        );
    }

    let mut out = String::from("AgentHub Ecosystem Roadmap\n");
    out.push_str("phase\tpost_1_0_foundation\n");
    out.push_str("default\tno_external_protocol_connections\n");
    out.push_str("guardrail\texplicit_approval_required_before_any_mcp_or_a2a_endpoint_runs\n");
    for surface in surfaces {
        out.push_str(&format!("protocol\t{}\n", surface.protocol));
        out.push_str(&format!("priority\t{}\n", surface.priority));
        out.push_str(&format!("status\t{}\n", surface.status));
        out.push_str(&format!("scope\t{}\n", surface.scope));
        out.push_str(&format!("transports\t{}\n", surface.transports));
        out.push_str(&format!("policy\t{}\n", surface.policy));
        out.push_str(&format!("gate\t{}\n", surface.gate));
        out.push_str(&format!("next_files\t{}\n", surface.next_files));
    }
    out
}
