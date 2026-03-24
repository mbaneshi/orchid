# Web UI

## Starting the Server

```bash
orchid web
```

The web server starts on `http://localhost:3100` by default.

## Endpoints

| Route | Method | Description |
|-------|--------|-------------|
| `/` | GET | Landing page with project overview |
| `/health` | GET | Health check (returns `orchid ok`) |

## Configuration

The host and port can be configured in `~/.orchid/config.toml`:

```toml
[web]
host = "127.0.0.1"
port = 3100
```

## Architecture

The web UI is built on:

- **Axum** — async HTTP framework
- **Tower** — middleware stack
- **orchid-core** — shared config and storage layer

## Current State

The web surface currently serves a static landing page. Future work includes:

- REST API for running agents and flows
- SvelteKit frontend for dashboards, content calendar, and CRM views
- WebSocket support for streaming agent output
