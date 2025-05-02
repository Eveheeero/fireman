use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn imul() -> &'static [IrStatement] {
    todo!()
}

#[box_to_static_reference]
pub(super) fn inc() -> &'static [IrStatement] {
    let add = b::add(o1(), c(1));
    let calc_flags = calc_flags_automatically(add.clone(), o1_size(), &[&of, &sf, &zf, &af, &pf]);
    let assignment = assign(add, o1(), &o1_size());
    [calc_flags, assignment].into()
}
