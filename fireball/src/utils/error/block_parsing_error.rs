#[derive(Debug, Clone, Default)]
pub enum BlockParsingError {
    #[default]
    Unknown,
    NoInstruction,
}

impl std::fmt::Display for BlockParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown Error Occured!"),
            Self::NoInstruction => write!(f, "No Instruction Found!"),
        }
    }
}
