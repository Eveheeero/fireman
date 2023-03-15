#[derive(Debug, Clone, Default)]
pub enum DecompileError {
    #[default]
    Unknown,
    UnknwonWithMessage(String),
    HeaderParsingError,
}

impl std::fmt::Display for DecompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown Error Occured!"),
            Self::UnknwonWithMessage(msg) => write!(f, "Unknown Error Occured! {}", msg),
            Self::HeaderParsingError => write!(f, "Header Parsing Error Occured!"),
        }
    }
}

impl From<goblin::error::Error> for DecompileError {
    fn from(_: goblin::error::Error) -> Self {
        Self::HeaderParsingError
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
