use anyhow::{anyhow, Result};

use crate::aal::diagnostics::AalDiagnostic;

pub(crate) fn parse_u16(line: usize, value: &str) -> Result<u16> {
    value.parse().map_err(|_| integer_error(line, value))
}

pub(crate) fn parse_u32(line: usize, value: &str) -> Result<u32> {
    value.parse().map_err(|_| integer_error(line, value))
}

pub(crate) fn parse_u64(line: usize, value: &str) -> Result<u64> {
    value.parse().map_err(|_| integer_error(line, value))
}

pub(crate) fn parse_bool(line: usize, value: &str) -> Result<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(anyhow!(
            "{}",
            AalDiagnostic::error(line, format!("expected true or false, got `{value}`")).render()
        )),
    }
}

pub(crate) fn join(tokens: &[String]) -> String {
    tokens.join(" ")
}

fn integer_error(line: usize, value: &str) -> anyhow::Error {
    anyhow!(
        "{}",
        AalDiagnostic::error(line, format!("expected integer, got `{value}`")).render()
    )
}
