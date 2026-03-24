use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};
use tokio::process::Command;

use crate::tool::Tool;

pub struct GitLogTool;

#[async_trait]
impl Tool for GitLogTool {
    fn name(&self) -> &str {
        "git_log"
    }

    fn description(&self) -> &str {
        "Fetch recent git commits from a repository"
    }

    async fn execute(&self, args: Value) -> Result<Value> {
        let repo_path = args["repo_path"]
            .as_str()
            .context("repo_path is required")?;
        let count = args["count"].as_u64().unwrap_or(10);

        let output = Command::new("git")
            .args([
                "-C",
                repo_path,
                "log",
                &format!("-n{count}"),
                "--pretty=format:COMMIT_SEP%n%H%n%an%n%s%n%ai",
                "--stat",
            ])
            .output()
            .await
            .context("failed to run git log")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git log failed: {stderr}");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut commits = Vec::new();

        for block in stdout.split("COMMIT_SEP\n").filter(|s| !s.is_empty()) {
            let lines: Vec<&str> = block.lines().collect();
            if lines.len() >= 4 {
                let stat_lines: Vec<&str> = lines[4..].to_vec();
                commits.push(json!({
                    "hash": lines[0],
                    "author": lines[1],
                    "message": lines[2],
                    "date": lines[3],
                    "stat": stat_lines.join("\n"),
                }));
            }
        }

        Ok(json!({ "commits": commits }))
    }
}
