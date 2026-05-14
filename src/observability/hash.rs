use anyhow::Result;
use serde_json::Value;
use sha2::{Digest, Sha256};

pub fn sha256_json(value: &Value) -> Result<String> {
    let bytes = serde_json::to_vec(value)?;
    Ok(sha256_hex(&bytes))
}

pub fn sha256_short(bytes: &[u8]) -> String {
    sha256_hex(bytes)[..12].to_string()
}

pub fn normalize_reason(reason: &str) -> String {
    let mut normalized = reason
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    while normalized.contains("__") {
        normalized = normalized.replace("__", "_");
    }
    normalized.trim_matches('_').chars().take(40).collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
