use crate::tui::{
    TuiApp,
    tab::{DisplayCurrentASTData, TuiTab, handle_del_tab, handle_new_tab, handle_turn_tab},
};
use crossterm::event;
use ratatui::{Frame, prelude::*};

pub fn draw(data: &mut DisplayCurrentASTData, area: Rect, terminal: &mut Frame) {
    clamp_cursor(data);
    terminal.render_stateful_widget(&data.list, area, &mut data.state);
}
pub fn handle_event(app: &mut TuiApp, event: event::Event) {
    if handle_turn_tab(app, &event) || handle_new_tab(app, &event) || handle_del_tab(app, &event) {
        return;
    }

    let current_tab_index = app.data.tab.current_tab_index;
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    let TuiTab::DisplayCurrentAST(data) = current_tab else {
        unreachable!()
    };
    let Some(event) = event.as_key_press_event() else {
        return;
    };
    let last_line = data.list.len().saturating_sub(1);
    let cursor = data.state.selected().unwrap_or(0);
    match event.code {
        event::KeyCode::Up => {
            data.state.select(Some(cursor.saturating_sub(1)));
        }
        event::KeyCode::Down => {
            data.state.select(Some(last_line.min(cursor + 1)));
        }
        event::KeyCode::PageUp => {
            data.state.select(Some(cursor.saturating_sub(5)));
        }
        event::KeyCode::PageDown => {
            data.state.select(Some(last_line.min(cursor + 5)));
        }
        event::KeyCode::Home => {
            data.state.select(Some(0));
        }
        event::KeyCode::End => {
            data.state.select(Some(last_line));
        }
        _ => {}
    }
    clamp_cursor(data);
}

/// Keeps the cursor inside the list
fn clamp_cursor(data: &mut DisplayCurrentASTData) {
    if data.list.is_empty() {
        data.state.select(None);
        return;
    }
    let last_line = data.list.len() - 1;
    let cursor = data.state.selected().unwrap_or(0).min(last_line);
    data.state.select(Some(cursor));
}
