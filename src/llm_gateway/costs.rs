#[derive(Debug, Clone)]
pub struct PriceEstimate {
    pub cost_usd: f64,
    pub source: String,
}

pub fn estimate(adapter: &str, prompt_tokens: usize, completion_tokens: usize) -> PriceEstimate {
    let input_rate =
        env_rate("AGENTHUB_INPUT_USD_PER_1K").unwrap_or_else(|| default_input(adapter));
    let output_rate =
        env_rate("AGENTHUB_OUTPUT_USD_PER_1K").unwrap_or_else(|| default_output(adapter));
    let cost_usd = (prompt_tokens as f64 / 1000.0 * input_rate)
        + (completion_tokens as f64 / 1000.0 * output_rate);
    PriceEstimate {
        cost_usd,
        source: if input_rate == 0.0 && output_rate == 0.0 {
            "local_or_unpriced".to_string()
        } else {
            "configured_estimate".to_string()
        },
    }
}

fn env_rate(name: &str) -> Option<f64> {
    std::env::var(name).ok()?.parse().ok()
}

fn default_input(adapter: &str) -> f64 {
    match adapter {
        "command" => 0.0,
        "deepseek" => 0.00027,
        "kimi" => 0.00050,
        _ => 0.0,
    }
}

fn default_output(adapter: &str) -> f64 {
    match adapter {
        "command" => 0.0,
        "deepseek" => 0.00110,
        "kimi" => 0.00200,
        _ => 0.0,
    }
}
