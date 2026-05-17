use crate::observability::sha256_short;

pub fn command_target(command: &str) -> String {
    let tokens = command.split_whitespace().collect::<Vec<_>>();
    let Some(first) = tokens.first().map(|value| value.to_ascii_lowercase()) else {
        return "localhost".to_string();
    };
    match first.as_str() {
        "ssh" => ssh_target(&tokens).unwrap_or_else(|| "unknown-ssh-host".to_string()),
        "scp" | "rsync" => {
            file_transfer_target(&tokens).unwrap_or_else(|| "unknown-remote".to_string())
        }
        "systemctl" | "service" | "journalctl" | "docker" => "localhost".to_string(),
        "kubectl" | "helm" => "kubernetes-context".to_string(),
        "terraform" => "terraform-workspace".to_string(),
        _ => "localhost".to_string(),
    }
}

pub(super) fn canonical_target(target: &str) -> String {
    target
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim_end_matches(':')
        .to_ascii_lowercase()
}

pub(super) fn host_id(target: &str) -> String {
    let normalized = canonical_target(target);
    let slug = normalized
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .chars()
        .take(40)
        .collect::<String>();
    let slug = if slug.is_empty() {
        "host"
    } else {
        slug.as_str()
    };
    format!("ops-host-{slug}-{}", sha256_short(normalized.as_bytes()))
}

fn ssh_target(tokens: &[&str]) -> Option<String> {
    let mut index = 1;
    while index < tokens.len() {
        let token = tokens[index];
        if matches!(token, "-p" | "-i" | "-F" | "-l" | "-o") {
            index += 2;
            continue;
        }
        if token.starts_with('-') {
            index += 1;
            continue;
        }
        return Some(canonical_target(token));
    }
    None
}

fn file_transfer_target(tokens: &[&str]) -> Option<String> {
    tokens.iter().skip(1).find_map(|token| {
        let token = token.trim_matches('"').trim_matches('\'');
        token
            .split_once(':')
            .map(|(target, _)| target)
            .filter(|target| !target.is_empty() && !target.starts_with('/'))
            .map(canonical_target)
    })
}
