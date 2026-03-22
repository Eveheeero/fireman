use super::MachO;
use core::fmt::{Debug, Formatter};

impl Debug for MachO {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachO")
            .field("File Path", &self.path)
            .finish()
    }
}
