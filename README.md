# Orchid — Agentic Dev OS

A single Rust binary that orchestrates AI agents, workflows, and developer tools into a unified operating system for software development.

## Vision

Orchid gives you a composable agent framework with three surfaces — **CLI**, **TUI dashboard**, and **Web UI** — all sharing the same 6-crate core.

One binary. One database. Agentic workflows from commit to content.

---

## Architecture

```
orchid (CLI)
├── orchid-web      — Axum web server & REST API
├── orchid-tui      — Ratatui terminal dashboard
├── orchid-workflow  — Multi-step workflow orchestration
├── orchid-agent    — Agent framework + Anthropic LLM integration
└── orchid-core     — Config, SQLite storage, serialization, utilities
```

## Features

### CLI Commands

| Command     | Description                                |
|-------------|--------------------------------------------|
| `web`       | Launch the web UI server                   |
| `workspace` | Open the TUI workspace                     |
| `agent`     | Run individual agents (git-summarizer, content-drafter) |
| `flow`      | Execute end-to-end pipelines (dev-to-content) |
| `version`   | Show version info                          |

### Agent Framework

- **GitSummarizerAgent** — summarizes git history into structured changelogs
- **ContentDrafterAgent** — drafts developer-facing content from structured input
- **Anthropic LLM client** — native Claude integration for all agents

### Workflow Engine

- Multi-step pipeline orchestration
- Agent composition and chaining
- JSON-defined workflow definitions

### Three Surfaces

- **CLI** — scriptable commands for CI/CD and terminal workflows
- **TUI** — interactive ratatui dashboard with keyboard navigation
- **Web** — Axum-powered REST API and web interface

---

## Quick Start

```bash
# Clone and build
git clone https://github.com/mbaneshi/orchid.git
cd orchid
cargo build --release

# Run the CLI
cargo run -- version
cargo run -- web        # start web server
cargo run -- agent      # run an agent
cargo run -- flow       # execute a workflow pipeline
```

## Documentation

Full documentation is available at [mbaneshi.github.io/orchid](https://mbaneshi.github.io/orchid/).

---

## Project Status

| Phase   | Focus                                          | Status      |
|---------|-------------------------------------------------|-------------|
| Phase 0 | Foundation — core crates, agent abstraction, CLI | Done        |
| Phase 1 | Agent implementations + LLM integration          | Done        |
| Phase 2 | Workflows, Web API, TUI, polish                  | In Progress |

## By the Numbers

| Metric       | Value                |
|--------------|----------------------|
| Crates       | 6                    |
| Surfaces     | 3 (CLI, TUI, Web)   |
| Agents       | 2 (+ extensible)    |
| License      | MIT                  |

---

## License

MIT
