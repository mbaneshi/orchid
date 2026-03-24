use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RunFlowRequest {
    pub workflow: String,
    pub repo: Option<String>,
    pub input: Option<String>,
}

#[derive(Serialize)]
pub struct FlowStepResult {
    pub name: String,
    pub agent: String,
    pub output: String,
}

#[derive(Serialize)]
pub struct RunFlowResponse {
    pub workflow: String,
    pub steps: Vec<FlowStepResult>,
}

#[derive(Serialize)]
pub struct FlowInfo {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ListFlowsResponse {
    pub flows: Vec<FlowInfo>,
}

pub async fn list_flows() -> Json<ListFlowsResponse> {
    Json(ListFlowsResponse {
        flows: vec![FlowInfo {
            name: "dev-to-content".into(),
            description: "Summarize git activity then draft social content".into(),
        }],
    })
}

pub async fn get_flow(Path(name): Path<String>) -> Result<Json<serde_json::Value>, ApiError> {
    match name.as_str() {
        "dev-to-content" => Ok(Json(serde_json::json!({
            "name": "dev-to-content",
            "trigger": { "type": "Manual" },
            "steps": [
                { "name": "summarize", "agent_name": "git-summarizer" },
                { "name": "draft", "agent_name": "content-drafter" }
            ]
        }))),
        other => Err(ApiError::NotFound(format!("workflow not found: {other}"))),
    }
}

pub async fn run_flow(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RunFlowRequest>,
) -> Result<Json<RunFlowResponse>, ApiError> {
    let repo = req.repo.unwrap_or_else(|| ".".into());

    match req.workflow.as_str() {
        "dev-to-content" => {
            let llm: Box<dyn orchid_agent::LlmClient> =
                Box::new(orchid_agent::llm::EchoLlmClient);
            let mut summarizer =
                orchid_agent::agents::GitSummarizerAgent::new(repo.clone(), llm);
            let runner = orchid_agent::AgentRunner::new(10);
            let summary = runner
                .run(&mut summarizer)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

            let llm2: Box<dyn orchid_agent::LlmClient> =
                Box::new(orchid_agent::llm::EchoLlmClient);
            let mut drafter =
                orchid_agent::agents::ContentDrafterAgent::new(summary.clone(), llm2);
            let draft = runner
                .run(&mut drafter)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

            Ok(Json(RunFlowResponse {
                workflow: req.workflow,
                steps: vec![
                    FlowStepResult {
                        name: "summarize".into(),
                        agent: "git-summarizer".into(),
                        output: summary,
                    },
                    FlowStepResult {
                        name: "draft".into(),
                        agent: "content-drafter".into(),
                        output: draft,
                    },
                ],
            }))
        }
        other => Err(ApiError::BadRequest(format!("unknown workflow: {other}"))),
    }
}
