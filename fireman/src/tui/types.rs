use crate::model::{EditPosition, EditorTarget, OptimizationSettings};
use serde::Deserialize;

pub(crate) const LOG_LIMIT: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum View {
    Sections,
    Assembly,
    Ir,
    Ast,
    Editor,
    Optimization,
    Patch,
    Logs,
}

impl View {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Sections => 0,
            Self::Assembly => 1,
            Self::Ir => 2,
            Self::Ast => 3,
            Self::Editor => 4,
            Self::Optimization => 5,
            Self::Patch => 6,
            Self::Logs => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OptimizationFocus {
    Settings,
    Scripts,
    Buffer,
}

impl OptimizationFocus {
    pub(crate) const fn next(self) -> Self {
        match self {
            Self::Settings => Self::Scripts,
            Self::Scripts => Self::Buffer,
            Self::Buffer => Self::Settings,
        }
    }

    pub(crate) const fn previous(self) -> Self {
        match self {
            Self::Settings => Self::Buffer,
            Self::Scripts => Self::Settings,
            Self::Buffer => Self::Scripts,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PromptKind {
    OpenFile,
    AnalyzeAddress,
    EditLine(EditorTarget),
    AddScriptPath,
    LoadBufferPath,
    SaveBufferPath,
    SavePatchPath,
    EditBuffer,
}

pub(crate) struct PromptState {
    pub(crate) kind: PromptKind,
    pub(crate) title: String,
    pub(crate) text: String,
    pub(crate) cursor: usize,
    pub(crate) multiline: bool,
    pub(crate) help: String,
    pub(crate) file_browser: Option<FileBrowserState>,
}

pub(crate) struct FileBrowserState {
    pub(crate) entries: Vec<FileBrowserEntry>,
    pub(crate) selected_index: usize,
}

#[derive(Clone)]
pub(crate) struct FileBrowserEntry {
    pub(crate) name: String,
    pub(crate) is_dir: bool,
    pub(crate) matched: bool,
}

impl FileBrowserState {
    pub(crate) fn new() -> Self {
        let mut state = Self {
            entries: Vec::new(),
            selected_index: 0,
        };
        state.update_from_path("");
        state
    }

    pub(crate) fn update_from_path(&mut self, input: &str) {
        self.entries.clear();

        let (dir_to_scan, filter) = resolve_scan_target(input);

        if let Ok(read_dir) = std::fs::read_dir(&dir_to_scan) {
            for entry in read_dir.flatten() {
                if let Some(name) = entry.file_name().to_str().map(String::from) {
                    let is_dir = entry.path().is_dir();
                    let matched = !filter.is_empty()
                        && name.to_lowercase().starts_with(&filter.to_lowercase());
                    self.entries.push(FileBrowserEntry {
                        name,
                        is_dir,
                        matched,
                    });
                }
            }
        }

        // Sort: matched first, then dirs before files, then alphabetical
        self.entries.sort_by(|a, b| {
            b.matched
                .cmp(&a.matched)
                .then(b.is_dir.cmp(&a.is_dir))
                .then(a.name.cmp(&b.name))
        });

        self.clamp_index();
    }

    pub(crate) fn move_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub(crate) fn move_down(&mut self) {
        if self.selected_index + 1 < self.entries.len() {
            self.selected_index += 1;
        }
    }

    pub(crate) fn move_page_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(10);
    }

    pub(crate) fn move_page_down(&mut self) {
        if self.selected_index + 10 < self.entries.len() {
            self.selected_index += 10;
        } else if !self.entries.is_empty() {
            self.selected_index = self.entries.len() - 1;
        }
    }

    pub(crate) fn selected_entry(&self) -> Option<&FileBrowserEntry> {
        self.entries.get(self.selected_index)
    }

    /// Complete the path from the currently selected entry.
    /// Returns the new full path string.
    pub(crate) fn complete_path(&self, current_input: &str) -> Option<String> {
        let entry = self.selected_entry()?;
        let (dir, _filter) = resolve_scan_target(current_input);
        let mut completed = dir.join(&entry.name).to_string_lossy().to_string();
        if entry.is_dir && !completed.ends_with(std::path::MAIN_SEPARATOR) {
            completed.push(std::path::MAIN_SEPARATOR);
        }
        Some(completed)
    }

    fn clamp_index(&mut self) {
        if self.entries.is_empty() {
            self.selected_index = 0;
        } else if self.selected_index >= self.entries.len() {
            self.selected_index = self.entries.len() - 1;
        }
    }
}

fn resolve_scan_target(input: &str) -> (std::path::PathBuf, String) {
    use std::path::{Path, PathBuf};

    if input.is_empty() {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        return (cwd, String::new());
    }

    let path = if Path::new(input).is_absolute() {
        PathBuf::from(input)
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(input)
    };

    if path.is_dir() {
        (path, String::new())
    } else if let Some(parent) = path.parent() {
        let parent = if parent.as_os_str().is_empty() {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        } else {
            parent.to_path_buf()
        };
        let filter = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        (parent, filter)
    } else {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        (cwd, String::new())
    }
}

#[derive(Deserialize)]
pub(crate) struct OptimizationStoreEnvelope {
    pub(crate) optimization_store: crate::model::OptimizationStore,
}

#[derive(Clone, Copy)]
pub(crate) struct OptimizationField {
    pub(crate) label: &'static str,
    pub(crate) description: &'static str,
    pub(crate) get: fn(&OptimizationSettings) -> bool,
    pub(crate) set: fn(&mut OptimizationSettings, bool),
}

pub(crate) struct OptimizationGroup {
    pub(crate) label: &'static str,
    pub(crate) fields: &'static [OptimizationField],
}

pub(crate) const OPTIMIZATION_GROUPS: &[OptimizationGroup] = &[
    OptimizationGroup {
        label: "Analysis",
        fields: &[
            OptimizationField {
                label: "IR analyzation",
                description: "Builds the IR analysis layer used by later AST passes",
                get: |s| s.ir_analyzation,
                set: |s, v| s.ir_analyzation = v,
            },
            OptimizationField {
                label: "Parameter analyzation",
                description: "Infers function parameters from recovered usage",
                get: |s| s.parameter_analyzation,
                set: |s, v| s.parameter_analyzation = v,
            },
            OptimizationField {
                label: "Call argument analyzation",
                description: "Propagates argument information into recovered calls",
                get: |s| s.call_argument_analyzation,
                set: |s, v| s.call_argument_analyzation = v,
            },
            OptimizationField {
                label: "Name recovery",
                description: "Recovers variable and helper names when possible",
                get: |s| s.name_recovery,
                set: |s, v| s.name_recovery = v,
            },
            OptimizationField {
                label: "Signedness inference",
                description: "Refines integer semantics from instruction behavior",
                get: |s| s.signedness_inference,
                set: |s, v| s.signedness_inference = v,
            },
            OptimizationField {
                label: "Auto comment",
                description: "Emits automatically derived AST comments",
                get: |s| s.auto_comment,
                set: |s, v| s.auto_comment = v,
            },
        ],
    },
    OptimizationGroup {
        label: "Simplification",
        fields: &[
            OptimizationField {
                label: "Constant folding",
                description: "Evaluates constant expressions during optimization",
                get: |s| s.constant_folding,
                set: |s, v| s.constant_folding = v,
            },
            OptimizationField {
                label: "Copy propagation",
                description: "Eliminates temporary copies when values can be forwarded",
                get: |s| s.copy_propagation,
                set: |s, v| s.copy_propagation = v,
            },
            OptimizationField {
                label: "Expression inlining",
                description: "Inlines short temporary expressions into their uses",
                get: |s| s.expression_inlining,
                set: |s, v| s.expression_inlining = v,
            },
            OptimizationField {
                label: "Dead store elimination",
                description: "Removes writes that never affect later behavior",
                get: |s| s.dead_store_elimination,
                set: |s, v| s.dead_store_elimination = v,
            },
            OptimizationField {
                label: "Collapse unused variable",
                description: "Drops redundant variables that do not survive analysis",
                get: |s| s.collapse_unused_varaible,
                set: |s, v| s.collapse_unused_varaible = v,
            },
            OptimizationField {
                label: "Lifetime scoping",
                description: "Shrinks recovered variable lifetimes around real usage",
                get: |s| s.lifetime_scoping,
                set: |s, v| s.lifetime_scoping = v,
            },
        ],
    },
    OptimizationGroup {
        label: "Structure Recovery",
        fields: &[
            OptimizationField {
                label: "Control flow cleanup",
                description: "Removes structural noise before higher-level recovery",
                get: |s| s.control_flow_cleanup,
                set: |s, v| s.control_flow_cleanup = v,
            },
            OptimizationField {
                label: "Loop analyzation",
                description: "Recovers loop constructs from CFG structure",
                get: |s| s.loop_analyzation,
                set: |s, v| s.loop_analyzation = v,
            },
            OptimizationField {
                label: "Ternary recovery",
                description: "Rebuilds ternary expressions from compact branches",
                get: |s| s.ternary_recovery,
                set: |s, v| s.ternary_recovery = v,
            },
            OptimizationField {
                label: "Boolean recovery",
                description: "Normalizes predicate-heavy code into boolean expressions",
                get: |s| s.boolean_recovery,
                set: |s, v| s.boolean_recovery = v,
            },
            OptimizationField {
                label: "Switch reconstruction",
                description: "Detects and prints switch-style control flow",
                get: |s| s.switch_reconstruction,
                set: |s, v| s.switch_reconstruction = v,
            },
            OptimizationField {
                label: "Early return normalization",
                description: "Prefers normalized early-return shapes in the AST",
                get: |s| s.early_return_normalization,
                set: |s, v| s.early_return_normalization = v,
            },
        ],
    },
    OptimizationGroup {
        label: "Pattern Engine",
        fields: &[OptimizationField {
            label: "Pattern matching",
            description: "Runs predefined and selected .fb pattern scripts",
            get: |s| s.pattern_matching_enabled,
            set: |s, v| s.pattern_matching_enabled = v,
        }],
    },
];

/// Flat iterator over all optimization fields across groups.
pub(crate) fn all_optimization_fields() -> impl Iterator<Item = &'static OptimizationField> {
    OPTIMIZATION_GROUPS
        .iter()
        .flat_map(|group| group.fields.iter())
}

/// Total number of optimization fields.
pub(crate) fn optimization_field_count() -> usize {
    OPTIMIZATION_GROUPS
        .iter()
        .map(|group| group.fields.len())
        .sum()
}

pub(crate) fn next_position(position: EditPosition, forward: bool) -> EditPosition {
    match (position, forward) {
        (EditPosition::Replace, true) => EditPosition::Before,
        (EditPosition::Before, true) => EditPosition::After,
        (EditPosition::After, true) => EditPosition::Replace,
        (EditPosition::Replace, false) => EditPosition::After,
        (EditPosition::Before, false) => EditPosition::Replace,
        (EditPosition::After, false) => EditPosition::Before,
    }
}
