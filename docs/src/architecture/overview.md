# Architecture Overview

Orchid is organized as a Rust workspace with 6 crates, layered from foundational to surface-level:

```
orchid (CLI binary — clap subcommands)
├── orchid-web        — Axum web server (port 3100)
├── orchid-tui        — Ratatui terminal dashboard (planned)
├── orchid-workflow   — Workflow/Trigger/Step DAG model
├── orchid-agent      — Agent framework + LLM clients + tools
└── orchid-core       — Config, SQLite storage, session types
```

## Design Principles

1. **Single binary** — everything compiles into one executable
2. **Shared core** — all surfaces use the same data layer and logic
3. **Agent composability** — agents are independent units that can be chained into workflows
4. **Local-first** — SQLite embedded, config at `~/.orchid/`, no cloud required
5. **Claude Max native** — uses your existing subscription, no API credits needed

## Data Flow

```
User Input → Surface (CLI/TUI/Web)
                ↓
           Flow Dispatcher
                ↓
           AgentRunner (step loop)
                ↓
           Agent (tools + LLM calls)
                ↓
         Storage (SQLite artifacts + sessions)
```

## LLM Resolution

```
1. ~/.orchid/config.toml [llm].api_key  →  AnthropicClient (HTTP API)
2. ANTHROPIC_API_KEY env var            →  AnthropicClient (HTTP API)
3. Claude CLI + Max subscription        →  ClaudeCliClient (subprocess)
```
