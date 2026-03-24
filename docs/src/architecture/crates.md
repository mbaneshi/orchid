# Crate Structure

## orchid (binary)

The main CLI binary. Provides the entry point and command routing via `clap`.

**Commands:** `web`, `workspace`, `agent`, `flow`, `version`

**Key logic:** `make_llm()` resolves the LLM client — tries API key first, falls back to Claude CLI.

## orchid-core

Foundational library shared by all other crates.

- **Config** — loads from `~/.orchid/config.toml`, with `LlmConfig` and `WebConfig`
- **Session types** — `Session`, `CodeSession`, `ContentSession`, `RelationshipSession`
- **SqliteStorage** — `sessions` and `artifacts` tables, auto-creates `~/.orchid/orchid.db`
- **Storage trait** — abstract interface for persistence

## orchid-agent

Agent framework, tools, and LLM client implementations.

- **`Agent` trait** — `name()`, `system_prompt()`, `tools()`, `step(&mut self, messages) -> StepResult`
- **`Tool` trait** — `name()`, `description()`, `execute(args: Value) -> Value`
- **`LlmClient` trait** — `chat(messages) -> Message`
- **`AgentRunner`** — drives the step loop with a max-steps limit
- **LLM clients:**
  - `AnthropicClient` — direct HTTP to `api.anthropic.com/v1/messages`
  - `ClaudeCliClient` — spawns `claude -p` with `CLAUDE_USE_SUBSCRIPTION=true`
- **Tools:**
  - `GitLogTool` — runs `git log`, parses commits (hash, author, message, date, stat)
  - `GitDiffTool` — runs `git show`, returns diff (truncated at 8000 chars)
- **Built-in agents:**
  - `GitSummarizerAgent` — 2-step: fetch commits/diffs, then summarize via LLM
  - `ContentDrafterAgent` — 1-step: draft tweet/LinkedIn/blog from summary

## orchid-workflow

Workflow data model (not yet wired to execution).

- `Workflow` — named DAG of steps
- `Trigger` — `Manual`, `Schedule { cron }`, `Webhook { path }`
- `Step` — references an agent by name, with `depends_on` for ordering

## orchid-web

Web server and dashboard surface.

- Axum-based HTTP server on `127.0.0.1:3100`
- `GET /` — HTML landing page with project overview
- `GET /health` — health check endpoint

## orchid-tui

Terminal user interface (placeholder).

- Dependencies: ratatui + crossterm
- Not yet implemented — returns error if launched
