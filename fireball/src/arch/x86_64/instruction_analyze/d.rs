use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn dec() -> &'static [IrStatement] {
    let sub = b::sub(o1(), c(1));
    let calc_flags = calc_flags_automatically(sub.clone(), o1_size(), &[&of, &sf, &zf, &af, &pf]);
    let assignment = assign(sub, o1(), o1_size());
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn div() -> &'static [IrStatement] {
    let operand_bit_size = bit_size_of_o1();
    let div_8 = [
        assign(b::unsigned_div(ax.clone(), o1()), al.clone(), o1_size()),
        assign(b::unsigned_rem(ax.clone(), o1()), ah.clone(), o1_size()),
    ];
    let value = b::add(
        b::shl(sized(rdx.clone(), o1_size()), operand_bit_size.clone()),
        sized(rax.clone(), o1_size()),
    );
    let div_etc = [
        assign(b::unsigned_div(value.clone(), o1()), rax.clone(), o1_size()),
        assign(b::unsigned_rem(value, o1()), rdx.clone(), o1_size()),
    ];
    let div = condition(
        b::equal(operand_bit_size, c(8), size_unlimited()),
        div_8,
        div_etc,
    );
    extend_undefined_flags(&[div], &[&of, &sf, &zf, &af, &cf, &pf])
}

#[box_to_static_reference]
pub(super) fn divps() -> &'static [IrStatement] {
    // DIVPS divides four single-precision floating-point values from source to destination
    let size = o1_size();
    // For floating-point division, we should use the regular div operator
    // The IR analysis will handle the float-specific semantics
    let div = b::unsigned_div(o1(), o2());
    let assignment = assign(div, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn divpd() -> &'static [IrStatement] {
    // DIVPD divides two double-precision floating-point values from source to destination
    let size = o1_size();
    let div = b::unsigned_div(o1(), o2());
    let assignment = assign(div, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn divss() -> &'static [IrStatement] {
    // DIVSS divides the low single-precision floating-point values from source to destination
    let size = o1_size();
    let div = b::unsigned_div(o1(), o2());
    let assignment = assign(div, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn divsd() -> &'static [IrStatement] {
    // DIVSD divides the low double-precision floating-point values from source to destination
    let size = o1_size();
    let div = b::unsigned_div(o1(), o2());
    let assignment = assign(div, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}
