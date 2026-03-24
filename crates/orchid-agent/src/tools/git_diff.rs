use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{json, Value};
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

        let diff = String::from_utf8_lossy(&output.stdout).to_string();
        let diff = if diff.len() > MAX_DIFF_CHARS {
            // Find a safe char boundary to truncate at
            let mut end = MAX_DIFF_CHARS;
            while end > 0 && !diff.is_char_boundary(end) {
                end -= 1;
            }
            format!("{}\n[truncated]", &diff[..end])
        } else {
            diff
        };

        Ok(json!({ "diff": diff }))
    }
}
