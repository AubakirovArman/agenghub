use super::file_request::FileRequest;
use super::ResolvedDefaults;

pub(super) fn agent_spec_yaml(
    route: &str,
    defaults: &ResolvedDefaults,
    approval_required: bool,
) -> String {
    let task_id = format!(
        "add_{}_page",
        route.trim_start_matches('/').replace('/', "_")
    );
    let route_dir = route.trim_start_matches('/');
    let approval = if approval_required {
        "  approval_required: true\n"
    } else {
        ""
    };

    format!(
        r#"task:
  id: {task_id}
  type: code.add_page
  title: Add {route} page
  target: {route}

agent:
  adapter: {adapter}
  role: {role}

workspace:
  type: {workspace_type}
  isolation: {workspace_isolation}

skills:
  - code.nextjs.add_page
  - verifier.web_runtime_smoke

execution:
  commands: []

scope:
  allow:
    - src/app/{route_dir}/**
    - src/components/**
  deny:
    - .agent/**
    - .env*

rules:
  - R_SCOPE_ONLY
  - R_REUSE_FIRST

verify:
  profile: {verify_profile}
  commands:
    - npm run build

transaction:
{approval}  max_repair_attempts: {max_repair_attempts}
  rollback_on_failure: true
  commit_on_success: {commit_on_success}
  memory_promotion: {memory_promotion}
"#,
        adapter = defaults.agent_adapter.as_str(),
        role = defaults.agent_role.as_str(),
        workspace_type = defaults.workspace_type.as_str(),
        workspace_isolation = defaults.workspace_isolation.as_str(),
        verify_profile = defaults.verify_profile.as_str(),
        max_repair_attempts = defaults.max_repair_attempts,
        commit_on_success = defaults.commit_on_success,
        memory_promotion = defaults.memory_promotion.as_str(),
    )
}

pub(super) fn file_create_spec_yaml(
    file: &FileRequest,
    defaults: &ResolvedDefaults,
    approval_required: bool,
) -> String {
    let task_id = format!("create_{}", sanitize_id(&file.path));
    let approval = if approval_required {
        "  approval_required: true\n"
    } else {
        ""
    };
    let command = yaml_quote(&create_file_command(&file.path, &file.content));
    let verify_command = yaml_quote(&format!("test -f {}", shell_quote(&file.path)));
    format!(
        r#"task:
  id: {task_id}
  type: code.file_create
  title: Create {path}
  target: {path}

agent:
  adapter: {adapter}
  role: {role}

workspace:
  type: {workspace_type}
  isolation: {workspace_isolation}

skills:
  - core.file.create

execution:
  commands:
    - {command}

scope:
  allow:
    - {path}
  deny:
    - .agent/**
    - .env*

rules:
  - R_SCOPE_ONLY

verify:
  profile: code_build
  commands:
    - {verify_command}

transaction:
{approval}  max_repair_attempts: 0
  rollback_on_failure: true
  commit_on_success: {commit_on_success}
  memory_promotion: {memory_promotion}
"#,
        path = file.path,
        verify_command = verify_command,
        adapter = defaults.agent_adapter.as_str(),
        role = defaults.agent_role.as_str(),
        workspace_type = defaults.workspace_type.as_str(),
        workspace_isolation = defaults.workspace_isolation.as_str(),
        commit_on_success = defaults.commit_on_success,
        memory_promotion = defaults.memory_promotion.as_str(),
    )
}

fn create_file_command(path: &str, content: &str) -> String {
    path.rsplit_once('/').map_or_else(
        || {
            format!(
                "printf '%s\\n' {} > {}",
                shell_quote(content),
                shell_quote(path)
            )
        },
        |(dir, _)| {
            format!(
                "mkdir -p {} && printf '%s\\n' {} > {}",
                shell_quote(dir),
                shell_quote(content),
                shell_quote(path)
            )
        },
    )
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn yaml_quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}
