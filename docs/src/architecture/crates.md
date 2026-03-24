# Crate Structure

## orchid

The main CLI binary. Provides the entry point and command routing via `clap`.

**Commands:** `web`, `workspace`, `agent`, `flow`, `version`

## orchid-core

Foundational library shared by all other crates.

- Configuration management
- SQLite storage layer (WAL mode)
- Data serialization (JSON, TOML)
- Utility helpers (UUID, time, directories)

## orchid-agent

Agent framework and built-in implementations.

- `Agent` async trait — base abstraction for all agents
- `AnthropicClient` — native Claude LLM integration
- `AgentRunner` — executes agents with context
- Built-in agents: `GitSummarizerAgent`, `ContentDrafterAgent`

## orchid-workflow

Workflow orchestration engine.

- Multi-step pipeline definitions
- Agent composition and chaining
- JSON-serializable workflow specs

## orchid-web

Web server and API surface.

- Axum-based HTTP server
- REST API endpoints
- Static asset serving

## orchid-tui

Terminal user interface.

- Ratatui rendering
- Crossterm terminal backend
- Interactive keyboard-driven navigation
