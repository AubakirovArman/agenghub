use crate::aal::diagnostics::{AalDiagnostic, AalSeverity};
use crate::aal::draft::Draft;

pub(crate) fn workspace_domain(draft: &Draft) -> &str {
    draft
        .workspace
        .as_deref()
        .unwrap_or("code.git")
        .split('.')
        .next()
        .unwrap_or("code")
}

pub(crate) fn error(code: &str, line: usize, message: impl Into<String>) -> AalDiagnostic {
    AalDiagnostic::with_code(AalSeverity::Error, code, line, message)
}

pub(crate) fn warning(code: &str, line: usize, message: impl Into<String>) -> AalDiagnostic {
    AalDiagnostic::with_code(AalSeverity::Warning, code, line, message)
}
