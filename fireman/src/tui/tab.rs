use crate::tui::TuiApp;
use crossterm::event;
use ratatui::Frame;

#[derive(Default)]
pub struct TuiTabData {
    tabs: Vec<TuiTab>,
}
struct TuiTab {
    index: usize,
    kind: TuiTabKind,
}
enum TuiTabKind {}

pub fn draw(_app: &mut TuiApp, _terminal: &mut Frame) {
    todo!()
}
pub fn handle_event(_app: &mut TuiApp, _event: event::Event) {
    todo!()
}
