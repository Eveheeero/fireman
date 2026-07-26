mod display_current_ast;
mod select_optimization;
mod select_target_block;

use crate::tui::TuiApp;
use crossterm::event;
use fireball::{
    abstract_syntax_tree::Ast,
    core::{Address, FireRaw},
};
use ratatui::{Frame, style, widgets};

#[derive(Default)]
pub struct TuiTabData<'tui> {
    init: bool,
    pub tabs: Vec<TuiTab<'tui>>,
    ast_and_tab_index: Vec<(Ast, usize)>,
    tab_widget: widgets::Tabs<'tui>,
    current_tab_index: usize,
}
#[allow(private_interfaces)]
pub enum TuiTab<'tui> {
    SelectTargetBlock(Box<SelectTargetBlockData<'tui>>), // tabs[0]
    SelectOptimization(Box<SelectOptimizationData<'tui>>),
    DisplayCurrentAST(Box<DisplayCurrentASTData<'tui>>),
}

struct SelectTargetBlockData<'tui> {
    input: String,
    blocks: Vec<SelectTargetBlockDataBlock>,
    blocks_list: widgets::List<'tui>,
    state: widgets::ListState,
}
struct SelectTargetBlockDataBlock {
    start_address: u64,
    end_address: Option<u64>,
    analyzed: bool,
    selected: bool,
}
struct SelectOptimizationData<'tui> {
    selected: usize,
    list: widgets::List<'tui>,
    state: widgets::ListState,
    custom_path: String,
    custom: Vec<String>,
    custom_list: widgets::List<'tui>,
    custom_x_cursor: usize,
    custom_y_cursor: widgets::ListState,
    focus: u8, // 0-list, 1-path, 2-buf
}
struct DisplayCurrentASTData<'tui> {
    list: widgets::List<'tui>,
    state: widgets::ListState,
}

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
        TuiTab::SelectTargetBlock(data) => select_target_block::draw(data, area, terminal),
        TuiTab::SelectOptimization(data) => select_optimization::draw(data, area, terminal),
        TuiTab::DisplayCurrentAST(data) => display_current_ast::draw(data, area, terminal),
    }
}
pub fn handle_event(app: &mut TuiApp, event: event::Event) {
    let current_tab_index = app.data.tab.current_tab_index;
    let current_tab = &mut app.data.tab.tabs[current_tab_index];
    match current_tab {
        TuiTab::SelectTargetBlock(_) => select_target_block::handle_event(app, event),
        TuiTab::SelectOptimization(_) => select_optimization::handle_event(app, event),
        TuiTab::DisplayCurrentAST(_) => display_current_ast::handle_event(app, event),
    }
}

fn init(app: &mut TuiApp) {
    if app.data.tab.init {
        return;
    }
    app.data.tab.init = true;

    let data = SelectTargetBlockData {
        input: String::new(),
        blocks: Vec::new(),
        blocks_list: widgets::List::default(),
        state: widgets::ListState::default(),
    };

    app.data
        .tab
        .tabs
        .push(TuiTab::SelectTargetBlock(Box::new(data)));
    refresh_tab_widget(app);
    refresh_decompile(app);
}

fn refresh_tab_widget(app: &mut TuiApp) {
    let tabs = app
        .data
        .tab
        .tabs
        .iter()
        .map(|tab| match tab {
            TuiTab::SelectTargetBlock(_) => "S",
            TuiTab::SelectOptimization(_) => "O",
            TuiTab::DisplayCurrentAST(_) => "D",
        })
        .collect::<Vec<_>>();
    let tabs = widgets::Tabs::new(tabs);
    app.data.tab.tab_widget = tabs;
}
/// decompile sequence from current tab
fn refresh_decompile(app: &mut TuiApp) {
    let data = &mut app.data.tab;
    let fireball = app.fireball.as_ref().unwrap();
    data.ast_and_tab_index
        .retain(|(_, tab_index)| *tab_index < data.current_tab_index);

    for current_tab in data.current_tab_index..data.tabs.len() {
        match &mut data.tabs[current_tab] {
            TuiTab::SelectTargetBlock(dat) => {
                let blocks = &dat.blocks;
                let selected_blocks = blocks
                    .iter()
                    .filter(|block| block.selected)
                    .collect::<Vec<_>>();
                let sections = fireball.get_sections();
                let blocks = fireball.get_blocks();
                let mut v = Vec::new();
                for selected_block in selected_blocks {
                    let address =
                        Address::from_virtual_address(&sections, selected_block.start_address);
                    v.push(blocks.get_by_start_address(&address).unwrap());
                }
                let ast = fireball::ir::analyze::generate_ast_with_pre_defined_symbols(
                    v,
                    fireball.get_defined(),
                )
                .unwrap();
                data.ast_and_tab_index.push((ast, current_tab));
            }
            TuiTab::SelectOptimization(dat) => {
                let opt = select_optimization::selected_to_ast_optimization_kind(dat);
                let previous = data.ast_and_tab_index.last().unwrap();
                let ast = previous.0.optimize(Some(opt.into()));
                if let Ok(ast) = ast {
                    data.ast_and_tab_index.push((ast, current_tab));
                }
            }
            TuiTab::DisplayCurrentAST(dat) => {
                let ast = &data.ast_and_tab_index.last().unwrap().0;
                let ast = ast.print(Some(app.print_config));
                dat.list = widgets::List::new(ast.split("\n").map(|x| x.to_string()))
                    .highlight_style(style::Style::new().fg(style::Color::Blue))
                    .block(widgets::Block::bordered());
            }
        }
    }
}

