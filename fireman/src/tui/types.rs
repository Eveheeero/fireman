use crate::model::OptimizationSettings;
use serde::Deserialize;

pub(crate) const LOG_LIMIT: usize = 256;

/// Types of content that can be displayed in a tab
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TabType {
    Input,   // Binary input/file loading
    Logs,    // Application logs
    Opt,     // Optimization pass selection (was Pick)
    Preview, // Read-only AST snapshot (was Result)
}

impl TabType {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            TabType::Input => "Input",
            TabType::Logs => "Logs",
            TabType::Opt => "Opt",
            TabType::Preview => "Preview",
        }
    }
}

/// A single tab in the flexible tab bar
#[derive(Clone, Debug)]
pub(crate) struct Tab {
    pub(crate) tab_type: TabType,
    pub(crate) label: String,
    pub(crate) closable: bool,
}

impl Tab {
    pub(crate) fn new(tab_type: TabType) -> Self {
        let label = tab_type.name().to_string();
        let closable = !matches!(tab_type, TabType::Input | TabType::Logs);
        Self {
            tab_type,
            label,
            closable,
        }
    }

    pub(crate) fn with_label(tab_type: TabType, label: impl Into<String>) -> Self {
        let closable = !matches!(tab_type, TabType::Input | TabType::Logs);
        Self {
            tab_type,
            label: label.into(),
            closable,
        }
    }
}

/// Flexible tab manager - tabs can be added/removed/reordered
#[derive(Clone, Debug)]
pub(crate) struct TabManager {
    pub(crate) tabs: Vec<Tab>,
    pub(crate) current_index: usize,
}

impl Default for TabManager {
    fn default() -> Self {
        Self {
            tabs: vec![
                Tab::new(TabType::Input),
                Tab::new(TabType::Logs),
            ],
            current_index: 0,
        }
    }
}

