use anyhow::Result;
use async_trait::async_trait;

use crate::agent::{Agent, Message, StepResult};
use crate::llm::LlmClient;
use crate::tool::Tool;

pub struct CrmIngestionAgent {
    raw_data: String,
    llm: Box<dyn LlmClient>,
    tools: Vec<Box<dyn Tool>>,
    done: bool,
}

impl CrmIngestionAgent {
    pub fn new(raw_data: String, llm: Box<dyn LlmClient>) -> Self {
        Self {
            raw_data,
            llm,
            tools: vec![],
            done: false,
        }
    }
}

#[async_trait]
impl Agent for CrmIngestionAgent {
    fn name(&self) -> &str {
        "crm-ingestion"
    }

    fn system_prompt(&self) -> &str {
        "You are a CRM data processing agent. You receive raw contact and interaction data \
         (from GitHub, email, social media, or freeform notes) and normalize it into structured \
         CRM records. Output JSON with the following schema for each contact:\n\
         {\n  \"name\": string,\n  \"handle\": string,\n  \"source\": string,\n  \
         \"email\": string | null,\n  \"organization\": string | null,\n  \
         \"interactions\": [{ \"date\": string, \"type\": string, \"summary\": string }],\n  \
         \"tags\": [string],\n  \"notes\": string\n}\n\
         Extract as much structure as possible. If data is ambiguous, make reasonable inferences \
         and note them. Return a JSON array of contact records."
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
                "Here is raw contact/interaction data to ingest into CRM format:\n\n{}\n\nPlease normalize this into structured CRM records as JSON.",
                self.raw_data
            ),
        });

        let response = self.llm.chat(messages).await?;
        let records = response.content.clone();
        messages.push(response);

        Ok(StepResult::Done(records))
    }
}
