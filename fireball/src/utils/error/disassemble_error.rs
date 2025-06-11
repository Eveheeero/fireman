#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DisassembleError {
    Unknown(Option<String>),
    TriedToParseOutsideOfSection,
    CapstoneFailed(String),
}

impl std::fmt::Display for DisassembleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(msg) => {
                write!(f, "Unknown Error Occured! {}", msg.as_deref().unwrap_or(""))
            }
            Self::TriedToParseOutsideOfSection => {
                write!(f, "Tried to parse outside of section!")
            }
            Self::CapstoneFailed(msg) => write!(f, "Capstone Error Occured! {}", msg),
        }
    }
}

impl From<String> for DisassembleError {
    fn from(msg: String) -> Self {
        Self::Unknown(Some(msg))
    }
}

impl From<&String> for DisassembleError {
    fn from(msg: &String) -> Self {
        Self::Unknown(Some(msg.clone()))
    }
}

impl From<&str> for DisassembleError {
    fn from(msg: &str) -> Self {
        Self::Unknown(Some(msg.to_string()))
    }
}
