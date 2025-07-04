use fireball::core::Address;
use ratatui::widgets;
use std::collections::{HashMap, HashSet};

pub struct Context {
    pub data: HashMap<HashSet<Address>, Data>,
    pub list: Vec<String>,
    pub list_selected: widgets::ListState,
}

pub struct Data {}

impl Context {
    pub fn new() -> Self {
        Context {
            data: HashMap::new(),
            list: Vec::new(),
            list_selected: widgets::ListState::default(),
        }
    }
}
