use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

/// A tool that an agent can invoke.
#[async_trait]
pub trait Tool: Send + Sync {
    /// Unique name for this tool.
    fn name(&self) -> &str;

    /// Human-readable description for the LLM.
    fn description(&self) -> &str;

    /// Execute the tool with the given JSON arguments, returning a JSON result.
    async fn execute(&self, args: Value) -> Result<Value>;
}
