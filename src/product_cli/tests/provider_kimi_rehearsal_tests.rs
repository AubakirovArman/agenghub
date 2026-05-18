use anyhow::Result;

use super::providers;
use super::support::{with_env_vars, with_kimi_env};

#[test]
fn providers_kimi_unblock_rehearsal_accepts_plain_candidate_without_write_or_network() -> Result<()>
{
    with_kimi_env(None, None, || {
        let dir = tempfile::tempdir()?;
        let current = dir.path().join(".kimi");
        let candidate = dir.path().join("candidate-kimi-key.txt");
        std::fs::write(
            &current,
            r#"{"access_token":"cli-access-secret","refresh_token":"cli-refresh-secret","scope":"kimi-code","token_type":"Bearer"}"#,
        )?;
        std::fs::write(&candidate, "  candidate-kimi-secret  \n")?;

        let result = providers::rehearse_provider_unblock(
            dir.path(),
            "kimi",
            providers::KimiUnblockRehearsalOptions {
                from_file: Some(candidate.clone()),
                ..Default::default()
            },
        )?;
        let stored = std::fs::read_to_string(&current)?;

        assert!(!result.failed);
        assert_eq!(
            stored,
            r#"{"access_token":"cli-access-secret","refresh_token":"cli-refresh-secret","scope":"kimi-code","token_type":"Bearer"}"#
        );
        assert!(result.output.contains("AgentHub Kimi unblock rehearsal"));
        assert!(result.output.contains("status\tready_to_preflight"));
        assert!(result.output.contains("writes_key\tfalse"));
        assert!(result.output.contains("network\tfalse"));
        assert!(result
            .output
            .contains("current_classification\tkimi_code_cli_oauth"));
        assert!(result
            .output
            .contains("candidate_classification\tplain_api_key_candidate"));
        assert!(result.output.contains("candidate_status\tcandidate"));
        assert!(result.output.contains(&format!(
            "agenthub providers inspect-key kimi --from-file {}",
            candidate.display()
        )));
        assert!(result.output.contains(&format!(
            "agenthub providers preflight-key kimi --from-file {}",
            candidate.display()
        )));
        assert!(result.output.contains(&format!(
            "agenthub providers rc-unblock kimi --from-file {}",
            candidate.display()
        )));
        assert!(result
            .output
            .contains("guard\t4\tKimi Code CLI OAuth JSON is rejected"));
        assert!(!result.output.contains("candidate-kimi-secret"));
        assert!(!result.output.contains("cli-access-secret"));
        assert!(!result.output.contains("cli-refresh-secret"));
        Ok(())
    })
}

#[test]
fn providers_kimi_unblock_rehearsal_json_is_machine_readable_without_secret() -> Result<()> {
    with_kimi_env(None, None, || {
        let dir = tempfile::tempdir()?;
        let candidate = dir.path().join("candidate-kimi-key.txt");
        std::fs::write(&candidate, "candidate-kimi-secret\n")?;

        let result = providers::rehearse_provider_unblock(
            dir.path(),
            "kimi",
            providers::KimiUnblockRehearsalOptions {
                json: true,
                from_file: Some(candidate.clone()),
                ..Default::default()
            },
        )?;
        let parsed: serde_json::Value = serde_json::from_str(&result.output)?;

        assert!(!result.failed);
        assert_eq!(parsed["provider"], "kimi");
        assert_eq!(
            parsed["objective"],
            "kimi_replacement_key_unblock_rehearsal"
        );
        assert_eq!(parsed["status"], "ready_to_preflight");
        assert_eq!(parsed["writes_key"], false);
        assert_eq!(parsed["network"], false);
        assert_eq!(
            parsed["candidate"]["classification"],
            "plain_api_key_candidate"
        );
        assert_eq!(parsed["command_plan"][2]["network"], true);
        assert_eq!(parsed["command_plan"][2]["writes_key"], false);
        assert_eq!(parsed["command_plan"][3]["network"], true);
        assert_eq!(parsed["command_plan"][3]["writes_key"], true);
        assert!(parsed["next_commands"]
            .as_array()
            .unwrap()
            .iter()
            .any(|command| command
                .as_str()
                .unwrap()
                .contains("agenthub providers rc-unblock kimi --from-file")));
        assert!(!result.output.contains("candidate-kimi-secret"));
        Ok(())
    })
}

#[test]
fn providers_kimi_unblock_rehearsal_requires_candidate_source() -> Result<()> {
    with_env_vars(
        &[
            ("KIMI_API_KEY", None),
            ("MOONSHOT_API_KEY", None),
            ("KIMI_API_KEY_FILE", None),
            ("MOONSHOT_API_KEY_FILE", None),
        ],
        || {
            let dir = tempfile::tempdir()?;

            let result = providers::rehearse_provider_unblock(
                dir.path(),
                "kimi",
                providers::KimiUnblockRehearsalOptions::default(),
            )?;

            assert!(result.failed);
            assert!(result.output.contains("status\tneeds_candidate"));
            assert!(result.output.contains("current_status\tmissing"));
            assert!(result.output.contains("candidate_status\tmissing"));
            assert!(result.output.contains(
                "next\t3\tagenthub providers rehearse-unblock kimi --from-file <new-key-file>"
            ));
            Ok(())
        },
    )
}

#[test]
fn providers_kimi_unblock_rehearsal_rejects_oauth_candidate_without_secret() -> Result<()> {
    with_kimi_env(None, None, || {
        let dir = tempfile::tempdir()?;
        let candidate = dir.path().join("kimi-code.json");
        std::fs::write(
            &candidate,
            r#"{"access_token":"cli-access-secret","refresh_token":"cli-refresh-secret","scope":"kimi-code","token_type":"Bearer"}"#,
        )?;

        let result = providers::rehearse_provider_unblock(
            dir.path(),
            "kimi",
            providers::KimiUnblockRehearsalOptions {
                from_file: Some(candidate),
                ..Default::default()
            },
        )?;

        assert!(result.failed);
        assert!(result.output.contains("status\tblocked"));
        assert!(result
            .output
            .contains("candidate_classification\tkimi_code_cli_oauth"));
        assert!(result.output.contains("Moonshot OpenAI-compatible API key"));
        assert!(!result.output.contains("cli-access-secret"));
        assert!(!result.output.contains("cli-refresh-secret"));
        Ok(())
    })
}
