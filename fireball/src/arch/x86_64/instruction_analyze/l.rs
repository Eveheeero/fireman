use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn lea() -> &'static [IrStatement] {
    let address = u::zero_extend(d(o2()));
    let assignment = assign(address, o1(), o1_size());
    [assignment].into()
}

/// `leave` is equivalent to `mov rsp, rbp; pop rbp`
#[box_to_static_reference]
pub(super) fn leave() -> &'static [IrStatement] {
    // mov rsp, rbp
    let restore_sp = assign(rbp.clone(), rsp.clone(), size_architecture());
    // pop rbp: load [rsp] into rbp, then rsp += 8
    let pop_rbp = assign(d(rsp.clone()), rbp.clone(), size_architecture());
    let inc_sp = assign(
        b::add(rsp.clone(), architecture_byte_size()),
        rsp.clone(),
        size_architecture(),
    );
    [restore_sp, pop_rbp, inc_sp].into()
}
