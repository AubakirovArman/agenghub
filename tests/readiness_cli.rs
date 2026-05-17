use std::{fs, process::Command};

use anyhow::Result;

#[test]
fn readiness_blockers_json_check_suppresses_evidence_collector_output() -> Result<()> {
    let fixture = CliReadinessFixture::new()?;
    let output = Command::new(env!("CARGO_BIN_EXE_agenthub"))
        .current_dir(fixture.root.path())
        .arg("readiness")
        .arg("blockers")
        .arg("--json")
        .arg("--check")
        .env("AGENTHUB_API_AUDIT_EVIDENCE", &fixture.evidence)
        .env("AGENTHUB_API_AUDIT_HISTORY_DIR", &fixture.history)
        .env("AGENTHUB_API_AUDIT_KIMI_REPORT", &fixture.kimi)
        .env("AGENTHUB_API_AUDIT_V04_PLAN", &fixture.plan)
        .env("AGENTHUB_API_AUDIT_AFTER_PLAN", &fixture.after)
        .env("AGENTHUB_API_AUDIT_ROADMAP_DOC", &fixture.roadmap)
        .env(
            "AGENTHUB_API_AUDIT_PROVIDER_STATUS",
            "deepseek\tok\tdefault\thttps://api.deepseek.com/v1\nkimi\tok\t-\thttps://api.moonshot.ai/v1",
        )
        .env("AGENTHUB_API_AUDIT_MIN_REAL_SESSIONS", "3")
        .env("AGENTHUB_API_AUDIT_MIN_OPS_FLOWS", "1")
        .env("AGENTHUB_API_AUDIT_MIN_PROJECT_EDIT_FLOWS", "1")
        .env("AGENTHUB_API_AUDIT_MIN_COST_RECEIPTS", "3")
        .output()?;

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    let parsed: serde_json::Value = serde_json::from_str(&stdout)?;

    assert_eq!(parsed["status"], "blocked");
    assert_eq!(parsed["failed"], true);
    assert!(stderr.contains("readiness blockers present"));
    assert!(!stdout.contains("collector stdout"));
    assert!(!stderr.contains("collector stderr"));
    Ok(())
}

struct CliReadinessFixture {
    root: tempfile::TempDir,
    plan: std::path::PathBuf,
    after: std::path::PathBuf,
    roadmap: std::path::PathBuf,
    evidence: std::path::PathBuf,
    history: std::path::PathBuf,
    kimi: std::path::PathBuf,
}

impl CliReadinessFixture {
    fn new() -> Result<Self> {
        let root = tempfile::tempdir()?;
        let scripts = root.path().join("scripts");
        fs::create_dir_all(&scripts)?;
        write_evidence_collector(&scripts.join("rc-evidence-collect.sh"))?;

        let plan = root.path().join("agenthub_v04_api_native.md");
        let after = root.path().join("agenthub_after_10_roadmap.md");
        let roadmap = root.path().join("roadmap-after-1.0.ru.md");
        let evidence = root.path().join("rc-evidence.jsonl");
        let history = root.path().join("history");
        let kimi = root.path().join("kimi-auth-report.json");
        fs::create_dir_all(history.join("runs/suite-1"))?;
        fs::create_dir_all(history.join("runs/suite-2"))?;
        fs::create_dir_all(history.join("runs/suite-3"))?;
        fs::create_dir_all(history.join("runs/provider-deepseek"))?;
        fs::create_dir_all(history.join("runs/provider-kimi"))?;
        for path in [&plan, &after, &roadmap] {
            fs::write(path, "fixture\n")?;
        }
        for path in [
            history.join("runs/suite-1/dogfood-report.json"),
            history.join("runs/suite-2/dogfood-report.json"),
            history.join("runs/suite-3/dogfood-report.json"),
            history.join("runs/provider-deepseek/provider-dogfood-report.json"),
            history.join("runs/provider-kimi/provider-dogfood-report.json"),
        ] {
            fs::write(path, "{}\n")?;
        }
        fs::write(history.join("index.jsonl"), history_index(&history))?;
        fs::write(
            &kimi,
            r#"{"provider":"kimi","status":"blocked","auth_key_sha256_12":"f117c7b5fb4e","auth_key_source":"file:/tmp/.kimi","credential_warning":"Kimi Code CLI OAuth credentials are not Moonshot OpenAI-compatible API keys","next_action":"replace or rotate the Kimi/Moonshot API key with a plain Moonshot OpenAI-compatible API key"}"#,
        )?;

        Ok(Self {
            root,
            plan,
            after,
            roadmap,
            evidence,
            history,
            kimi,
        })
    }
}

fn write_evidence_collector(path: &std::path::Path) -> Result<()> {
    fs::write(
        path,
        r#"#!/usr/bin/env bash
set -euo pipefail
echo "collector stdout"
echo "collector stderr" >&2
cat > "$AGENTHUB_RC_EVIDENCE" <<'JSONL'
{"kind":"session","session_id":"chat-1","mode":"chat","flow":"chat","status":"passed","cost_receipt":true}
{"kind":"session","session_id":"ops-1","mode":"ops","flow":"ops","status":"passed","cost_receipt":true}
{"kind":"session","session_id":"project-1","mode":"project","flow":"project_edit","status":"passed","cost_receipt":true}
{"kind":"provider","provider":"deepseek","status":"passed"}
{"kind":"provider","provider":"kimi","status":"passed"}
{"kind":"check","id":"chat_no_bootstrap","status":"passed"}
{"kind":"check","id":"ops_no_bootstrap","status":"passed"}
{"kind":"check","id":"resume","status":"passed"}
{"kind":"check","id":"rewind","status":"passed"}
{"kind":"check","id":"stats","status":"passed"}
{"kind":"check","id":"cost_receipts","status":"passed"}
{"kind":"check","id":"ops_receipts","status":"passed"}
{"kind":"check","id":"approval_ux","status":"passed"}
{"kind":"check","id":"long_session_latency","status":"passed"}
{"kind":"blocker","id":"kimi-auth","severity":"critical","status":"open"}
JSONL
"#,
    )?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

fn history_index(history: &std::path::Path) -> String {
    [
        serde_json::json!({"run_id":"suite-1","archived_at":"2026-05-14T00:00:00Z","kind":"suite","report":history.join("runs/suite-1/dogfood-report.json"),"provider_status":"skipped"}),
        serde_json::json!({"run_id":"suite-2","archived_at":"2026-05-15T00:00:00Z","kind":"suite","report":history.join("runs/suite-2/dogfood-report.json"),"provider_status":"skipped"}),
        serde_json::json!({"run_id":"suite-3","archived_at":"2026-05-16T00:00:00Z","kind":"suite","report":history.join("runs/suite-3/dogfood-report.json"),"provider_status":"skipped"}),
        serde_json::json!({"run_id":"provider-deepseek","archived_at":"2026-05-16T01:00:00Z","kind":"provider","report":history.join("runs/provider-deepseek/provider-dogfood-report.json"),"provider":"deepseek","provider_status":"passed"}),
        serde_json::json!({"run_id":"provider-kimi","archived_at":"2026-05-16T01:30:00Z","kind":"provider","report":history.join("runs/provider-kimi/provider-dogfood-report.json"),"provider":"kimi","provider_status":"passed"}),
    ]
    .into_iter()
    .map(|entry| format!("{entry}\n"))
    .collect()
}
