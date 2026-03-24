use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use tokio::process::Command;

use crate::agent::Message;
use crate::llm::LlmClient;

/// LLM client that spawns the `claude` CLI binary using Max subscription.
/// This avoids needing API credits — uses the same auth as Claude Code.
pub struct ClaudeCliClient {
    model: String,
    timeout_secs: u64,
}

#[derive(Deserialize)]
struct CliResponse {
    result: Option<String>,
}

impl ClaudeCliClient {
    /// Create a client that uses the Claude Max subscription.
    pub fn max_subscription(model: String) -> Self {
        Self {
            model,
            timeout_secs: 300,
        }
    }
}

#[async_trait]
impl LlmClient for ClaudeCliClient {
    async fn chat(&self, messages: &[Message]) -> Result<Message> {
        // Build the prompt from messages
        let mut system_prompt = String::new();
        let mut user_content = String::new();

        for msg in messages {
            match msg.role.as_str() {
                "system" => {
                    system_prompt = msg.content.clone();
                }
                "user" => {
                    user_content.push_str(&msg.content);
                    user_content.push('\n');
                }
                "assistant" => {
                    // Include prior assistant messages as context
                    user_content.push_str(&format!("\n[Previous response]:\n{}\n", msg.content));
                }
                _ => {}
            }
        }

        let full_prompt = if system_prompt.is_empty() {
            user_content.trim().to_string()
        } else {
            format!(
                "[System instructions]: {}\n\n[User request]: {}",
                system_prompt,
                user_content.trim()
            )
        };

        let mut cmd = Command::new("claude");
        cmd.args([
            "-p",
            &full_prompt,
            "--output-format",
            "json",
            "--model",
            &self.model,
        ]);

        // Route through Max subscription — no API key needed
        cmd.env("CLAUDE_CODE_ENTRYPOINT", "sdk-max");
        cmd.env("CLAUDE_USE_SUBSCRIPTION", "true");
        cmd.env("CLAUDE_BYPASS_BALANCE_CHECK", "true");
        cmd.env_remove("ANTHROPIC_API_KEY");

        tracing::info!(model = %self.model, "calling claude CLI (Max subscription)");

        let output = tokio::time::timeout(
            std::time::Duration::from_secs(self.timeout_secs),
            cmd.output(),
        )
        .await
        .context("claude CLI timed out")?
        .context("failed to spawn claude CLI")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("claude CLI failed: {stderr}");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Try to parse as JSON first
        if let Ok(resp) = serde_json::from_str::<CliResponse>(&stdout) {
            if let Some(result) = resp.result {
                return Ok(Message {
                    role: "assistant".to_string(),
                    content: result,
                });
            }
        }

        // Fallback: treat entire stdout as the response
        let text = stdout.trim().to_string();
        if text.is_empty() {
            anyhow::bail!("claude CLI returned empty response");
        }

        Ok(Message {
            role: "assistant".to_string(),
            content: text,
        })
    }
}
