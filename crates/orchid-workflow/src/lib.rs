pub mod engine;
pub mod registry;
pub mod storage;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A workflow is a named DAG of steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
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

/// How a step gets its input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputMapping {
    /// Use the CLI --input / --repo args
    FromCli,
    /// Use output of a previous step
    FromStep(Uuid),
    /// Static text
    Static(String),
}

/// A single step in a workflow, dispatching to an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub id: Uuid,
    pub name: String,
    pub agent_name: String,
    pub depends_on: Vec<Uuid>,
    pub input_mapping: InputMapping,
}
