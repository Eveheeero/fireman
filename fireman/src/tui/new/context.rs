use std::{
    fs,
    path::{Path, PathBuf},
};

pub const PREFIX_DIR: &str = "📁 ";
pub const PREFIX_FILE: &str = "📄 ";

pub struct Context {
    pub path: String,
    pub message: Option<String>,
    pub file_tree: Vec<String>,
    pub selected_index: usize,
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = Context {
            path: String::new(),
            message: None,
            file_tree: Vec::new(),
            selected_index: 0,
        };
        ctx.update_file_tree();
        ctx
    }

    pub fn update_file_tree(&mut self) {
        self.file_tree.clear();
        if self.path.is_empty() {
            // Show current directory if no path entered
            if let Ok(current_dir) = std::env::current_dir() {
                if let Ok(entries) = fs::read_dir(&current_dir) {
                    for entry in entries.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            let prefix = if entry.path().is_dir() {
                                PREFIX_DIR
                            } else {
                                PREFIX_FILE
                            };
                            self.file_tree.push(format!("{}{}", prefix, name));
                        }
                    }
                }
            }
            self.clamp_selected_index();
            return;
        }

        // Convert to absolute path for consistent handling
        let path = self.resolve_path(&self.path);

        // If path's filename is ., highlight files
        // Pathbuf doesn't treat . as a filename
        if self.path.ends_with(".") && !self.path.ends_with("..") {
            self.show_directory_with_filter(&path, ".");
            self.clamp_selected_index();
            return;
        }

        // Try to find the best directory to show
        let (dir_to_show, input_filename) = if path.exists() {
            // If path exists, show its contents if it's a directory, or parent if it's a file
            if path.is_dir() {
                // When showing directory contents, don't match against directory name
                (Some(path), "")
            } else {
                // When showing parent, match against the file name
                (
                    path.parent().map(|x| x.to_path_buf()),
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or(""),
                )
            }
        } else {
            // If path doesn't exist, find the deepest existing parent directory
            let mut current = path.as_path();
            while let Some(parent) = current.parent() {
                if parent.exists() && parent.is_dir() {
                    // Use the non-existing part as the filename to match
                    let filename = current
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("");
                    self.show_directory_with_filter(parent, filename);
                    self.clamp_selected_index();
                    return;
                }
                current = parent;
            }
            // Fallback to current directory for relative paths
            let current_dir =
                std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
            (
                Some(current_dir),
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or(""),
            )
        };

        if let Some(dir) = dir_to_show {
            self.show_directory_with_filter(&dir, input_filename);
        }
        self.clamp_selected_index();
    }

    fn resolve_path(&self, input_path: &str) -> PathBuf {
        let path = PathBuf::from(input_path);

        // If it's already an absolute path, return as is
        if path.is_absolute() {
            return path;
        }

        // For relative paths, resolve against current directory
        if let Ok(current_dir) = std::env::current_dir() {
            current_dir.join(path)
        } else {
            // Fallback if current_dir fails
            path
        }
    }

    fn show_directory_with_filter(&mut self, dir: &Path, filter: &str) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    let prefix = if entry.path().is_dir() {
                        PREFIX_DIR
                    } else {
                        PREFIX_FILE
                    };
                    let display_name = format!("{}{}", prefix, name);

                    // Highlight files that match the current input
                    if !filter.is_empty() && name.starts_with(filter) {
                        self.file_tree.push(format!(">> {}", display_name));
                    } else {
                        self.file_tree.push(display_name);
                    }
                }
            }

            // Sort so highlighted items appear first
            self.file_tree.sort_by(|a, b| {
                let a_highlighted = a.starts_with(">>");
                let b_highlighted = b.starts_with(">>");
                match (a_highlighted, b_highlighted) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.cmp(b),
                }
            });
        }
    }

    fn clamp_selected_index(&mut self) {
        if self.file_tree.is_empty() {
            self.selected_index = 0;
        } else if self.selected_index >= self.file_tree.len() {
            self.selected_index = self.file_tree.len() - 1;
        }
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index + 1 < self.file_tree.len() {
            self.selected_index += 1;
        }
    }

    pub fn move_to_first(&mut self) {
        self.selected_index = 0;
    }

    pub fn move_to_last(&mut self) {
        if !self.file_tree.is_empty() {
            self.selected_index = self.file_tree.len() - 1;
        }
    }

    pub fn move_page_up(&mut self) {
        const PAGE_SIZE: usize = 10;
        if self.selected_index >= PAGE_SIZE {
            self.selected_index -= PAGE_SIZE;
        } else {
            self.selected_index = 0;
        }
    }

    pub fn move_page_down(&mut self) {
        const PAGE_SIZE: usize = 10;
        if self.selected_index + PAGE_SIZE < self.file_tree.len() {
            self.selected_index += PAGE_SIZE;
        } else if !self.file_tree.is_empty() {
            self.selected_index = self.file_tree.len() - 1;
        }
    }
}
