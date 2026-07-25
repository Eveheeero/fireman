use crate::tui::{
    TuiApp,
    tab::{SelectTargetSectionData, TuiTab},
};
use crossterm::event;
use ratatui::{Frame, prelude::*};

pub fn draw(_data: &mut SelectTargetSectionData, _area: Rect, _terminal: &mut Frame) {}
pub fn handle_event(app: &mut TuiApp, _event: event::Event) {
    let current_tab_index = app.data.tab.current_tab_index;
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    let TuiTab::SelectTargetSection(_data) = current_tab else {
        panic!()
    };
}
