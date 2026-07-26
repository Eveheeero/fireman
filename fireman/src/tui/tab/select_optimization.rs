use crate::tui::{
    TuiApp,
    tab::{SelectOptimizationData, TuiTab, handle_del_tab, handle_new_tab, handle_turn_tab},
};
use crossterm::event;
use fireball::{abstract_syntax_tree::AstOptimizationKind, pattern_matching::AstPattern};
use ratatui::{Frame, prelude::*, widgets};
use std::path::Path;

pub fn draw(data: &mut SelectOptimizationData, mut area: Rect, terminal: &mut Frame) {
    // render list
    let mut list_area = area;
    list_area.width /= 2;
    area.x += list_area.width;
    area.width -= list_area.width;
    terminal.render_stateful_widget(&data.list, list_area, &mut data.state);

    if data.state.selected().unwrap() == CUSTOM_PATTERN_INDEX {
        // render custom pattern
        let mut custom_pattern_path_area = area;
        custom_pattern_path_area.height = 1;
        area.y += 1;
        area.height -= 1;
        let custom_pattern_path = if data.custom_path.is_empty() {
            "enter custom pattern path"
        } else {
            &data.custom_path
        };
        terminal.render_widget(custom_pattern_path, custom_pattern_path_area);
        terminal.render_widget(widgets::Block::bordered(), area);
        area.x += 1;
        area.height -= 2;
        area.y += 1;
        area.width -= 2;
        terminal.render_widget(data.custom.as_str(), area);
    } else {
        // render help
        terminal.render_widget(widgets::Block::bordered(), area);
        area.x += 1;
        area.height -= 2;
        area.y += 1;
        area.width -= 2;
        terminal.render_widget("help goes here", area);
    }
}
pub fn handle_event(app: &mut TuiApp, event: event::Event) {
    if handle_turn_tab(app, &event) || handle_new_tab(app, &event) || handle_del_tab(app, &event) {
        return;
    }

    let current_tab_index = app.data.tab.current_tab_index;
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    let TuiTab::SelectOptimization(_data) = current_tab else {
        unreachable!()
    };
}

/// See [fireball::abstract_syntax_tree::AstOptimizationKind]
pub const OPTIMIZATION_KIND: &[&str] = &[
    "Ir Analyzation",
    "Parameter Analyzation",
    "Call Argument Analyzation",
    "Constant Folding",
    "Control Flow Cleanup",
    "Collapse Unused Variables",
    "Custom Pattern",
    "Loop Analyzation",
    "Copy Propagation",
    "Expression Inlining",
    "Ternary Recovery",
    "If Conversion Reversal",
    "Boolean Recovery",
    "Switch Reconstruction",
    "Operator Canonicalization",
    "Common Subexpression Elimination",
    "Bit Trick Recognition",
    "Cast Minimization",
    "Magic Division Recovery",
    "Goto Containment",
    "Induction Variable Analysis",
    "Temporary Elimination",
    "Lifetime Scoping",
    "Variable Coalescing",
    "Signedness Inference",
    "Name Recovery",
    "Early Return Normalization",
    "Assertion Recovery",
    "DoWhile Recovery",
];
pub const CUSTOM_PATTERN_INDEX: usize = 6;
fn selected_to_ast_optimization_kind(data: &mut SelectOptimizationData) -> AstOptimizationKind {
    let selected = data.selected;
    let custom_pattern = if selected == CUSTOM_PATTERN_INDEX {
        if Path::new(&data.custom_path).is_file()
            && let Ok(content) = std::fs::read_to_string(&data.custom_path)
        {
            AstPattern::new(&data.custom_path, content)
        } else {
            AstPattern::new("", &data.custom)
        }
    } else {
        AstPattern::new("", "")
    };
    match selected {
        0 => AstOptimizationKind::IrAnalyzation,
        1 => AstOptimizationKind::ParameterAnalyzation,
        2 => AstOptimizationKind::CallArgumentAnalyzation,
        3 => AstOptimizationKind::ConstantFolding,
        4 => AstOptimizationKind::ControlFlowCleanup,
        5 => AstOptimizationKind::CollapseUnusedVariables,
        6 => AstOptimizationKind::PatternMatching(Box::new(custom_pattern)),
        7 => AstOptimizationKind::LoopAnalyzation,
        8 => AstOptimizationKind::CopyPropagation,
        9 => AstOptimizationKind::ExpressionInlining,
        10 => AstOptimizationKind::TernaryRecovery,
        11 => AstOptimizationKind::IfConversionReversal,
        12 => AstOptimizationKind::BooleanRecovery,
        13 => AstOptimizationKind::SwitchReconstruction,
        14 => AstOptimizationKind::OperatorCanonicalization,
        15 => AstOptimizationKind::CommonSubexpressionElimination,
        16 => AstOptimizationKind::BitTrickRecognition,
        17 => AstOptimizationKind::CastMinimization,
        18 => AstOptimizationKind::MagicDivisionRecovery,
        19 => AstOptimizationKind::GotoContainment,
        20 => AstOptimizationKind::InductionVariableAnalysis,
        21 => AstOptimizationKind::TemporaryElimination,
        22 => AstOptimizationKind::LifetimeScoping,
        23 => AstOptimizationKind::VariableCoalescing,
        24 => AstOptimizationKind::SignednessInference,
        25 => AstOptimizationKind::NameRecovery,
        26 => AstOptimizationKind::EarlyReturnNormalization,
        27 => AstOptimizationKind::AssertionRecovery,
        28 => AstOptimizationKind::DoWhileRecovery,
        _ => unreachable!(),
    }
}
fn refresh_list(data: &mut SelectOptimizationData) {
    let selected = data.selected;
    let list = widgets::List::new(
        OPTIMIZATION_KIND
            .iter()
            .enumerate()
            .map(|(i, kind)| format!("[{}] {}", if i == selected { "v" } else { " " }, kind))
            .collect::<Vec<_>>(),
    )
    .highlight_style(style::Style::new().fg(style::Color::Blue))
    .block(widgets::Block::bordered());
    data.list = list;
}
