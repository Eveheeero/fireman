use crate::{
    Firebat,
    gui::board::{BoardWindow, BoardWindowKind},
};
use fireball::{abstract_syntax_tree::AstOptimizationKind, pattern_matching::AstPattern};
use std::path::Path;

/// See [fireball::abstract_syntax_tree::AstOptimizationKind], mirrored from the tui.
pub const OPTIMIZATION_KIND: &[&str] = &[
    "Ir Analyzation",
    "Parameter Analyzation",
    "Constant Folding",
    "Collapse Unused Variables",
    "Custom Pattern",
];

pub const CUSTOM_PATTERN_INDEX: usize = 6;

/// Optimization applied by a single window.
#[derive(Clone)]
pub struct SelectOptimizationChoice {
    pub selected: usize,
    pub custom_path: String,
    pub custom: String,
}

impl Default for SelectOptimizationChoice {
    fn default() -> Self {
        Self {
            selected: CUSTOM_PATTERN_INDEX,
            custom_path: String::new(),
            custom: String::new(),
        }
    }
}

impl SelectOptimizationChoice {
    /// Body of the custom pattern, read from the file when a path is given.
    pub fn custom_pattern_source(&self) -> String {
        if Path::new(&self.custom_path).is_file() {
            std::fs::read_to_string(&self.custom_path).unwrap_or_default()
        } else {
            self.custom.clone()
        }
    }
}

/// State of the window which picks a single optimization step.
#[derive(Default)]
pub struct SelectOptimizationData {
    choice: SelectOptimizationChoice,
}

impl SelectOptimizationData {
    /// Optimization currently picked by the window.
    pub fn choice(&self) -> &SelectOptimizationChoice {
        &self.choice
    }
}

/// Index of the optimization shown under the given name.
pub fn index_of(name: &str) -> Option<usize> {
    OPTIMIZATION_KIND.iter().position(|it| *it == name)
}

/// Creates the window which picks a single optimization step.
pub fn window(id: impl Into<String>, pos: egui::Pos2) -> BoardWindow {
    window_with(id, pos, CUSTOM_PATTERN_INDEX)
}

/// Creates the window with the given optimization already picked.
pub fn window_with(id: impl Into<String>, pos: egui::Pos2, selected: usize) -> BoardWindow {
    BoardWindow::new(
        id,
        "Select Optimization",
        pos,
        BoardWindowKind::SelectOptimization(SelectOptimizationData {
            choice: SelectOptimizationChoice {
                selected,
                ..Default::default()
            },
        }),
    )
}

pub fn ui(_app: &mut Firebat, id: &str, data: &mut SelectOptimizationData, ui: &mut egui::Ui) {
    ui.set_min_width(200.0);

    let mut changed = false;

    for (index, kind) in OPTIMIZATION_KIND.iter().enumerate() {
        changed |= ui
            .radio_value(&mut data.choice.selected, index, *kind)
            .changed();
    }

    if data.choice.selected == CUSTOM_PATTERN_INDEX {
        ui.separator();
        changed |= ui
            .add(
                egui::TextEdit::singleline(&mut data.choice.custom_path)
                    .hint_text("custom pattern path"),
            )
            .changed();
        changed |= ui
            .add(
                egui::TextEdit::multiline(&mut data.choice.custom)
                    .code_editor()
                    .desired_rows(6)
                    .hint_text("custom pattern"),
            )
            .changed();
    }

    if changed {
        tracing::debug!(
            "optimization of {id} changed to {}",
            OPTIMIZATION_KIND[data.choice.selected]
        );
    }
}

/// See [fireball::abstract_syntax_tree::AstOptimizationKind]
pub fn choice_to_ast_optimization_kind(choice: &SelectOptimizationChoice) -> AstOptimizationKind {
    let custom_pattern = if choice.selected == CUSTOM_PATTERN_INDEX {
        if Path::new(&choice.custom_path).is_file() {
            AstPattern::from_file(&choice.custom_path)
        } else {
            AstPattern::new("", &choice.custom)
        }
    } else {
        AstPattern::new("", "")
    };
    match choice.selected {
        0 => AstOptimizationKind::IrAnalyzation,
        1 => AstOptimizationKind::ParameterAnalyzation,
        2 => AstOptimizationKind::ConstantFolding,
        3 => AstOptimizationKind::CollapseUnusedVariables,
        4 => AstOptimizationKind::PatternMatching(Box::new(custom_pattern)),
        _ => unreachable!(),
    }
}
