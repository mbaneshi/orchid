use anyhow::Result;
use async_trait::async_trait;

use crate::agent::{Agent, Message, StepResult};
use crate::llm::LlmClient;
use crate::tool::Tool;

pub struct ContentDrafterAgent {
    input_summary: String,
    llm: Box<dyn LlmClient>,
    tools: Vec<Box<dyn Tool>>,
    done: bool,
}

impl ContentDrafterAgent {
    pub fn new(input_summary: String, llm: Box<dyn LlmClient>) -> Self {
        Self {
            input_summary,
            llm,
            tools: vec![],
            done: false,
        }
    }
}

#[async_trait]
impl Agent for ContentDrafterAgent {
    fn name(&self) -> &str {
        "content-drafter"
    }

    fn system_prompt(&self) -> &str {
        "You are a developer advocate content writer. Given a summary of recent development work, \
         draft social media content in three formats:\n\
         1. A tweet (max 280 chars)\n\
         2. A LinkedIn post (2-3 paragraphs, professional tone)\n\
         3. A blog paragraph (technical but accessible)\n\
         Be specific and engaging. Reference actual changes, not generic platitudes."
    }

    fn tools(&self) -> &[Box<dyn Tool>] {
        &self.tools
    }

    async fn step(&mut self, messages: &mut Vec<Message>) -> Result<StepResult> {
        if self.done {
            anyhow::bail!("agent already completed");
        }
        self.done = true;

        messages.push(Message {
            role: "user".to_string(),
            content: format!(
                "Here is a summary of recent development work:\n\n{}\n\nPlease draft content in all three formats.",
                self.input_summary
            ),
        });

        let response = self.llm.chat(messages).await?;
        let draft = response.content.clone();
        messages.push(response);

        Ok(StepResult::Done(draft))
    }
}
