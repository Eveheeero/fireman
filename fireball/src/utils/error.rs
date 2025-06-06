pub mod decompile_error;
pub mod disassemble_error;
pub mod io_error;
pub mod ir_analyze_assertion_error;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum FireballError {
    #[default]
    Unknown,
    IoError(io_error::IoError),
}

impl std::fmt::Display for FireballError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Some Error Occured!")
    }
}

impl From<io_error::IoError> for FireballError {
    fn from(err: io_error::IoError) -> Self {
        Self::IoError(err)
    }
}
