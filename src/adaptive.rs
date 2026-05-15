mod classifier;
pub mod scoreboard;
#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::spec::AgentSpec;

pub use classifier::{classify, Classification, ClassificationInputs, RiskLevel, TaskClass};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveDecision {
    pub enabled: bool,
    pub deterministic: bool,
    pub original_topology: String,
    pub selected_topology: String,
    pub task_class: TaskClass,
    pub risk: RiskLevel,
    pub inputs: ClassificationInputs,
    pub signals: Vec<String>,
    pub explanation: String,
    pub model: Option<String>,
}

pub fn apply(spec: &mut AgentSpec) -> AdaptiveDecision {
    let classification = classify(spec);
    let original = spec.topology.kind.clone();
    let mut decision = decision_for(spec, classification, original);
    if !decision.enabled {
        return decision;
    }
    spec.topology.kind = decision.selected_topology.clone();
    adjust_topology_defaults(spec, &mut decision);
    decision
}

fn decision_for(
    spec: &AgentSpec,
    classification: Classification,
    original: String,
) -> AdaptiveDecision {
    if !spec.topology.routing.adaptive {
        return AdaptiveDecision {
            enabled: false,
            deterministic: true,
            selected_topology: original.clone(),
            original_topology: original,
            task_class: classification.task_class,
            risk: classification.risk,
            inputs: classification.inputs,
            signals: classification.signals,
            explanation: "adaptive routing disabled; using configured topology".to_string(),
            model: None,
        };
    }
    let selected = select_topology(classification.task_class, classification.risk);
    AdaptiveDecision {
        enabled: true,
        deterministic: false,
        original_topology: original,
        selected_topology: selected.to_string(),
        task_class: classification.task_class,
        risk: classification.risk,
        inputs: classification.inputs,
        signals: classification.signals,
        explanation: format!(
            "adaptive routing selected `{selected}` for `{:?}` with `{:?}` risk",
            classification.task_class, classification.risk
        ),
        model: None,
    }
}

fn select_topology(task_class: TaskClass, risk: RiskLevel) -> &'static str {
    if task_class == TaskClass::HighRisk || risk == RiskLevel::High {
        return "executor_reviewer_repair";
    }
    match task_class {
        TaskClass::SimpleEdit => "single_executor",
        TaskClass::Bugfix | TaskClass::Infra | TaskClass::Unknown => "planner_executor",
        TaskClass::Research => "swarm_research",
        TaskClass::Content => "generator_critic",
        TaskClass::Feature | TaskClass::Refactor => "manager_worker",
        TaskClass::HighRisk => "executor_reviewer_repair",
    }
}

fn adjust_topology_defaults(spec: &mut AgentSpec, decision: &mut AdaptiveDecision) {
    if matches!(
        decision.selected_topology.as_str(),
        "manager_worker" | "swarm_research"
    ) {
        spec.topology.swarm_size = spec.topology.swarm_size.max(2);
    }
    if decision.selected_topology == "executor_reviewer_repair" && spec.review.commands.is_empty() {
        spec.review.commands.push("true".to_string());
        decision
            .signals
            .push("default_review_command:true".to_string());
    }
}
