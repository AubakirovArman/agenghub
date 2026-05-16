use std::collections::VecDeque;
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result};

use super::types::{ToolExecutionStatus, ToolNetworkPolicy, ToolPathPolicy, MAX_SEARCH_FILES};

pub(super) struct ResolvedPath {
    pub(super) path: PathBuf,
    pub(super) policy: ToolPathPolicy,
}

pub(super) struct PathPolicyError {
    pub(super) status: ToolExecutionStatus,
    pub(super) policy: ToolPathPolicy,
    pub(super) reason: String,
}

pub(super) fn resolve_existing(
    root: &Path,
    relative: &str,
) -> std::result::Result<ResolvedPath, Box<PathPolicyError>> {
    if relative.contains('\0') {
        let policy = denied_path_policy(relative, None, false, false, "path contains NUL byte");
        return Err(path_policy_error(
            ToolExecutionStatus::Error,
            policy,
            "path contains NUL byte",
        ));
    }
    let raw = Path::new(relative);
    if raw.is_absolute()
        || raw
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        let reason = format!("path escapes AgentHub worktree: {relative}");
        let policy = denied_path_policy(relative, None, false, false, &reason);
        return Err(path_policy_error(
            ToolExecutionStatus::Error,
            policy,
            reason,
        ));
    }
    let root = match root.canonicalize() {
        Ok(root) => root,
        Err(error) => {
            let reason = format!("canonicalize {}: {error}", root.display());
            let policy = denied_path_policy(relative, None, false, false, &reason);
            return Err(path_policy_error(
                ToolExecutionStatus::Error,
                policy,
                reason,
            ));
        }
    };
    let candidate = root.join(raw);
    let symlink = fs::symlink_metadata(&candidate)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false);
    if symlink {
        let reason = "symlink paths require explicit approval before tool execution";
        let policy = denied_path_policy(relative, None, true, false, reason);
        return Err(path_policy_error(
            ToolExecutionStatus::ApprovalRequired,
            policy,
            reason,
        ));
    }
    let resolved = match candidate.canonicalize() {
        Ok(resolved) => resolved,
        Err(error) => {
            let reason = format!("canonicalize {}: {error}", candidate.display());
            let policy = denied_path_policy(relative, None, symlink, false, &reason);
            return Err(path_policy_error(
                ToolExecutionStatus::Error,
                policy,
                reason,
            ));
        }
    };
    if !resolved.starts_with(&root) {
        let reason = format!("path escapes AgentHub worktree: {relative}");
        let policy = denied_path_policy(relative, Some(&resolved), symlink, false, &reason);
        return Err(path_policy_error(
            ToolExecutionStatus::Error,
            policy,
            reason,
        ));
    }
    let protected = protected_path(&root, &resolved);
    if protected {
        let reason = "protected workspace path requires explicit approval before tool execution";
        let policy = denied_path_policy(relative, Some(&resolved), symlink, true, reason);
        return Err(path_policy_error(
            ToolExecutionStatus::ApprovalRequired,
            policy,
            reason,
        ));
    }
    let target_kind = target_kind(&resolved);
    Ok(ResolvedPath {
        path: resolved.clone(),
        policy: ToolPathPolicy {
            requested: relative.to_string(),
            resolved: Some(relative_display(&root, &resolved)),
            target_kind: Some(target_kind),
            within_workspace: true,
            protected: false,
            symlink,
            symlink_allowed: false,
            decision: "allowed".to_string(),
            reason: "path is inside workspace, not protected, and not a symlink".to_string(),
        },
    })
}

fn path_policy_error(
    status: ToolExecutionStatus,
    policy: ToolPathPolicy,
    reason: impl ToString,
) -> Box<PathPolicyError> {
    Box::new(PathPolicyError {
        status,
        policy,
        reason: reason.to_string(),
    })
}

fn denied_path_policy(
    requested: &str,
    resolved: Option<&Path>,
    symlink: bool,
    protected: bool,
    reason: &str,
) -> ToolPathPolicy {
    ToolPathPolicy {
        requested: requested.to_string(),
        resolved: resolved.map(|path| path.display().to_string()),
        target_kind: resolved.map(target_kind),
        within_workspace: false,
        protected,
        symlink,
        symlink_allowed: false,
        decision: "denied".to_string(),
        reason: reason.to_string(),
    }
}

