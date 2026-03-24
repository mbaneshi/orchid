# Orchid — Agentic Dev OS

A single Rust binary that orchestrates AI agents, workflows, and developer tools into a unified operating system for software development.

## Vision

Orchid gives you a composable agent framework with three surfaces — **CLI**, **TUI dashboard**, and **Web UI** — all sharing the same 6-crate core.

One binary. One database. Agentic workflows from commit to content.

---

## Architecture

```
orchid (CLI binary)
├── orchid-web        — Axum web server (localhost:3100)
├── orchid-tui        — Ratatui terminal dashboard (planned)
├── orchid-workflow   — Trigger/step/agent workflow DAG
├── orchid-agent      — Agent framework, tools, LLM clients
│   ├── AnthropicClient  — Direct API (requires credits)
│   └── ClaudeCliClient  — Claude Max subscription (free via CLI)
└── orchid-core       — Config (~/.orchid/), SQLite storage, sessions
```

## Features

### CLI Commands

| Command | Description | Example |
|---------|-------------|---------|
| `agent` | Run individual agents | `orchid agent -n git-summarizer -r .` |
| `flow`  | Execute end-to-end pipelines | `orchid flow -n dev-to-content -r .` |
| `web`   | Launch the web UI server | `orchid web` |
| `workspace` | Open the TUI workspace | `orchid workspace` |
| `version` | Show version info | `orchid version` |

### Agent Framework

- **GitSummarizerAgent** — analyzes git log + diffs, produces structured summaries via LLM
- **ContentDrafterAgent** — transforms summaries into tweet, LinkedIn post, and blog paragraph
- **Tool system** — `GitLogTool` and `GitDiffTool` for repo analysis
- **AgentRunner** — drives multi-step agent execution loops

### LLM Integration

Two authentication methods:

1. **Claude Max subscription** (default) — spawns `claude` CLI with `CLAUDE_USE_SUBSCRIPTION=true`. No API credits needed.
2. **Anthropic API key** — direct HTTP calls via `ANTHROPIC_API_KEY` env var or `~/.orchid/config.toml`

### Workflow Engine

- `Workflow` / `Trigger` / `Step` DAG model (manual, cron, webhook triggers)
- `dev-to-content` flow: git summarizer → content drafter

### Storage

- SQLite via rusqlite (bundled, no system dependency)
- `sessions` table for Code/Content/Relationship sessions
- `artifacts` table for agent outputs (summaries, drafts)
- Config at `~/.orchid/config.toml`

---

## Quick Start

```bash
# Clone and build
git clone https://github.com/mbaneshi/orchid.git
cd orchid
cargo build --release

# Run the full dev-to-content pipeline (uses Claude Max if available)
./target/release/orchid flow -n dev-to-content -r .

# Run individual agents
./target/release/orchid agent -n git-summarizer -r /path/to/repo
./target/release/orchid agent -n content-drafter -i "your summary text"

# Start the web dashboard
./target/release/orchid web
# Open http://localhost:3100
```

### LLM Setup

If you have Claude Code with a Max subscription, it works out of the box — no configuration needed.

Otherwise, set an API key:

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

Or add to `~/.orchid/config.toml`:

```toml
[llm]
api_key = "sk-ant-..."
model = "claude-sonnet-4-20250514"
```

## Documentation

Full documentation is available at [mbaneshi.github.io/orchid](https://mbaneshi.github.io/orchid/).

---

## Project Status

| Phase   | Focus | Status |
|---------|-------|--------|
| Phase 0 | Foundation — workspace, core crates, CLI skeleton | Done |
| Phase 1 | Working agents + LLM integration + git tools | Done |
| Phase 2 | Claude Max auth, web landing page, artifact storage | Done |
| Phase 3 | TUI workspace, SvelteKit frontend, more agents | Next |

## By the Numbers

| Metric | Value |
|--------|-------|
| Crates | 6 |
| Rust source files | 21 |
| Surfaces | 3 (CLI, TUI, Web) |
| Agents | 2 (+ extensible) |
| LLM providers | 2 (API + CLI/Max) |
| License | MIT |

---

## License

MIT
