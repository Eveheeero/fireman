use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn shl() -> &'static [IrStatement] {
    let shl_1 = b::shl(o1(), o2());
    let shl_1_flags = calc_flags_automatically(shl_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let shl_2 = b::shl(o1(), c(1));
    let shl_2_flags = calc_flags_automatically(shl_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let condition = condition(
        is_o2_exists(),
        [shl_1_flags, assign(shl_1, o1(), o1_size())],
        [shl_2_flags, assign(shl_2, o1(), o1_size())],
    );
    extend_undefined_flags(&[condition], &[&of, &af, &cf])
}

#[box_to_static_reference]
pub(super) fn shr() -> &'static [IrStatement] {
    let shr_1 = b::shr(o1(), o2());
    let shr_1_flags = calc_flags_automatically(shr_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let shr_2 = b::shr(o1(), c(1));
    let shr_2_flags = calc_flags_automatically(shr_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let condition = condition(
        is_o2_exists(),
        [shr_1_flags, assign(shr_1, o1(), o1_size())],
        [shr_2_flags, assign(shr_2, o1(), o1_size())],
    );
    extend_undefined_flags(&[condition], &[&of, &af, &cf])
}

#[box_to_static_reference]
pub(super) fn sub() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn subps() -> &'static [IrStatement] {
    // SUBPS subtracts four single-precision floating-point values from source to destination
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn subpd() -> &'static [IrStatement] {
    // SUBPD subtracts two double-precision floating-point values from source to destination
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn subss() -> &'static [IrStatement] {
    // SUBSS subtracts the low single-precision floating-point values from source to destination
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn subsd() -> &'static [IrStatement] {
    // SUBSD subtracts the low double-precision floating-point values from source to destination
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sqrtps() -> &'static [IrStatement] {
    // SQRTPS computes square root of four single-precision floating-point values
    // TODO: Implement square root unary operator in IR
    // For now, mark as unknown operation
    let size = o1_size();
    let sqrt = unknown_data();
    let assignment = assign(sqrt, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sqrtpd() -> &'static [IrStatement] {
    // SQRTPD computes square root of two double-precision floating-point values
    // TODO: Implement square root unary operator in IR
    let size = o1_size();
    let sqrt = unknown_data();
    let assignment = assign(sqrt, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sqrtss() -> &'static [IrStatement] {
    // SQRTSS computes square root of the low single-precision floating-point value
    // TODO: Implement square root unary operator in IR
    let size = o1_size();
    let sqrt = unknown_data();
    let assignment = assign(sqrt, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
pub(super) fn sqrtsd() -> &'static [IrStatement] {
    // SQRTSD computes square root of the low double-precision floating-point value
    // TODO: Implement square root unary operator in IR
    let size = o1_size();
    let sqrt = unknown_data();
    let assignment = assign(sqrt, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}
