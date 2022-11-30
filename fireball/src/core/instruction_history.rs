use std::sync::RwLock;

use super::Instruction;

#[derive(Debug, Default)]
pub struct InstructionHistory {
    pub(crate) data: Vec<Instruction>,
}
