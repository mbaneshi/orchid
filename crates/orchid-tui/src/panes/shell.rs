use tokio::process::Command;

pub struct ShellPane {
    pub output_lines: Vec<String>,
    pub input: String,
    pub scroll: u16,
    cwd: std::path::PathBuf,
}

impl ShellPane {
    pub fn new(cwd: std::path::PathBuf) -> Self {
        Self {
            output_lines: vec!["$ Welcome to Orchid Shell".to_string()],
            input: String::new(),
            scroll: 0,
            cwd,
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop_char(&mut self) {
        self.input.pop();
    }

    pub async fn submit(&mut self) -> anyhow::Result<()> {
        let cmd_str = self.input.trim().to_string();
        if cmd_str.is_empty() {
            return Ok(());
        }
        self.input.clear();
        self.output_lines
            .push(format!("$ {cmd_str}"));

        // Handle cd specially
        if let Some(dir) = cmd_str.strip_prefix("cd ") {
            let dir = dir.trim();
            let new_cwd = if dir.starts_with('/') {
                std::path::PathBuf::from(dir)
            } else {
                self.cwd.join(dir)
            };
            if new_cwd.is_dir() {
                self.cwd = new_cwd.canonicalize().unwrap_or(new_cwd);
                self.output_lines
                    .push(format!("(cwd: {})", self.cwd.display()));
            } else {
                self.output_lines
                    .push(format!("cd: no such directory: {dir}"));
            }
            return Ok(());
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd_str)
            .current_dir(&self.cwd)
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        for line in stdout.lines() {
            self.output_lines.push(line.to_string());
        }
        for line in stderr.lines() {
            self.output_lines.push(format!("[err] {line}"));
        }

        Ok(())
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }
}
