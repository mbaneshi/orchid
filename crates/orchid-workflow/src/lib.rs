use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A workflow is a named DAG of steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub trigger: Trigger,
    pub steps: Vec<Step>,
}

/// What kicks off a workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Trigger {
    Manual,
    Schedule { cron: String },
    Webhook { path: String },
}

/// A single step in a workflow, dispatching to an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub id: Uuid,
    pub name: String,
    pub agent_name: String,
    pub depends_on: Vec<Uuid>,
}
