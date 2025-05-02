use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn lea() -> &'static [IrStatement] {
    let address = u::zero_extend(d(o2()));
    let assignment = assign(address, o1(), o1_size());
    [assignment].into()
}
