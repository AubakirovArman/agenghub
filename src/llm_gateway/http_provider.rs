use std::io::{Read, Write};
use std::net::TcpStream;

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};

use crate::llm_gateway::provider::{metadata_for_adapter, LlmProvider};
use crate::llm_gateway::types::{LlmRequest, LlmResponse, ProviderMetadata, TokenCount};

#[derive(Debug, Clone)]
pub struct HttpProvider {
    endpoint: String,
    api_key: Option<String>,
    model: Option<String>,
}

impl HttpProvider {
    pub fn new(
        endpoint: impl Into<String>,
        api_key: Option<String>,
        model: Option<String>,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key,
            model,
        }
    }
}

impl LlmProvider for HttpProvider {
    fn complete(&self, request: LlmRequest) -> Result<LlmResponse> {
        let body = json!({
            "model": request.model.clone().or_else(|| self.model.clone()).unwrap_or_else(|| "default".to_string()),
            "messages": [{ "role": "user", "content": request.prompt.clone().unwrap_or_default() }],
            "stream": false
        });
        let response = post_json(
            &completion_url(&self.endpoint),
            self.api_key.as_deref(),
            &body,
        )?;
        let content = response
            .pointer("/choices/0/message/content")
            .and_then(Value::as_str)
            .or_else(|| response.pointer("/choices/0/text").and_then(Value::as_str))
            .map(str::to_string);
        let completion_tokens = response
            .pointer("/usage/completion_tokens")
            .and_then(Value::as_u64)
            .unwrap_or_else(|| content.as_deref().map(estimate_tokens).unwrap_or(0) as u64)
            as usize;
        Ok(LlmResponse {
            request_id: request.id,
            status: "ok".to_string(),
            content,
            completion_tokens,
            error: None,
        })
    }

    fn stream_capability(&self) -> ProviderMetadata {
        metadata_for_adapter("openai-http")
    }

    fn count_tokens(&self, input: &str) -> Result<TokenCount> {
        let prompt_tokens = estimate_tokens(input);
        Ok(TokenCount {
            prompt_tokens,
            completion_tokens: 0,
            total_tokens: prompt_tokens,
            method: "estimated_chars_div_4".to_string(),
        })
    }
}

fn completion_url(endpoint: &str) -> String {
    let endpoint = endpoint.trim_end_matches('/');
    if endpoint.ends_with("/v1/chat/completions") {
        endpoint.to_string()
    } else {
        format!("{endpoint}/v1/chat/completions")
    }
}

fn post_json(url: &str, api_key: Option<&str>, body: &Value) -> Result<Value> {
    let parsed = HttpUrl::parse(url)?;
    let body = serde_json::to_string(body)?;
    let mut request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
        parsed.path,
        parsed.host_header(),
        body.len()
    );
    if let Some(api_key) = api_key.filter(|key| !key.is_empty()) {
        request.push_str(&format!("Authorization: Bearer {api_key}\r\n"));
    }
    request.push_str("\r\n");
    request.push_str(&body);

    let mut stream = TcpStream::connect((parsed.host.as_str(), parsed.port))
        .with_context(|| format!("connect {}", parsed.host_header()))?;
    stream.write_all(request.as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    parse_response(&response)
}

fn parse_response(response: &str) -> Result<Value> {
    let (head, body) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| anyhow!("invalid HTTP response"))?;
    let status = head.lines().next().unwrap_or_default();
    if !status.contains(" 2") {
        return Err(anyhow!("HTTP provider returned {status}"));
    }
    serde_json::from_str(body.trim()).context("parse OpenAI-compatible response JSON")
}

#[derive(Debug)]
struct HttpUrl {
    host: String,
    port: u16,
    path: String,
}

impl HttpUrl {
    fn parse(url: &str) -> Result<Self> {
        let rest = url
            .strip_prefix("http://")
            .ok_or_else(|| anyhow!("only http:// OpenAI-compatible endpoints are supported"))?;
        let (host_port, path) = rest.split_once('/').unwrap_or((rest, ""));
        let (host, port) = if let Some((host, port)) = host_port.rsplit_once(':') {
            (host.to_string(), port.parse::<u16>()?)
        } else {
            (host_port.to_string(), 80)
        };
        Ok(Self {
            host,
            port,
            path: format!("/{}", path),
        })
    }

    fn host_header(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn estimate_tokens(value: &str) -> usize {
    (value.len() / 4).max(1)
}
