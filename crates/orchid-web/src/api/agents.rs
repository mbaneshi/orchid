use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RunAgentRequest {
    pub agent: String,
    pub repo: Option<String>,
    pub input: Option<String>,
}

#[derive(Serialize)]
pub struct RunAgentResponse {
    pub agent: String,
    pub output: String,
    pub artifact_id: String,
}

#[derive(Serialize)]
pub struct AgentInfo {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ListAgentsResponse {
    pub agents: Vec<AgentInfo>,
}

pub async fn list_agents() -> Json<ListAgentsResponse> {
    Json(ListAgentsResponse {
        agents: vec![
            AgentInfo {
                name: "git-summarizer".into(),
                description: "Summarize recent git activity".into(),
            },
            AgentInfo {
                name: "content-drafter".into(),
                description: "Draft social content from summaries".into(),
            },
        ],
    })
}

pub async fn run_agent(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RunAgentRequest>,
) -> Result<Json<RunAgentResponse>, ApiError> {
    let repo = req.repo.unwrap_or_else(|| ".".into());
    let llm: Box<dyn orchid_agent::LlmClient> = Box::new(orchid_agent::llm::EchoLlmClient);

    let mut agent: Box<dyn orchid_agent::Agent> = match req.agent.as_str() {
        "git-summarizer" => Box::new(orchid_agent::agents::GitSummarizerAgent::new(
            repo.clone(),
            llm,
        )),
        "content-drafter" => {
            let input = req.input.unwrap_or_default();
            Box::new(orchid_agent::agents::ContentDrafterAgent::new(input, llm))
        }
        other => return Err(ApiError::BadRequest(format!("unknown agent: {other}"))),
    };

    let runner = orchid_agent::AgentRunner::new(10);
    let output = runner
        .run(agent.as_mut())
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let artifact_id = uuid::Uuid::new_v4();
    {
        let storage = state
            .storage
            .lock()
            .map_err(|e| ApiError::Internal(e.to_string()))?;
        storage.save_artifact(&artifact_id, &req.agent, &output, None)?;
    }

    Ok(Json(RunAgentResponse {
        agent: req.agent,
        output,
        artifact_id: artifact_id.to_string(),
    }))
}
