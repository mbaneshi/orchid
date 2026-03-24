# Installation

## Prerequisites

- [Rust](https://rustup.rs/) (1.75+ recommended)
- SQLite (bundled via `rusqlite`)

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
```
