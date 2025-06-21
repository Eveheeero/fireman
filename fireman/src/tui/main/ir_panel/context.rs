use ratatui::widgets;

pub struct Context {
    pub list: Vec<String>,
    pub list_selected: widgets::ListState,
}

impl Context {
    pub fn new() -> Self {
        Context {
            list: Vec::new(),
            list_selected: widgets::ListState::default(),
        }
    }
}
