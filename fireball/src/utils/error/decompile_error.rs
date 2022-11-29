#[derive(Debug, Clone, Default)]
pub enum DecompileError {
    #[default]
    Unknown,
    HeaderParsingError,
}

impl std::fmt::Display for DecompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown Error Occured!"),
            Self::HeaderParsingError => write!(f, "Header Parsing Error Occured!"),
        }
    }
}

impl From<goblin::error::Error> for DecompileError {
    fn from(_: goblin::error::Error) -> Self {
        Self::HeaderParsingError
    }
}
