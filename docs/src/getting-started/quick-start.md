# Quick Start

## Run the Full Pipeline

The `dev-to-content` flow summarizes your git history and drafts social media content:

```bash
orchid flow -n dev-to-content -r /path/to/your/repo
```

This runs two agents in sequence:
1. **GitSummarizerAgent** — fetches commits + diffs, sends to Claude for summary
2. **ContentDrafterAgent** — drafts a tweet, LinkedIn post, and blog paragraph

## Run Individual Agents

```bash
# Summarize git history
orchid agent -n git-summarizer -r /path/to/repo

# Draft content from a summary
orchid agent -n content-drafter -i "your summary text here"
```

## Start the Web Dashboard

```bash
orchid web
# Open http://localhost:3100
```

## Check Health

```bash
curl http://localhost:3100/health
# orchid ok
```

## What Next?

- Explore the [CLI Reference](../guide/cli.md) for all available commands
- Read about the [Architecture](../architecture/overview.md) to understand how Orchid works
- Try the [Web UI](../guide/web.md) for a browser-based experience