pub(super) fn relative_display(root: &Path, resolved: &Path) -> String {
    resolved
        .strip_prefix(root)
        .unwrap_or(resolved)
        .display()
        .to_string()
}

fn target_kind(path: &Path) -> String {
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => "dir",
        Ok(metadata) if metadata.is_file() => "file",
        Ok(_) => "other",
        Err(_) => "missing",
    }
    .to_string()
}

fn protected_path(root: &Path, resolved: &Path) -> bool {
    let relative = resolved.strip_prefix(root).unwrap_or(resolved);
    relative.components().any(|component| {
        let name = component.as_os_str().to_string_lossy();
        is_protected_component(&name)
    })
}

fn is_protected_component(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | ".agent"
            | ".ssh"
            | ".aws"
            | ".gcloud"
            | ".kube"
            | ".docker"
            | ".npmrc"
            | ".pypirc"
            | ".netrc"
            | ".kimi"
            | ".deepseek"
    ) || name.starts_with(".env")
        || name.ends_with(".pem")
        || name.ends_with(".key")
}

pub(super) fn candidate_files(start: &Path) -> Result<Vec<PathBuf>> {
    if start.is_file() {
        return Ok(vec![start.to_path_buf()]);
    }
    let mut files = Vec::new();
    let mut queue = VecDeque::from([start.to_path_buf()]);
    while let Some(dir) = queue.pop_front() {
        for entry in fs::read_dir(&dir).with_context(|| format!("read {}", dir.display()))? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if should_skip(&name) {
                continue;
            }
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                queue.push_back(path);
            } else if file_type.is_file() {
                files.push(path);
            }
            if files.len() >= MAX_SEARCH_FILES {
                return Ok(files);
            }
        }
    }
    Ok(files)
}

fn should_skip(name: &str) -> bool {
    is_protected_component(name)
        || matches!(name, "target" | "node_modules" | ".venv" | "__pycache__")
}

pub(super) fn shell_network_policy(command: &str) -> ToolNetworkPolicy {
    let lower = normalize_shell(command);
    let blocked = starts_with_any_shell(
        &lower,
        &[
            "curl", "wget", "ssh", "scp", "sftp", "rsync", "nc", "netcat", "telnet", "ftp",
        ],
    ) || lower.contains("://")
        || lower.contains(" kubectl exec")
        || lower.contains(" docker exec");
    if blocked {
        return ToolNetworkPolicy {
            allowed: false,
            decision: "approval_required".to_string(),
            reason: "network or remote shell inspection can expose workspace or host data"
                .to_string(),
        };
    }
    ToolNetworkPolicy {
        allowed: true,
        decision: "allowed".to_string(),
        reason: "command does not match the registry network/remote denylist".to_string(),
    }
}

pub(super) fn shell_protected_path_policy(command: &str) -> Option<ToolPathPolicy> {
    let lower = normalize_shell(command);
    let protected = [
        ".env",
        ".git/",
        ".agent/",
        ".ssh",
        ".aws",
        ".gcloud",
        ".kube",
        ".docker",
        ".npmrc",
        ".pypirc",
        ".netrc",
        ".kimi",
        ".deepseek",
        ".pem",
        ".key",
    ]
    .into_iter()
    .find(|marker| lower.contains(marker));
    protected.map(|marker| {
        denied_path_policy(
            marker,
            None,
            false,
            true,
            "shell command references protected paths or secret-like files",
        )
    })
}

fn normalize_shell(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn starts_with_any_shell(value: &str, words: &[&str]) -> bool {
    words
        .iter()
        .any(|word| value == *word || value.starts_with(&format!("{word} ")))
}

pub(super) fn bound_text(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_string();
    }
    let mut out = text
        .chars()
        .take(limit.saturating_sub(32))
        .collect::<String>();
    out.push_str("\n... truncated by AgentHub ...");
    out
}
