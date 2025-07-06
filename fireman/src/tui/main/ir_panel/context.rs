use fireball::{
    core::{Address, Instruction},
    ir::Ir,
};
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
    pub instructions: Arc<[Instruction]>,
    pub origin: Vec<Ir>,
    pub displayed: Arc<Mutex<Vec<String>>>,
}

impl Data {
    pub fn new(instructions: Arc<[Instruction]>, origin: Vec<Ir>) -> Self {
        let mut displayed = Vec::new();
        let mut now;
        for (i, ir) in origin.iter().enumerate() {
            let instruction = &instructions[i];
            now = "// ".to_string();
            now.push_str(&instruction.to_string());
            displayed.push(now);
            if let Some(statements) = ir.statements {
                for statement in statements {
                    let mut now = "  ".to_string();
                    now.push_str(&statement.to_string());
                    displayed.push(now);
                }
            }
        }
        Data {
            instructions,
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
            list_cursor: None,
        }
    }
}
