use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ArtifactQuery {
    #[serde(rename = "type")]
    pub artifact_type: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct ArtifactItem {
    pub id: String,
    #[serde(rename = "type")]
    pub artifact_type: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct ListArtifactsResponse {
    pub artifacts: Vec<ArtifactItem>,
}

pub async fn list_artifacts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ArtifactQuery>,
) -> Result<Json<ListArtifactsResponse>, ApiError> {
    let artifact_type = query.artifact_type.unwrap_or_else(|| "git-summary".into());
    let limit = query.limit.unwrap_or(20);

    let storage = state
        .storage
        .lock()
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    let rows = storage.list_artifacts_by_type(&artifact_type)?;

    let artifacts: Vec<ArtifactItem> = rows
        .into_iter()
        .take(limit)
        .map(|(id, content, created_at)| ArtifactItem {
            id: id.to_string(),
            artifact_type: artifact_type.clone(),
            content,
            created_at,
        })
        .collect();

    Ok(Json(ListArtifactsResponse { artifacts }))
}

pub async fn get_artifact(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ArtifactItem>, ApiError> {
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| ApiError::BadRequest(format!("invalid uuid: {id}")))?;

    let storage = state
        .storage
        .lock()
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    match storage.get_artifact(&uuid)? {
        Some((artifact_type, content, _metadata, created_at)) => Ok(Json(ArtifactItem {
            id: uuid.to_string(),
            artifact_type,
            content,
            created_at,
        })),
        None => Err(ApiError::NotFound(format!("artifact not found: {id}"))),
    }
}
