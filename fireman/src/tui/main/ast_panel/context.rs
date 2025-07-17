use fireball::{core::Address, ir::analyze::ir_to_ast::abstract_syntax_tree::Ast};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

pub struct Context {
    pub data: HashMap<Key, Data>,
    pub list: Arc<Mutex<Vec<String>>>,
    pub list_cursor: Option<usize>,
}
#[derive(Eq, PartialEq)]
pub struct Key(pub HashSet<Address>);

pub struct Data {
    pub _origin: Ast,
    pub displayed: Arc<Mutex<Vec<String>>>,
}

impl Data {
    pub fn new(origin: Ast) -> Self {
        let displayed = origin
            .print(None)
            .trim()
            .lines()
            .map(|x| x.to_owned())
            .collect();
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

impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for address in &self.0 {
            address.hash(state);
        }
    }
}
