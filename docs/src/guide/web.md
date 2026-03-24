# Web UI

## Starting the Server

```bash
orchid web
```

The web server starts on `http://localhost:3000` by default using Axum.

## API Endpoints

The web surface exposes REST API endpoints for programmatic access to all Orchid capabilities.

## Architecture

The web UI is built on:

- **Axum** — async HTTP framework
- **Tower** — middleware stack
- **SQLite** — shared data layer with CLI and TUI
