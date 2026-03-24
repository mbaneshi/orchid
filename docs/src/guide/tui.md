# TUI Dashboard

## Status

The TUI is **not yet implemented**. Running `orchid workspace` will print a placeholder message.

## Planned Design

Zellij-inspired pane layout with:

- Shell pane — embedded terminal
- Agent chat pane — interact with agents
- File tree pane — Yazi-inspired file navigator
- Task list pane — current session tasks

## Architecture

The TUI will be built on:

- **Ratatui** — terminal rendering framework
- **Crossterm** — cross-platform terminal backend
- **orchid-core** — shared config, storage, and session layer
