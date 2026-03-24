# Workflow Engine

## Overview

The workflow engine orchestrates multi-step pipelines by composing agents in sequence. Each step's output feeds into the next step's input.

## Built-in Workflows

### dev-to-content

Chains `GitSummarizerAgent` → `ContentDrafterAgent` to automatically transform git commits into polished developer content.

```bash
orchid flow dev-to-content
```

## Workflow Definition

Workflows are defined as JSON-serializable specifications listing the steps, their agents, and data flow between them.

## Extending Workflows

Create new workflows by composing existing agents or custom agents into pipeline definitions.
