pub struct Context {
    pub list: Vec<String>,
    pub list_cursor: Option<usize>,
    pub list_selected: Vec<usize>,
    pub input: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            list: Vec::new(),
            list_cursor: None,
            list_selected: Vec::new(),
            input: String::new(),
        }
    }
}
