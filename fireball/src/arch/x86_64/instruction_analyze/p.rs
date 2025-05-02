use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn pop() -> &'static [IrStatement] {
    let pop = assign(d(rsp.clone()), o1(), o1_size());
    let set_sp = assign(
        b::add(rsp.clone(), architecture_byte_size()),
        rsp.clone(),
        size_architecture(),
    );
    [pop, set_sp].into()
}

#[box_to_static_reference]
pub(super) fn push() -> &'static [IrStatement] {
    let set_sp = assign(
        b::sub(rsp.clone(), architecture_byte_size()),
        rsp.clone(),
        size_architecture(),
    );
    let push = assign(rsp.clone(), o1(), o1_size());
    [set_sp, push].into()
}
