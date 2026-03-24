use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;

use orchid_core::{Session, Storage};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    pub name: String,
}

pub async fn list_sessions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let storage = state
        .storage
        .lock()
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let sessions = storage.list_sessions()?;
    Ok(Json(serde_json::json!({ "sessions": sessions })))
}

pub async fn get_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest(format!("invalid uuid: {id}")))?;

    let storage = state
        .storage
        .lock()
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    match storage.load_session(&uuid)? {
        Some(session) => Ok(Json(serde_json::json!(session))),
        None => Err(ApiError::NotFound(format!("session not found: {id}"))),
    }
}

pub async fn create_session(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateSessionRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = Session::new(req.name);
    let storage = state
        .storage
        .lock()
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    storage.save_session(&session)?;
    Ok(Json(serde_json::json!(session)))
}

pub async fn delete_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest(format!("invalid uuid: {id}")))?;

    let storage = state
        .storage
        .lock()
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    if storage.delete_session(&uuid)? {
        Ok(Json(serde_json::json!({ "deleted": true })))
    } else {
        Err(ApiError::NotFound(format!("session not found: {id}")))
    }
}
