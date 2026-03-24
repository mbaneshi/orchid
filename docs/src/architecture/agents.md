# Agent Framework

## Overview

Agents are autonomous units that perform specific tasks. Each agent implements the async `Agent` trait and can be run independently or composed into workflows.

## The Agent Trait

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    async fn run(&self, context: &AgentContext) -> Result<AgentOutput>;
}
```

## Built-in Agents

### GitSummarizerAgent

Reads git history and produces structured summaries of changes, grouped by category (features, fixes, refactors, etc.).

### ContentDrafterAgent

Takes structured input (like a git summary) and drafts developer-facing content — blog posts, changelogs, release notes.

## LLM Integration

All agents use the `AnthropicClient` for Claude API access. Set `ANTHROPIC_API_KEY` in your environment to enable LLM-powered reasoning.

## Creating Custom Agents

Implement the `Agent` trait and register your agent with the `AgentRunner`:

```rust
pub struct MyAgent;

#[async_trait]
impl Agent for MyAgent {
    async fn run(&self, context: &AgentContext) -> Result<AgentOutput> {
        // Your agent logic here
        todo!()
    }
}
```
