pub mod x64;
pub use x64::{register::X64Register, statement::X64Statement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    /// aka. opcode
    pub statement: Result<Statement, DisassembleError>,
    /// aka. mnemnonic
    pub arguments: Vec<Argument>,
    /// original bytes
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Statement {
    X64(X64Statement),
}
pub trait StatementInner {
    fn is_jcc(&self) -> bool;
    fn is_call(&self) -> bool;
    fn is_ret(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Argument {
    Register(Register),
    Constant(u64),
    Memory(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    X64(X64Register),
}
pub trait RegisterInner {}

impl Instruction {
    /// From disassembled instruction, parse bytes (if bytes not exists)
    ///
    /// ### Returns
    /// - `Result<Vec<u8>>`: bytes, err if unknown statement
    pub fn get_bytes(&self) -> Result<Vec<u8>, DisassembleError> {
        unimplemented!()
    }
    pub fn is_jcc(&self) -> bool {
        self.statement.is_ok() && self.statement.unwrap().is_jcc()
    }
    pub fn is_call(&self) -> bool {
        self.statement.is_ok() && self.statement.unwrap().is_call()
    }
    pub fn is_ret(&self) -> bool {
        self.statement.is_ok() && self.statement.unwrap().is_ret()
    }
}

impl Statement {
    pub fn is_jcc(&self) -> bool {
        match self {
            Statement::X64(statement) => statement.is_jcc(),
        }
    }
    pub fn is_call(&self) -> bool {
        match self {
            Statement::X64(statement) => statement.is_call(),
        }
    }
    pub fn is_ret(&self) -> bool {
        match self {
            Statement::X64(statement) => statement.is_ret(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisassembleError {
    UnknownStatement,
}

unsafe impl Send for Instruction {}
unsafe impl Sync for Instruction {}

pub fn parse_statement(
    arch: Architecture,
    op: impl AsRef<str>,
) -> Result<Statement, DisassembleError> {
    match arch {
        Architecture::X64 => X64Statement::parse(op),
    }
}
