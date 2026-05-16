use std::path::Path;

use anyhow::{Context, Result};

use crate::product_cli::{config, providers};
use crate::{git, home};

pub(super) fn prepare(root: &Path) -> Result<()> {
    println!("AgentHub {}", crate::product_cli::version());
    println!("Working folder: {}", root.display());
    std::fs::create_dir_all(root).with_context(|| format!("create {}", root.display()))?;
    print_mode(root);
    suggest_provider(root)?;
    println!("Type a message. Use / for commands, /cd <folder> to switch projects.");
    Ok(())
}

fn suggest_provider(root: &Path) -> Result<()> {
    let current = config::default_provider(root)?;
    if current != "command" || config::path(root).exists() {
        println!("Provider: {current}  (change with /providers)");
        return Ok(());
    }
    let preferred = providers::statuses(root)?
        .into_iter()
        .find(|status| status.available && matches!(status.info.id.as_str(), "deepseek" | "kimi"));
    if let Some(status) = preferred {
        println!(
            "Provider: {} ready  (change with /providers)",
            status.info.id
        );
        return Ok(());
    }
    println!("Provider: command  (configure DeepSeek/Kimi with /providers)");
    Ok(())
}

fn print_mode(root: &Path) {
    if home::project_has_runtime(root) {
        println!(
            "Mode: project  Git: {}  .agent: ok",
            if git::is_repo(root) { "ok" } else { "missing" }
        );
    } else if git::is_repo(root) {
        println!("Mode: chat  Git: detected  .agent: not initialized");
        println!("Project transactions are available after /init or `agenthub run ...`.");
    } else {
        println!("Mode: chat  Git: not required  .agent: not required");
    }
}
