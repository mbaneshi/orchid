use anyhow::Result;
use async_trait::async_trait;

use crate::agent::{Agent, Message, StepResult};
use crate::tool::Tool;

/// Agent that drafts content from a topic and context.
pub struct ContentDrafterAgent {
    tools: Vec<Box<dyn Tool>>,
}

impl ContentDrafterAgent {
    pub fn new() -> Self {
        Self { tools: vec![] }
    }
}

#[async_trait]
impl Agent for ContentDrafterAgent {
    fn name(&self) -> &str {
        "content-drafter"
    }

    fn system_prompt(&self) -> &str {
        "You are a content drafter. Given a topic and context, produce a well-structured draft."
    }

    fn tools(&self) -> &[Box<dyn Tool>] {
        &self.tools
    }

    async fn step(&mut self, _messages: &mut Vec<Message>) -> Result<StepResult> {
        Ok(StepResult::Done(
            "Draft: (not yet implemented)".to_string(),
        ))
    }
}
