pub mod arm32;
pub mod arm64;
pub mod x64;
pub mod x86;

pub use arm32::{register::Arm32Register, statement::Arm32Statement};
pub use arm64::{register::Arm64Register, statement::Arm64Statement};
pub use x64::{register::X64Register, statement::X64Statement};
pub use x86::{register::X86Register, statement::X86Statement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Architecture {
    X64,
    X86,
    Arm32,
    Arm64,
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
    X86(X86Statement),
    Arm32(Arm32Statement),
    Arm64(Arm64Statement),
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
pub struct Memory {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub scale: u8,
    pub displacement: i64,
    pub size: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Register {
    X64(X64Register),
    X86(X86Register),
    Arm32(Arm32Register),
    Arm64(Arm64Register),
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
            Statement::X86(statement) => statement.is_jcc(),
            Statement::Arm32(statement) => statement.is_jcc(),
            Statement::Arm64(statement) => statement.is_jcc(),
        }
    }
    pub fn is_jmp(&self) -> bool {
        match self {
            Statement::X64(statement) => statement.is_jmp(),
            Statement::X86(statement) => statement.is_jmp(),
            Statement::Arm32(statement) => statement.is_jmp(),
            Statement::Arm64(statement) => statement.is_jmp(),
        }
    }
    pub fn is_call(&self) -> bool {
        match self {
            Statement::X64(statement) => statement.is_call(),
            Statement::X86(statement) => statement.is_call(),
            Statement::Arm32(statement) => statement.is_call(),
            Statement::Arm64(statement) => statement.is_call(),
        }
    }
    pub fn is_ret(&self) -> bool {
        match self {
            Statement::X64(statement) => statement.is_ret(),
            Statement::X86(statement) => statement.is_ret(),
            Statement::Arm32(statement) => statement.is_ret(),
            Statement::Arm64(statement) => statement.is_ret(),
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
    let mnemonic = mnemonic.as_ref();
    match arch {
        Architecture::X64 => X64Statement::parse(mnemonic).map(Statement::X64),
        Architecture::X86 => X86Statement::parse(mnemonic).map(Statement::X86),
        Architecture::Arm32 => Arm32Statement::parse(mnemonic).map(Statement::Arm32),
        Architecture::Arm64 => Arm64Statement::parse(mnemonic).map(Statement::Arm64),
    }
}
pub fn parse_argument(
    arch: Architecture,
    op: impl AsRef<str>,
) -> Result<Argument, DisassembleError> {
    match arch {
        Architecture::X64 => x64::parse_argument(op),
        Architecture::X86 => x86::parse_argument(op),
        Architecture::Arm32 => arm32::parse_argument(op),
        Architecture::Arm64 => arm64::parse_argument(op),
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
            Statement::X86(statement) => write!(f, "{}", statement),
            Statement::Arm32(statement) => write!(f, "{}", statement),
            Statement::Arm64(statement) => write!(f, "{}", statement),
        }
    }
}
impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Register(register) => write!(f, "{}", register),
            Argument::Constant(constant) => write!(f, "0x{:X}", constant),
            Argument::Memory(memory) => write!(f, "{}", memory),
        }
    }
}
impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::X64(register) => write!(f, "{}", register),
            Register::X86(register) => write!(f, "{}", register),
            Register::Arm32(register) => write!(f, "{}", register),
            Register::Arm64(register) => write!(f, "{}", register),
        }
    }
}
impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut first = true;

        if let Some(base) = &self.base {
            write!(f, "{}", base)?;
            first = false;
        }

        if let Some(index) = &self.index {
            if !first {
                write!(f, " + ")?;
            }
            write!(f, "{}", index)?;
            if self.scale > 1 {
                write!(f, " * {}", self.scale)?;
            }
            first = false;
        }

        if self.displacement != 0 {
            if !first {
                if self.displacement >= 0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
                write!(f, "0x{:X}", self.displacement.abs())?;
            } else {
                write!(f, "0x{:X}", self.displacement)?;
            }
        }

        write!(f, "]")
    }
}
