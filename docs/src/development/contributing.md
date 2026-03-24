# Contributing

## Development Setup

```bash
git clone https://github.com/mbaneshi/orchid.git
cd orchid
cargo build
cargo test
```

## Project Structure

```
orchid/
├── crates/
│   ├── orchid/          # CLI binary (main.rs)
│   ├── orchid-core/     # Config, storage, sessions
│   ├── orchid-agent/    # Agent framework, tools, LLM clients
│   │   ├── agents/      # GitSummarizerAgent, ContentDrafterAgent
│   │   └── tools/       # GitLogTool, GitDiffTool
│   ├── orchid-workflow/ # Workflow/Trigger/Step model
│   ├── orchid-web/      # Axum web server
│   └── orchid-tui/      # Ratatui TUI (placeholder)
├── docs/                # mdBook documentation
└── Cargo.toml           # Workspace manifest
```

## Key Files

| File | Purpose |
|------|---------|
| `crates/orchid/src/main.rs` | CLI entry point, LLM resolution, command dispatch |
| `crates/orchid-agent/src/agent.rs` | Agent trait + Message/StepResult types |
| `crates/orchid-agent/src/llm_claude_cli.rs` | Claude Max subscription client |
| `crates/orchid-agent/src/llm_anthropic.rs` | Direct Anthropic API client |
| `crates/orchid-core/src/storage.rs` | SQLite storage (sessions + artifacts) |
| `crates/orchid-core/src/config.rs` | Config loading from ~/.orchid/config.toml |

## Guidelines

- Run `cargo clippy` before submitting
- Run `cargo fmt` for consistent formatting
- Add tests for new functionality
- Keep crate boundaries clean — core should not depend on surfaces
- New agents should implement the `Agent` trait from orchid-agent
