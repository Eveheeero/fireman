pub mod x64;
pub use x64::{register::X64Register, statement::X64Statement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Architecture {
    X64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Instruction {
    /// aka. opcode
    pub statement: Result<Statement, DisassembleError>,
    /// aka. mnemnonic
    pub arguments: Box<[Argument]>,
    /// original bytes
    pub bytes: Option<Box<[u8]>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    X64(X64Statement),
}
pub trait StatementInner {
    fn is_jcc(&self) -> bool;
    fn is_jmp(&self) -> bool;
    fn is_call(&self) -> bool;
    fn is_ret(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Argument {
    Register(Register),
    Constant(u64),
    Memory(Memory),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Memory {
    AbsoluteAddressing(u64),
    RelativeAddressing(Box<[RelativeAddressingArgument]>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RelativeAddressingArgument {
    Register(Register),
    Constant(i128),
    Operator(AddressingOperator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AddressingOperator {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Register {
    X64(X64Register),
}
pub trait RegisterInner {}

impl Instruction {
    /// From disassembled instruction, parse bytes (if bytes not exists)
    ///
    /// ### Returns
    /// - `Result<&[u8], DisassembleError>`: bytes, err if unknown statement
    pub fn get_bytes(&self) -> Result<&[u8], DisassembleError> {
        self.bytes.as_deref().ok_or(DisassembleError::Unknown) // TODO Instruction to bytes not implemented
    }
    pub fn is_jcc(&self) -> bool {
        self.statement.is_ok() && self.statement.unwrap().is_jcc()
    }
    pub fn is_jmp(&self) -> bool {
        self.statement.is_ok() && self.statement.unwrap().is_jmp()
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
    pub fn is_jmp(&self) -> bool {
        match self {
            Statement::X64(statement) => statement.is_jmp(),
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DisassembleError {
    Unknown,
    UnknownStatement,
    UnknownRegister,
}

unsafe impl Send for Instruction {}
unsafe impl Sync for Instruction {}

pub fn parse_statement(
    arch: Architecture,
    mnemonic: impl AsRef<str>,
) -> Result<Statement, DisassembleError> {
    match arch {
        Architecture::X64 => X64Statement::parse(mnemonic),
    }
}
pub fn parse_argument(
    arch: Architecture,
    op: impl AsRef<str>,
) -> Result<Argument, DisassembleError> {
    match arch {
        Architecture::X64 => x64::parse_argument(op),
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(statement) = self.statement {
            write!(f, "{}", statement)?;
            for arg in self.arguments.iter() {
                write!(f, " {}", arg)?;
            }
            Ok(())
        } else {
            let bytes = self.bytes.as_ref().unwrap();
            for byte in bytes {
                write!(f, "{:02X}", byte)?;
            }
            Ok(())
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::X64(statement) => write!(f, "{}", statement),
        }
    }
}
impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Register(register) => write!(f, "{}", register),
            Argument::Constant(constant) => write!(f, "{:X}", constant),
            Argument::Memory(memory) => write!(f, "{}", memory),
        }
    }
}
impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::X64(register) => write!(f, "{}", register),
        }
    }
}
impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Memory::AbsoluteAddressing(address) => write!(f, "{}", address),
            Memory::RelativeAddressing(arguments) => {
                write!(f, "[")?;
                for arg in arguments.iter() {
                    write!(f, "{}", arg)?;
                }
                write!(f, "]")
            }
        }
    }
}
impl std::fmt::Display for RelativeAddressingArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeAddressingArgument::Register(register) => write!(f, "{}", register),
            RelativeAddressingArgument::Constant(constant) => write!(f, "{:X}", constant),
            RelativeAddressingArgument::Operator(operator) => write!(f, "{}", operator),
        }
    }
}
impl std::fmt::Display for AddressingOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressingOperator::Add => write!(f, "+"),
            AddressingOperator::Sub => write!(f, "-"),
            AddressingOperator::Mul => write!(f, "*"),
        }
    }
}
