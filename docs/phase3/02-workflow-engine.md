# Spec 02 — Workflow Engine

## Goal

Wire the existing `orchid-workflow` data model to an actual execution engine. Workflows are DAGs of agent steps with triggers.

## Current state

`crates/orchid-workflow/src/lib.rs` already defines:
```rust
pub struct Workflow { id: Uuid, name: String, trigger: Trigger, steps: Vec<Step> }
pub enum Trigger { Manual, Schedule { cron: String }, Webhook { path: String } }
pub struct Step { id: Uuid, name: String, agent_name: String, depends_on: Vec<Uuid> }
```

The `dev-to-content` flow in `main.rs` is currently hardcoded. This spec makes it data-driven.

## Architecture

### New files to create

```
crates/orchid-workflow/src/engine.rs      — DAG executor
crates/orchid-workflow/src/registry.rs    — built-in workflow definitions
crates/orchid-workflow/src/storage.rs     — CRUD for workflows in SQLite
```

### Files to modify

```
crates/orchid-workflow/src/lib.rs         — add modules, extend data model
crates/orchid-workflow/Cargo.toml         — add async-trait, tracing deps
crates/orchid-core/src/storage.rs         — add workflows table migration
crates/orchid/src/main.rs                 — replace hardcoded flow with engine
```

## Detailed design

### 1. Extended data model (`lib.rs`)

Add to `Step`:
```rust
pub struct Step {
    pub id: Uuid,
    pub name: String,
    pub agent_name: String,
    pub depends_on: Vec<Uuid>,
    pub input_mapping: InputMapping,  // NEW
}

pub enum InputMapping {
    /// Use the CLI --input / --repo args
    FromCli,
    /// Use output of a previous step
    FromStep(Uuid),
    /// Static text
    Static(String),
}
```

Add to `Workflow`:
```rust
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: String,  // NEW
    pub trigger: Trigger,
    pub steps: Vec<Step>,
}
```

### 2. WorkflowEngine (`engine.rs`)

```rust
pub struct WorkflowEngine {
    config: Config,
}

impl WorkflowEngine {
    pub fn new(config: Config) -> Self;

    /// Execute a workflow. Returns map of step_id → output string.
    pub async fn execute(
        &self,
        workflow: &Workflow,
        cli_input: Option<&str>,
        repo: Option<&str>,
    ) -> Result<HashMap<Uuid, String>>;
}
```

**Execution logic**:
1. Topological sort steps by `depends_on`
2. For each step (in order, respecting deps):
   a. Resolve input from `InputMapping`
   b. Instantiate agent by `agent_name` (match on known names)
   c. Create `AgentRunner`, run agent
   d. Store output in results map
3. Save all outputs as artifacts with `artifact_type = "workflow-step"`
4. Return results map

**Agent resolution** — the engine needs a function:
```rust
fn make_agent(
    name: &str,
    input: &str,
    repo: Option<&str>,
    llm: Box<dyn LlmClient>,
) -> Result<Box<dyn Agent>>
```
This matches on agent names: "git-summarizer", "content-drafter", "reply-monitor", "outreach-drafter", "crm-ingestion".

### 3. Built-in workflows (`registry.rs`)

```rust
pub fn builtin_workflows() -> Vec<Workflow>;
```

Returns predefined workflows:
- **dev-to-content**: git-summarizer → content-drafter (replaces hardcoded flow in main.rs)
- **repo-health**: git-summarizer → reply-monitor (summarize + check issues)

### 4. Workflow storage (`storage.rs`)

Extend `SqliteStorage` with a new table:

```sql
CREATE TABLE IF NOT EXISTS workflows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    definition TEXT NOT NULL,  -- JSON serialized Workflow
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

Methods to add on `SqliteStorage`:
```rust
pub fn save_workflow(&self, workflow: &Workflow) -> Result<()>;
pub fn load_workflow(&self, name: &str) -> Result<Option<Workflow>>;
pub fn list_workflows(&self) -> Result<Vec<Workflow>>;
pub fn delete_workflow(&self, name: &str) -> Result<()>;
```

### 5. CLI changes (`main.rs`)

Replace the hardcoded `Commands::Flow` handler with:
```rust
Commands::Flow { name, repo, input } => {
    let storage = SqliteStorage::open(&config.db_path)?;
    storage.init()?;

    // Try built-in, then stored workflows
    let workflow = builtin_workflows()
        .into_iter()
        .find(|w| w.name == name)
        .or_else(|| storage.load_workflow(&name).ok().flatten())
        .context(format!("Unknown workflow: {name}"))?;

    let engine = WorkflowEngine::new(config);
    let results = engine.execute(&workflow, input.as_deref(), repo.as_deref()).await?;

    // Print final step output
    if let Some(last) = workflow.steps.last() {
        if let Some(output) = results.get(&last.id) {
            println!("{output}");
        }
    }
}
```

Add new subcommand: `orchid flow list` — lists all available workflows (built-in + stored).

## Testing

- Unit test: topological sort correctness
- Unit test: InputMapping resolution
- Integration test: execute dev-to-content workflow with EchoLlmClient
- Test that circular dependencies are detected and rejected

## Important constraints

- Steps execute sequentially in topological order (no parallel step execution yet — keep it simple)
- The engine must NOT directly depend on specific agent types — use the `make_agent` factory pattern
- Workflow definitions must be JSON-serializable for storage
