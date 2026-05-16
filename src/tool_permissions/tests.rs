use super::*;

#[test]
fn classifies_chat_read_only_workspace_ops_and_http_profiles() {
    let empty = classify_shell_command("");
    assert_eq!(empty.profile, ToolPermissionProfile::Chat);
    assert!(!empty.approval_required);

    let git_status = classify_shell_command("git status --short");
    assert_eq!(git_status.profile, ToolPermissionProfile::ReadOnly);
    assert_eq!(git_status.risk, ToolRisk::Low);

    let mkdir = classify_shell_command("mkdir -p app/cache");
    assert_eq!(mkdir.profile, ToolPermissionProfile::WorkspaceWrite);
    assert_eq!(mkdir.risk, ToolRisk::Medium);
    assert!(!mkdir.approval_required);

    let kubectl = classify_shell_command("kubectl delete pod api-1");
    assert_eq!(kubectl.profile, ToolPermissionProfile::OpsHost);
    assert_eq!(kubectl.risk, ToolRisk::High);
    assert!(kubectl.approval_required);

    let http_get = classify_shell_command("curl https://example.com/health");
    assert_eq!(http_get.profile, ToolPermissionProfile::ReadOnly);
    assert!(!http_get.approval_required);

    let http_post = classify_shell_command("curl -X POST -d '{}' https://example.com/deploy");
    assert_eq!(http_post.profile, ToolPermissionProfile::OpsHost);
    assert!(http_post.approval_required);
}
