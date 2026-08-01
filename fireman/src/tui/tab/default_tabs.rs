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

    let max_pass_iterations = config.max_pass_iterations.max(1);
    for _ in 0..max_pass_iterations {
        if config.constant_folding {
            insert_tab(app, "Constant Folding");
        }
        if config.collapse_unused_variable {
            insert_tab(app, "Collapse Unused Variables");
        }
    }
}
