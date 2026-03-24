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
│   ├── orchid/          # CLI binary
│   ├── orchid-core/     # Core library
│   ├── orchid-agent/    # Agent framework
│   ├── orchid-workflow/ # Workflow engine
│   ├── orchid-web/      # Web server
│   └── orchid-tui/      # Terminal UI
├── docs/                # mdBook documentation
└── Cargo.toml           # Workspace manifest
```

## Guidelines

- Run `cargo clippy` before submitting
- Add tests for new functionality
- Keep crate boundaries clean — core should not depend on surfaces
