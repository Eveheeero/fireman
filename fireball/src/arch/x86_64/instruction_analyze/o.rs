use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn or() -> &'static [IrStatement] {
    let size = o1_size();
    let or = b::or(o1(), o2());
    let assignment = assign(or.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(or, size, &[&sf, &zf, &pf]);
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    let set_af = assign(undefined_data(), af.clone(), size_relative(af.clone()));
    [calc_flags, set_of, set_cf, set_af, assignment].into()
}

#[box_to_static_reference]
pub(super) fn orps() -> &'static [IrStatement] {
    // ORPS performs bitwise logical OR of packed single-precision floating-point values
    let size = o1_size();
    let or = b::or(o1(), o2());
    let assignment = assign(or, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn orpd() -> &'static [IrStatement] {
    // ORPD performs bitwise logical OR of packed double-precision floating-point values
    let size = o1_size();
    let or = b::or(o1(), o2());
    let assignment = assign(or, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}
