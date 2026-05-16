use std::fs;

use anyhow::Result;

use super::*;

#[test]
fn summarizes_turn_tokens_and_cost_by_provider() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let chats = dir.path().join(".agent/shell/chats");
    fs::create_dir_all(&chats)?;
    fs::write(
        chats.join("chat-costs.jsonl"),
        "{\"at\":\"2026-01-01T00:00:00Z\",\"kind\":\"created\"}\n\
         {\"at\":\"2026-01-01T00:00:01Z\",\"kind\":\"turn_finished\",\"provider\":\"deepseek\",\"status\":\"succeeded\",\"prompt_tokens\":3,\"completion_tokens\":2,\"total_tokens\":5,\"estimated_cost_usd\":0.00000301,\"pricing_source\":\"configured_estimate\",\"text\":\"turn succeeded\"}\n\
         {\"at\":\"2026-01-01T00:00:02Z\",\"kind\":\"turn_finished\",\"provider\":\"kimi\",\"status\":\"failed\",\"prompt_tokens\":10,\"completion_tokens\":0,\"total_tokens\":10,\"estimated_cost_usd\":0.000005,\"pricing_source\":\"configured_estimate\",\"text\":\"turn failed\"}\n",
    )?;

    let summary = summarize(dir.path())?;
    let rendered = render_summary(&summary);

    assert_eq!(summary.turns, 2);
    assert_eq!(summary.prompt_tokens, 13);
    assert_eq!(summary.completion_tokens, 2);
    assert_eq!(summary.total_tokens, 15);
    assert!((summary.estimated_cost_usd - 0.00000801).abs() < f64::EPSILON);
    assert!(rendered.contains("estimated_cost_usd\t0.00000801"));
    assert!(rendered.contains("provider\tdeepseek\tturns\t1"));
    assert!(rendered.contains("provider\tkimi\tturns\t1"));
    Ok(())
}
