pub mod decompile_error;
pub mod disassemble_error;
pub mod io_error;
pub mod ir_analyze_assertion_error;

// Re-export DecompileError for convenience
pub use decompile_error::DecompileError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FireballError {
    Unknown,
    IoError(io_error::IoError),
    InvalidBinary(String),
    Unimplemented(String),
}

impl std::fmt::Display for FireballError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FireballError::Unknown => write!(f, "Unknown error occurred"),
            FireballError::IoError(e) => write!(f, "IO error: {}", e),
            FireballError::InvalidBinary(msg) => write!(f, "Invalid binary: {}", msg),
            FireballError::Unimplemented(feature) => write!(f, "Unimplemented: {}", feature),
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
        Self::IoError(io_error::IoError::StdIoError(err.to_string()))
    }
}
