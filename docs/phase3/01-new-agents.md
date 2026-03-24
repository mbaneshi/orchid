# Spec 01 — New Agents

## Goal

Add 3 new agents to `orchid-agent` and wire them into the CLI.

## Agents to build

### 1. ReplyMonitorAgent

**Purpose**: Monitor a repo's GitHub issues/discussions for items needing reply.

**File**: `crates/orchid-agent/src/agents/reply_monitor.rs`

**Tools needed** (new):
- `GitHubIssuesTool` → `crates/orchid-agent/src/tools/github_issues.rs`
  - Runs `gh issue list --repo {repo} --state open --json number,title,author,createdAt,comments --limit 20`
  - Returns JSON array of issues with comment counts

**State machine**:
1. `FetchIssues` — call GitHubIssuesTool
2. `Analyze { issues }` — send to LLM: "Which of these need a reply from the maintainer? Summarize each with priority (high/medium/low)."
3. `Done(String)` — return prioritized list

**System prompt**: "You are a developer relations assistant. Given a list of GitHub issues with their comment history, identify which ones need a reply from the maintainer. Prioritize by: unanswered questions, bug reports without acknowledgment, feature requests with community support. Return a prioritized action list."

**CLI**: `orchid agent -n reply-monitor -r <repo>`

### 2. OutreachDrafterAgent

**Purpose**: Given a contact name and context, draft a personalized outreach message.

**File**: `crates/orchid-agent/src/agents/outreach_drafter.rs`

**Tools needed**: None (LLM-only, like ContentDrafterAgent)

**State machine**: Single-step (like ContentDrafterAgent)
1. Send input to LLM → return drafted message

**System prompt**: "You are a professional networking assistant. Given a contact's name and context about them, draft a brief, warm, personalized outreach message. Be genuine, reference specific shared interests or work. Keep it under 150 words. Draft both a short DM version and an email version."

**CLI**: `orchid agent -n outreach-drafter -i "Name: Alice, Context: Met at RustConf, works on async runtimes"`

### 3. CrmIngestionAgent

**Purpose**: Parse unstructured text (meeting notes, emails) and extract structured contact/interaction data.

**File**: `crates/orchid-agent/src/agents/crm_ingestion.rs`

**Tools needed**: None (LLM-only)

**State machine**: Single-step
1. Send unstructured text to LLM → return structured JSON

**System prompt**: "You are a CRM data extraction assistant. Given unstructured text (meeting notes, emails, chat logs), extract structured data: contacts mentioned (name, role, company, email if present), action items, follow-up dates, topics discussed. Return valid JSON with keys: contacts (array), action_items (array), follow_ups (array of {contact, date, topic}), summary (string)."

**CLI**: `orchid agent -n crm-ingestion -i "Meeting notes: ..."`

## Files to create

```
crates/orchid-agent/src/agents/reply_monitor.rs
crates/orchid-agent/src/agents/outreach_drafter.rs
crates/orchid-agent/src/agents/crm_ingestion.rs
crates/orchid-agent/src/tools/github_issues.rs
```

## Files to modify

1. **`crates/orchid-agent/src/agents/mod.rs`** — add `pub mod` + `pub use` for 3 new agents
2. **`crates/orchid-agent/src/tools/mod.rs`** — add `pub mod github_issues` + `pub use`
3. **`crates/orchid-agent/src/lib.rs`** — add re-exports for new agents and tools
4. **`crates/orchid/src/main.rs`** — add match arms in Agent command for the 3 new agent names; add `--context` CLI arg for outreach-drafter

## Existing code patterns to follow

Use `GitSummarizerAgent` as the template for `ReplyMonitorAgent` (multi-step with tools).
Use `ContentDrafterAgent` as the template for `OutreachDrafterAgent` and `CrmIngestionAgent` (single-step LLM-only).

**Agent trait** (from `crates/orchid-agent/src/agent.rs`):
```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn system_prompt(&self) -> &str;
    fn tools(&self) -> &[Box<dyn Tool>];
    async fn step(&mut self, messages: &mut Vec<Message>) -> Result<StepResult>;
}
```

**Tool trait** (from `crates/orchid-agent/src/tool.rs`):
```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: Value) -> Result<Value>;
}
```

**main.rs agent dispatch pattern** (existing):
```rust
"git-summarizer" => {
    let repo = repo.context("--repo required for git-summarizer")?;
    let llm = make_llm(&config)?;
    let mut agent = GitSummarizerAgent::new(repo, llm);
    let output = runner.run(&mut agent).await?;
    // save artifact...
}
```

## Testing

- `cargo test -p orchid-agent` — unit tests for each agent
- For ReplyMonitorAgent: mock `gh` output by creating a test that uses EchoLlmClient
- For single-step agents: test that step() returns Done with non-empty content using EchoLlmClient

## Artifact storage

All agents should save their output as artifacts:
- reply-monitor: `artifact_type = "reply-monitor"`
- outreach-drafter: `artifact_type = "outreach-draft"`
- crm-ingestion: `artifact_type = "crm-data"`
