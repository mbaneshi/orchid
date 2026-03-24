pub mod app;
pub mod input;
pub mod panes;
pub mod ui;

use std::path::PathBuf;

use anyhow::Result;
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use app::App;
use input::{handle_key, InputAction};
use orchid_agent::llm::LlmClient;

pub async fn run_with_llm(cwd: PathBuf, llm: Box<dyn LlmClient>) -> Result<()> {
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(cwd, llm);

    let result = event_loop(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    result
}

async fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match handle_key(app, key) {
                    InputAction::SubmitChat => {
                        if let Err(e) = app.agent_chat.submit().await {
                            app.agent_chat.messages.push(panes::agent_chat::ChatMessage {
                                role: "system".to_string(),
                                content: format!("Error: {e}"),
                            });
                        }
                    }
                    InputAction::SubmitShell => {
                        if let Err(e) = app.shell.submit().await {
                            app.shell
                                .output_lines
                                .push(format!("[error] {e}"));
                        }
                    }
                    InputAction::None => {}
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

pub async fn run() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let llm: Box<dyn LlmClient> = Box::new(orchid_agent::llm::EchoLlmClient);
    run_with_llm(cwd, llm).await
}
