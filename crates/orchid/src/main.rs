use anyhow::Result;
use clap::{Parser, Subcommand};

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
        /// Name of the agent to run
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Print version information
    Version,
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
        Commands::Agent { name } => {
            let agent_name = name.unwrap_or_else(|| "git-summarizer".to_string());
            println!("running agent: {agent_name}");
        }
        Commands::Version => {
            println!("orchid {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
