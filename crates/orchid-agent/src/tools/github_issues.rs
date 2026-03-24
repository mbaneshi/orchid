use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::process::Command;

use crate::tool::Tool;

pub struct GitHubIssuesTool;

#[async_trait]
impl Tool for GitHubIssuesTool {
    fn name(&self) -> &str {
        "github_issues"
    }

    fn description(&self) -> &str {
        "Fetch open GitHub issues from a repository using the gh CLI"
    }

    async fn execute(&self, args: Value) -> Result<Value> {
        let repo = args["repo"]
            .as_str()
            .context("repo is required (e.g. owner/name)")?;
        let limit = args["limit"].as_u64().unwrap_or(20);
        let labels = args["labels"].as_str().unwrap_or("");

        let mut cmd_args = vec![
            "issue".to_string(),
            "list".to_string(),
            "--repo".to_string(),
            repo.to_string(),
            "--limit".to_string(),
            limit.to_string(),
            "--json".to_string(),
            "number,title,body,author,labels,createdAt,comments,state".to_string(),
        ];

        if !labels.is_empty() {
            cmd_args.push("--label".to_string());
            cmd_args.push(labels.to_string());
        }

        let output = Command::new("gh")
            .args(&cmd_args)
            .output()
            .await
            .context("failed to run gh issue list — is the gh CLI installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("gh issue list failed: {stderr}");
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let issues: Value = serde_json::from_str(&stdout).unwrap_or(json!([]));

        Ok(json!({ "issues": issues }))
    }
}
