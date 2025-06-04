use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn xchg() -> &'static [IrStatement] {
    let set_tmp = assign(o1(), tmp64.clone(), o1_size());
    let set_o1 = assign(o2(), o1(), o1_size());
    let set_o2 = assign(tmp64.clone(), o2(), o2_size());
    [set_tmp, set_o1, set_o2].into()
}

#[box_to_static_reference]
pub(super) fn xor() -> &'static [IrStatement] {
    let cond = b::equal(o1(), o2(), o1_size());
    let true_b = [
        assign(c(0), o1(), o1_size()),
        assign(c(1), zf.clone(), size_relative(zf.clone())),
        assign(c(0), sf.clone(), size_relative(sf.clone())),
        assign(c(0), pf.clone(), size_relative(pf.clone())),
    ];
    let false_b = b::xor(o1(), o2());
    let false_b = [
        assign(false_b.clone(), o1(), o1_size()),
        assign(c(0), zf.clone(), size_relative(zf.clone())),
        calc_flags_automatically(false_b, o1_size(), &[&sf, &pf]),
    ];
    let xor = condition(cond, true_b, false_b);
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    extend_undefined_flags(&[xor, set_of, set_cf], &[&af])
}
