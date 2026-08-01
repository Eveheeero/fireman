use super::*;
use crate::tui::tab::select_optimization::OPTIMIZATION_KIND;

fn insert_tab(app: &mut TuiApp, name: &str) {
    let index: usize = OPTIMIZATION_KIND
        .iter()
        .enumerate()
        .find_map(|(i, current_name)| if name == *current_name { Some(i) } else { None })
        .unwrap();
    app.data.tab.tabs.push(TuiTab::SelectOptimization(Box::new(
        SelectOptimizationData {
            selected: index,
            list: widgets::List::new(
                select_optimization::OPTIMIZATION_KIND
                    .iter()
                    .enumerate()
                    .map(|(i, kind)| format!("[{}] {}", if i == index { "v" } else { " " }, kind))
                    .collect::<Vec<_>>(),
            )
            .highlight_style(style::Style::new().fg(style::Color::Blue))
            .block(widgets::Block::bordered()),
            state: widgets::ListState::default().with_selected(Some(index)),
            custom_path: "".to_string(),
            custom: ["".to_string()].into(),
            custom_list: widgets::List::new([""]),
            custom_x_cursor: 0,
            custom_y_cursor: widgets::ListState::default().with_selected(Some(0)),
            focus: 0,
        },
    )));
}
pub fn default_tabs(app: &mut TuiApp) {
    let config = app.optimization_config.clone();
    if config.ir_analyzation {
        insert_tab(app, "Ir Analyzation");
    }
    if config.parameter_analyzation {
        insert_tab(app, "Parameter Analyzation");
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
    let max_pass_iterations = config.max_pass_iterations.max(1);
    if run_iterative_passes {
        for _ in 0..max_pass_iterations {
            if config.operator_canonicalization {
                insert_tab(app, "Operator Canonicalization");
            }
            if config.magic_division_recovery {
                insert_tab(app, "Magic Division Recovery");
            }
            if config.constant_folding {
                insert_tab(app, "Constant Folding");
            }
            if config.copy_propagation {
                insert_tab(app, "Copy Propagation");
            }
            if config.expression_inlining {
                insert_tab(app, "Expression Inlining");
            }
            if config.loop_analyzation {
                insert_tab(app, "Loop Analyzation");
            }
            if config.collapse_unused_variable {
                insert_tab(app, "Collapse Unused Variables");
            }
            if config.control_flow_cleanup {
                insert_tab(app, "Control Flow Cleanup");
            }
            if config.boolean_recovery {
                insert_tab(app, "Boolean Recovery");
            }

            if config.ternary_recovery {
                insert_tab(app, "Ternary Recovery");
            }
            if config.assertion_recovery {
                insert_tab(app, "Assertion Recovery");
            }
            if config.do_while_recovery {
                insert_tab(app, "DoWhile Recovery");
            }

            if config.if_conversion_reversal {
                insert_tab(app, "If Conversion Reversal");
            }
            if config.bit_trick_recognition {
                insert_tab(app, "Bit Trick Recognition");
            }
            if config.cast_minimization {
                insert_tab(app, "Cast Minimization");
            }
        }
        if config.control_flow_cleanup {
            insert_tab(app, "Control Flow Cleanup");
        }
        if config.loop_analyzation {
            insert_tab(app, "Loop Analyzation");
        }
        if config.switch_reconstruction {
            insert_tab(app, "Switch Reconstruction");
        }
        if config.early_return_normalization {
            insert_tab(app, "Early Return Normalization");
        }
        if config.expression_inlining {
            insert_tab(app, "Expression Inlining");
        }
        if config.collapse_unused_variable {
            insert_tab(app, "Collapse Unused Variables");
        }
        if config.lifetime_scoping {
            insert_tab(app, "Lifetime Scoping");
        }
        if config.name_recovery {
            insert_tab(app, "Name Recovery");
        }
    }
}
