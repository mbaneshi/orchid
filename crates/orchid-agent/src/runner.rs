use anyhow::Result;
use tracing::info;

use crate::agent::{Agent, Message, StepResult};

/// Drives an agent's step loop to completion.
pub struct AgentRunner {
    max_steps: usize,
}

impl AgentRunner {
    pub fn new(max_steps: usize) -> Self {
        Self { max_steps }
    }

    pub async fn run(&self, agent: &mut dyn Agent) -> Result<String> {
        let mut messages = vec![Message {
            role: "system".to_string(),
            content: agent.system_prompt().to_string(),
        }];

        for step in 0..self.max_steps {
            info!(step, agent = agent.name(), "executing step");
            match agent.step(&mut messages).await? {
                StepResult::Continue => continue,
                StepResult::Done(output) => {
                    info!(step, "agent finished");
                    return Ok(output);
                }
            }
        }

        anyhow::bail!(
            "agent '{}' hit max step limit ({})",
            agent.name(),
            self.max_steps
        )
    }
}
