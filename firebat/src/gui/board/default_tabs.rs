use super::{BoardData, display_current_ast, select_optimization};
use crate::gui::board::select_optimization::{CUSTOM_PATTERN_INDEX, window_custom_pattern};
use fireball::abstract_syntax_tree::AstOptimizationKind;

/// Amount of windows placed on a single row before the chain folds back.
const PER_ROW: usize = 8;

/// Distance between two windows of the same row, in scene coordinates.
const COLUMN_OFFSET: egui::Vec2 = egui::vec2(300.0, 0.0);

/// Distance between two rows of the chain, in scene coordinates.
const ROW_OFFSET: egui::Vec2 = egui::vec2(0.0, 720.0);

/// Builds the window chain the tui opens by default, mirroring `tui::tab::default_tabs`.
///
/// Every optimization of the default config becomes a Select Optimization window connected to the
/// previous one, and the chain ends with a Display Current AST window.
pub fn default_tabs(board: &mut BoardData, root: &str, root_pos: egui::Pos2) {
    let optimizations = AstOptimizationKind::all();
    let flatten_optimizations = flatten_optimizations(optimizations);

    let mut parent = root.to_owned();
    let mut placed = 0;
    for optimization in flatten_optimizations {
        let optimization_name = optimization_to_name(&optimization);
        let Some(selected) = select_optimization::index_of(optimization_name) else {
            tracing::warn!("unknown optimization: {optimization_name}");
            continue;
        };
        placed += 1;
        let id = board.pipeline.spawn_id(&parent, "optimization");
        let pos = position(root_pos, placed);
        if selected == CUSTOM_PATTERN_INDEX {
            let custom_path = if let AstOptimizationKind::PatternMatching(pattern) = optimization {
                pattern.name().to_string()
            } else {
                panic!()
            };
            window_custom_pattern(id.as_str(), pos, custom_path);
        } else {
            board.add_window(select_optimization::window_with(id.as_str(), pos, selected));
        }
        board.connect(&parent, &id);
        parent = id;
    }

    let id = board.pipeline.spawn_id(&parent, "ast");
    let pos = position(root_pos, placed + 1);
    board.add_window(display_current_ast::window(id.as_str(), pos));
    board.connect(&parent, &id);

    tracing::debug!("opened the default chain of {} windows", placed + 1);
}

/// Position of the window at the given step of the chain, folded into rows.
fn position(root_pos: egui::Pos2, step: usize) -> egui::Pos2 {
    let row = step / PER_ROW;
    let column = step % PER_ROW;
    // Odd rows run backwards, so the last window of a row stays next to the first of the next one.
    let column = if row % 2 == 0 {
        column
    } else {
        PER_ROW - 1 - column
    };
    root_pos + COLUMN_OFFSET * column as f32 + ROW_OFFSET * row as f32
}

/// flatten `OptimizationLoop` from optimization lists
fn flatten_optimizations(optimizations: Vec<AstOptimizationKind>) -> Vec<AstOptimizationKind> {
    let mut list = Vec::new();
    for optimization in optimizations {
        if let AstOptimizationKind::OptimizationLoop(optimizations, loop_count) = optimization {
            for _ in 0..loop_count {
                list.extend_from_slice(&optimizations);
            }
        } else {
            list.push(optimization);
        }
    }
    list
}

fn optimization_to_name(optimization: &AstOptimizationKind) -> &'static str {
    match optimization {
        AstOptimizationKind::IrAnalyzation => "Ir Analyzation",
        AstOptimizationKind::ParameterAnalyzation => "Parameter Analyzation",
        AstOptimizationKind::ConstantFolding => "Constant Folding",
        AstOptimizationKind::CollapseUnusedVariables => "Collapse Unused Variables",
        AstOptimizationKind::OptimizationLoop(_, _) => unreachable!(),
        AstOptimizationKind::PatternMatching(_) => "Custom Pattern",
    }
}
