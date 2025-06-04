use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn test() -> &'static [IrStatement] {
    let and = b::and(o1(), o2());
    let sf_zf_pf = calc_flags_automatically(and, o1_size(), &[&sf, &zf, &pf]);
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    extend_undefined_flags(&[sf_zf_pf, set_of, set_cf], &[&af])
}
