use anyhow::Result;

use crate::product_cli::readiness;

use super::readiness_support::{with_readiness_fixture, ReadinessFixture};

#[test]
fn readiness_audit_json_reports_ready_fixture() -> Result<()> {
    let fixture = ReadinessFixture::ready()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_audit(
            fixture.root.path(),
            readiness::AuditOptions {
                json: true,
                no_refresh: true,
            },
        )?;
        let parsed: serde_json::Value = serde_json::from_str(&result.output)?;

        assert!(!result.failed);
        assert_eq!(parsed["status"], "ready");
        assert_eq!(parsed["failed"], false);
        assert!(parsed.get("blocker_scope").is_none());
        assert!(parsed.get("blocker_kinds").is_none());
        assert!(parsed.get("blocked_checks").is_none());
        assert_eq!(parsed["metrics"]["real_sessions"], 3);
        assert!(result.output.contains(r#""id": "ecosystem_surfaces""#));
        assert!(result.output.contains(r#""id": "provider_surface""#));
        assert!(result.output.contains(r#""id": "provider_kimi""#));
        assert!(!result.output.contains("kimi-secret"));
        Ok(())
    })
}

#[test]
fn readiness_audit_json_reports_blocked_kimi_without_secret() -> Result<()> {
    let fixture = ReadinessFixture::blocked_kimi()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_audit(
            fixture.root.path(),
            readiness::AuditOptions {
                json: true,
                no_refresh: true,
            },
        )?;
        let parsed: serde_json::Value = serde_json::from_str(&result.output)?;

        assert!(result.failed);
        assert_eq!(parsed["status"], "incomplete");
        assert_eq!(parsed["failed"], true);
        assert_eq!(parsed["blocker_scope"], "external_only");
        assert_eq!(parsed["blocker_kinds"][0], "dependent_gate");
        assert_eq!(parsed["blocker_kinds"][1], "external_credential");
        assert_eq!(parsed["blocked_checks"][0], "open_blockers");
        assert_eq!(parsed["blocked_checks"][1], "kimi_auth");
        assert_eq!(parsed["blocked_checks"][2], "rc_dogfood_gate");
        assert_eq!(parsed["metrics"]["open_blockers"], 1);
        assert!(result.output.contains("1 blocker/critical open: kimi-auth"));
        assert!(result.output.contains(r#""id": "kimi_auth""#));
        assert!(result.output.contains(r#""status": "blocked""#));
        let kimi_auth = parsed["checks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"] == "kimi_auth")
            .expect("kimi auth check");
        assert_eq!(kimi_auth["blocker_kind"], "external_credential");
        assert!(kimi_auth["next_commands"]
            .as_array()
            .unwrap()
            .iter()
            .any(|command| command
                == "agenthub providers rc-unblock kimi --from-file <new-key-file>"));
        assert!(result
            .output
            .contains("agenthub readiness audit --json --check"));
        assert!(result.output.contains("source:file:/tmp/.kimi"));
        assert!(!result.output.contains("kimi-secret"));
        Ok(())
    })
}

#[test]
fn readiness_audit_text_keeps_human_checklist() -> Result<()> {
    let fixture = ReadinessFixture::blocked_kimi()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_audit(
            fixture.root.path(),
            readiness::AuditOptions {
                json: false,
                no_refresh: true,
            },
        )?;

        assert!(result.failed);
        assert!(result
            .output
            .contains("AgentHub API-native readiness audit"));
        assert!(result.output.contains("check\tkimi_auth\tblocked"));
        assert!(result
            .output
            .contains("check_blocker_kind\tkimi_auth\texternal_credential"));
        assert!(result.output.contains(
            "check_next\tkimi_auth\t4\tagenthub providers rc-unblock kimi --from-file <new-key-file>"
        ));
        assert!(result.output.contains("blocker_scope\texternal_only"));
        assert!(result
            .output
            .contains("blocker_kinds\tdependent_gate,external_credential"));
        assert!(result
            .output
            .contains("blocked_checks\topen_blockers,kimi_auth,rc_dogfood_gate"));
        assert!(result.output.contains("status\tincomplete"));
        assert!(result
            .output
            .contains("next\t15\tagenthub readiness evidence --json --check"));
        assert!(result
            .output
            .contains("next\t16\tagenthub readiness audit --json --check"));
        Ok(())
    })
}

#[test]
fn readiness_blockers_json_reports_only_unpassed_checks() -> Result<()> {
    let fixture = ReadinessFixture::blocked_kimi()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_blockers(
            fixture.root.path(),
            readiness::AuditOptions {
                json: true,
                no_refresh: true,
            },
        )?;
        let parsed: serde_json::Value = serde_json::from_str(&result.output)?;
        let blocker_ids = parsed["blockers"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|entry| entry["id"].as_str())
            .collect::<Vec<_>>();

        assert!(result.failed);
        assert_eq!(parsed["status"], "blocked");
        assert_eq!(parsed["blocker_scope"], "external_only");
        assert_eq!(parsed["blocker_kinds"][0], "dependent_gate");
        assert_eq!(parsed["blocker_kinds"][1], "external_credential");
        assert_eq!(parsed["blocked_checks"][0], "open_blockers");
        assert_eq!(parsed["blocked_checks"][1], "kimi_auth");
        assert_eq!(parsed["blocked_checks"][2], "rc_dogfood_gate");
        assert!(blocker_ids.contains(&"kimi_auth"));
        assert!(blocker_ids.contains(&"open_blockers"));
        assert!(blocker_ids.contains(&"rc_dogfood_gate"));
        assert!(result.output.contains("1 blocker/critical open: kimi-auth"));
        let kimi_auth = parsed["blockers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"] == "kimi_auth")
            .expect("kimi auth blocker");
        assert_eq!(kimi_auth["blocker_kind"], "external_credential");
        assert!(kimi_auth["next_commands"]
            .as_array()
            .unwrap()
            .iter()
            .any(|command| command
                == "agenthub providers rc-unblock kimi --from-file <new-key-file>"));
        assert!(!blocker_ids.contains(&"provider_kimi"));
        assert!(!blocker_ids.contains(&"provider_surface"));
        assert!(result
            .output
            .contains("agenthub providers preflight-key kimi --from-file"));
        assert!(!result.output.contains("kimi-secret"));
        Ok(())
    })
}

