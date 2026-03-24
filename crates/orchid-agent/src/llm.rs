use anyhow::Result;
use async_trait::async_trait;

use crate::agent::Message;

/// Abstraction over LLM providers.
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, messages: &[Message]) -> Result<Message>;
}

/// Stub LLM client that echoes back for testing.
pub struct EchoLlmClient;

#[async_trait]
impl LlmClient for EchoLlmClient {
    async fn chat(&self, messages: &[Message]) -> Result<Message> {
        let last = messages
            .last()
            .map(|m| m.content.clone())
            .unwrap_or_default();
        Ok(Message {
            role: "assistant".to_string(),
            content: format!("[echo] {last}"),
        })
    }
}
