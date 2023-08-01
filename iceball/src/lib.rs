pub mod x64;
pub use x64::{register::X64Register, statement::X64Statement};

#[derive(Debug, Clone)]
pub struct Instruction {
    /// aka. opcode
    pub statement: Statement,
    /// aka. mnemnonic
    pub arguments: Option<Arguments>,
}

#[derive(Debug, Clone, Copy)]
pub enum Statement {
    X64(X64Statement),
}

#[derive(Debug, Clone, Copy)]
pub enum Arguments {
    Register(Register),
    Constant(u64),
    Memory(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    X64(X64Register),
}
