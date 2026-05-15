use serde::{Deserialize, Serialize};

use crate::spec::AgentSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskClass {
    SimpleEdit,
    Feature,
    Bugfix,
    Refactor,
    Research,
    Infra,
    Content,
    HighRisk,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    pub task_class: TaskClass,
    pub risk: RiskLevel,
    pub inputs: ClassificationInputs,
    pub signals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationInputs {
    pub task_id: String,
    pub task_type: String,
    pub title: Option<String>,
    pub workspace: String,
    pub verify_profile: Option<String>,
    pub allow_count: usize,
    pub command_count: usize,
}

pub fn classify(spec: &AgentSpec) -> Classification {
    let inputs = ClassificationInputs::from_spec(spec);
    let text = searchable_text(spec);
    let mut signals = Vec::new();
    let high_risk = has_any(
        &text,
        &["auth", "secret", "payment", "security", "migration"],
    );
    if high_risk {
        signals.push("high_risk_keyword".to_string());
    }
    let task_class = if high_risk {
        TaskClass::HighRisk
    } else {
        classify_domain(spec, &text, &mut signals)
    };
    let risk = risk_for(spec, task_class);
    Classification {
        task_class,
        risk,
        inputs,
        signals,
    }
}

fn classify_domain(spec: &AgentSpec, text: &str, signals: &mut Vec<String>) -> TaskClass {
    let workspace = spec.workspace.kind.as_str();
    if workspace.starts_with("research.") || text.contains("research") {
        signals.push("research_domain".to_string());
        return TaskClass::Research;
    }
    if workspace.starts_with("infra.") || has_any(text, &["terraform", "kubernetes", "docker"]) {
        signals.push("infra_domain".to_string());
        return TaskClass::Infra;
    }
    if workspace.starts_with("content.") {
        signals.push("content_domain".to_string());
        return TaskClass::Content;
    }
    if has_any(text, &["bug", "fix", "error", "failing", "broken"]) {
        signals.push("bugfix_keyword".to_string());
        return TaskClass::Bugfix;
    }
    if has_any(text, &["refactor", "cleanup", "rename", "restructure"]) {
        signals.push("refactor_keyword".to_string());
        return TaskClass::Refactor;
    }
    if has_any(text, &["add", "create", "feature", "implement"]) {
        signals.push("feature_keyword".to_string());
        return TaskClass::Feature;
    }
    if simple_edit(spec) {
        signals.push("small_scope".to_string());
        return TaskClass::SimpleEdit;
    }
    TaskClass::Unknown
}

fn risk_for(spec: &AgentSpec, task_class: TaskClass) -> RiskLevel {
    if task_class == TaskClass::HighRisk || spec.workspace.kind.starts_with("infra.") {
        RiskLevel::High
    } else if spec.scope.allow.len() > 4 || spec.execution.commands.len() > 3 {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    }
}

fn simple_edit(spec: &AgentSpec) -> bool {
    spec.scope.allow.len() <= 2 && spec.execution.commands.len() <= 2
}

fn searchable_text(spec: &AgentSpec) -> String {
    let mut values = vec![
        spec.task.id.as_str(),
        spec.task.kind.as_str(),
        spec.workspace.kind.as_str(),
    ];
    if let Some(title) = &spec.task.title {
        values.push(title);
    }
    if let Some(profile) = &spec.verify.profile {
        values.push(profile);
    }
    values.extend(spec.execution.commands.iter().map(String::as_str));
    values.join(" ").to_ascii_lowercase()
}

fn has_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

impl ClassificationInputs {
    fn from_spec(spec: &AgentSpec) -> Self {
        Self {
            task_id: spec.task.id.clone(),
            task_type: spec.task.kind.clone(),
            title: spec.task.title.clone(),
            workspace: spec.workspace.kind.clone(),
            verify_profile: spec.verify.profile.clone(),
            allow_count: spec.scope.allow.len(),
            command_count: spec.execution.commands.len(),
        }
    }
}
