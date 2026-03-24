use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::agent::Message;
use crate::llm::LlmClient;

pub struct AnthropicClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
    /// If true, use Authorization: Bearer (OAuth) instead of x-api-key
    use_oauth: bool,
}

#[derive(Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    messages: Vec<ApiMessage>,
}

#[derive(Serialize)]
struct ApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: Option<String>,
}

#[derive(Deserialize)]
struct ApiError {
    error: ApiErrorDetail,
}

#[derive(Deserialize)]
struct ApiErrorDetail {
    message: String,
}

impl AnthropicClient {
    pub fn new(api_key: String, model: String) -> Self {
        let use_oauth = api_key.starts_with("sk-ant-oat");
        Self {
            client: reqwest::Client::new(),
            api_key,
            model,
            use_oauth,
        }
    }
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat(&self, messages: &[Message]) -> Result<Message> {
        let mut system = None;
        let mut api_messages = Vec::new();

        for msg in messages {
            if msg.role == "system" {
                system = Some(msg.content.clone());
            } else {
                api_messages.push(ApiMessage {
                    role: msg.role.clone(),
                    content: msg.content.clone(),
                });
            }
        }

        let request = ApiRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            system,
            messages: api_messages,
        };

        let mut req = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json");

        if self.use_oauth {
            req = req.header("Authorization", format!("Bearer {}", self.api_key));
        } else {
            req = req.header("x-api-key", &self.api_key);
        }

        let response = req
            .json(&request)
            .send()
            .await
            .context("failed to call Anthropic API")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<ApiError>(&body) {
                anyhow::bail!("Anthropic API error ({}): {}", status, err.error.message);
            }
            anyhow::bail!("Anthropic API error ({}): {}", status, body);
        }

        let api_response: ApiResponse = response
            .json()
            .await
            .context("failed to parse Anthropic response")?;

        let text = api_response
            .content
            .into_iter()
            .filter_map(|block| block.text)
            .collect::<Vec<_>>()
            .join("");

        Ok(Message {
            role: "assistant".to_string(),
            content: text,
        })
    }
}

/// Resolve API key with the following priority:
/// 1. Config file (~/.orchid/config.toml [llm].api_key)
/// 2. ANTHROPIC_API_KEY env var
/// 3. Claude Max OAuth token from macOS keychain
pub fn resolve_api_key(config: &orchid_core::config::LlmConfig) -> Result<String> {
    // 1. Explicit config
    if let Some(ref key) = config.api_key {
        if !key.is_empty() {
            return Ok(key.clone());
        }
    }

    // 2. Environment variable
    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        if !key.is_empty() {
            return Ok(key);
        }
    }

    // 3. Claude Max OAuth token from macOS keychain
    if let Some(token) = read_claude_max_token() {
        tracing::info!("using Claude Max OAuth token from keychain");
        return Ok(token);
    }

    anyhow::bail!(
        "No API key found. Options:\n\
         1. Set ANTHROPIC_API_KEY env var\n\
         2. Add api_key to ~/.orchid/config.toml under [llm]\n\
         3. Log in to Claude Code with a Max subscription (token read from keychain)"
    )
}

/// Read the Claude Max OAuth access token from macOS keychain.
/// Claude Code stores credentials under service "Claude Code-credentials".
fn read_claude_max_token() -> Option<String> {
    let output = std::process::Command::new("security")
        .args([
            "find-generic-password",
            "-s",
            "Claude Code-credentials",
            "-w",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let json_str = String::from_utf8(output.stdout).ok()?;
    let data: serde_json::Value = serde_json::from_str(json_str.trim()).ok()?;

    // Extract claudeAiOauth.accessToken
    data.get("claudeAiOauth")
        .and_then(|oauth| oauth.get("accessToken"))
        .and_then(|token| token.as_str())
        .map(|s| s.to_string())
}