#[test]
fn readiness_blockers_text_reports_blocker_kind() -> Result<()> {
    let fixture = ReadinessFixture::blocked_kimi()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_blockers(
            fixture.root.path(),
            readiness::AuditOptions {
                json: false,
                no_refresh: true,
            },
        )?;

        assert!(result.failed);
        assert!(result.output.contains("blocker_scope\texternal_only"));
        assert!(result
            .output
            .contains("blocker_kinds\tdependent_gate,external_credential"));
        assert!(result
            .output
            .contains("blocked_checks\topen_blockers,kimi_auth,rc_dogfood_gate"));
        assert!(result.output.contains("blocker\tkimi_auth\tblocked"));
        assert!(result
            .output
            .contains("blocker_kind\tkimi_auth\texternal_credential"));
        Ok(())
    })
}

#[test]
fn readiness_blockers_text_reports_clear_fixture() -> Result<()> {
    let fixture = ReadinessFixture::ready()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_blockers(
            fixture.root.path(),
            readiness::AuditOptions {
                json: false,
                no_refresh: true,
            },
        )?;

        assert!(!result.failed);
        assert!(result.output.contains("AgentHub readiness blockers"));
        assert!(result.output.contains("blockers\tclear"));
        assert!(result.output.contains("status\tclear"));
        assert!(!result.output.contains("blocked_checks\t"));
        assert!(!result.output.contains("next\t"));
        assert!(!result.output.contains("blocker_next\t"));
        Ok(())
    })
}

