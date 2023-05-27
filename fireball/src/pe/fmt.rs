//! 출력에 대한 구현이 담겨있는 모듈

use super::PE;
use core::fmt::{Debug, Formatter};

impl Debug for PE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PE").field("File Path", &self.path).finish()
    }
}
