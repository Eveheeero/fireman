mod display_current_ast;
mod select_optimization;
mod select_target_section;

use crate::tui::TuiApp;
use crossterm::event;
use ratatui::{Frame, widgets};

#[derive(Default)]
pub struct TuiTabData<'tui> {
    init: bool,
    tabs: Vec<TuiTab>,
    tab_widget: widgets::Tabs<'tui>,
    current_tab_index: usize,
}
enum TuiTab {
    SelectTargetSection(Box<SelectTargetSectionData>), // tabs[0]
    SelectOptimization(Box<SelectOptimizationData>),
    DisplayCurrentAST(Box<DisplayCurrentASTData>),
}

struct SelectTargetSectionData {
    input: String,
    sections: Vec<SelectTargetSectionDataSection>,
}
struct SelectTargetSectionDataSection {
    start_address: usize,
    end_address: Option<usize>,
    analyzed: bool,
    selected: bool,
}
struct SelectOptimizationData {}
struct DisplayCurrentASTData {}

pub fn draw(app: &mut TuiApp, terminal: &mut Frame) {
    init(app);
    let current_tab_index = app.data.tab.current_tab_index;
    let mut area = terminal.area();
    let mut tab_widget_area = area;
    let tab_widget = std::mem::take(&mut app.data.tab.tab_widget);
    app.data.tab.tab_widget = tab_widget.select(Some(current_tab_index));
    tab_widget_area.height = 1;
    area.y += 1;
    area.height -= 1;
    terminal.render_widget(&app.data.tab.tab_widget, tab_widget_area);
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    match current_tab {
        TuiTab::SelectTargetSection(data) => select_target_section::draw(data, area, terminal),
        TuiTab::SelectOptimization(data) => select_optimization::draw(data, area, terminal),
        TuiTab::DisplayCurrentAST(data) => display_current_ast::draw(data, area, terminal),
    }
}
pub fn handle_event(app: &mut TuiApp, event: event::Event) {
    let current_tab_index = app.data.tab.current_tab_index;
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    match current_tab {
        TuiTab::SelectTargetSection(_) => select_target_section::handle_event(app, event),
        TuiTab::SelectOptimization(_) => select_optimization::handle_event(app, event),
        TuiTab::DisplayCurrentAST(_) => display_current_ast::handle_event(app, event),
    }
}

fn init(app: &mut TuiApp) {
    if app.data.tab.init {
        return;
    }
    app.data.tab.init = true;

    let data = SelectTargetSectionData {
        input: String::new(),
        sections: Vec::new(),
    };

    app.data
        .tab
        .tabs
        .push(TuiTab::SelectTargetSection(Box::new(data)));
    refresh_tab_widget(app);
}

fn refresh_tab_widget(app: &mut TuiApp) {
    let tabs = app
        .data
        .tab
        .tabs
        .iter()
        .map(|tab| match tab {
            TuiTab::SelectTargetSection(_) => "S",
            TuiTab::SelectOptimization(_) => "O",
            TuiTab::DisplayCurrentAST(_) => "D",
        })
        .collect::<Vec<_>>();
    let tabs = widgets::Tabs::new(tabs);
    app.data.tab.tab_widget = tabs;
}
