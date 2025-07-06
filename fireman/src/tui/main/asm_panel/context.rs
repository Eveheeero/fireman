use fireball::core::{Address, Instruction};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Context {
    pub data: HashMap<Address, Data>,
    pub list: Arc<Mutex<Vec<String>>>,
    pub list_cursor: Option<usize>,
}

pub struct Data {
    pub _origin: Arc<[Instruction]>,
    pub displayed: Arc<Mutex<Vec<String>>>,
}

impl Data {
    pub fn new(origin: Arc<[Instruction]>) -> Self {
        let displayed = origin.iter().map(|x| x.to_string()).collect();
        Data {
            _origin: origin,
            displayed: Arc::new(Mutex::new(displayed)),
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            data: HashMap::new(),
            list: Arc::new(Mutex::new(Vec::new())),
            list_cursor: None,
        }
    }
}
