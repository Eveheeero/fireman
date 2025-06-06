use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Context {
    pub path: String,
    pub error_message: Option<String>,
    pub file_tree: Vec<String>,
    pub selected_index: usize,
}

impl Context {
    pub fn new() -> Self {
        Context {
            path: String::new(),
            error_message: None,
            file_tree: Vec::new(),
            selected_index: 0,
        }
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
                                "📁 "
                            } else {
                                "📄 "
                            };
                            self.file_tree.push(format!("{}{}", prefix, name));
                        }
                    }
                }
            }
            self.clamp_selected_index();
            return;
        }

        let path = PathBuf::from(&self.path);

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
            // Fallback to parent directory
            (
                current.parent().map(|x| x.to_path_buf()),
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

    fn show_directory_with_filter(&mut self, dir: &Path, filter: &str) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    let prefix = if entry.path().is_dir() {
                        "📁 "
                    } else {
                        "📄 "
                    };
                    let display_name = format!("{}{}", prefix, name);

                    // Highlight files that match the current input
                    if !filter.is_empty() && name.starts_with(filter) {
                        self.file_tree.push(format!(">>> {}", display_name));
                    } else {
                        self.file_tree.push(display_name);
                    }
                }
            }

            // Sort so highlighted items appear first
            self.file_tree.sort_by(|a, b| {
                let a_highlighted = a.starts_with(">>>");
                let b_highlighted = b.starts_with(">>>");
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
}
