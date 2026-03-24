# Phase 3 — Implementation Specs

Each file below is a self-contained spec that a subagent can pick up and implement independently.
The specs are ordered by dependency (least → most dependent on others).

| # | Spec | Crate(s) touched | Depends on |
|---|------|-------------------|------------|
| 1 | [01-new-agents.md](01-new-agents.md) | orchid-agent, orchid (main.rs) | nothing |
| 2 | [02-workflow-engine.md](02-workflow-engine.md) | orchid-workflow, orchid-core (storage), orchid (main.rs) | nothing |
| 3 | [03-tui-workspace.md](03-tui-workspace.md) | orchid-tui, orchid (main.rs) | nothing |
| 4 | [04-rest-api.md](04-rest-api.md) | orchid-web, orchid (main.rs) | nothing (uses existing traits) |
| 5 | [05-sveltekit-frontend.md](05-sveltekit-frontend.md) | new: frontend/ dir, orchid-web (embed) | spec 04 (API endpoints) |

## Parallel execution strategy

Specs 1–4 can run in parallel with zero conflicts (they touch different files).
Spec 5 depends on spec 4's API shape but can start scaffolding immediately.

## Shared conventions

- **Error handling**: `thiserror` in libraries, `anyhow` in binary
- **Async runtime**: tokio (already configured)
- **Config**: extend `orchid_core::Config` as needed via new sections
- **Storage**: extend `SqliteStorage` with new tables via migration pattern
- **LLM**: reuse `make_llm()` from main.rs — all agents get the same client
- **Testing**: `cargo test` — integration tests where possible
- **Commits**: conventional commits (`feat:`, `fix:`, etc.)
