use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn adc() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let add = b::add(add, u::zero_extend(cf.clone()));
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(cf.clone(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1, type2, type3].into()
}

#[box_to_static_reference]
pub(super) fn add() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, assignment, type1, type2].into()
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
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_of, set_cf, set_af, assignment, type1, type2].into()
}
