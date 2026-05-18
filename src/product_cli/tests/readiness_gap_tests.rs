use anyhow::Result;

use crate::product_cli::readiness;

use super::readiness_support::{with_readiness_fixture, ReadinessFixture};

#[test]
fn readiness_completion_evidence_and_checklist_surface_named_gaps() -> Result<()> {
    let fixture = ReadinessFixture::blocked_kimi()?;
    std::fs::write(
        &fixture.evidence,
        "{\"kind\":\"session\",\"session_id\":\"chat-1\",\"mode\":\"chat\",\"flow\":\"chat\",\"status\":\"passed\",\"cost_receipt\":true}\n\
         {\"kind\":\"provider\",\"provider\":\"deepseek\",\"status\":\"passed\"}\n",
    )?;
    with_readiness_fixture(&fixture, || {
        let evidence = readiness::render_evidence(
            fixture.root.path(),
            readiness::AuditOptions {
                json: true,
                no_refresh: true,
            },
        )?;
        let parsed: serde_json::Value = serde_json::from_str(&evidence.output)?;
        for id in ["shell_ux_aliases", "kimi", "dogfood", "latency", "approval"] {
            assert!(parsed["gaps"]
                .as_array()
                .unwrap()
                .iter()
                .any(|gap| gap["id"] == id));
        }
        let kimi_gap = parsed["gaps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|gap| gap["id"] == "kimi")
            .expect("kimi gap");
        let kimi_auth = kimi_gap["checks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|check| check["id"] == "kimi_auth")
            .expect("kimi auth gap check");
        assert!(kimi_auth["next_commands"]
            .as_array()
            .unwrap()
            .iter()
            .any(|command| command
                == "agenthub providers rc-unblock kimi --from-file <new-key-file>"));

        let checklist = readiness::render_checklist(
            fixture.root.path(),
            readiness::AuditOptions {
                json: false,
                no_refresh: true,
            },
        )?;
        let completion = readiness::render_completion(
            fixture.root.path(),
            readiness::AuditOptions {
                json: false,
                no_refresh: true,
            },
        )?;

        for output in [&checklist.output, &completion.output] {
            assert!(output.contains("gap\tshell_ux_aliases\tmissing"));
            assert!(output.contains("gap\tkimi\tblocked"));
            assert!(output.contains("gap\tdogfood\tblocked"));
            assert!(output.contains("gap\tlatency\tmissing"));
            assert!(output.contains("gap\tapproval\tmissing"));
            assert!(output.contains("gap_check\tdogfood\treal_sessions\tmissing\t1/3"));
            assert!(output.contains("gap_check\tapproval\trc_check_approval_ux\tmissing"));
            assert!(output.contains("gap_next\tkimi\t"));
            assert!(output.contains(
                "gap_check_next\tkimi\tkimi_auth\t5\tagenthub providers rc-unblock kimi --from-file <new-key-file>"
            ));
            assert!(!output.contains("kimi-secret"));
        }
        Ok(())
    })
}
