use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;

use crate::agent::{Agent, Message, StepResult};
use crate::llm::LlmClient;
use crate::tool::Tool;
use crate::tools::{GitDiffTool, GitLogTool};

enum State {
    FetchCommits,
    Summarize { git_output: String },
}

pub struct GitSummarizerAgent {
    repo_path: String,
    llm: Box<dyn LlmClient>,
    tools: Vec<Box<dyn Tool>>,
    state: State,
}

impl GitSummarizerAgent {
    pub fn new(repo_path: String, llm: Box<dyn LlmClient>) -> Self {
        Self {
            repo_path,
            llm,
            tools: vec![Box::new(GitLogTool), Box::new(GitDiffTool)],
            state: State::FetchCommits,
        }
    }
}

#[async_trait]
impl Agent for GitSummarizerAgent {
    fn name(&self) -> &str {
        "git-summarizer"
    }

    fn system_prompt(&self) -> &str {
        "You are a git commit summarizer. You will receive recent git log output and diffs. \
         Produce a concise, well-organized summary covering:\n\
         - What changed (features, fixes, refactors)\n\
         - Key files affected\n\
         - Overall direction of the work\n\
         Keep it to 2-4 paragraphs. Be specific about what the code changes do."
    }

    fn tools(&self) -> &[Box<dyn Tool>] {
        &self.tools
    }

    async fn step(&mut self, messages: &mut Vec<Message>) -> Result<StepResult> {
        match std::mem::replace(&mut self.state, State::FetchCommits) {
            State::FetchCommits => {
                let log_result = GitLogTool
                    .execute(json!({ "repo_path": self.repo_path, "count": 10 }))
                    .await?;

                let commits = log_result["commits"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();

                let mut git_output = String::new();
                git_output.push_str("=== Recent Commits ===\n");
                for commit in &commits {
                    let hash = commit["hash"].as_str().unwrap_or("");
                    let short = &hash[..8.min(hash.len())];
                    git_output.push_str(&format!(
                        "\n{} - {} ({})\n  {}\n",
                        short,
                        commit["message"].as_str().unwrap_or(""),
                        commit["author"].as_str().unwrap_or(""),
                        commit["stat"].as_str().unwrap_or(""),
                    ));
                }

                // Fetch diffs for top 3 commits
                for commit in commits.iter().take(3) {
                    if let Some(hash) = commit["hash"].as_str() {
                        let short = &hash[..8.min(hash.len())];
                        match GitDiffTool
                            .execute(json!({ "repo_path": self.repo_path, "commit_hash": hash }))
                            .await
                        {
                            Ok(diff_result) => {
                                git_output.push_str(&format!(
                                    "\n=== Diff for {} ===\n{}\n",
                                    short,
                                    diff_result["diff"].as_str().unwrap_or("")
                                ));
                            }
                            Err(e) => {
                                git_output.push_str(&format!(
                                    "\n=== Diff for {} failed: {} ===\n",
                                    short, e
                                ));
                            }
                        }
                    }
                }

                self.state = State::Summarize { git_output };
                Ok(StepResult::Continue)
            }
            State::Summarize { git_output } => {
                messages.push(Message {
                    role: "user".to_string(),
                    content: format!(
                        "Here is the recent git activity for the repository at '{}':\n\n{}\n\nPlease summarize this development work.",
                        self.repo_path, git_output
                    ),
                });

                let response = self.llm.chat(messages).await?;
                let summary = response.content.clone();
                messages.push(response);

                Ok(StepResult::Done(summary))
            }
        }
    }
}
