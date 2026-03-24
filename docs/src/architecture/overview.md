# Architecture Overview

Orchid is organized as a Rust workspace with 6 crates, layered from foundational to surface-level:

```
orchid (CLI binary)
├── orchid-web        — Axum web server & REST API
├── orchid-tui        — Ratatui terminal dashboard
├── orchid-workflow   — Multi-step workflow orchestration
├── orchid-agent      — Agent framework + Anthropic LLM
└── orchid-core       — Config, SQLite, serialization, utils
```

## Design Principles

1. **Single binary** — everything compiles into one executable
2. **Shared core** — all surfaces use the same data layer and logic
3. **Agent composability** — agents are independent units that can be chained into workflows
4. **Three surfaces** — CLI, TUI, and Web all expose the same capabilities

## Data Flow

```
User Input → Surface (CLI/TUI/Web)
                ↓
           Workflow Engine
                ↓
           Agent Runner
                ↓
         Core (SQLite + Config)
```
