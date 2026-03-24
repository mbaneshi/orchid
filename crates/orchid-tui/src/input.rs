use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{ActivePane, App};

/// Handle a key event, returning true if an async action (submit) was triggered.
pub fn handle_key(app: &mut App, key: KeyEvent) -> InputAction {
    // Global bindings
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.quit();
        return InputAction::None;
    }

    if key.code == KeyCode::Tab {
        app.cycle_pane();
        return InputAction::None;
    }

    match app.active {
        ActivePane::FileTree => handle_file_tree(app, key),
        ActivePane::AgentChat => handle_agent_chat(app, key),
        ActivePane::Shell => handle_shell(app, key),
    }
}

pub enum InputAction {
    None,
    SubmitChat,
    SubmitShell,
}

fn handle_file_tree(app: &mut App, key: KeyEvent) -> InputAction {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Up | KeyCode::Char('k') => app.file_tree.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.file_tree.move_down(),
        KeyCode::Enter | KeyCode::Char(' ') => app.file_tree.toggle_expand(),
        _ => {}
    }
    InputAction::None
}

fn handle_agent_chat(app: &mut App, key: KeyEvent) -> InputAction {
    match key.code {
        KeyCode::Enter => return InputAction::SubmitChat,
        KeyCode::Backspace => app.agent_chat.pop_char(),
        KeyCode::Char(c) => app.agent_chat.push_char(c),
        KeyCode::Up => app.agent_chat.scroll_up(),
        KeyCode::Down => app.agent_chat.scroll_down(),
        KeyCode::Esc => app.active = ActivePane::FileTree,
        _ => {}
    }
    InputAction::None
}

fn handle_shell(app: &mut App, key: KeyEvent) -> InputAction {
    match key.code {
        KeyCode::Enter => return InputAction::SubmitShell,
        KeyCode::Backspace => app.shell.pop_char(),
        KeyCode::Char(c) => app.shell.push_char(c),
        KeyCode::Up => app.shell.scroll_up(),
        KeyCode::Down => app.shell.scroll_down(),
        KeyCode::Esc => app.active = ActivePane::FileTree,
        _ => {}
    }
    InputAction::None
}
