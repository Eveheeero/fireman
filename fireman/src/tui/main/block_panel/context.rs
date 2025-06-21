use ratatui::widgets;

pub struct Context {
    pub list: Vec<String>,
    pub list_selected: widgets::ListState,
    pub input: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            list: Vec::new(),
            list_selected: widgets::ListState::default(),
            input: String::new(),
        }
    }
}
