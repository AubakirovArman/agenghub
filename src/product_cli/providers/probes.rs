use std::path::PathBuf;

use super::ProviderInfo;

#[derive(Debug, Clone)]
pub struct CredentialProbe {
    pub state: &'static str,
    pub markers: Vec<String>,
}

pub fn credential_probe(info: &ProviderInfo) -> CredentialProbe {
    if info.credential_env.is_empty() && info.credential_paths.is_empty() {
        return CredentialProbe {
            state: "not_required",
            markers: Vec::new(),
        };
    }

    let mut markers = Vec::new();
    for key in info.credential_env {
        if std::env::var(key)
            .ok()
            .is_some_and(|value| !value.is_empty())
        {
            markers.push((*key).to_string());
        }
    }
    for path in info.credential_paths {
        if expand_marker_path(path).is_some_and(|path| path.exists()) {
            markers.push((*path).to_string());
        }
    }

    CredentialProbe {
        state: if markers.is_empty() {
            "cli_managed_unknown"
        } else {
            "configured"
        },
        markers,
    }
}

pub fn credential_marker_list(info: &ProviderInfo) -> String {
    info.credential_env
        .iter()
        .chain(info.credential_paths.iter())
        .copied()
        .collect::<Vec<_>>()
        .join(",")
}

fn expand_marker_path(path: &str) -> Option<PathBuf> {
    if let Some(rest) = path.strip_prefix("$HOME/") {
        return std::env::var_os("HOME").map(|home| PathBuf::from(home).join(rest));
    }
    if let Some(rest) = path.strip_prefix("$CODEX_HOME/") {
        return std::env::var_os("CODEX_HOME").map(|home| PathBuf::from(home).join(rest));
    }
    Some(PathBuf::from(path))
}
