#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum IoError {
    #[default]
    Unknown,
    UnknwonWithMessage(String),
    FileCannotRead,
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown Error Occured!"),
            Self::UnknwonWithMessage(msg) => write!(f, "Unknown Error Occured! {}", msg),
            Self::FileCannotRead => write!(f, "File Cannot Read!"),
        }
    }
}

impl From<std::io::Error> for IoError {
    fn from(_: std::io::Error) -> Self {
        Self::FileCannotRead
    }
}

impl From<String> for IoError {
    fn from(msg: String) -> Self {
        Self::UnknwonWithMessage(msg)
    }
}

impl From<&String> for IoError {
    fn from(msg: &String) -> Self {
        Self::UnknwonWithMessage(msg.clone())
    }
}

impl From<&str> for IoError {
    fn from(msg: &str) -> Self {
        Self::UnknwonWithMessage(msg.to_string())
    }
}
