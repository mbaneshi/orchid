# Roadmap

## Phase 0 — Foundation (Done)

- Cargo workspace with 6 crates
- Core types: Config, Session, Storage trait, SqliteStorage
- CLI entry point with clap (workspace, web, agent, flow, version)
- Agent/Tool/LlmClient trait definitions

## Phase 1 — Working Agents (Done)

- GitSummarizerAgent (2-step: fetch commits + summarize)
- ContentDrafterAgent (1-step: draft tweet/LinkedIn/blog)
- GitLogTool and GitDiffTool
- AnthropicClient (direct HTTP API)
- AgentRunner step loop
- `orchid agent` and `orchid flow` CLI commands

## Phase 2 — Claude Max + Web (Done)

- ClaudeCliClient using Max subscription (CLAUDE_USE_SUBSCRIPTION=true)
- Automatic LLM fallback: API key → Claude CLI
- Web landing page at localhost:3100
- Artifacts table in SQLite for persisting agent outputs
- End-to-end `dev-to-content` flow tested and working

## Phase 3 — TUI + SvelteKit (Next)

- Ratatui TUI workspace with panes (shell, agent chat, file tree, tasks)
- SvelteKit frontend embedded in the binary
- REST API endpoints for agents and flows
- Content calendar view
- Session management UI

## Future

- More agents: reply monitor, outreach drafter, CRM ingestion
- Workflow engine dynamic execution (not hardcoded)
- Webhook triggers and cron scheduling
- Rathole-inspired tunneling for remote access
- Plugin system for third-party agents
- Multi-model LLM support (OpenAI, Ollama, local)
- Dashboard analytics and metrics
