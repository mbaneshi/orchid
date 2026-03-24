# Workflow Engine

## Overview

The workflow engine orchestrates multi-step pipelines by composing agents. Each step references an agent by name and declares dependencies on other steps.

## Data Model

```rust
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub trigger: Trigger,
    pub steps: Vec<Step>,
}

pub enum Trigger {
    Manual,
    Schedule { cron: String },
    Webhook { path: String },
}

pub struct Step {
    pub id: Uuid,
    pub name: String,
    pub agent_name: String,
    pub depends_on: Vec<Uuid>,
}
```

## Built-in Flows

### dev-to-content

Chains `GitSummarizerAgent` → `ContentDrafterAgent` to automatically transform git commits into social media content.

```bash
orchid flow -n dev-to-content -r /path/to/repo
```

**Step 1:** Summarize git history (commits + diffs → structured summary)
**Step 2:** Draft content (summary → tweet + LinkedIn post + blog paragraph)

Both outputs are saved as artifacts in SQLite.

## Current State

The `Workflow`/`Trigger`/`Step` data model exists but flows are currently hardcoded in `main.rs`. The next phase will wire up the workflow engine to dynamically execute workflow definitions from storage.
