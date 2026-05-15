use std::path::Path;
use std::process::{Command, Stdio};

use crate::domain_runtime::RuntimePack;
use crate::spec::WorkspaceProfile;

pub fn matches(
    project_root: &Path,
    workspace: WorkspaceProfile,
    verifier_profile: Option<&str>,
    pack: &RuntimePack,
) -> bool {
    pack.domain == workspace.domain()
        && (verifier_matches(verifier_profile, pack) || file_matches(project_root, pack))
}

pub fn tool_warnings(pack: &RuntimePack) -> Vec<String> {
    pack.required_tools
        .iter()
        .filter(|tool| !tool_available(tool))
        .map(|tool| format!("required tool `{tool}` was not found on PATH"))
        .collect()
}

fn verifier_matches(verifier_profile: Option<&str>, pack: &RuntimePack) -> bool {
    verifier_profile
        .map(|profile| pack.verifier_profiles.iter().any(|item| item == profile))
        .unwrap_or(false)
}

fn file_matches(project_root: &Path, pack: &RuntimePack) -> bool {
    match pack.id.as_str() {
        "code.rust" => project_root.join("Cargo.toml").exists(),
        "infra.terraform" => project_root.join("infra").exists(),
        "data.python" => project_root.join("data").exists(),
        "media.render" => project_root.join("media").exists(),
        "research.citations" => project_root.join("research").exists(),
        _ => false,
    }
}

fn tool_available(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}
