use fireball::core::{Address, Instruction};
use ratatui::widgets;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Context {
    pub data: HashMap<Address, Data>,
    pub list: Arc<Mutex<Vec<String>>>,
    pub list_selected: widgets::ListState,
}

pub struct Data {
    pub origin: Arc<[Instruction]>,
    pub displayed: Arc<Mutex<Vec<String>>>,
}

impl Data {
    pub fn new(origin: Arc<[Instruction]>) -> Self {
        let displayed = origin.iter().map(|x| x.to_string()).collect();
        Data {
            origin,
            displayed: Arc::new(Mutex::new(displayed)),
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            data: HashMap::new(),
            list: Arc::new(Mutex::new(Vec::new())),
            list_selected: widgets::ListState::default(),
        }
    }
}
