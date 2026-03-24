use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{ActivePane, App};

pub fn draw(frame: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.area());

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(outer[1]);

    draw_file_tree(frame, app, outer[0]);
    draw_agent_chat(frame, app, right[0]);
    draw_shell(frame, app, right[1]);
}

fn pane_border_style(active: ActivePane, this: ActivePane) -> Style {
    if active == this {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn draw_file_tree(frame: &mut Frame, app: &App, area: Rect) {
    let style = pane_border_style(app.active, ActivePane::FileTree);
    let block = Block::default()
        .title(" Files ")
        .borders(Borders::ALL)
        .border_style(style);

    let items: Vec<ListItem> = app
        .file_tree
        .entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let indent = "  ".repeat(entry.depth);
            let icon = if entry.is_dir {
                if entry.expanded {
                    "v "
                } else {
                    "> "
                }
            } else {
                "  "
            };
            let text = format!("{indent}{icon}{}", entry.name);
            let style = if i == app.file_tree.selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if entry.is_dir {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(Span::styled(text, style)))
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_agent_chat(frame: &mut Frame, app: &App, area: Rect) {
    let style = pane_border_style(app.active, ActivePane::AgentChat);
    let block = Block::default()
        .title(" Agent Chat ")
        .borders(Borders::ALL)
        .border_style(style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split inner into messages area and input area
    let chat_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(inner);

    // Messages
    let mut lines: Vec<Line> = Vec::new();
    for msg in &app.agent_chat.messages {
        let (prefix, color) = match msg.role.as_str() {
            "user" => ("You: ", Color::Green),
            "assistant" => ("AI: ", Color::Cyan),
            _ => ("", Color::White),
        };
        lines.push(Line::from(Span::styled(
            format!("{prefix}{}", msg.content),
            Style::default().fg(color),
        )));
        lines.push(Line::from(""));
    }
    let messages = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((app.agent_chat.scroll, 0));
    frame.render_widget(messages, chat_layout[0]);

    // Input
    let input_block = Block::default()
        .title(" > ")
        .borders(Borders::TOP);
    let input = Paragraph::new(app.agent_chat.input.as_str()).block(input_block);
    frame.render_widget(input, chat_layout[1]);
}

fn draw_shell(frame: &mut Frame, app: &App, area: Rect) {
    let style = pane_border_style(app.active, ActivePane::Shell);
    let block = Block::default()
        .title(" Shell ")
        .borders(Borders::ALL)
        .border_style(style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let shell_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(inner);

    // Output
    let lines: Vec<Line> = app
        .shell
        .output_lines
        .iter()
        .map(|l| Line::from(l.as_str()))
        .collect();

    let total = lines.len() as u16;
    let visible = shell_layout[0].height;
    let auto_scroll = if total > visible {
        total - visible
    } else {
        0
    };
    let scroll = if app.shell.scroll > 0 {
        app.shell.scroll
    } else {
        auto_scroll
    };

    let output = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(output, shell_layout[0]);

    // Input
    let input_block = Block::default()
        .title(" $ ")
        .borders(Borders::TOP);
    let input = Paragraph::new(app.shell.input.as_str()).block(input_block);
    frame.render_widget(input, shell_layout[1]);
}