impl TabManager {
    pub(crate) fn current_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.current_index)
    }

    pub(crate) fn current_tab_type(&self) -> Option<TabType> {
        self.current_tab().map(|t| t.tab_type)
    }

    pub(crate) fn next_tab(&mut self) {
        if self.current_index < self.tabs.len().saturating_sub(1) {
            self.current_index += 1;
        }
    }

    pub(crate) fn prev_tab(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub(crate) fn goto_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.current_index = index;
        }
    }

    /// Add a new tab after the current one
    pub(crate) fn add_tab(&mut self, tab: Tab) -> usize {
        let insert_idx = (self.current_index + 1).min(self.tabs.len());
        self.tabs.insert(insert_idx, tab);
        insert_idx
    }

    /// Remove a tab by index (cannot remove Input or Logs)
    pub(crate) fn remove_tab(&mut self, index: usize) -> bool {
        if let Some(tab) = self.tabs.get(index) {
            if !tab.closable {
                return false;
            }
        } else {
            return false;
        }

        self.tabs.remove(index);
        if self.current_index >= index && self.current_index > 0 {
            self.current_index -= 1;
        }
        true
    }

    /// Remove current tab if closable
    pub(crate) fn remove_current(&mut self) -> bool {
        self.remove_tab(self.current_index)
    }

    /// Get tab labels for display
    pub(crate) fn labels(&self) -> Vec<String> {
        self.tabs
            .iter()
            .enumerate()
            .map(|(i, t)| format!("{} {}", i + 1, t.label))
            .collect()
    }

    /// Add a Preview tab for pipeline step N
    pub(crate) fn add_preview_tab(&mut self, n: usize) -> usize {
        let tab = Tab::with_label(TabType::Preview, format!("Preview {}", n));
        self.add_tab(tab)
    }

    /// Add an Opt tab for pipeline step N
    pub(crate) fn add_opt_tab(&mut self, n: usize) -> usize {
        let tab = Tab::with_label(TabType::Opt, format!("Opt {}", n));
        self.add_tab(tab)
    }

    /// Reset to default tabs
    pub(crate) fn reset(&mut self) {
        self.tabs = vec![
            Tab::new(TabType::Input),
            Tab::new(TabType::Logs),
        ];
        self.current_index = 0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OptimizationFocus {
    Settings,
    Script,
}

impl OptimizationFocus {
    pub(crate) const fn next(self) -> Self {
        match self {
            Self::Settings => Self::Script,
            Self::Script => Self::Settings,
        }
    }

    pub(crate) const fn previous(self) -> Self {
        match self {
            Self::Settings => Self::Script,
            Self::Script => Self::Settings,
        }
    }
}

/// Per-pipeline optimization stage. Holds settings and cached output.
pub(crate) struct OptStage {
    pub(crate) store: crate::model::OptimizationStore,
    pub(crate) focus: OptimizationFocus,
    pub(crate) setting_cursor: usize,
    pub(crate) script_cursor: usize,
    /// Cached output AST after this stage's optimization is applied
    pub(crate) output_ast: Option<fireball::abstract_syntax_tree::Ast>,
    pub(crate) output: Option<crate::model::DecompileResult>,
}

impl OptStage {
    pub(crate) fn new(store: crate::model::OptimizationStore) -> Self {
        Self {
            store,
            focus: OptimizationFocus::Settings,
            setting_cursor: 0,
            script_cursor: 0,
            output_ast: None,
            output: None,
        }
    }
}

/// Lightweight read-only snapshot at a pipeline point.
pub(crate) struct PreviewState {
    /// Snapshot of AST at this pipeline point (from nearest preceding Opt or raw decompile)
    pub(crate) ast: Option<fireball::abstract_syntax_tree::Ast>,
    pub(crate) outputs: Option<crate::model::DecompileResult>,
    pub(crate) cursor: usize,
}

impl PreviewState {
    pub(crate) fn new() -> Self {
        Self {
            ast: None,
            outputs: None,
            cursor: 0,
        }
    }
}

/// A single entry in the optimization pipeline.
pub(crate) enum PipelineEntry {
    Opt(OptStage),
    Preview(PreviewState),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PromptKind {
    OpenFile,
    AnalyzeAddress,
    LoadBufferPath,
    SaveBufferPath,
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
        (
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            String::new(),
        )
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

pub(crate) const OPTIMIZATION_FIELDS: &[OptimizationField] = &[
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
    OptimizationField {
        label: "Pattern matching",
        description: "Runs predefined and selected .fb pattern scripts",
        get: |s| s.pattern_matching_enabled,
        set: |s, v| s.pattern_matching_enabled = v,
    },
    OptimizationField {
        label: "Operator canonicalization",
        description: "Normalizes operator ordering for consistent comparison",
        get: |s| s.operator_canonicalization,
        set: |s, v| s.operator_canonicalization = v,
    },
    OptimizationField {
        label: "Magic division recovery",
        description: "Recovers division from magic-number multiplication patterns",
        get: |s| s.magic_division_recovery,
        set: |s, v| s.magic_division_recovery = v,
    },
    OptimizationField {
        label: "Identity simplification",
        description: "Simplifies identity operations like x+0, x*1",
        get: |s| s.identity_simplification,
        set: |s, v| s.identity_simplification = v,
    },
    OptimizationField {
        label: "Bit trick recognition",
        description: "Recognizes bit manipulation idioms",
        get: |s| s.bit_trick_recognition,
        set: |s, v| s.bit_trick_recognition = v,
    },
    OptimizationField {
        label: "Cast minimization",
        description: "Removes redundant type casts",
        get: |s| s.cast_minimization,
        set: |s, v| s.cast_minimization = v,
    },
    OptimizationField {
        label: "Assertion recovery",
        description: "Recovers assertion patterns from conditional aborts",
        get: |s| s.assertion_recovery,
        set: |s, v| s.assertion_recovery = v,
    },
    OptimizationField {
        label: "Do-while recovery",
        description: "Recovers do-while loops from CFG",
        get: |s| s.do_while_recovery,
        set: |s, v| s.do_while_recovery = v,
    },
    OptimizationField {
        label: "Clamp recovery",
        description: "Recovers clamp/min/max patterns",
        get: |s| s.clamp_recovery,
        set: |s, v| s.clamp_recovery = v,
    },
    OptimizationField {
        label: "Loop cleanup",
        description: "Cleans up loop structure after recovery",
        get: |s| s.loop_cleanup,
        set: |s, v| s.loop_cleanup = v,
    },
    OptimizationField {
        label: "If-conversion reversal",
        description: "Reverses compiler if-conversion optimizations",
        get: |s| s.if_conversion_reversal,
        set: |s, v| s.if_conversion_reversal = v,
    },
    OptimizationField {
        label: "Anti-debug AST suppression",
        description: "Suppresses anti-debug code patterns in output",
        get: |s| s.anti_debug_ast_suppression,
        set: |s, v| s.anti_debug_ast_suppression = v,
    },
    OptimizationField {
        label: "Logging suppression",
        description: "Suppresses logging boilerplate in output",
        get: |s| s.logging_suppression,
        set: |s, v| s.logging_suppression = v,
    },
    OptimizationField {
        label: "Static guard suppression",
        description: "Suppresses static guard patterns in output",
        get: |s| s.static_guard_suppression,
        set: |s, v| s.static_guard_suppression = v,
    },
    OptimizationField {
        label: "Security scaffold suppression",
        description: "Suppresses security scaffold patterns in output",
        get: |s| s.security_scaffold_suppression,
        set: |s, v| s.security_scaffold_suppression = v,
    },
];

pub(crate) fn all_optimization_fields() -> impl Iterator<Item = &'static OptimizationField> {
    OPTIMIZATION_FIELDS.iter()
}

pub(crate) fn optimization_field_count() -> usize {
    OPTIMIZATION_FIELDS.len()
}
