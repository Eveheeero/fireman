pub mod decompile_error;
pub mod disassemble_error;
pub mod io_error;
pub mod ir_analyze_assertion_error;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum FireballError {
    #[default]
    Unknown,
    IoError(io_error::IoError),
    DecompileError(decompile_error::DecompileError),
    DisassembleError(disassemble_error::DisassembleError),
    PeParsingFailed(String),
    CapstoneInitializationFailed(String),
    MalformedPe(String),
}

impl std::fmt::Display for FireballError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown fireball error occurred"),
            Self::IoError(err) => write!(f, "I/O error: {err}"),
            Self::DecompileError(err) => write!(f, "Decompile error: {err}"),
            Self::DisassembleError(err) => write!(f, "Disassemble error: {err}"),
            Self::PeParsingFailed(err) => write!(f, "PE parsing failed: {err}"),
            Self::CapstoneInitializationFailed(err) => {
                write!(f, "Capstone initialization failed: {err}")
            }
            Self::MalformedPe(err) => write!(f, "Malformed PE metadata: {err}"),
        }
    }
}

impl From<io_error::IoError> for FireballError {
    fn from(err: io_error::IoError) -> Self {
        Self::IoError(err)
    }
}

impl From<std::io::Error> for FireballError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.into())
    }
}

impl From<decompile_error::DecompileError> for FireballError {
    fn from(err: decompile_error::DecompileError) -> Self {
        Self::DecompileError(err)
    }
}

impl From<disassemble_error::DisassembleError> for FireballError {
    fn from(err: disassemble_error::DisassembleError) -> Self {
        Self::DisassembleError(err)
    }
}

impl From<goblin::error::Error> for FireballError {
    fn from(err: goblin::error::Error) -> Self {
        Self::PeParsingFailed(err.to_string())
    }
}

impl From<capstone::Error> for FireballError {
    fn from(err: capstone::Error) -> Self {
        Self::CapstoneInitializationFailed(err.to_string())
    }
}
