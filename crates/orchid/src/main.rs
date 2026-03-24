use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use uuid::Uuid;

use orchid_agent::agents::{ContentDrafterAgent, GitSummarizerAgent};
use orchid_agent::{resolve_api_key, AgentRunner, AnthropicClient, ClaudeCliClient, LlmClient};
use orchid_core::{Config, SqliteStorage};
use orchid_workflow::engine::WorkflowEngine;
use orchid_workflow::registry::builtin_workflows;
use orchid_workflow::storage::WorkflowStorage;

#[derive(Parser)]
#[command(
    name = "orchid",
    about = "Orchid — Agentic Dev OS",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the TUI workspace
    Workspace,
    /// Start the web UI server
    Web,
    /// Run an agent
    Agent {
        /// Agent name: git-summarizer, content-drafter
        #[arg(short, long)]
        name: String,
        /// Repository path (for git-summarizer)
        #[arg(short, long)]
        repo: Option<String>,
        /// Input text (for content-drafter)
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Run an end-to-end flow
    Flow {
        #[command(subcommand)]
        action: FlowAction,
    },
    /// Print version information
    Version,
}

#[derive(Subcommand)]
enum FlowAction {
    /// Run a workflow by name
    Run {
        /// Workflow name
        #[arg(short, long)]
        name: String,
        /// Repository path
        #[arg(short, long)]
        repo: Option<String>,
        /// Input text
        #[arg(short, long)]
        input: Option<String>,
    },
    /// List all available workflows
    List,
}

/// Create an LLM client. Tries API key first, falls back to Claude CLI (Max subscription).
fn make_llm(config: &Config) -> Result<Box<dyn LlmClient>> {
    match resolve_api_key(&config.llm) {
        Ok(api_key) => {
            println!("using Anthropic API key");
            Ok(Box::new(AnthropicClient::new(api_key, config.llm.model.clone())))
        }
        Err(_) => {
            println!("using Claude CLI (Max subscription)");
            Ok(Box::new(ClaudeCliClient::max_subscription(config.llm.model.clone())))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Workspace => {
            println!("launching TUI workspace (not yet implemented)");
        }
        Commands::Web => {
            println!("starting web server...");
            orchid_web::serve().await?;
        }
        Commands::Agent { name, repo, input } => {
            let config = Config::load()?;
            let storage = SqliteStorage::open(&config.db_path)?;
            let runner = AgentRunner::new(5);

            match name.as_str() {
                "git-summarizer" => {
                    let repo = repo
                        .ok_or_else(|| anyhow::anyhow!("--repo is required for git-summarizer"))?;
                    let llm = make_llm(&config)?;
                    let mut agent = GitSummarizerAgent::new(repo, llm);
                    let output = runner.run(&mut agent).await?;
                    storage.save_artifact(&Uuid::new_v4(), "git_summary", &output, None)?;
                    println!("{output}");
                }
                "content-drafter" => {
                    let input = input.ok_or_else(|| {
                        anyhow::anyhow!("--input is required for content-drafter")
                    })?;
                    let llm = make_llm(&config)?;
                    let mut agent = ContentDrafterAgent::new(input, llm);
                    let output = runner.run(&mut agent).await?;
                    storage.save_artifact(&Uuid::new_v4(), "content_draft", &output, None)?;
                    println!("{output}");
                }
                other => anyhow::bail!(
                    "unknown agent: {other}. Available: git-summarizer, content-drafter"
                ),
            }
        }
        Commands::Flow { action } => match action {
            FlowAction::Run { name, repo, input } => {
                let config = Config::load()?;
                let storage = SqliteStorage::open(&config.db_path)?;

                let workflow = builtin_workflows()
                    .into_iter()
                    .find(|w| w.name == name)
                    .or_else(|| storage.load_workflow(&name).ok().flatten())
                    .context(format!("Unknown workflow: {name}"))?;

                let engine = WorkflowEngine::new(config);
                let results = engine
                    .execute(&workflow, input.as_deref(), repo.as_deref())
                    .await?;

                if let Some(last) = workflow.steps.last() {
                    if let Some(output) = results.get(&last.id) {
                        println!("{output}");
                    }
                }
            }
            FlowAction::List => {
                let mut workflows = builtin_workflows();

                let config = Config::load()?;
                if let Ok(storage) = SqliteStorage::open(&config.db_path) {
                    if let Ok(stored) = storage.list_workflows() {
                        for w in stored {
                            if !workflows.iter().any(|bw| bw.name == w.name) {
                                workflows.push(w);
                            }
                        }
                    }
                }

                println!("Available workflows:\n");
                for w in &workflows {
                    let step_names: Vec<&str> = w.steps.iter().map(|s| s.name.as_str()).collect();
                    println!(
                        "  {} — {} (steps: {})",
                        w.name,
                        w.description,
                        step_names.join(" → ")
                    );
                }
            }
        },
        Commands::Version => {
            println!("orchid {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
