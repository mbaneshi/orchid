# Orchid — Agentic Dev OS

Orchid is a single Rust binary that orchestrates AI agents, workflows, and developer tools into a unified operating system for software development.

## What is Orchid?

Orchid provides a composable agent framework with three surfaces:

- **CLI** — scriptable commands for agents, flows, and automation
- **TUI** — interactive ratatui dashboard (planned)
- **Web** — Axum-powered web dashboard at `localhost:3100`

All three share the same 6-crate core, giving you a consistent experience regardless of how you interact with Orchid.

## Key Concepts

### Agents

Agents are autonomous units that perform specific tasks using tools and LLMs. Orchid ships with:

- **GitSummarizerAgent** — reads git log and diffs, sends to LLM for structured summary
- **ContentDrafterAgent** — transforms a summary into tweet, LinkedIn post, and blog paragraph

### Tools

Agents use tools to interact with the outside world:

- **GitLogTool** — runs `git log` and parses commits
- **GitDiffTool** — runs `git show` to get diffs (with truncation for large changes)

### LLM Clients

Two ways to connect to Claude:

1. **ClaudeCliClient** — spawns the `claude` CLI binary using your Max subscription (default, no API credits needed)
2. **AnthropicClient** — direct HTTP calls to `api.anthropic.com` (requires API key with credits)

### Workflows

Workflows compose multiple agents into multi-step pipelines. The `dev-to-content` flow chains the git summarizer into the content drafter to go from commits to published content automatically.

### Storage

All agent outputs (summaries, drafts) are saved as artifacts in SQLite at `~/.orchid/orchid.db`. Sessions track ongoing work across code, content, and relationship domains.

## Source Code

Orchid is open source under the MIT license: [github.com/mbaneshi/orchid](https://github.com/mbaneshi/orchid)
