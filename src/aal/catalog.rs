pub(crate) const WORKSPACES: &[&str] = &[
    "code.git",
    "content.git",
    "data.git",
    "infra.git",
    "media.git",
    "research.git",
];

pub(crate) const TOPOLOGIES: &[&str] = &[
    "single_executor",
    "planner_executor",
    "executor_reviewer_repair",
    "generator_critic",
    "swarm_research",
    "manager_worker",
    "tournament",
];

pub(crate) const DOMAINS: &[&str] = &["code", "content", "data", "infra", "media", "research"];

pub(crate) const VERIFY_PROFILES: &[&str] = &[
    "backend_tdd",
    "code_build",
    "content_quality",
    "data_quality",
    "db_migration",
    "infra_plan",
    "media_render",
    "research_report",
    "web_runtime_smoke",
];

pub(crate) fn profile_domain(profile: &str) -> Option<&'static str> {
    match profile {
        "backend_tdd" | "code_build" | "db_migration" | "web_runtime_smoke" => Some("code"),
        "content_quality" => Some("content"),
        "data_quality" => Some("data"),
        "infra_plan" => Some("infra"),
        "media_render" => Some("media"),
        "research_report" => Some("research"),
        _ => None,
    }
}

pub(crate) fn list(values: &[&str]) -> String {
    values.join(", ")
}
