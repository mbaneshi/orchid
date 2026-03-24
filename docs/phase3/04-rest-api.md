# Spec 04 — REST API

## Goal

Expose agents, flows, and storage as REST endpoints in `orchid-web`. The web server becomes the backend for both the SvelteKit frontend and external integrations.

## Current state

`crates/orchid-web/src/lib.rs` has an Axum server with two routes:
- `GET /` → HTML landing page
- `GET /health` → "orchid ok"

## Architecture

### New files to create

```
crates/orchid-web/src/api/mod.rs         — API router
crates/orchid-web/src/api/agents.rs      — Agent endpoints
crates/orchid-web/src/api/flows.rs       — Flow/workflow endpoints
crates/orchid-web/src/api/artifacts.rs   — Artifact CRUD
crates/orchid-web/src/api/sessions.rs    — Session CRUD
crates/orchid-web/src/state.rs           — Shared app state
crates/orchid-web/src/error.rs           — API error types
```

### Files to modify

```
crates/orchid-web/src/lib.rs             — mount API router, add shared state
crates/orchid-web/Cargo.toml             — add orchid-agent, orchid-workflow, serde_json, tower-http deps
```

## Detailed design

### 1. Shared state (`state.rs`)

```rust
use orchid_core::{Config, SqliteStorage};
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub config: Config,
    pub storage: Arc<Mutex<SqliteStorage>>,
}
```

Axum state: `Arc<AppState>` via `axum::extract::State`.

Note: `SqliteStorage` wraps `rusqlite::Connection` which is `!Sync`. Use `Mutex` for thread safety. For v1 this is fine — single connection behind a lock.

### 2. API error type (`error.rs`)

```rust
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, axum::Json(serde_json::json!({ "error": message }))).into_response()
    }
}
```

### 3. Agent endpoints (`api/agents.rs`)

```
POST /api/agents/run
```

**Request body**:
```json
{
    "agent": "git-summarizer",
    "repo": "/path/to/repo",
    "input": null
}
```

**Response** (200):
```json
{
    "agent": "git-summarizer",
    "output": "The recent commits show...",
    "artifact_id": "uuid-here"
}
```

**Implementation**:
- Validate agent name (known set)
- Create LLM client via `make_llm` (extract this to a shared util in orchid-agent)
- Instantiate agent, run with AgentRunner
- Save artifact to SQLite
- Return output + artifact ID

```
GET /api/agents
```

**Response**: List of available agent names with descriptions.
```json
{
    "agents": [
        { "name": "git-summarizer", "description": "Summarize recent git activity" },
        { "name": "content-drafter", "description": "Draft social content from summaries" },
        ...
    ]
}
```

### 4. Flow endpoints (`api/flows.rs`)

```
POST /api/flows/run
```

**Request body**:
```json
{
    "workflow": "dev-to-content",
    "repo": "/path/to/repo",
    "input": null
}
```

**Response** (200):
```json
{
    "workflow": "dev-to-content",
    "steps": [
        { "name": "summarize", "agent": "git-summarizer", "output": "..." },
        { "name": "draft", "agent": "content-drafter", "output": "..." }
    ]
}
```

```
GET /api/flows
```

**Response**: List of available workflows (built-in + stored).

```
GET /api/flows/:name
```

**Response**: Workflow definition (steps, triggers, etc.).

### 5. Artifact endpoints (`api/artifacts.rs`)

```
GET /api/artifacts?type=git-summary&limit=20
```

**Response**:
```json
{
    "artifacts": [
        { "id": "uuid", "type": "git-summary", "content": "...", "created_at": "..." }
    ]
}
```

```
GET /api/artifacts/:id
```

**Response**: Single artifact by ID.

### 6. Session endpoints (`api/sessions.rs`)

```
GET    /api/sessions           — list all sessions
GET    /api/sessions/:id       — get session by ID
POST   /api/sessions           — create session
DELETE /api/sessions/:id       — delete session
```

### 7. Router assembly (`api/mod.rs`)

```rust
pub fn api_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/agents", get(agents::list_agents))
        .route("/api/agents/run", post(agents::run_agent))
        .route("/api/flows", get(flows::list_flows))
        .route("/api/flows/run", post(flows::run_flow))
        .route("/api/flows/{name}", get(flows::get_flow))
        .route("/api/artifacts", get(artifacts::list_artifacts))
        .route("/api/artifacts/{id}", get(artifacts::get_artifact))
        .route("/api/sessions", get(sessions::list_sessions).post(sessions::create_session))
        .route("/api/sessions/{id}", get(sessions::get_session).delete(sessions::delete_session))
}
```

### 8. Mount in `lib.rs`

```rust
pub async fn serve() -> Result<()> {
    let config = Config::load()?;
    let storage = SqliteStorage::open(&config.db_path)?;
    storage.init()?;

    let state = Arc::new(AppState {
        config: config.clone(),
        storage: Arc::new(Mutex::new(storage)),
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .merge(api::api_router())
        .with_state(state);

    // ... bind and serve
}
```

## Dependencies to add

In `crates/orchid-web/Cargo.toml`:
```toml
orchid-agent.workspace = true
orchid-workflow.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
tower-http = { version = "0.6", features = ["cors"] }
```

Add CORS middleware for SvelteKit dev server:
```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

let app = Router::new()
    // ... routes
    .layer(cors);
```

## Testing

- `cargo test -p orchid-web` — test API handlers with test state
- Integration: `curl localhost:3100/api/agents` should return agent list
- Integration: `curl -X POST localhost:3100/api/agents/run -d '{"agent":"git-summarizer","repo":"."}'`

## Constraints

- Agent runs are synchronous (blocking the request). For v1 this is fine — agents take 5-30 seconds.
- Future: add WebSocket endpoint for streaming agent output
- All responses are JSON except `/` (HTML) and `/health` (plain text)
- CORS must be permissive for local SvelteKit dev server
