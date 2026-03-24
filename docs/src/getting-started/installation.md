# Installation

## Prerequisites

- [Rust](https://rustup.rs/) (1.75+ recommended)
- SQLite (bundled via `rusqlite` — no system install needed)
- [Claude Code](https://claude.ai/claude-code) with Max subscription (for LLM features, optional)

## Build from Source

```bash
git clone https://github.com/mbaneshi/orchid.git
cd orchid
cargo build --release
```

The binary will be at `target/release/orchid`.

## Install via Cargo

```bash
cargo install --path crates/orchid
```

## Verify Installation

```bash
orchid version
# orchid 0.1.0
```

## LLM Setup

Orchid supports two authentication methods for Claude:

### Option 1: Claude Max Subscription (Recommended)

If you have Claude Code installed with a Max subscription, Orchid uses it automatically. No configuration needed.

### Option 2: Anthropic API Key

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

Or add to `~/.orchid/config.toml`:

```toml
[llm]
api_key = "sk-ant-..."
model = "claude-sonnet-4-20250514"
```
