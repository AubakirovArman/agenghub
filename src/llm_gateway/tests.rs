use anyhow::Result;
use serde_json::json;

use super::write_gateway_artifacts;

#[test]
fn writes_model_call_metadata_for_routes() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let context = json!({
        "agent_spec": { "task": { "id": "demo" } },
        "agent_routes": {
            "executor": {
                "requested_adapter": "codex",
                "selected_adapter": "command",
                "role": "executor",
                "model": "demo-model"
            },
            "reviewer": null,
            "repair": null
        },
        "skills": [],
        "memory": []
    });

    let artifacts = write_gateway_artifacts(dir.path(), &context, "hash")?;

    assert_eq!(artifacts.model_calls.len(), 1);
    assert!(dir.path().join("model_call_metadata.json").exists());
    assert!(dir.path().join("llm_gateway_summary.json").exists());
    assert!(dir.path().join("redacted_api.jsonl").exists());
    assert!(!dir.path().join("raw_api.jsonl").exists());
    Ok(())
}
