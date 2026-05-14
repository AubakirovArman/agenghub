use std::fs;
use std::path::Path;

use anyhow::Result;

use super::{install_package, list_installed, InstallOptions, PluginTrust};
use crate::agent_dir::init_project;

#[test]
fn installs_skill_package_and_updates_locks() -> Result<()> {
    let project = tempfile::tempdir()?;
    let package = tempfile::tempdir()?;
    init_project(project.path(), false)?;
    write_package(package.path(), "demo.skill", "1.0.0")?;

    let result = install_package(project.path(), package.path(), local_options())?;

    assert_eq!(result.package_id, "demo.package");
    assert!(project.path().join("skills/demo.skill/skill.yaml").exists());
    assert_eq!(list_installed(project.path())?.len(), 1);
    Ok(())
}

#[test]
fn blocks_untrusted_install_without_override() -> Result<()> {
    let project = tempfile::tempdir()?;
    let package = tempfile::tempdir()?;
    init_project(project.path(), false)?;
    write_package(package.path(), "demo.skill", "1.0.0")?;

    let err = install_package(
        project.path(),
        package.path(),
        InstallOptions {
            trust: PluginTrust::Untrusted,
            allow_untrusted: false,
            force: false,
        },
    )
    .expect_err("untrusted install should be blocked");

    assert!(err.to_string().contains("--allow-untrusted"));
    Ok(())
}

fn local_options() -> InstallOptions {
    InstallOptions {
        trust: PluginTrust::Local,
        allow_untrusted: false,
        force: false,
    }
}

fn write_package(root: &Path, skill_id: &str, version: &str) -> Result<()> {
    let skill_dir = root.join("skills").join(skill_id);
    fs::create_dir_all(&skill_dir)?;
    fs::write(root.join("agenthub-plugin.yaml"), package_yaml())?;
    fs::write(
        skill_dir.join("skill.yaml"),
        format!("skill:\n  id: {skill_id}\n  version: {version}\n  description: Demo skill\n"),
    )?;
    Ok(())
}

fn package_yaml() -> &'static str {
    "package:\n  id: demo.package\n  version: 0.1.0\n  description: Demo package\nskills:\n  - path: skills/demo.skill/skill.yaml\n"
}
