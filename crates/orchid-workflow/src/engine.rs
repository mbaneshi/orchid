use std::collections::HashMap;

use anyhow::{Context, Result};
use tracing::info;
use uuid::Uuid;

use orchid_agent::agents::{ContentDrafterAgent, GitSummarizerAgent};
use orchid_agent::{resolve_api_key, Agent, AgentRunner, AnthropicClient, ClaudeCliClient, LlmClient};
use orchid_core::{Config, SqliteStorage};

use crate::{InputMapping, Step, Workflow};

pub struct WorkflowEngine {
    config: Config,
}

impl WorkflowEngine {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Execute a workflow. Returns map of step_id -> output string.
    pub async fn execute(
        &self,
        workflow: &Workflow,
        cli_input: Option<&str>,
        repo: Option<&str>,
    ) -> Result<HashMap<Uuid, String>> {
        let sorted = topological_sort(&workflow.steps)?;
        let storage = SqliteStorage::open(&self.config.db_path)?;
        let mut results: HashMap<Uuid, String> = HashMap::new();

        for step in &sorted {
            info!(step = %step.name, agent = %step.agent_name, "executing workflow step");
            println!("--- Step: {} ({}) ---\n", step.name, step.agent_name);

            let input = resolve_input(&step.input_mapping, cli_input, repo, &results)?;

            let llm = make_llm(&self.config)?;
            let mut agent = make_agent(&step.agent_name, &input, repo, llm)?;

            let runner = AgentRunner::new(5);
            let output = runner.run(agent.as_mut()).await?;

            storage.save_artifact(
                &Uuid::new_v4(),
                "workflow-step",
                &output,
                Some(&format!(
                    "{{\"workflow\":\"{}\",\"step\":\"{}\"}}",
                    workflow.name, step.name
                )),
            )?;

            results.insert(step.id, output);
        }

        Ok(results)
    }
}

fn make_llm(config: &Config) -> Result<Box<dyn LlmClient>> {
    match resolve_api_key(&config.llm) {
        Ok(api_key) => {
            println!("using Anthropic API key");
            Ok(Box::new(AnthropicClient::new(api_key, config.llm.model.clone())))
        }
        Err(_) => {
            println!("using Claude CLI (Max subscription)");
            Ok(Box::new(ClaudeCliClient::max_subscription(
                config.llm.model.clone(),
            )))
        }
    }
}

fn make_agent(
    name: &str,
    input: &str,
    repo: Option<&str>,
    llm: Box<dyn LlmClient>,
) -> Result<Box<dyn Agent>> {
    match name {
        "git-summarizer" => {
            let repo = repo
                .context("git-summarizer requires a --repo argument")?
                .to_string();
            Ok(Box::new(GitSummarizerAgent::new(repo, llm)))
        }
        "content-drafter" => Ok(Box::new(ContentDrafterAgent::new(input.to_string(), llm))),
        other => anyhow::bail!("unknown agent: {other}"),
    }
}

fn resolve_input(
    mapping: &InputMapping,
    cli_input: Option<&str>,
    repo: Option<&str>,
    results: &HashMap<Uuid, String>,
) -> Result<String> {
    match mapping {
        InputMapping::FromCli => {
            if let Some(input) = cli_input {
                Ok(input.to_string())
            } else if let Some(r) = repo {
                Ok(r.to_string())
            } else {
                Ok(String::new())
            }
        }
        InputMapping::FromStep(id) => results
            .get(id)
            .cloned()
            .context(format!("step output not found for {id}")),
        InputMapping::Static(s) => Ok(s.clone()),
    }
}

/// Topological sort via Kahn's algorithm. Returns error on cycles.
pub fn topological_sort(steps: &[Step]) -> Result<Vec<Step>> {
    let step_map: HashMap<Uuid, &Step> = steps.iter().map(|s| (s.id, s)).collect();

    // Build in-degree map
    let mut in_degree: HashMap<Uuid, usize> = steps.iter().map(|s| (s.id, 0)).collect();
    for step in steps {
        for dep in &step.depends_on {
            if !step_map.contains_key(dep) {
                anyhow::bail!("step '{}' depends on unknown step {}", step.name, dep);
            }
            *in_degree.entry(step.id).or_insert(0) += 1;
        }
    }

    // Find initial nodes with no dependencies
    let mut queue: Vec<Uuid> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();

    let mut sorted = Vec::new();

    while let Some(id) = queue.pop() {
        sorted.push(step_map[&id].clone());

        // For each step that depends on this one, decrement in-degree
        for step in steps {
            if step.depends_on.contains(&id) {
                let deg = in_degree.get_mut(&step.id).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push(step.id);
                }
            }
        }
    }

    if sorted.len() != steps.len() {
        anyhow::bail!("workflow contains circular dependencies");
    }

    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InputMapping, Step};

    fn step(id: Uuid, name: &str, deps: Vec<Uuid>) -> Step {
        Step {
            id,
            name: name.to_string(),
            agent_name: "test".to_string(),
            depends_on: deps,
            input_mapping: InputMapping::FromCli,
        }
    }

    #[test]
    fn topological_sort_linear() {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let c = Uuid::new_v4();

        let steps = vec![step(c, "c", vec![b]), step(a, "a", vec![]), step(b, "b", vec![a])];

        let sorted = topological_sort(&steps).unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].id, a);
        assert_eq!(sorted[1].id, b);
        assert_eq!(sorted[2].id, c);
    }

    #[test]
    fn topological_sort_parallel_roots() {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let c = Uuid::new_v4();

        let steps = vec![
            step(a, "a", vec![]),
            step(b, "b", vec![]),
            step(c, "c", vec![a, b]),
        ];

        let sorted = topological_sort(&steps).unwrap();
        assert_eq!(sorted.len(), 3);
        // c must be last
        assert_eq!(sorted[2].id, c);
    }

    #[test]
    fn topological_sort_cycle_detected() {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();

        let steps = vec![step(a, "a", vec![b]), step(b, "b", vec![a])];

        let result = topological_sort(&steps);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("circular"));
    }

    #[test]
    fn resolve_input_from_cli() {
        let results = HashMap::new();
        let input = resolve_input(&InputMapping::FromCli, Some("hello"), None, &results).unwrap();
        assert_eq!(input, "hello");
    }

    #[test]
    fn resolve_input_from_step() {
        let id = Uuid::new_v4();
        let mut results = HashMap::new();
        results.insert(id, "step output".to_string());

        let input = resolve_input(&InputMapping::FromStep(id), None, None, &results).unwrap();
        assert_eq!(input, "step output");
    }

    #[test]
    fn resolve_input_static() {
        let results = HashMap::new();
        let input =
            resolve_input(&InputMapping::Static("fixed".to_string()), None, None, &results)
                .unwrap();
        assert_eq!(input, "fixed");
    }

    #[test]
    fn resolve_input_from_step_missing() {
        let results = HashMap::new();
        let result =
            resolve_input(&InputMapping::FromStep(Uuid::new_v4()), None, None, &results);
        assert!(result.is_err());
    }
}
