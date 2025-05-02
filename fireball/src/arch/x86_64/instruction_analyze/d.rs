use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn dec() -> &'static [IrStatement] {
    let sub = b::sub(o1(), c(1));
    let calc_flags = calc_flags_automatically(sub.clone(), o1_size(), &[&of, &sf, &zf, &af, &pf]);
    let assignment = assign(sub, o1(), &o1_size());
    [calc_flags, assignment].into()
}

#[box_to_static_reference]
pub(super) fn div() -> &'static [IrStatement] {
    let operand_bit_size = bit_size_of_o1();
    let div_8 = [
        assign(b::unsigned_div(r(&ax), o1()), r(&al), o1_size()),
        assign(b::unsigned_rem(r(&ax), o1()), r(&ah), o1_size()),
    ];
    let value = b::add(
        b::shl(
            sized(r(&rdx), size_result_bit(operand_bit_size.clone())),
            operand_bit_size.clone(),
        ),
        sized(r(&rax), size_result_bit(operand_bit_size.clone())),
    );
    let div_etc = [
        assign(b::unsigned_div(value.clone(), o1()), r(&rax), o1_size()),
        assign(b::unsigned_rem(value, o1()), r(&rdx), o1_size()),
    ];
    let div = condition(
        b::equal(operand_bit_size, c(8), size_unlimited()),
        div_8,
        div_etc,
    );
    extend_undefined_flags(&[div], &[&of, &sf, &zf, &af, &cf, &pf])
}
