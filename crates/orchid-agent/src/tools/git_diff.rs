use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};
use tokio::process::Command;

use crate::tool::Tool;

const MAX_DIFF_CHARS: usize = 8000;

pub struct GitDiffTool;

#[async_trait]
impl Tool for GitDiffTool {
    fn name(&self) -> &str {
        "git_diff"
    }

    fn description(&self) -> &str {
        "Get the diff for a specific commit"
    }

    async fn execute(&self, args: Value) -> Result<Value> {
        let repo_path = args["repo_path"]
            .as_str()
            .context("repo_path is required")?;
        let commit_hash = args["commit_hash"]
            .as_str()
            .context("commit_hash is required")?;

        let output = Command::new("git")
            .args(["-C", repo_path, "show", commit_hash, "--stat", "--patch"])
            .output()
            .await
            .context("failed to run git show")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git show failed: {stderr}");
        }

        let mut diff = String::from_utf8_lossy(&output.stdout).to_string();
        if diff.len() > MAX_DIFF_CHARS {
            diff.truncate(MAX_DIFF_CHARS);
            diff.push_str("\n[truncated]");
        }

        Ok(json!({ "diff": diff }))
    }
}
