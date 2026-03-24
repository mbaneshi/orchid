use anyhow::Result;
use async_trait::async_trait;

use crate::agent::{Agent, Message, StepResult};
use crate::llm::LlmClient;
use crate::tool::Tool;

pub struct OutreachDrafterAgent {
    context: String,
    llm: Box<dyn LlmClient>,
    tools: Vec<Box<dyn Tool>>,
    done: bool,
}

impl OutreachDrafterAgent {
    pub fn new(context: String, llm: Box<dyn LlmClient>) -> Self {
        Self {
            context,
            llm,
            tools: vec![],
            done: false,
        }
    }
}

#[async_trait]
impl Agent for OutreachDrafterAgent {
    fn name(&self) -> &str {
        "outreach-drafter"
    }

    fn system_prompt(&self) -> &str {
        "You are a developer relations outreach specialist. Given context about a project, \
         community activity, or collaboration opportunity, draft outreach messages in three formats:\n\
         1. A cold email (professional, concise, 3-5 sentences)\n\
         2. A Twitter/X DM (casual, under 500 chars)\n\
         3. A GitHub issue comment or discussion post (technical, helpful tone)\n\
         Be genuine and specific. Avoid generic templates. Reference real details from the context."
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
                "Here is the context for outreach:\n\n{}\n\nPlease draft outreach messages in all three formats.",
                self.context
            ),
        });

        let response = self.llm.chat(messages).await?;
        let draft = response.content.clone();
        messages.push(response);

        Ok(StepResult::Done(draft))
    }
}
