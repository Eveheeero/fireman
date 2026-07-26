use crate::tui::{
    TuiApp,
    tab::{
        SelectOptimizationData, TuiTab, handle_del_tab, handle_new_tab, handle_turn_tab,
        refresh_decompile,
    },
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
        clamp_cursor(data);
        let list = std::mem::take(&mut data.custom_list);
        terminal.render_stateful_widget(&list, area, &mut data.custom_y_cursor);
        data.custom_list = list;
        if data.focus == 2 {
            let y_cursor = data.custom_y_cursor.selected().unwrap_or(0);
            let offset = data.custom_y_cursor.offset();
            if y_cursor >= offset {
                let row = (y_cursor - offset) as u16;
                let column = display_width(&data.custom[y_cursor], data.custom_x_cursor) as u16;
                if row < area.height && column < area.width {
                    terminal.set_cursor_position((area.x + column, area.y + row));
                }
            }
        }
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
    let current_tab_index = app.data.tab.current_tab_index;
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    let TuiTab::SelectOptimization(data) = current_tab else {
        unreachable!()
    };
    let focus = data.focus;
    match focus {
        0 => {
            if handle_turn_tab(app, &event)
                || handle_new_tab(app, &event)
                || handle_del_tab(app, &event)
            {
                return;
            }

            let current_tab = &mut app.data.tab.tabs[current_tab_index];
            let TuiTab::SelectOptimization(data) = current_tab else {
                unreachable!()
            };
            let cursor = data.state.selected().unwrap();
            if let Some(event) = event.as_key_press_event() {
                match event.code {
                    event::KeyCode::Char(' ') => {
                        data.selected = cursor;
                        refresh_list(data);
                        refresh_decompile(app);
                    }
                    event::KeyCode::Enter => {
                        if cursor == CUSTOM_PATTERN_INDEX {
                            data.focus += 1;
                        }
                    }
                    event::KeyCode::Up => {
                        if cursor > 0 {
                            data.state.select(Some(cursor - 1));
                        }
                    }
                    event::KeyCode::PageUp => {
                        let mut cursor = cursor;
                        for _ in 0..3 {
                            if cursor > 0 {
                                cursor -= 1;
                            }
                        }
                        data.state.select(Some(0.min(cursor)));
                    }
                    event::KeyCode::Down => {
                        data.state
                            .select(Some(OPTIMIZATION_KIND.len().min(cursor + 1)));
                    }
                    event::KeyCode::PageDown => {
                        data.state
                            .select(Some(OPTIMIZATION_KIND.len().min(cursor + 3)));
                    }
                    _ => {}
                }
            }
        }
        1 => {
            if let Some(event) = event.as_key_press_event() {
                match event.code {
                    event::KeyCode::Char(c) => data.custom_path.push(c),
                    event::KeyCode::Backspace => {
                        data.custom_path.pop();
                    }
                    event::KeyCode::Enter => {
                        data.focus += 1;
                    }
                    event::KeyCode::Esc => {
                        data.focus -= 1;
                        refresh_decompile(app);
                    }
                    _ => {}
                }
            }
        }
        2 => {
            if let Some(event) = event.as_key_press_event() {
                handle_editor_event(data, event);
            }
        }
        _ => unreachable!(),
    }
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
pub fn selected_to_ast_optimization_kind(data: &mut SelectOptimizationData) -> AstOptimizationKind {
    let selected = data.selected;
    let custom_pattern = if selected == CUSTOM_PATTERN_INDEX {
        if Path::new(&data.custom_path).is_file() {
            AstPattern::from_file(&data.custom_path)
        } else {
            AstPattern::new("", &data.custom.join("\n"))
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
fn refresh_custom_list(data: &mut SelectOptimizationData) {
    if data.custom.is_empty() {
        data.custom.push(String::new());
    }
    data.custom_list = widgets::List::new(data.custom.clone())
        .highlight_style(style::Style::new().fg(style::Color::Blue));
}

/// Number of lines moved by page up / page down
const EDITOR_PAGE_STEP: usize = 10;

/// Keeps the y cursor inside the buffer and the x cursor inside the current line
fn clamp_cursor(data: &mut SelectOptimizationData) {
    if data.custom.is_empty() {
        data.custom.push(String::new());
    }
    let last_line = data.custom.len() - 1;
    let y = data.custom_y_cursor.selected().unwrap_or(0).min(last_line);
    data.custom_y_cursor.select(Some(y));
    let line_len = data.custom[y].chars().count();
    if data.custom_x_cursor > line_len {
        data.custom_x_cursor = line_len;
    }
}

/// Characters treated as word boundaries, see [crate::tui::navigate_input]
const SPECIALS: &str = " `~!@#$%^&*()+-=[]\\;',./{}|:\"<>?";

/// Returns the terminal column width of the first `char_count` characters of the line.
///
/// East asian wide / fullwidth characters occupy two columns, combining marks occupy none.
fn display_width(line: &str, char_count: usize) -> usize {
    line.chars().take(char_count).map(char_width).sum()
}

/// Returns the terminal column width of a single character
fn char_width(c: char) -> usize {
    let code = c as u32;
    let is_zero_width = matches!(
        code,
        0x0300..=0x036F
            | 0x200B..=0x200F
            | 0x1AB0..=0x1AFF
            | 0x20D0..=0x20F0
            | 0xFE00..=0xFE0F
            | 0xFE20..=0xFE2F
    );
    if is_zero_width {
        return 0;
    }
    let is_wide = matches!(
        code,
        0x1100..=0x115F
            | 0x2E80..=0x303E
            | 0x3041..=0x33FF
            | 0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xA000..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x1F300..=0x1F64F
            | 0x1F900..=0x1F9FF
            | 0x20000..=0x3FFFD
    );
    if is_wide { 2 } else { 1 }
}

/// Splits the line at the given character index
fn split_at_char(line: &str, char_index: usize) -> (String, String) {
    let before: String = line.chars().take(char_index).collect();
    let after: String = line.chars().skip(char_index).collect();
    (before, after)
}

/// Inserts the text at the given character index of the line
fn insert_str(line: &mut String, char_index: usize, text: &str) {
    let (before, after) = split_at_char(line, char_index);
    *line = before;
    line.push_str(text);
    line.push_str(&after);
}

/// Removes a single character at the given character index of the line
fn remove_char(line: &mut String, char_index: usize) {
    let before: String = line.chars().take(char_index).collect();
    let after: String = line.chars().skip(char_index + 1).collect();
    *line = before;
    line.push_str(&after);
}

/// Moves the x cursor to the previous word boundary of the current line
fn move_word_left(line: &str, mut x: usize) -> usize {
    let mut is_first = true;
    while x > 0 {
        let next = line.chars().nth(x - 1).unwrap();
        let next_is_special = SPECIALS.contains(next);
        if !is_first && next_is_special {
            break;
        }
        x -= 1;
        is_first = false;
    }
    x
}

/// Moves the x cursor to the next word boundary of the current line
fn move_word_right(line: &str, mut x: usize) -> usize {
    let line_len = line.chars().count();
    let mut is_first = true;
    while x < line_len {
        let next = line.chars().nth(x).unwrap();
        let next_is_special = SPECIALS.contains(next);
        if !is_first && next_is_special {
            break;
        }
        x += 1;
        is_first = false;
    }
    x
}

fn handle_editor_event(data: &mut SelectOptimizationData, event: event::KeyEvent) {
    clamp_cursor(data);
    let y = data.custom_y_cursor.selected().unwrap_or(0);
    let x = data.custom_x_cursor;
    let is_control = event.modifiers.contains(event::KeyModifiers::CONTROL);
    match event.code {
        event::KeyCode::Char(c) => {
            insert_str(&mut data.custom[y], x, c.encode_utf8(&mut [0u8; 4]));
            data.custom_x_cursor = x + 1;
            refresh_custom_list(data);
        }
        event::KeyCode::Tab => {
            insert_str(&mut data.custom[y], x, "    ");
            data.custom_x_cursor = x + 4;
            refresh_custom_list(data);
        }
        event::KeyCode::Enter => {
            let (before, rest) = split_at_char(&data.custom[y], x);
            data.custom[y] = before;
            data.custom.insert(y + 1, rest);
            data.custom_y_cursor.select(Some(y + 1));
            data.custom_x_cursor = 0;
            refresh_custom_list(data);
        }
        event::KeyCode::Backspace => {
            if x > 0 {
                remove_char(&mut data.custom[y], x - 1);
                data.custom_x_cursor = x - 1;
            } else if y > 0 {
                let line = data.custom.remove(y);
                let previous_len = data.custom[y - 1].chars().count();
                data.custom[y - 1].push_str(&line);
                data.custom_y_cursor.select(Some(y - 1));
                data.custom_x_cursor = previous_len;
            }
            refresh_custom_list(data);
        }
        event::KeyCode::Delete => {
            let line_len = data.custom[y].chars().count();
            if x < line_len {
                remove_char(&mut data.custom[y], x);
            } else if y + 1 < data.custom.len() {
                let line = data.custom.remove(y + 1);
                data.custom[y].push_str(&line);
            }
            refresh_custom_list(data);
        }
        event::KeyCode::Left => {
            if is_control {
                data.custom_x_cursor = move_word_left(&data.custom[y], x);
            } else if x > 0 {
                data.custom_x_cursor = x - 1;
            } else if y > 0 {
                data.custom_y_cursor.select(Some(y - 1));
                data.custom_x_cursor = data.custom[y - 1].chars().count();
            }
        }
        event::KeyCode::Right => {
            let line_len = data.custom[y].chars().count();
            if is_control {
                data.custom_x_cursor = move_word_right(&data.custom[y], x);
            } else if x < line_len {
                data.custom_x_cursor = x + 1;
            } else if y + 1 < data.custom.len() {
                data.custom_y_cursor.select(Some(y + 1));
                data.custom_x_cursor = 0;
            }
        }
        event::KeyCode::Up => {
            if y > 0 {
                data.custom_y_cursor.select(Some(y - 1));
            }
        }
        event::KeyCode::Down => {
            if y + 1 < data.custom.len() {
                data.custom_y_cursor.select(Some(y + 1));
            }
        }
        event::KeyCode::PageUp => {
            data.custom_y_cursor
                .select(Some(y.saturating_sub(EDITOR_PAGE_STEP)));
        }
        event::KeyCode::PageDown => {
            let last_line = data.custom.len() - 1;
            data.custom_y_cursor
                .select(Some(last_line.min(y + EDITOR_PAGE_STEP)));
        }
        event::KeyCode::Home => {
            data.custom_x_cursor = 0;
        }
        event::KeyCode::End => {
            data.custom_x_cursor = data.custom[y].chars().count();
        }
        event::KeyCode::Esc => {
            data.focus -= 1;
        }
        _ => {}
    }
    clamp_cursor(data);
}
