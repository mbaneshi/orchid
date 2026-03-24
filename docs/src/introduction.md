# Orchid — Agentic Dev OS

Orchid is a single Rust binary that orchestrates AI agents, workflows, and developer tools into a unified operating system for software development.

## What is Orchid?

Orchid provides a composable agent framework with three surfaces:

- **CLI** — scriptable commands for CI/CD and terminal workflows
- **TUI** — interactive ratatui dashboard with keyboard navigation
- **Web** — Axum-powered REST API and web interface

All three share the same 6-crate core, giving you a consistent experience regardless of how you interact with Orchid.

## Key Concepts

### Agents

Agents are autonomous units that perform specific tasks. Orchid ships with:

- **GitSummarizerAgent** — summarizes git history into structured changelogs
- **ContentDrafterAgent** — drafts developer-facing content from structured input

All agents use the Anthropic Claude API for LLM-powered reasoning.

### Workflows

Workflows compose multiple agents into multi-step pipelines. For example, the `dev-to-content` flow chains the git summarizer into the content drafter to go from commits to published content automatically.

### Surfaces

Orchid exposes its capabilities through three interfaces, all backed by the same core logic and database.

## Source Code

Orchid is open source under the MIT license: [github.com/mbaneshi/orchid](https://github.com/mbaneshi/orchid)
