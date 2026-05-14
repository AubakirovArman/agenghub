use anyhow::Result;
use regex::Regex;
use serde_json::Value;

pub fn redact_text(input: &str) -> Result<String> {
    let replacements = [
        (
            r#"(?i)(api[_-]?key|token|password|secret|database_url|db_url)\s*[:=]\s*['"]?[^'"\s]+"#,
            "$1=<redacted>",
        ),
        (r#"(?i)bearer\s+[A-Za-z0-9._\-]+"#, "Bearer <redacted>"),
        (r#"sk-[A-Za-z0-9_\-]{10,}"#, "sk-<redacted>"),
        (
            r#"(?i)(postgres|postgresql|mysql|mongodb|redis)://[^'"\s]+"#,
            "$1://<redacted>",
        ),
    ];

    let mut output = input.to_string();
    for (pattern, replacement) in replacements {
        let regex = Regex::new(pattern)?;
        output = regex.replace_all(&output, replacement).to_string();
    }
    Ok(output)
}

pub fn redact_value(value: &Value) -> Result<Value> {
    match value {
        Value::String(text) => Ok(Value::String(redact_text(text)?)),
        Value::Array(items) => Ok(Value::Array(
            items.iter().map(redact_value).collect::<Result<Vec<_>>>()?,
        )),
        Value::Object(map) => {
            let mut redacted = serde_json::Map::new();
            for (key, value) in map {
                redacted.insert(key.clone(), redact_value(value)?);
            }
            Ok(Value::Object(redacted))
        }
        other => Ok(other.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_common_secret_shapes() -> Result<()> {
        let text = "token=abcd1234 Bearer secret.jwt.value postgres://user:pass@localhost/db sk-1234567890abcdef";
        let redacted = redact_text(text)?;

        assert!(!redacted.contains("abcd1234"));
        assert!(!redacted.contains("secret.jwt.value"));
        assert!(!redacted.contains("user:pass"));
        assert!(!redacted.contains("1234567890abcdef"));
        assert!(redacted.contains("<redacted>"));
        Ok(())
    }
}
