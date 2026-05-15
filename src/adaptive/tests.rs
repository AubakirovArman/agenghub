use crate::adaptive::{apply, classify, RiskLevel, TaskClass};
use crate::spec::{
    AgentConfig, AgentSpec, ExecutionSpec, RepairSpec, ReviewSpec, RoleAgents, ScopeSpec, TaskSpec,
    TopologySpec, TransactionSpec, VerifySpec, WorkspaceSpec,
};

#[test]
fn classifier_detects_domains_and_risk() {
    let mut spec = sample_spec(
        "research_report",
        "research.git",
        "Collect research sources",
    );
    let classified = classify(&spec);
    assert_eq!(classified.task_class, TaskClass::Research);

    spec = sample_spec("auth_migration", "code.git", "Change auth migration");
    let classified = classify(&spec);
    assert_eq!(classified.task_class, TaskClass::HighRisk);
    assert_eq!(classified.risk, RiskLevel::High);
}

#[test]
fn adaptive_selection_is_opt_in() {
    let mut spec = sample_spec("add_feature", "code.git", "Add useful feature");
    let decision = apply(&mut spec);
    assert!(!decision.enabled);
    assert_eq!(spec.topology.kind, "single_executor");

    spec.topology.routing.adaptive = true;
    let decision = apply(&mut spec);
    assert!(decision.enabled);
    assert_eq!(decision.selected_topology, "manager_worker");
    assert_eq!(spec.topology.kind, "manager_worker");
}

#[test]
fn high_risk_adds_reviewer_gate_default() {
    let mut spec = sample_spec("payment_fix", "code.git", "Fix payment auth flow");
    spec.topology.routing.adaptive = true;
    let decision = apply(&mut spec);

    assert_eq!(decision.selected_topology, "executor_reviewer_repair");
    assert_eq!(spec.review.commands, vec!["true"]);
    assert!(decision
        .signals
        .contains(&"default_review_command:true".to_string()));
}

fn sample_spec(id: &str, workspace: &str, title: &str) -> AgentSpec {
    AgentSpec {
        task: TaskSpec {
            id: id.to_string(),
            kind: "code.command".to_string(),
            title: Some(title.to_string()),
            target: None,
        },
        topology: TopologySpec::default(),
        workspace: WorkspaceSpec {
            kind: workspace.to_string(),
            isolation: Some("git_worktree".to_string()),
            root: None,
        },
        execution: ExecutionSpec {
            commands: vec!["true".to_string()],
            ..ExecutionSpec::default()
        },
        scope: ScopeSpec {
            allow: vec!["generated/**".to_string()],
            deny: Vec::new(),
        },
        agent: AgentConfig::default(),
        agents: RoleAgents::default(),
        skills: Vec::new(),
        verify: VerifySpec::default(),
        review: ReviewSpec::default(),
        repair: RepairSpec::default(),
        transaction: TransactionSpec::default(),
        rules: Vec::new(),
    }
}
