# Agent Framework

## Overview

Agents are autonomous units that perform specific tasks using tools and LLMs. Each agent implements the async `Agent` trait and can be run independently or composed into workflows.

## The Agent Trait

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn system_prompt(&self) -> &str;
    fn tools(&self) -> &[Box<dyn Tool>];
    async fn step(&mut self, messages: &mut Vec<Message>) -> Result<StepResult>;
}
```

**StepResult** is either `Continue` (more steps needed) or `Done(String)` (final output).

## The Tool Trait

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: Value) -> Result<Value>;
}
```

## Built-in Agents

### GitSummarizerAgent

A 2-step agent:
1. **FetchCommits** — uses `GitLogTool` (last 10 commits) and `GitDiffTool` (top 3 diffs)
2. **Summarize** — sends the git output to the LLM with a structured prompt

```bash
orchid agent -n git-summarizer -r /path/to/repo
```

### ContentDrafterAgent

A 1-step agent that takes a summary and drafts three content formats:
1. Tweet (max 280 chars)
2. LinkedIn post (2-3 paragraphs)
3. Blog paragraph

```bash
orchid agent -n content-drafter -i "your summary"
```

## LLM Integration

Two client implementations:

| Client | Auth | Use Case |
|--------|------|----------|
| `ClaudeCliClient` | Claude Max subscription | Default, no credits needed |
| `AnthropicClient` | `ANTHROPIC_API_KEY` | Direct API, requires credits |

The `ClaudeCliClient` spawns the `claude` CLI with special env vars:
- `CLAUDE_CODE_ENTRYPOINT=sdk-max`
- `CLAUDE_USE_SUBSCRIPTION=true`
- `CLAUDE_BYPASS_BALANCE_CHECK=true`

## Creating Custom Agents

Implement the `Agent` trait:

```rust
pub struct MyAgent {
    llm: Box<dyn LlmClient>,
    tools: Vec<Box<dyn Tool>>,
}

#[async_trait]
impl Agent for MyAgent {
    fn name(&self) -> &str { "my-agent" }
    fn system_prompt(&self) -> &str { "You are a helpful assistant." }
    fn tools(&self) -> &[Box<dyn Tool>] { &self.tools }

    async fn step(&mut self, messages: &mut Vec<Message>) -> Result<StepResult> {
        messages.push(Message { role: "user".into(), content: "Hello".into() });
        let response = self.llm.chat(messages).await?;
        Ok(StepResult::Done(response.content))
    }
}
```
