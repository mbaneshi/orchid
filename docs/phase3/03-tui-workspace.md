# Spec 03 — TUI Workspace

## Goal

Implement `orchid workspace` as a ratatui-based TUI with 3 panes: file tree, agent chat, and shell output.

## Current state

`crates/orchid-tui/src/lib.rs` has a single `pub async fn run()` that returns an error "TUI not yet implemented". Dependencies `ratatui` 0.28 and `crossterm` 0.28 are already in Cargo.toml.

## Architecture

### New files to create

```
crates/orchid-tui/src/app.rs       — App state and event loop
crates/orchid-tui/src/ui.rs        — Layout and rendering
crates/orchid-tui/src/panes/mod.rs
crates/orchid-tui/src/panes/file_tree.rs
crates/orchid-tui/src/panes/agent_chat.rs
crates/orchid-tui/src/panes/shell.rs
crates/orchid-tui/src/input.rs     — Key event handling
```

### Files to modify

```
crates/orchid-tui/src/lib.rs       — replace placeholder with real entry point
crates/orchid-tui/Cargo.toml       — add orchid-agent dep for chat pane
```

## Detailed design

### 1. App state (`app.rs`)

```rust
pub struct App {
    pub active_pane: Pane,
    pub file_tree: FileTreeState,
    pub agent_chat: AgentChatState,
    pub shell: ShellState,
    pub should_quit: bool,
    pub working_dir: PathBuf,
}

pub enum Pane {
    FileTree,
    AgentChat,
    Shell,
}

impl App {
    pub fn new(working_dir: PathBuf) -> Self;
    pub fn next_pane(&mut self);     // Tab cycles panes
    pub fn prev_pane(&mut self);     // Shift-Tab
}
```

### 2. Layout (`ui.rs`)

```
┌──────────────┬──────────────────────────────┐
│              │                              │
│  File Tree   │       Agent Chat             │
│  (20%)       │       (80%)                  │
│              │                              │
│              │                              │
│              ├──────────────────────────────┤
│              │                              │
│              │       Shell Output           │
│              │       (30% of right)         │
│              │                              │
└──────────────┴──────────────────────────────┘
```

Use ratatui `Layout::default().direction(Horizontal)` for left/right split,
then `Layout::default().direction(Vertical)` for right-side top/bottom split.

Render function:
```rust
pub fn draw(frame: &mut Frame, app: &App);
```

Each pane gets a `Block` with a title. Active pane has highlighted border (Color::Cyan).

### 3. File tree pane (`panes/file_tree.rs`)

```rust
pub struct FileTreeState {
    pub entries: Vec<FileEntry>,
    pub selected: usize,
    pub scroll_offset: usize,
}

pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,  // for directories
}
```

- On init: scan `working_dir` (1 level deep, skip hidden files and `target/`)
- Up/Down: move selection
- Enter on dir: toggle expand (lazy-load children)
- Enter on file: show path in shell pane as info
- Render: use `List` widget with indentation based on depth, directories get `▸`/`▾` prefix

### 4. Agent chat pane (`panes/agent_chat.rs`)

```rust
pub struct AgentChatState {
    pub messages: Vec<ChatMessage>,
    pub input_buffer: String,
    pub scroll_offset: usize,
}

pub struct ChatMessage {
    pub role: String,   // "user" or "assistant"
    pub content: String,
    pub timestamp: String,
}
```

- Shows scrollable message history
- Bottom input line with `> ` prompt
- Type message + Enter: adds user message, shows "[running...]", sends to agent
- Agent integration: for v1, spawn a simple Q&A agent that uses the LLM client directly
  - Create `TuiChatAgent` inline — single-step, sends user message to LLM, returns response
  - Use `tokio::spawn` so UI doesn't block
- Render: messages in `Paragraph` widget with line wrapping, input in separate `Paragraph`

### 5. Shell pane (`panes/shell.rs`)

```rust
pub struct ShellState {
    pub lines: Vec<String>,
    pub input_buffer: String,
    pub scroll_offset: usize,
}
```

- Shows command output log
- Bottom input with `$ ` prompt
- Type command + Enter: spawn `tokio::process::Command` with shell, capture stdout/stderr, append to lines
- Scroll: Up/Down when this pane is active
- Render: `Paragraph` widget with scroll

### 6. Event loop (`app.rs` or `lib.rs`)

```rust
pub async fn run() -> Result<()> {
    let mut terminal = ratatui::init();
    crossterm::terminal::enable_raw_mode()?;

    let working_dir = std::env::current_dir()?;
    let mut app = App::new(working_dir);

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = crossterm::event::read()? {
                input::handle_key(&mut app, key).await?;
            }
        }

        if app.should_quit { break; }
    }

    ratatui::restore();
    Ok(())
}
```

### 7. Key bindings (`input.rs`)

| Key | Context | Action |
|-----|---------|--------|
| `q` / `Ctrl-c` | Global (not in input) | Quit |
| `Tab` | Global | Next pane |
| `Shift-Tab` | Global | Previous pane |
| `Up/Down` | File tree | Navigate |
| `Enter` | File tree | Expand/collapse |
| `Up/Down` | Chat/Shell (not typing) | Scroll |
| `Enter` | Chat/Shell (typing) | Submit |
| `Esc` | Any input | Cancel input / deselect |

When a pane is active and has an input field, typed characters go to the input buffer.

## Dependencies to add

In `crates/orchid-tui/Cargo.toml`:
```toml
orchid-agent.workspace = true
tracing.workspace = true
```

## Testing

- `cargo test -p orchid-tui` — test App state transitions (pane cycling, file tree navigation)
- Manual testing: `cargo run -- workspace`
- No need to test rendering (ratatui handles that)

## Constraints

- Keep the TUI simple — no async agent streaming for v1
- File tree should not recurse more than 3 levels deep
- Shell commands run with a 30-second timeout
- Chat agent runs in background task, UI shows "[thinking...]" spinner dots
