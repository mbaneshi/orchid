use anyhow::Result;
use async_trait::async_trait;

use crate::tool::Tool;

/// Messages exchanged between agent and LLM.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// The result of a single agent step.
pub enum StepResult {
    /// The agent wants to continue with more steps.
    Continue,
    /// The agent is done; final output attached.
    Done(String),
}

/// Core agent trait.
#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn system_prompt(&self) -> &str;
    fn tools(&self) -> &[Box<dyn Tool>];
    async fn step(&mut self, messages: &mut Vec<Message>) -> Result<StepResult>;
}
