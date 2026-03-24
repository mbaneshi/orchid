use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,
}

pub struct FileTreePane {
    pub entries: Vec<FileEntry>,
    pub selected: usize,
    root: PathBuf,
}

impl FileTreePane {
    pub fn new(root: PathBuf) -> Self {
        let mut pane = Self {
            entries: Vec::new(),
            selected: 0,
            root,
        };
        pane.scan();
        pane
    }

    pub fn scan(&mut self) {
        self.entries.clear();
        self.scan_dir(&self.root.clone(), 0);
        if self.selected >= self.entries.len() && !self.entries.is_empty() {
            self.selected = self.entries.len() - 1;
        }
    }

    fn scan_dir(&mut self, dir: &Path, depth: usize) {
        let Ok(read_dir) = std::fs::read_dir(dir) else {
            return;
        };

        let mut items: Vec<_> = read_dir.filter_map(|e| e.ok()).collect();
        items.sort_by(|a, b| {
            let a_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let b_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
            b_dir.cmp(&a_dir).then_with(|| a.file_name().cmp(&b.file_name()))
        });

        for item in items {
            let name = item.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }

            let path = item.path();
            let is_dir = item.file_type().map(|t| t.is_dir()).unwrap_or(false);

            // Check if this dir was previously expanded
            let was_expanded = self
                .entries
                .iter()
                .any(|e| e.path == path && e.expanded);

            self.entries.push(FileEntry {
                path: path.clone(),
                name,
                is_dir,
                depth,
                expanded: false,
            });

            if is_dir && was_expanded {
                let idx = self.entries.len() - 1;
                self.entries[idx].expanded = true;
                self.scan_dir(&path, depth + 1);
            }
        }
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        if !self.entries.is_empty() {
            self.selected = (self.selected + 1).min(self.entries.len() - 1);
        }
    }

    pub fn toggle_expand(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let entry = &self.entries[self.selected];
        if !entry.is_dir {
            return;
        }

        let path = entry.path.clone();
        let depth = entry.depth;
        let was_expanded = entry.expanded;

        if was_expanded {
            // Collapse: remove children
            self.entries[self.selected].expanded = false;
            let remove_start = self.selected + 1;
            let mut remove_end = remove_start;
            while remove_end < self.entries.len() && self.entries[remove_end].depth > depth {
                remove_end += 1;
            }
            self.entries.drain(remove_start..remove_end);
        } else {
            // Expand: insert children
            self.entries[self.selected].expanded = true;
            let mut children = Vec::new();
            Self::collect_children(&path, depth + 1, &mut children);
            let insert_pos = self.selected + 1;
            for (i, child) in children.into_iter().enumerate() {
                self.entries.insert(insert_pos + i, child);
            }
        }
    }

    fn collect_children(dir: &Path, depth: usize, out: &mut Vec<FileEntry>) {
        let Ok(read_dir) = std::fs::read_dir(dir) else {
            return;
        };

        let mut items: Vec<_> = read_dir.filter_map(|e| e.ok()).collect();
        items.sort_by(|a, b| {
            let a_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let b_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
            b_dir.cmp(&a_dir).then_with(|| a.file_name().cmp(&b.file_name()))
        });

        for item in items {
            let name = item.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let is_dir = item.file_type().map(|t| t.is_dir()).unwrap_or(false);
            out.push(FileEntry {
                path: item.path(),
                name,
                is_dir,
                depth,
                expanded: false,
            });
        }
    }

    pub fn selected_path(&self) -> Option<&Path> {
        self.entries.get(self.selected).map(|e| e.path.as_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scan_temp_dir() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("hello.txt"), "hi").unwrap();
        fs::create_dir(tmp.path().join("subdir")).unwrap();
        fs::write(tmp.path().join("subdir/nested.txt"), "yo").unwrap();

        let pane = FileTreePane::new(tmp.path().to_path_buf());
        assert!(pane.entries.len() >= 2);
        // dirs come first
        assert!(pane.entries[0].is_dir);
    }

    #[test]
    fn expand_collapse() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir(tmp.path().join("adir")).unwrap();
        fs::write(tmp.path().join("adir/file.txt"), "x").unwrap();

        let mut pane = FileTreePane::new(tmp.path().to_path_buf());
        // First entry should be the directory
        assert_eq!(pane.entries[0].name, "adir");
        assert!(!pane.entries[0].expanded);

        let initial_len = pane.entries.len();
        pane.toggle_expand();
        assert!(pane.entries[0].expanded);
        assert!(pane.entries.len() > initial_len);

        pane.toggle_expand();
        assert!(!pane.entries[0].expanded);
        assert_eq!(pane.entries.len(), initial_len);
    }
}
