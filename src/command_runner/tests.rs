use super::*;

#[test]
fn marks_timed_out_command() -> Result<()> {
    let result = run_shell("sleep 2", Path::new("."), Duration::from_millis(50))?;
    assert!(result.timed_out);
    assert!(!result.success);
    Ok(())
}

#[test]
fn timeout_terminates_background_child() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let marker = dir.path().join("late-marker");
    let command = format!("(sleep 2; touch '{}') & wait", marker.display());

    let result = run_shell(&command, dir.path(), Duration::from_millis(50))?;
    thread::sleep(Duration::from_millis(300));

    assert!(result.timed_out);
    assert!(!marker.exists());
    Ok(())
}

#[test]
fn level_one_sandbox_sets_metadata_and_tmpdir() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let result = run_shell_with_sandbox(
        "test \"$AGENTHUB_SANDBOX_LEVEL\" = 1 && test -d \"$TMPDIR\"",
        dir.path(),
        Duration::from_secs(1),
        CommandSandbox { level: 1 },
    )?;

    assert!(result.success);
    assert_eq!(result.sandbox_level, 1);
    Ok(())
}
