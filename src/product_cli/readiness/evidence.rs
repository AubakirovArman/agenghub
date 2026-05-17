use std::{path::Path, process::Command};

use anyhow::{Context, Result};

pub(super) fn refresh_evidence(project_root: &Path, evidence: &Path) -> Result<()> {
    if cfg!(windows) {
        return Ok(());
    }
    let script = project_root.join("scripts/rc-evidence-collect.sh");
    if !script.exists() {
        return Ok(());
    }
    let output = Command::new(&script)
        .env("AGENTHUB_RC_EVIDENCE", evidence)
        .output()
        .with_context(|| format!("run {}", script.display()))?;
    if !output.status.success() {
        anyhow::bail!(
            "{} failed with {}; stdout: {}; stderr: {}",
            script.display(),
            output.status,
            process_text(&output.stdout),
            process_text(&output.stderr)
        );
    }
    Ok(())
}

fn process_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}
