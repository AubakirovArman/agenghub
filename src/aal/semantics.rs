use std::collections::HashSet;

use crate::aal::catalog;
use crate::aal::diagnostics::AalDiagnostic;
use crate::aal::draft::Draft;
use crate::aal::semantic_imports;
use crate::aal::semantic_support::{error, warning, workspace_domain};

pub(crate) fn validate(draft: &Draft) -> Vec<AalDiagnostic> {
    let mut diagnostics = Vec::new();
    validate_version(draft, &mut diagnostics);
    validate_workspace(draft, &mut diagnostics);
    validate_topology(draft, &mut diagnostics);
    validate_imports(draft, &mut diagnostics);
    semantic_imports::validate_usage(draft, &mut diagnostics);
    validate_skills(draft, &mut diagnostics);
    validate_verify_profile(draft, &mut diagnostics);
    validate_policy(draft, &mut diagnostics);
    validate_runtime(draft, &mut diagnostics);
    diagnostics
}

fn validate_version(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    if let Some(version) = draft.version.as_deref() {
        if !matches!(version, "0.1" | "0.2") {
            diagnostics.push(error(
                "aal.version.unsupported",
                0,
                format!("unsupported AAL version `{version}`"),
            ));
        }
    }
}

fn validate_workspace(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    let Some(workspace) = draft.workspace.as_deref() else {
        return;
    };
    if !catalog::WORKSPACES.contains(&workspace) {
        diagnostics.push(
            error(
                "aal.workspace.unknown",
                draft.workspace_line.unwrap_or(0),
                format!("unknown workspace `{workspace}`"),
            )
            .with_help(format!(
                "supported workspaces: {}",
                catalog::list(catalog::WORKSPACES)
            )),
        );
    }
}

fn validate_topology(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    let Some(topology) = draft.topology.as_deref() else {
        return;
    };
    if !catalog::TOPOLOGIES.contains(&topology) {
        diagnostics.push(
            error(
                "aal.topology.unknown",
                draft.topology_line.unwrap_or(0),
                format!("unknown topology `{topology}`"),
            )
            .with_help(format!(
                "supported topologies: {}",
                catalog::list(catalog::TOPOLOGIES)
            )),
        );
    }
}

fn validate_imports(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    for import in &draft.imports {
        if import.version.as_deref().is_some_and(str::is_empty) {
            diagnostics.push(error(
                "aal.import.version_empty",
                import.line,
                format!("import `{}` has an empty version", import.id),
            ));
        }
        if import.kind == "skill" {
            validate_skill_namespace(&import.id, import.line, diagnostics);
        }
    }
}

fn validate_skills(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    let workspace = workspace_domain(draft);
    for (index, skill) in draft.skills.iter().enumerate() {
        let line = draft.skill_lines.get(index).copied().unwrap_or(0);
        let Some(domain) = skill.split('.').next() else {
            continue;
        };
        if !validate_skill_namespace(skill, line, diagnostics) {
            continue;
        }
        if domain != "core" && domain != workspace {
            diagnostics.push(error(
                "aal.skill.workspace_mismatch",
                line,
                format!("skill `{skill}` is not compatible with `{workspace}.git`"),
            ));
        }
    }
}

fn validate_skill_namespace(
    skill: &str,
    line: usize,
    diagnostics: &mut Vec<AalDiagnostic>,
) -> bool {
    let Some(domain) = skill.split('.').next() else {
        return true;
    };
    if !catalog::DOMAINS.contains(&domain) && domain != "core" {
        diagnostics.push(error(
            "aal.skill.unknown",
            line,
            format!("unknown skill namespace `{domain}` in `{skill}`"),
        ));
        return false;
    }
    true
}

fn validate_verify_profile(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    let Some(profile) = draft.verify_profile.as_deref() else {
        return;
    };
    if !catalog::VERIFY_PROFILES.contains(&profile) {
        diagnostics.push(error(
            "aal.verify.unknown_profile",
            draft.verify_profile_line.unwrap_or(0),
            format!("unknown verifier profile `{profile}`"),
        ));
        return;
    }
    let workspace = workspace_domain(draft);
    if catalog::profile_domain(profile).is_some_and(|domain| domain != workspace) {
        diagnostics.push(
            warning(
                "aal.verify.workspace_mismatch",
                draft.verify_profile_line.unwrap_or(0),
                format!("verifier profile `{profile}` is usually for another workspace"),
            )
            .with_help(format!(
                "current workspace domain is `{workspace}`; choose a matching profile or change workspace"
            )),
        );
    }
}

fn validate_policy(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    let allow: HashSet<_> = draft.allow.iter().collect();
    for (index, denied) in draft.deny.iter().enumerate() {
        if allow.contains(denied) {
            diagnostics.push(error(
                "aal.policy.allow_deny_overlap",
                draft.deny_lines.get(index).copied().unwrap_or(0),
                format!("scope entry `{denied}` appears in both allow and deny"),
            ));
        }
    }
}

fn validate_runtime(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    if !draft.routes.is_empty() && draft.runtime.start_command.is_none() {
        diagnostics.push(warning(
            "aal.runtime.start_missing",
            draft.route_lines.first().copied().unwrap_or(0),
            "runtime_smoke routes are recorded but not executed until runtime_start is set",
        ));
    }
    if draft.verify_profile.as_deref() == Some("web_runtime_smoke") && draft.routes.is_empty() {
        diagnostics.push(
            warning(
                "aal.runtime.route_missing",
                draft.verify_profile_line.unwrap_or(0),
                "web_runtime_smoke profile has no runtime_smoke routes",
            )
            .with_help("add `- runtime_smoke route \"/\" expect 200` under verify"),
        );
    }
}
