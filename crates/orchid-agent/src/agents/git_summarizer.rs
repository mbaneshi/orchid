use anyhow::Result;
use async_trait::async_trait;

use crate::agent::{Agent, Message, StepResult};
use crate::tool::Tool;

/// Agent that summarizes recent git activity.
pub struct GitSummarizerAgent {
    tools: Vec<Box<dyn Tool>>,
}

impl GitSummarizerAgent {
    pub fn new() -> Self {
        Self { tools: vec![] }
    }
}

#[async_trait]
impl Agent for GitSummarizerAgent {
    fn name(&self) -> &str {
        "git-summarizer"
    }

    fn system_prompt(&self) -> &str {
        "You are a git history summarizer. Analyze recent commits and produce a concise summary."
    }

    fn tools(&self) -> &[Box<dyn Tool>] {
        &self.tools
    }

    async fn step(&mut self, _messages: &mut Vec<Message>) -> Result<StepResult> {
        Ok(StepResult::Done(
            "Git summary: (not yet implemented)".to_string(),
        ))
    }
}
