#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum DecompileError {
    #[default]
    Unknown,
    UnknownWithMessage(String),
    HeaderParsingFailed,
    DisassembleFailed(super::disassemble_error::DisassembleError),
    EntryNotFound,
}

impl std::fmt::Display for DecompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown Error Occured!"),
            Self::UnknownWithMessage(msg) => write!(f, "Unknown Error Occured! {}", msg),
            Self::HeaderParsingFailed => write!(f, "Header Parsing Failed!"),
            Self::DisassembleFailed(err) => write!(f, "Fail to disassemble block! {}", err),
            Self::EntryNotFound => write!(f, "Entry Not Found!"),
        }
    }
}

impl From<goblin::error::Error> for DecompileError {
    fn from(_: goblin::error::Error) -> Self {
        Self::HeaderParsingFailed
    }
}

impl From<String> for DecompileError {
    fn from(msg: String) -> Self {
        Self::UnknownWithMessage(msg)
    }
}

impl From<&String> for DecompileError {
    fn from(msg: &String) -> Self {
        Self::UnknownWithMessage(msg.clone())
    }
}

impl From<&str> for DecompileError {
    fn from(msg: &str) -> Self {
        Self::UnknownWithMessage(msg.to_string())
    }
}

impl From<super::disassemble_error::DisassembleError> for DecompileError {
    fn from(err: super::disassemble_error::DisassembleError) -> Self {
        Self::DisassembleFailed(err)
    }
}
