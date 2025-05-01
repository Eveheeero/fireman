use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn adc() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let add = b::add(add, u::zero_extend(r(&cf)));
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    [assignment, calc_flags].into()
}
