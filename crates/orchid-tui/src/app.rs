use std::path::PathBuf;

use crate::panes::{AgentChatPane, FileTreePane, ShellPane};
use orchid_agent::llm::LlmClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    FileTree,
    AgentChat,
    Shell,
}

impl ActivePane {
    pub fn next(self) -> Self {
        match self {
            Self::FileTree => Self::AgentChat,
            Self::AgentChat => Self::Shell,
            Self::Shell => Self::FileTree,
        }
    }
}

pub struct App {
    pub active: ActivePane,
    pub file_tree: FileTreePane,
    pub agent_chat: AgentChatPane,
    pub shell: ShellPane,
    pub should_quit: bool,
}

impl App {
    pub fn new(cwd: PathBuf, llm: Box<dyn LlmClient>) -> Self {
        Self {
            active: ActivePane::FileTree,
            file_tree: FileTreePane::new(cwd.clone()),
            agent_chat: AgentChatPane::new(llm),
            shell: ShellPane::new(cwd),
            should_quit: false,
        }
    }

    pub fn cycle_pane(&mut self) {
        self.active = self.active.next();
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
