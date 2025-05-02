use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn dec() -> &'static [IrStatement] {
    let sub = b::sub(o1(), c(1));
    let calc_flags = calc_flags_automatically(sub.clone(), o1_size(), &[&of, &sf, &zf, &af, &pf]);
    let assignment = assign(sub, o1(), &o1_size());
    [calc_flags, assignment].into()
}
