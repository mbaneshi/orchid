# CLI Reference

## Usage

```bash
orchid <COMMAND>
```

## Commands

### `version`

Print version information.

```bash
orchid version
# orchid 0.1.0
```

### `web`

Start the web UI server on `127.0.0.1:3100`.

```bash
orchid web
```

### `workspace`

Open the TUI workspace dashboard (not yet implemented).

```bash
orchid workspace
```

### `agent`

Run an individual agent.

```bash
# Summarize git history (requires --repo)
orchid agent --name git-summarizer --repo /path/to/repo

# Draft content from text (requires --input)
orchid agent --name content-drafter --input "your summary text"
```

**Options:**
- `-n, --name <NAME>` — agent name (required): `git-summarizer`, `content-drafter`
- `-r, --repo <PATH>` — repository path (for git-summarizer)
- `-i, --input <TEXT>` — input text (for content-drafter)

### `flow`

Execute an end-to-end workflow pipeline.

```bash
# Full pipeline: git history → summary → tweet/LinkedIn/blog
orchid flow --name dev-to-content --repo /path/to/repo
```

**Options:**
- `-n, --name <NAME>` — flow name (required): `dev-to-content`
- `-r, --repo <PATH>` — repository path

## LLM Resolution

The CLI automatically selects an LLM client:

1. If `ANTHROPIC_API_KEY` is set or configured → uses `AnthropicClient` (direct API)
2. Otherwise → uses `ClaudeCliClient` (Claude Max subscription via CLI)

## Output

Agent outputs are:
- Printed to stdout
- Saved as artifacts in `~/.orchid/orchid.db`
