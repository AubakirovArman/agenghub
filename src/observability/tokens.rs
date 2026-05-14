use serde_json::Value;

pub fn estimate_tokens(value: &Value) -> usize {
    serde_json::to_string(value)
        .map(|text| (text.len() / 4).max(1))
        .unwrap_or(0)
}
