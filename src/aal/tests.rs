use anyhow::Result;

use super::parse_aal;

#[test]
fn parses_prd_style_aal_into_agent_spec() -> Result<()> {
    let source = r#"
change AddCoursesPage {
  workspace code.git
  goal "Add /courses page"
  use skill code.nextjs.add_page

  allow edit:
    - "src/app/courses/**"
  deny edit:
    - "src/auth/**"
  rules:
    - R_SCOPE_ONLY
  verify:
    - command "npm run build"
    - runtime_smoke route "/courses" expect 200
  transaction:
    max_repair_attempts 3
    on_failure rollback
    on_success commit_code promote_memory
}
"#;

    let parsed = parse_aal(source)?;

    assert_eq!(parsed.spec.task.id, "add_courses_page");
    assert_eq!(parsed.spec.task.title.as_deref(), Some("Add /courses page"));
    assert_eq!(parsed.spec.workspace.kind, "code.git");
    assert_eq!(parsed.spec.skills, vec!["code.nextjs.add_page"]);
    assert_eq!(parsed.spec.scope.allow, vec!["src/app/courses/**"]);
    assert!(parsed.spec.execution.commands.is_empty());
    assert_eq!(parsed.spec.verify.commands, vec!["npm run build"]);
    assert_eq!(parsed.spec.verify.routes[0].path, "/courses");
    assert_eq!(parsed.spec.transaction.max_repair_attempts, 3);
    assert_eq!(parsed.diagnostics.len(), 1);
    Ok(())
}

#[test]
fn rejects_unknown_statement_with_line_number() {
    let error = parse_aal("change Bad {\n  mystery value\n}\n").unwrap_err();
    assert!(error.to_string().contains("error line 2"));
}