/// handles tab, n
///
/// ### Returns
/// bool -> true if handled
fn handle_turn_tab(app: &mut TuiApp, event: &event::Event) -> bool {
    fn next(app: &mut TuiApp) {
        if app.data.tab.current_tab_index < app.data.tab.tabs.len() - 1 {
            app.data.tab.current_tab_index += 1;
        } else {
            app.data.tab.current_tab_index = 0;
        }
    }
    fn previous(app: &mut TuiApp) {
        if app.data.tab.current_tab_index > 0 {
            app.data.tab.current_tab_index -= 1;
        } else {
            app.data.tab.current_tab_index = app.data.tab.tabs.len() - 1;
        }
    }
    // handle tab, shift tab, n, shift n
    if let Some(event) = event.as_key_press_event() {
        return match event.code {
            event::KeyCode::Tab if event.modifiers == event::KeyModifiers::SHIFT => {
                previous(app);
                true
            }
            event::KeyCode::Tab => {
                next(app);
                true
            }
            event::KeyCode::Char('n') | event::KeyCode::Char('N')
                if event.modifiers == event::KeyModifiers::SHIFT =>
            {
                previous(app);
                true
            }
            event::KeyCode::Char('n') | event::KeyCode::Char('N') => {
                next(app);
                true
            }
            _ => false,
        };
    }
    false
}

/// handles d, o
///
/// ### Returns
/// bool -> true if handled
fn handle_new_tab(app: &mut TuiApp, event: &event::Event) -> bool {
    // handle o, d
    let Some(event) = event.as_key_press_event() else {
        return false;
    };
    match event.code {
        event::KeyCode::Char('d') => {
            app.data.tab.current_tab_index += 1;
            app.data.tab.tabs.insert(
                app.data.tab.current_tab_index,
                TuiTab::DisplayCurrentAST(Box::new(DisplayCurrentASTData {
                    list: Default::default(),
                    state: Default::default(),
                })),
            );
            refresh_tab_widget(app);
            refresh_decompile(app);
            true
        }
        event::KeyCode::Char('o') => {
            app.data.tab.current_tab_index += 1;
            app.data.tab.tabs.insert(
                app.data.tab.current_tab_index,
                TuiTab::SelectOptimization(Box::new(SelectOptimizationData {
                    selected: select_optimization::CUSTOM_PATTERN_INDEX,
                    list: widgets::List::new(
                        select_optimization::OPTIMIZATION_KIND
                            .iter()
                            .enumerate()
                            .map(|(i, kind)| {
                                format!(
                                    "[{}] {}",
                                    if i == select_optimization::CUSTOM_PATTERN_INDEX {
                                        "v"
                                    } else {
                                        " "
                                    },
                                    kind
                                )
                            })
                            .collect::<Vec<_>>(),
                    )
                    .highlight_style(style::Style::new().fg(style::Color::Blue))
                    .block(widgets::Block::bordered()),
                    state: widgets::ListState::default()
                        .with_selected(Some(select_optimization::CUSTOM_PATTERN_INDEX)),
                    custom_path: "".to_string(),
                    custom: ["".to_string()].into(),
                    custom_list: widgets::List::new([""]),
                    custom_x_cursor: 0,
                    custom_y_cursor: widgets::ListState::default().with_selected(Some(0)),
                    focus: 0,
                })),
            );
            refresh_tab_widget(app);
            refresh_decompile(app);
            true
        }
        _ => false,
    }
}

/// handles D
///
/// ### Returns
/// bool -> true if handled
fn handle_del_tab(app: &mut TuiApp, event: &event::Event) -> bool {
    // handle D
    if let Some(event) = event.as_key_press_event()
        && event.code == event::KeyCode::Char('D')
    {
        let current_tab_index = app.data.tab.current_tab_index;
        if current_tab_index == 0 {
            return false;
        }
        let removed = app.data.tab.tabs.remove(current_tab_index);
        if current_tab_index == app.data.tab.tabs.len() {
            app.data.tab.current_tab_index -= 1;
        }
        refresh_tab_widget(app);
        if matches!(removed, TuiTab::DisplayCurrentAST(_)) {
            // display tab does not affect decompile sequence, only shift indices
            for (_, tab_index) in app.data.tab.ast_and_tab_index.iter_mut() {
                if *tab_index > current_tab_index {
                    *tab_index -= 1;
                }
            }
        } else {
            refresh_decompile(app);
        }
        true
    } else {
        false
    }
}
