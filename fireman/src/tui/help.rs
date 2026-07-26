use crate::tui::{TuiApp, TuiState, tab::TuiTab};
use crossterm::event;
use ratatui::{Frame, widgets};

pub fn draw(app: &mut TuiApp, terminal: &mut Frame) {
    let TuiState::Help { previous_state } = &app.state else {
        unreachable!()
    };
    let helps: Vec<&str> = match previous_state.as_ref() {
        TuiState::TempState => Vec::new(),
        TuiState::Init => Vec::new(),
        TuiState::NavigateInput => [
            "up/down/pu/pd: navigate",
            "tab: autocomplete",
            "enter: select program",
        ]
        .into(),
        TuiState::Tab(current_tab) => match app.data.tab.tabs[*current_tab] {
            TuiTab::SelectTargetBlock(..) => [
                "tab/shift+tab/n/shift+n: navigate tabs",
                "o: new optimization tab",
                "d: new display ast tab",
                "D: delete current tab",
                "up/down/pu/pd: navigate",
                "0~9,x: enter address",
                "a,enter: analyze current block",
                "analyze with no input, analyze entry",
                "space: select block",
                "ctrl+a: select all block",
                "shift+a: analyze all block",
            ]
            .into(),
            TuiTab::SelectOptimization(..) => [
                "tab/shift+tab/n/shift+n: navigate tabs (when focused on list)",
                "o: new optimization tab (when focused on list)",
                "d: new display ast tab (when focused on list)",
                "D: delete current tab (when focused on list)",
                "up/down/pu/pd: navigate list",
                "space: select optimization",
                "enter: focus to custom path or custom script input",
                "esc: focus to list or custom path",
            ]
            .into(),
            TuiTab::DisplayCurrentAST(..) => [].into(),
        },
        TuiState::Help { .. } => Vec::new(),
        TuiState::Exit => Vec::new(),
    };
    let helps = widgets::List::new(helps).block(widgets::Block::bordered());
    terminal.render_widget(helps, terminal.area());
}
pub fn handle_event(_app: &mut TuiApp, _event: event::Event) {}