#[test]
fn readiness_checklist_json_maps_requirements_to_artifacts() -> Result<()> {
    let fixture = ReadinessFixture::ready()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_checklist(
            fixture.root.path(),
            readiness::AuditOptions {
                json: true,
                no_refresh: true,
            },
        )?;
        let parsed: serde_json::Value = serde_json::from_str(&result.output)?;

        assert!(!result.failed);
        assert_eq!(parsed["status"], "ready");
        let requirements = parsed["requirements"].as_array().unwrap();
        let kimi = requirements
            .iter()
            .find(|entry| entry["id"] == "kimi_api")
            .expect("kimi requirement");
        assert_eq!(kimi["status"], "passed");
        assert!(kimi["artifacts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|artifact| artifact
                == "command:agenthub providers rc-unblock kimi --from-file <new-key-file>"));
        assert!(kimi["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|check| check["id"] == "kimi_auth" && check["status"] == "passed"));
        assert!(requirements
            .iter()
            .any(|entry| entry["id"] == "post_1_0_sequence"));
        assert!(!result.output.contains("kimi-secret"));
        Ok(())
    })
}

#[test]
fn readiness_checklist_text_surfaces_blocked_requirement_next_steps() -> Result<()> {
    let fixture = ReadinessFixture::blocked_kimi()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_checklist(
            fixture.root.path(),
            readiness::AuditOptions {
                json: false,
                no_refresh: true,
            },
        )?;

        assert!(result.failed);
        assert!(result
            .output
            .contains("AgentHub API-native readiness checklist"));
        assert!(result.output.contains("requirement\tkimi_api\tblocked"));
        assert!(result
            .output
            .contains("requirement_check\tkimi_api\tkimi_auth\tblocked"));
        assert!(result
            .output
            .contains("agenthub providers rc-unblock kimi --from-file <new-key-file>"));
        assert!(result.output.contains("blocker_scope\texternal_only"));
        assert!(result.output.contains("status\tincomplete"));
        assert!(!result.output.contains("kimi-secret"));
        Ok(())
    })
}

#[test]
fn readiness_evidence_json_reports_thresholds_and_gate_inputs() -> Result<()> {
    let fixture = ReadinessFixture::ready()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_evidence(
            fixture.root.path(),
            readiness::AuditOptions {
                json: true,
                no_refresh: true,
            },
        )?;
        let parsed: serde_json::Value = serde_json::from_str(&result.output)?;

        assert!(!result.failed);
        assert_eq!(parsed["status"], "ready");
        assert_eq!(parsed["history"]["status"], "ready");
        assert_eq!(parsed["history"]["suite_runs"], 3);
        assert_eq!(parsed["history"]["provider_passed_runs"], 2);
        assert!(parsed["thresholds"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["id"] == "real_sessions"
                && entry["status"] == "passed"
                && entry["missing"] == 0));
        assert!(parsed["providers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["id"] == "kimi" && entry["status"] == "passed"));
        assert!(!parsed["providers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["id"] == "surface"));
        assert!(parsed["rc_checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["id"] == "approval_ux" && entry["status"] == "passed"));
        assert_eq!(parsed["kimi_auth"]["status"], "passed");
        assert_eq!(parsed["gate"]["status"], "passed");
        assert!(!result.output.contains("kimi-secret"));
        Ok(())
    })
}

#[test]
fn readiness_evidence_text_surfaces_external_kimi_blocker() -> Result<()> {
    let fixture = ReadinessFixture::blocked_kimi()?;
    with_readiness_fixture(&fixture, || {
        let result = readiness::render_evidence(
            fixture.root.path(),
            readiness::AuditOptions {
                json: false,
                no_refresh: true,
            },
        )?;

        assert!(result.failed);
        assert!(result.output.contains("AgentHub RC evidence status"));
        assert!(result.output.contains("history\tready"));
        assert!(result
            .output
            .contains("provider\tkimi\tpassed\tprovider dogfood evidence found"));
        assert!(result.output.contains("kimi_auth\tkimi_auth\tblocked"));
        assert!(result
            .output
            .contains("kimi_auth_blocker_kind\tkimi_auth\texternal_credential"));
        assert!(result
            .output
            .contains("open_blockers\topen_blockers\tblocked"));
        assert!(result.output.contains("gate\trc_dogfood_gate\tblocked"));
        assert!(result.output.contains("blocker_scope\texternal_only"));
        assert!(result
            .output
            .contains("blocked_checks\topen_blockers,kimi_auth,rc_dogfood_gate"));
        assert!(!result.output.contains("kimi-secret"));
        Ok(())
    })
}
