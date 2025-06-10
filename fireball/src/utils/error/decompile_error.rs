#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DecompileError {
    Unknown(Option<String>),
    HeaderParsingFailed,
    DisassembleFailed(super::disassemble_error::DisassembleError),
    EntryNotFound,
    CASTGenerationFailed(Option<String>),
}

impl Default for DecompileError {
    fn default() -> Self {
        Self::Unknown(None)
    }
}

impl std::fmt::Display for DecompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(msg) => {
                write!(f, "Unknown Error Occured! {}", msg.as_deref().unwrap_or(""))
            }
            Self::HeaderParsingFailed => write!(f, "Header Parsing Failed!"),
            Self::DisassembleFailed(err) => write!(f, "Fail to disassemble block! {}", err),
            Self::EntryNotFound => write!(f, "Entry Not Found!"),
            Self::CASTGenerationFailed(msg) => write!(
                f,
                "C-AST Generation Failed! {}",
                msg.as_deref().unwrap_or("")
            ),
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
        Self::Unknown(Some(msg))
    }
}

impl From<&String> for DecompileError {
    fn from(msg: &String) -> Self {
        Self::Unknown(Some(msg.clone()))
    }
}

impl From<&str> for DecompileError {
    fn from(msg: &str) -> Self {
        Self::Unknown(Some(msg.to_string()))
    }
}

impl From<super::disassemble_error::DisassembleError> for DecompileError {
    fn from(err: super::disassemble_error::DisassembleError) -> Self {
        Self::DisassembleFailed(err)
    }
}

impl From<std::io::Error> for DecompileError {
    fn from(err: std::io::Error) -> Self {
        Self::Unknown(Some(format!("I/O error: {}", err)))
    }
}

impl From<super::FireballError> for DecompileError {
    fn from(err: super::FireballError) -> Self {
        match err {
            super::FireballError::Unknown => Self::Unknown(None),
            super::FireballError::IoError(e) => Self::Unknown(Some(format!("I/O error: {}", e))),
            super::FireballError::InvalidBinary(msg) => Self::HeaderParsingFailed,
            super::FireballError::Unimplemented(feature) => {
                Self::Unknown(Some(format!("Unimplemented: {}", feature)))
            }
        }
    }
}
