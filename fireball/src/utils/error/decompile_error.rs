#[derive(Debug, Clone, Default)]
pub enum DecompileError {
    #[default]
    Unknown,
    UnknwonWithMessage(String),
    HeaderParsingFailed,
    DisassembleFailed(super::disassemble_error::DisassembleError),
}

impl std::fmt::Display for DecompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown Error Occured!"),
            Self::UnknwonWithMessage(msg) => write!(f, "Unknown Error Occured! {}", msg),
            Self::HeaderParsingFailed => write!(f, "Header Parsing Failed!"),
            Self::DisassembleFailed(err) => write!(f, "Fail to disassemble block! {}", err),
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
        Self::UnknwonWithMessage(msg)
    }
}

impl From<&String> for DecompileError {
    fn from(msg: &String) -> Self {
        Self::UnknwonWithMessage(msg.clone())
    }
}

impl From<&str> for DecompileError {
    fn from(msg: &str) -> Self {
        Self::UnknwonWithMessage(msg.to_string())
    }
}

impl From<super::disassemble_error::DisassembleError> for DecompileError {
    fn from(err: super::disassemble_error::DisassembleError) -> Self {
        Self::DisassembleFailed(err)
    }
}
