use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::agent::Message;
use crate::llm::LlmClient;

pub struct AnthropicClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
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
        Self {
            client: reqwest::Client::new(),
            api_key,
            model,
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

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
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

/// Resolve API key from config or environment variable.
pub fn resolve_api_key(config: &orchid_core::config::LlmConfig) -> Result<String> {
    if let Some(ref key) = config.api_key {
        if !key.is_empty() {
            return Ok(key.clone());
        }
    }
    std::env::var("ANTHROPIC_API_KEY").context(
        "ANTHROPIC_API_KEY not set. Set it as an env var or in ~/.orchid/config.toml under [llm].api_key",
    )
}
