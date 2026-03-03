use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn neg() -> &'static [IrStatement] {
    let neg = u::neg(o1());
    let calc_flags =
        calc_flags_automatically(neg.clone(), o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    let assignment = assign(neg, o1(), o1_size());
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn not() -> &'static [IrStatement] {
    let not = u::not(o1());
    let assignment = assign(not, o1(), o1_size());
    [assignment].into()
}
