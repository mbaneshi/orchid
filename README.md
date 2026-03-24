
  Vision

  Nexus is a single Rust binary that gives you a complete picture of your machine:
  - Discovery — deep-index your entire home directory with instant full-text search
  - Config Management — backup, diff, restore, and provision all 25 tools in ~/.config
  - AI Integration — ask natural language questions about your filesystem via Claude
  - Three surfaces — CLI, TUI dashboard, and Web UI — all sharing the same 8-crate core

  One binary. One database. Complete picture of your machine.

  ---
  The 3-Phase Plan

  ┌─────────┬──────────────────────────────────────────────────────┬─────────────┐
  │  Phase  │                        Focus                         │   Status    │
  ├─────────┼──────────────────────────────────────────────────────┼─────────────┤
  │ Phase 0 │ Foundation — scan, index, search, config backup, CLI │ Done        │
  ├─────────┼──────────────────────────────────────────────────────┼─────────────┤
  │ Phase 1 │ Watcher daemon + AI integration                      │ Done        │
  ├─────────┼──────────────────────────────────────────────────────┼─────────────┤
  │ Phase 2 │ Surfaces — TUI, Web API, frontend, polish            │ In Progress │
  └─────────┴──────────────────────────────────────────────────────┴─────────────┘

  ---
  What's Done (Phase 0 + 1 + 2 partial)

  Core infrastructure
  - 8-crate workspace, ~9,000+ lines of Rust
  - SQLite WAL + FTS5 database (9 tables)
  - 34 tests passing, 0 clippy warnings

  17 CLI commands
  - scan, search, stats, changes
  - config list/show/backup/snapshots/restore/diff/init/path
  - config profile save/list/apply/delete
  - daemon start/stop/status
  - ask (AI), tui, serve

  4 TUI screens (built this session)
  - Overview with category breakdown
  - Configs with j/k navigation + syntax-highlighted file viewer
  - Interactive search with live FTS5 results
  - Recent changes panel

  10 API endpoints (built this session)
  - health, stats, search, config CRUD, daemon status, changes

  Daemon → DB wiring (built this session)
  - File changes persisted to database
  - Auto-snapshot on config directory changes

  ---
  What Remains (Phase 2 completion)

  1. SvelteKit frontend scaffold — pnpm + Tailwind, pages for dashboard, search, config browser
  2. Embed frontend in binary — rust-embed so nexus serve serves the SPA
  3. nexus config export/import — portable tar.gz for machine migration
  4. Syntax highlighting in CLI — nexus config show with syntect colors

  ---
  By the Numbers

  ┌───────────────┬─────────────────────┬───────────────────────────┐
  │    Metric     │ Before this session │           After           │
  ├───────────────┼─────────────────────┼───────────────────────────┤
  │ Tests         │ 17                  │ 34                        │
  ├───────────────┼─────────────────────┼───────────────────────────┤
  │ TUI screens   │ 3 (1 stub)          │ 4 (all interactive)       │
  ├───────────────┼─────────────────────┼───────────────────────────┤
  │ API endpoints │ 3                   │ 10                        │
  ├───────────────┼─────────────────────┼───────────────────────────┤
  │ CLI commands  │ 16                  │ 17                        │
  ├───────────────┼─────────────────────┼───────────────────────────┤
  │ Daemon → DB   │ logging only        │ persists + auto-snapshots │
  ├───────────────┼─────────────────────┼───────────────────────────┤
  │ Source files  │ ~37                 │ ~50                       │
  └───────────────┴─────────────────────┴───────────────────────────┘
