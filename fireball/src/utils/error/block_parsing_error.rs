#[derive(Debug, Clone, Default)]
pub enum BlockParsingError {
    #[default]
    Unknown,
    UnknwonWithMessage(String),
    NoInstruction,
    TriedToParseOutsideOfSection,
}

impl std::fmt::Display for BlockParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown Error Occured!"),
            Self::UnknwonWithMessage(msg) => write!(f, "Unknown Error Occured! {}", msg),
            Self::NoInstruction => write!(f, "No Instruction Found!"),
            Self::TriedToParseOutsideOfSection => {
                write!(f, "Tried to parse outside of section!")
            }
        }
    }
}

impl From<String> for BlockParsingError {
    fn from(msg: String) -> Self {
        Self::UnknwonWithMessage(msg)
    }
}

impl From<&String> for BlockParsingError {
    fn from(msg: &String) -> Self {
        Self::UnknwonWithMessage(msg.clone())
    }
}

impl From<&str> for BlockParsingError {
    fn from(msg: &str) -> Self {
        Self::UnknwonWithMessage(msg.to_string())
    }
}
