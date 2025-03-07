#[derive(Debug, Clone, Default)]
pub enum DisassembleError {
    #[default]
    Unknown,
    UnknwonWithMessage(String),
    TriedToParseOutsideOfSection,
    CapstoneFailed(String),
}

impl std::fmt::Display for DisassembleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown Error Occured!"),
            Self::UnknwonWithMessage(msg) => write!(f, "Unknown Error Occured! {}", msg),
            Self::TriedToParseOutsideOfSection => {
                write!(f, "Tried to parse outside of section!")
            }
            Self::CapstoneFailed(msg) => write!(f, "Capstone Error Occured! {}", msg),
        }
    }
}

impl From<String> for DisassembleError {
    fn from(msg: String) -> Self {
        Self::UnknwonWithMessage(msg)
    }
}

impl From<&String> for DisassembleError {
    fn from(msg: &String) -> Self {
        Self::UnknwonWithMessage(msg.clone())
    }
}

impl From<&str> for DisassembleError {
    fn from(msg: &str) -> Self {
        Self::UnknwonWithMessage(msg.to_string())
    }
}
