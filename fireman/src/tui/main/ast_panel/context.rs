use fireball::core::Address;
use std::collections::{HashMap, HashSet};

pub struct Context {
    pub data: HashMap<HashSet<Address>, Data>,
    pub list: Vec<String>,
    pub list_cursor: Option<usize>,
}

pub struct Data {}

impl Context {
    pub fn new() -> Self {
        Context {
            data: HashMap::new(),
            list: Vec::new(),
            list_cursor: None,
        }
    }
}
