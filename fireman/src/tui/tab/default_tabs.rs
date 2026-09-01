use super::*;
use crate::tui::tab::select_optimization::{CUSTOM_PATTERN_INDEX, OPTIMIZATION_KIND};
use fireball::{abstract_syntax_tree::AstOptimizationKind, pattern_matching::AstPattern};

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
fn insert_custom_pattern_tab(app: &mut TuiApp, pattern: &AstPattern) {
    let index: usize = CUSTOM_PATTERN_INDEX;
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
            custom_path: pattern.name().to_string(),
            custom: ["".to_string()].into(),
            custom_list: widgets::List::new([""]),
            custom_x_cursor: 0,
            custom_y_cursor: widgets::ListState::default().with_selected(Some(0)),
            focus: 0,
        },
    )));
}

pub fn default_tabs(app: &mut TuiApp) {
    let optimizations = app.optimizations.clone();
    fn inner(app: &mut TuiApp, optimizations: &[AstOptimizationKind]) {
        for optimization in optimizations {
            match optimization {
                AstOptimizationKind::IrAnalyzation => {
                    insert_tab(app, "Ir Analyzation");
                }
                AstOptimizationKind::ParameterAnalyzation => {
                    insert_tab(app, "Parameter Analyzation");
                }
                AstOptimizationKind::ConstantFolding => {
                    insert_tab(app, "Constant Folding");
                }
                AstOptimizationKind::CollapseUnusedVariables => {
                    insert_tab(app, "Collapse Unused Variables");
                }
                AstOptimizationKind::OptimizationLoop(optimizations, loop_count) => {
                    for _ in 0..*loop_count {
                        inner(app, optimizations);
                    }
                }
                AstOptimizationKind::PatternMatching(pattern) => {
                    insert_custom_pattern_tab(app, &pattern)
                }
            }
        }
    }
    inner(app, &optimizations);
}
