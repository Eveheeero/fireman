use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn mov() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn mul() -> &'static [IrStatement] {
    let operand_bit_size = bit_size_of_o1();

    let value = b::mul(sized(al.clone(), size_relative(al.clone())), o1());
    let mul_8 = [
        calc_flags_automatically(value.clone(), o1_size(), &[&of, &cf]),
        assign(value, ax.clone(), size_relative(ax.clone())),
    ];

    let value = b::mul(sized(rax.clone(), o1_size()), o1());
    let mul_etc = [
        calc_flags_automatically(value.clone(), o1_size(), &[&of, &cf]),
        assign(value.clone(), rax.clone(), o1_size()),
        assign(
            b::shr(u::zero_extend(value), operand_bit_size.clone()),
            rdx.clone(),
            o1_size(),
        ),
    ];

    let mul = condition(
        b::equal(operand_bit_size, c(8), size_unlimited()),
        mul_8,
        mul_etc,
    );
    extend_undefined_flags(&[mul], &[&sf, &zf, &af, &pf])
}
