use std::collections::HashSet;

use crate::aal::diagnostics::AalDiagnostic;
use crate::aal::draft::Draft;
use crate::aal::semantic_support::warning;

pub(crate) fn validate_usage(draft: &Draft, diagnostics: &mut Vec<AalDiagnostic>) {
    let used: HashSet<_> = draft.skills.iter().map(String::as_str).collect();
    for import in &draft.imports {
        if import.kind == "skill" && !used.contains(import.id.as_str()) {
            let help = format!("add `use skill {}` or remove the import", import.id);
            diagnostics.push(
                warning(
                    "aal.import.unused_skill",
                    import.line,
                    format!("imported skill `{}` is not used", import.id),
                )
                .with_help(help),
            );
        }
    }
}
