use super::{BoardData, display_current_ast, select_optimization};
use fireball::abstract_syntax_tree::AstOptimizationConfig;

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
    let names = default_optimizations(&AstOptimizationConfig::default());

    let mut parent = root.to_owned();
    let mut placed = 0;
    for name in names {
        let Some(selected) = select_optimization::index_of(name) else {
            tracing::warn!("unknown optimization: {name}");
            continue;
        };
        placed += 1;
        let id = board.pipeline.spawn_id(&parent, "optimization");
        let pos = position(root_pos, placed);
        board.add_window(select_optimization::window_with(id.as_str(), pos, selected));
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

/// Optimizations the default chain applies, in the very order the tui applies them.
fn default_optimizations(config: &AstOptimizationConfig) -> Vec<&'static str> {
    let mut names = Vec::new();

    if config.ir_analyzation {
        names.push("Ir Analyzation");
    }
    if config.parameter_analyzation {
        names.push("Parameter Analyzation");
    }
    if config.call_argument_analyzation {
        names.push("Call Argument Analyzation");
        if config.constant_folding {
            names.push("Constant Folding");
        }
    }

    let run_iterative_passes = config.loop_analyzation
        || config.constant_folding
        || config.control_flow_cleanup
        || config.pattern_matching_enabled
        || config.collapse_unused_variable
        || config.dead_store_elimination
        || config.copy_propagation
        || config.expression_inlining
        || config.operator_canonicalization
        || config.magic_division_recovery
        || config.identity_simplification
        || config.bit_trick_recognition
        || config.cast_minimization
        || config.ternary_recovery
        || config.boolean_recovery
        || config.assertion_recovery
        || config.do_while_recovery
        || config.clamp_recovery
        || config.loop_cleanup
        || config.if_conversion_reversal;
    if !run_iterative_passes {
        return names;
    }

    let max_pass_iterations = config.max_pass_iterations.max(1);
    for _ in 0..max_pass_iterations {
        if config.operator_canonicalization {
            names.push("Operator Canonicalization");
        }
        if config.magic_division_recovery {
            names.push("Magic Division Recovery");
        }
        if config.constant_folding {
            names.push("Constant Folding");
        }
        if config.copy_propagation {
            names.push("Copy Propagation");
        }
        if config.expression_inlining {
            names.push("Expression Inlining");
        }
        if config.loop_analyzation {
            names.push("Loop Analyzation");
        }
        if config.collapse_unused_variable {
            names.push("Collapse Unused Variables");
        }
        if config.control_flow_cleanup {
            names.push("Control Flow Cleanup");
        }
        if config.boolean_recovery {
            names.push("Boolean Recovery");
        }
        if config.ternary_recovery {
            names.push("Ternary Recovery");
        }
        if config.assertion_recovery {
            names.push("Assertion Recovery");
        }
        if config.do_while_recovery {
            names.push("DoWhile Recovery");
        }
        if config.if_conversion_reversal {
            names.push("If Conversion Reversal");
        }
        if config.bit_trick_recognition {
            names.push("Bit Trick Recognition");
        }
        if config.cast_minimization {
            names.push("Cast Minimization");
        }
    }

    if config.control_flow_cleanup {
        names.push("Control Flow Cleanup");
    }
    if config.loop_analyzation {
        names.push("Loop Analyzation");
    }
    if config.switch_reconstruction {
        names.push("Switch Reconstruction");
    }
    if config.early_return_normalization {
        names.push("Early Return Normalization");
    }
    if config.expression_inlining {
        names.push("Expression Inlining");
    }
    if config.collapse_unused_variable {
        names.push("Collapse Unused Variables");
    }
    if config.lifetime_scoping {
        names.push("Lifetime Scoping");
    }
    if config.name_recovery {
        names.push("Name Recovery");
    }

    names
}
