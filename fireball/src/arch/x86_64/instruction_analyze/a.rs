use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn adc() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let add = b::add(add, u::zero_extend(cf.clone()));
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn add() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn and() -> &'static [IrStatement] {
    let size = o1_size();
    let and = b::and(o1(), o2());
    let assignment = assign(and.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(and, size, &[&sf, &zf, &pf]);
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    let set_af = assign(undefined_data(), af.clone(), size_relative(af.clone()));
    [calc_flags, set_of, set_cf, set_af, assignment].into()
}
