use core::fmt::{Debug, Formatter};

use super::PE;

impl Debug for PE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PE").field("File Path", &self.path).finish()
    }
}
