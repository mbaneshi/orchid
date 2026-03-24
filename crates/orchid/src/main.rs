use anyhow::Result;
use clap::{Parser, Subcommand};
use uuid::Uuid;

use orchid_agent::agents::{
    ContentDrafterAgent, CrmIngestionAgent, GitSummarizerAgent, OutreachDrafterAgent,
    ReplyMonitorAgent,
};
use orchid_agent::{resolve_api_key, AgentRunner, AnthropicClient, ClaudeCliClient, LlmClient};
use orchid_core::{Config, SqliteStorage};

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
        /// Agent name: git-summarizer, content-drafter, reply-monitor, outreach-drafter, crm-ingestion
        #[arg(short, long)]
        name: String,
        /// Repository path (for git-summarizer) or GitHub repo (for reply-monitor, e.g. owner/name)
        #[arg(short, long)]
        repo: Option<String>,
        /// Input text (for content-drafter, outreach-drafter, crm-ingestion)
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Run an end-to-end flow
    Flow {
        /// Flow name: dev-to-content
        #[arg(short, long)]
        name: String,
        /// Repository path
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Print version information
    Version,
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
                "reply-monitor" => {
                    let repo = repo.ok_or_else(|| {
                        anyhow::anyhow!("--repo is required for reply-monitor (e.g. owner/name)")
                    })?;
                    let llm = make_llm(&config)?;
                    let mut agent = ReplyMonitorAgent::new(repo, llm);
                    let output = runner.run(&mut agent).await?;
                    storage.save_artifact(&Uuid::new_v4(), "reply_report", &output, None)?;
                    println!("{output}");
                }
                "outreach-drafter" => {
                    let input = input.ok_or_else(|| {
                        anyhow::anyhow!("--input is required for outreach-drafter")
                    })?;
                    let llm = make_llm(&config)?;
                    let mut agent = OutreachDrafterAgent::new(input, llm);
                    let output = runner.run(&mut agent).await?;
                    storage.save_artifact(&Uuid::new_v4(), "outreach_draft", &output, None)?;
                    println!("{output}");
                }
                "crm-ingestion" => {
                    let input = input.ok_or_else(|| {
                        anyhow::anyhow!("--input is required for crm-ingestion")
                    })?;
                    let llm = make_llm(&config)?;
                    let mut agent = CrmIngestionAgent::new(input, llm);
                    let output = runner.run(&mut agent).await?;
                    storage.save_artifact(&Uuid::new_v4(), "crm_records", &output, None)?;
                    println!("{output}");
                }
                other => anyhow::bail!(
                    "unknown agent: {other}. Available: git-summarizer, content-drafter, reply-monitor, outreach-drafter, crm-ingestion"
                ),
            }
        }
        Commands::Flow { name, repo } => {
            match name.as_str() {
                "dev-to-content" => {
                    let repo = repo.ok_or_else(|| {
                        anyhow::anyhow!("--repo is required for dev-to-content flow")
                    })?;
                    let config = Config::load()?;
                    let storage = SqliteStorage::open(&config.db_path)?;

                    // Step 1: Summarize git history
                    println!("--- Step 1: Summarizing git history ---\n");
                    let llm1 = make_llm(&config)?;
                    let mut summarizer = GitSummarizerAgent::new(repo, llm1);
                    let runner = AgentRunner::new(5);
                    let summary = runner.run(&mut summarizer).await?;
                    storage.save_artifact(&Uuid::new_v4(), "git_summary", &summary, None)?;
                    println!("{summary}\n");

                    // Step 2: Draft content from summary
                    println!("--- Step 2: Drafting content ---\n");
                    let llm2 = make_llm(&config)?;
                    let mut drafter = ContentDrafterAgent::new(summary, llm2);
                    let runner = AgentRunner::new(3);
                    let draft = runner.run(&mut drafter).await?;
                    storage.save_artifact(&Uuid::new_v4(), "content_draft", &draft, None)?;
                    println!("{draft}");
                }
                other => anyhow::bail!("unknown flow: {other}. Available: dev-to-content"),
            }
        }
        Commands::Version => {
            println!("orchid {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
