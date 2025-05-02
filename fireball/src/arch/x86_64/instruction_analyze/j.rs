use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn jmp() -> &'static [IrStatement] {
    let jmp = jump(o1());
    [jmp].into()
}
