use crate::tui::{
    TuiApp,
    tab::{SelectOptimizationData, TuiTab, handle_del_tab, handle_new_tab, handle_turn_tab},
};
use crossterm::event;
use ratatui::{Frame, prelude::*};

pub fn draw(_data: &mut SelectOptimizationData, _area: Rect, _terminal: &mut Frame) {}
pub fn handle_event(app: &mut TuiApp, event: event::Event) {
    if handle_turn_tab(app, &event) || handle_new_tab(app, &event) || handle_del_tab(app, &event) {
        return;
    }

    let current_tab_index = app.data.tab.current_tab_index;
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    let TuiTab::SelectOptimization(_data) = current_tab else {
        panic!()
    };
}
