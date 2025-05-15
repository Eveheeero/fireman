//! Module defining instruction information

/// Information about an assembly instruction
///
/// Since Capstone engine's `Instruction` cannot be cloned, we define a cloneable `Instruction`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    /// The instruction's virtual address
    pub(crate) address: u64,
    /// Parsed Instruction
    pub(crate) inner: iceball::Instruction,
}

impl Instruction {
    pub fn inner(&self) -> &iceball::Instruction {
        &self.inner
    }
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X} {}", self.address, self.inner)
    }
}
