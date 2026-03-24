use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;

use crate::agent::{Agent, Message, StepResult};
use crate::llm::LlmClient;
use crate::tool::Tool;
use crate::tools::GitHubIssuesTool;

enum State {
    FetchIssues,
    Analyze { issues_text: String },
}

pub struct ReplyMonitorAgent {
    repo: String,
    llm: Box<dyn LlmClient>,
    tools: Vec<Box<dyn Tool>>,
    state: State,
}

impl ReplyMonitorAgent {
    pub fn new(repo: String, llm: Box<dyn LlmClient>) -> Self {
        Self {
            repo,
            llm,
            tools: vec![Box::new(GitHubIssuesTool)],
            state: State::FetchIssues,
        }
    }
}

#[async_trait]
impl Agent for ReplyMonitorAgent {
    fn name(&self) -> &str {
        "reply-monitor"
    }

    fn system_prompt(&self) -> &str {
        "You are a GitHub issue triage assistant. You will receive a list of open issues \
         with their comments. Your job is to:\n\
         1. Identify issues that have no maintainer reply yet\n\
         2. Identify issues where the last comment is from an external contributor awaiting response\n\
         3. Prioritize by age and engagement\n\
         Output a structured report with: issue number, title, status (needs-reply / stale / ok), \
         and a suggested action for each."
    }

    fn tools(&self) -> &[Box<dyn Tool>] {
        &self.tools
    }

    async fn step(&mut self, messages: &mut Vec<Message>) -> Result<StepResult> {
        match std::mem::replace(&mut self.state, State::FetchIssues) {
            State::FetchIssues => {
                let result = GitHubIssuesTool
                    .execute(json!({ "repo": self.repo, "limit": 30 }))
                    .await?;

                let issues = result["issues"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();

                let mut issues_text = String::new();
                for issue in &issues {
                    let number = issue["number"].as_u64().unwrap_or(0);
                    let title = issue["title"].as_str().unwrap_or("");
                    let author = issue["author"]["login"].as_str().unwrap_or("unknown");
                    let created = issue["createdAt"].as_str().unwrap_or("");
                    let comment_count = issue["comments"]
                        .as_array()
                        .map(|a| a.len())
                        .unwrap_or(0);
                    let body = issue["body"].as_str().unwrap_or("").chars().take(300).collect::<String>();

                    issues_text.push_str(&format!(
                        "\n#{number} — {title}\n  Author: {author} | Created: {created} | Comments: {comment_count}\n  {body}\n",
                    ));
                }

                if issues_text.is_empty() {
                    return Ok(StepResult::Done("No open issues found.".to_string()));
                }

                self.state = State::Analyze { issues_text };
                Ok(StepResult::Continue)
            }
            State::Analyze { issues_text } => {
                messages.push(Message {
                    role: "user".to_string(),
                    content: format!(
                        "Here are the open issues for '{}':\n\n{}\n\nPlease analyze which need replies and produce a triage report.",
                        self.repo, issues_text
                    ),
                });

                let response = self.llm.chat(messages).await?;
                let report = response.content.clone();
                messages.push(response);

                Ok(StepResult::Done(report))
            }
        }
    }
}
