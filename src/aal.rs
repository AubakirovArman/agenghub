mod builder;
mod diagnostics;
mod draft;
mod lexer;
mod parser;
mod section;
#[cfg(test)]
mod tests;
mod values;

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::spec::AgentSpec;

pub use diagnostics::{AalDiagnostic, AalSeverity};
pub use parser::parse_aal;

#[derive(Debug, Clone)]
pub struct AalParseOutput {
    pub spec: AgentSpec,
    pub diagnostics: Vec<AalDiagnostic>,
}

pub fn parse_aal_file(path: &Path) -> Result<AalParseOutput> {
    let source = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    parse_aal(&source)
}
