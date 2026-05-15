#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AalSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AalDiagnostic {
    pub severity: AalSeverity,
    pub line: usize,
    pub message: String,
}

impl AalDiagnostic {
    pub fn warning(line: usize, message: impl Into<String>) -> Self {
        Self {
            severity: AalSeverity::Warning,
            line,
            message: message.into(),
        }
    }

    pub fn error(line: usize, message: impl Into<String>) -> Self {
        Self {
            severity: AalSeverity::Error,
            line,
            message: message.into(),
        }
    }

    pub fn render(&self) -> String {
        let level = match self.severity {
            AalSeverity::Warning => "warning",
            AalSeverity::Error => "error",
        };
        format!("{level} line {}: {}", self.line, self.message)
    }
}
