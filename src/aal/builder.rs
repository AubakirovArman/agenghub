use crate::aal::draft::Draft;
use crate::spec::{
    AgentConfig, AgentSpec, ExecutionSpec, RepairSpec, ReviewSpec, RoleAgents, RuntimeSmokeSpec,
    ScopeSpec, TaskSpec, TopologySpec, VerifySpec, WorkspaceSpec,
};

pub(crate) fn build_spec(draft: &Draft) -> AgentSpec {
    let workspace = draft
        .workspace
        .clone()
        .unwrap_or_else(|| "code.git".to_string());
    AgentSpec {
        task: TaskSpec {
            id: task_id(draft.name.as_deref().unwrap_or("aal_task")),
            kind: task_kind(&workspace),
            title: draft.goal.clone(),
            target: draft.routes.first().map(|route| route.path.clone()),
        },
        agent: AgentConfig::default(),
        agents: RoleAgents::default(),
        topology: TopologySpec {
            kind: draft
                .topology
                .clone()
                .unwrap_or_else(|| "single_executor".to_string()),
            ..TopologySpec::default()
        },
        workspace: WorkspaceSpec {
            kind: workspace,
            isolation: Some("git_worktree".to_string()),
            root: None,
        },
        skills: draft.skills.clone(),
        execution: ExecutionSpec {
            commands: draft.execution_commands.clone(),
            sandbox: Default::default(),
        },
        scope: ScopeSpec {
            allow: draft.allow.clone(),
            deny: draft.deny.clone(),
        },
        rules: draft.rules.clone(),
        verify: build_verify(draft),
        review: ReviewSpec::default(),
        repair: RepairSpec::default(),
        transaction: draft.transaction.clone(),
    }
}

fn build_verify(draft: &Draft) -> VerifySpec {
    VerifySpec {
        profile: draft.verify_profile.clone(),
        commands: draft.verify_commands.clone(),
        runtime: draft
            .runtime
            .start_command
            .as_ref()
            .map(|command| RuntimeSmokeSpec {
                start_command: command.clone(),
                base_url: draft
                    .runtime
                    .base_url
                    .clone()
                    .unwrap_or_else(|| "http://127.0.0.1:3000".to_string()),
                timeout_secs: draft.runtime.timeout_secs.unwrap_or(30),
            }),
        routes: draft.routes.clone(),
    }
}

fn task_kind(workspace: &str) -> String {
    let domain = workspace.split('.').next().unwrap_or("code");
    format!("{domain}.change")
}

fn task_id(name: &str) -> String {
    let mut out = String::new();
    for (index, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() && index > 0 {
            out.push('_');
        }
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}
