use super::Elf;
use core::fmt::{Debug, Formatter};

impl Debug for Elf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ELF")
            .field("File Path", &self.path)
            .finish()
    }
}
