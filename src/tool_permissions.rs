use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolPermissionProfile {
    Chat,
    ReadOnly,
    WorkspaceWrite,
    OpsHost,
}

impl ToolPermissionProfile {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Chat => "chat",
            Self::ReadOnly => "read-only",
            Self::WorkspaceWrite => "workspace-write",
            Self::OpsHost => "ops-host",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolRisk {
    Low,
    Medium,
    High,
}

impl ToolRisk {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolPermissionDecision {
    pub tool: String,
    pub action: String,
    pub profile: ToolPermissionProfile,
    pub approval_required: bool,
    pub risk: ToolRisk,
    pub reason: String,
}

impl ToolPermissionDecision {
    pub fn text(&self) -> String {
        let approval = if self.approval_required {
            "approval required"
        } else {
            "approval not required"
        };
        format!(
            "{} classified as {} ({}, risk: {})",
            self.tool,
            self.profile.as_str(),
            approval,
            self.risk.as_str()
        )
    }
}

pub fn classify_shell_command(command: &str) -> ToolPermissionDecision {
    let action = command.trim();
    if action.is_empty() {
        return decision(
            "shell",
            action,
            ToolPermissionProfile::Chat,
            false,
            ToolRisk::Low,
            "empty shell input has no tool effect",
        );
    }

    let lower = normalized(action);
    if is_elevated_or_destructive(&lower) {
        return decision(
            "shell",
            action,
            profile_for_destructive_command(&lower),
            true,
            ToolRisk::High,
            "destructive or elevated command can change local or host state",
        );
    }

    if lower.starts_with("git ") {
        return classify_git(action, &lower);
    }

    if is_http_command(&lower) {
        return classify_http(action, &lower);
    }

    if is_ops_command(&lower) {
        return classify_ops(action, &lower);
    }

    if is_dependency_change(&lower) {
        return decision(
            "shell",
            action,
            ToolPermissionProfile::WorkspaceWrite,
            true,
            ToolRisk::High,
            "dependency or package changes require explicit approval",
        );
    }

    if is_read_only_command(&lower) {
        return decision(
            "shell",
            action,
            ToolPermissionProfile::ReadOnly,
            false,
            ToolRisk::Low,
            "command is recognized as read-only",
        );
    }

    if is_workspace_write_command(&lower) || writes_with_redirection(&lower) {
        return decision(
            "shell",
            action,
            ToolPermissionProfile::WorkspaceWrite,
            false,
            ToolRisk::Medium,
            "command can write workspace files but is not classified as destructive",
        );
    }

    decision(
        "shell",
        action,
        ToolPermissionProfile::WorkspaceWrite,
        false,
        ToolRisk::Medium,
        "unclassified shell command is treated as workspace-affecting until proven read-only",
    )
}

fn classify_git(action: &str, lower: &str) -> ToolPermissionDecision {
    let subcommand = nth_token(lower, 1).unwrap_or("");
    if matches!(
        subcommand,
        "status"
            | "diff"
            | "log"
            | "show"
            | "rev-parse"
            | "branch"
            | "remote"
            | "ls-files"
            | "grep"
            | "describe"
    ) {
        return decision(
            "shell",
            action,
            ToolPermissionProfile::ReadOnly,
            false,
            ToolRisk::Low,
            "git inspection command is read-only",
        );
    }

    let approval_required = lower.contains(" reset ")
        || lower.contains(" reset --hard")
        || lower.contains(" clean ")
        || lower.contains(" push --force")
        || lower.contains(" checkout ")
        || lower.contains(" restore ");
    decision(
        "shell",
        action,
        ToolPermissionProfile::WorkspaceWrite,
        approval_required,
        if approval_required {
            ToolRisk::High
        } else {
            ToolRisk::Medium
        },
        if approval_required {
            "git command can discard or publish changes"
        } else {
            "git command changes repository state"
        },
    )
}

fn classify_http(action: &str, lower: &str) -> ToolPermissionDecision {
    if http_mutates(lower) {
        return decision(
            "shell",
            action,
            ToolPermissionProfile::OpsHost,
            true,
            ToolRisk::High,
            "HTTP request can mutate an external service",
        );
    }
    if http_writes_file(lower) {
        return decision(
            "shell",
            action,
            ToolPermissionProfile::WorkspaceWrite,
            false,
            ToolRisk::Medium,
            "HTTP fetch writes a local workspace file",
        );
    }
    decision(
        "shell",
        action,
        ToolPermissionProfile::ReadOnly,
        false,
        ToolRisk::Low,
        "HTTP fetch is read-only by default",
    )
}

fn classify_ops(action: &str, lower: &str) -> ToolPermissionDecision {
    if ops_mutates(lower) {
        return decision(
            "shell",
            action,
            ToolPermissionProfile::OpsHost,
            true,
            ToolRisk::High,
            "Ops command can mutate host, container, cluster, or remote state",
        );
    }
    let approval_required = lower.starts_with("ssh ") && !ssh_remote_command_is_read_only(lower);
    decision(
        "shell",
        action,
        ToolPermissionProfile::OpsHost,
        approval_required,
        if approval_required {
            ToolRisk::Medium
        } else {
            ToolRisk::Low
        },
        if approval_required {
            "remote shell command is not recognized as read-only"
        } else {
            "Ops command is recognized as host inspection"
        },
    )
}

fn decision(
    tool: &str,
    action: &str,
    profile: ToolPermissionProfile,
    approval_required: bool,
    risk: ToolRisk,
    reason: &str,
) -> ToolPermissionDecision {
    ToolPermissionDecision {
        tool: tool.to_string(),
        action: action.to_string(),
        profile,
        approval_required,
        risk,
        reason: reason.to_string(),
    }
}

fn normalized(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn nth_token(value: &str, index: usize) -> Option<&str> {
    value.split_whitespace().nth(index)
}

fn first_token(value: &str) -> &str {
    nth_token(value, 0).unwrap_or("")
}

fn starts_with_word(value: &str, word: &str) -> bool {
    value == word
        || value
            .strip_prefix(word)
            .is_some_and(|rest| rest.starts_with(' '))
}

fn starts_with_any(value: &str, words: &[&str]) -> bool {
    words.iter().any(|word| starts_with_word(value, word))
}

fn is_elevated_or_destructive(lower: &str) -> bool {
    starts_with_any(
        lower,
        &[
            "sudo", "su", "doas", "reboot", "shutdown", "poweroff", "halt", "mkfs", "dd", "rm",
        ],
    ) || lower.contains(" rm -rf")
        || lower.contains(" chmod -r")
        || lower.contains(" chown -r")
        || lower.starts_with("terraform apply")
        || lower.starts_with("terraform destroy")
}

fn profile_for_destructive_command(lower: &str) -> ToolPermissionProfile {
    if is_ops_command(lower) || starts_with_any(lower, &["sudo", "reboot", "shutdown", "poweroff"])
    {
        ToolPermissionProfile::OpsHost
    } else {
        ToolPermissionProfile::WorkspaceWrite
    }
}

fn is_dependency_change(lower: &str) -> bool {
    starts_with_any(
        lower,
        &[
            "npm install",
            "npm i",
            "npm add",
            "pnpm install",
            "pnpm add",
            "yarn add",
            "yarn install",
            "pip install",
            "uv add",
            "uv pip install",
            "cargo add",
            "cargo update",
            "poetry add",
            "bundle install",
            "go get",
        ],
    )
}

const READ_ONLY_COMMANDS: &[&str] = &[
    "pwd", "ls", "ll", "la", "tree", "cat", "head", "tail", "sed", "awk", "rg", "grep", "find",
    "wc", "du", "df", "free", "uptime", "top", "htop", "ps", "whoami", "id", "hostname", "date",
    "env", "printenv", "which", "whereis", "command", "echo", "printf", "true", "false",
];

fn is_read_only_command(lower: &str) -> bool {
    READ_ONLY_COMMANDS.contains(&first_token(lower))
}

fn is_workspace_write_command(lower: &str) -> bool {
    starts_with_any(
        lower,
        &[
            "touch",
            "mkdir",
            "mv",
            "cp",
            "tee",
            "truncate",
            "ln",
            "patch",
            "python",
            "python3",
            "node",
            "bash",
            "sh",
            "cargo build",
            "cargo test",
            "cargo fmt",
            "npm run",
            "npm test",
            "pnpm run",
            "yarn run",
            "pytest",
            "ruff",
            "black",
            "prettier",
        ],
    )
}

fn writes_with_redirection(lower: &str) -> bool {
    lower.contains(" >") || lower.contains(" 1>") || lower.contains(" 2>") || lower.contains(">>")
}

fn is_http_command(lower: &str) -> bool {
    starts_with_any(lower, &["curl", "wget", "http", "https"])
}

fn http_mutates(lower: &str) -> bool {
    lower.contains(" -x post")
        || lower.contains(" -x put")
        || lower.contains(" -x patch")
        || lower.contains(" -x delete")
        || lower.contains(" --request post")
        || lower.contains(" --request put")
        || lower.contains(" --request patch")
        || lower.contains(" --request delete")
        || lower.contains(" -d ")
        || lower.contains(" --data")
        || lower.contains(" --form")
}

fn http_writes_file(lower: &str) -> bool {
    starts_with_word(lower, "wget")
        || lower.contains(" -o ")
        || lower.contains(" --output ")
        || lower.contains(" --output-document")
}

fn is_ops_command(lower: &str) -> bool {
    starts_with_any(
        lower,
        &[
            "ssh",
            "scp",
            "rsync",
            "kubectl",
            "helm",
            "docker",
            "docker compose",
            "systemctl",
            "service",
            "journalctl",
            "terraform",
        ],
    )
}

fn ops_mutates(lower: &str) -> bool {
    lower.starts_with("kubectl apply")
        || lower.starts_with("kubectl delete")
        || lower.starts_with("kubectl rollout restart")
        || lower.starts_with("kubectl scale")
        || lower.starts_with("helm install")
        || lower.starts_with("helm upgrade")
        || lower.starts_with("helm uninstall")
        || lower.starts_with("docker rm")
        || lower.starts_with("docker rmi")
        || lower.starts_with("docker stop")
        || lower.starts_with("docker restart")
        || lower.starts_with("docker compose up")
        || lower.starts_with("docker compose down")
        || lower.starts_with("systemctl start")
        || lower.starts_with("systemctl stop")
        || lower.starts_with("systemctl restart")
        || lower.starts_with("service ")
        || lower.starts_with("terraform apply")
        || lower.starts_with("terraform destroy")
}

fn ssh_remote_command_is_read_only(lower: &str) -> bool {
    let remote = lower
        .split_whitespace()
        .skip(2)
        .collect::<Vec<_>>()
        .join(" ");
    if remote.is_empty() {
        return false;
    }
    is_read_only_command(&remote)
        || starts_with_any(
            &remote,
            &["systemctl status", "journalctl", "docker ps", "kubectl get"],
        )
}

#[cfg(test)]
mod tests;
