use std::io::{self, IsTerminal, Write};
use std::path::Path;

use anyhow::Result;

use crate::spec::AgentSpec;

pub(super) enum Decision {
    Run,
    Cancel,
}

pub(super) fn confirm_plan(spec_path: &Path) -> Result<Decision> {
    let spec = AgentSpec::load(spec_path)?;
    println!("Plan:");
    println!(
        "- task: {}",
        spec.task.title.as_deref().unwrap_or(&spec.task.id)
    );
    println!(
        "- target: {}",
        spec.task.target.as_deref().unwrap_or("<none>")
    );
    println!(
        "- provider: {}",
        spec.agent.adapter.as_deref().unwrap_or("command")
    );
    println!(
        "- verifier: {}",
        spec.verify.profile.as_deref().unwrap_or("default")
    );
    println!("- scope: {}", spec.scope.allow.join(", "));
    if !spec.verify.commands.is_empty() {
        println!("- commands: {}", spec.verify.commands.join(" && "));
    }
    if !io::stdin().is_terminal() {
        return Ok(Decision::Run);
    }
    loop {
        print!("Run this transaction? [Y/n/details] ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        match line.trim().to_ascii_lowercase().as_str() {
            "" | "y" | "yes" | "д" | "да" => return Ok(Decision::Run),
            "n" | "no" | "н" | "нет" => return Ok(Decision::Cancel),
            "details" | "d" | "детали" => {
                println!("{}", std::fs::read_to_string(spec_path)?);
            }
            _ => println!("Use Y, n, or details."),
        }
    }
}
