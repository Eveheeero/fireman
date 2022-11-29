#[derive(Debug, Clone, Default)]
pub enum IoError {
    #[default]
    Unknown,
    FileCannot,
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File Cannot Read!")
    }
}

impl From<std::io::Error> for IoError {
    fn from(_: std::io::Error) -> Self {
        Self::FileCannot
    }
}
