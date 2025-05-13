use crate::X64Register;

impl std::fmt::Display for X64Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().to_ascii_lowercase())
    }
}
