pub mod agents;
pub mod artifacts;
pub mod flows;
pub mod sessions;

use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

pub fn api_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/agents", get(agents::list_agents))
        .route("/api/agents/run", post(agents::run_agent))
        .route("/api/flows", get(flows::list_flows))
        .route("/api/flows/run", post(flows::run_flow))
        .route("/api/flows/{name}", get(flows::get_flow))
        .route("/api/artifacts", get(artifacts::list_artifacts))
        .route("/api/artifacts/{id}", get(artifacts::get_artifact))
        .route(
            "/api/sessions",
            get(sessions::list_sessions).post(sessions::create_session),
        )
        .route(
            "/api/sessions/{id}",
            get(sessions::get_session).delete(sessions::delete_session),
        )
}
