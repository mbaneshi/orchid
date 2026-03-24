# Quick Start

## Run the CLI

```bash
# Show version
cargo run -- version

# Start the web server
cargo run -- web

# Run an agent
cargo run -- agent git-summarizer

# Execute a workflow pipeline
cargo run -- flow dev-to-content
```

## Environment Setup

Set your Anthropic API key for agent LLM integration:

```bash
export ANTHROPIC_API_KEY="your-key-here"
```

## What Next?

- Explore the [CLI Reference](../guide/cli.md) for all available commands
- Read about the [Architecture](../architecture/overview.md) to understand how Orchid works
- Try the [Web UI](../guide/web.md) for a browser-based experience
