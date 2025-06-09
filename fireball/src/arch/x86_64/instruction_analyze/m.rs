use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn mov() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn movaps() -> &'static [IrStatement] {
    // MOVAPS moves 128 bits (4 single-precision floating-point values) from source to destination
    // Both operands must be 16-byte aligned
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

#[box_to_static_reference]
pub(super) fn mul() -> &'static [IrStatement] {
    let assertion = assertion(u::not(is_o2_exists()));

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
    extend_undefined_flags(&[assertion, mul], &[&sf, &zf, &af, &pf])
}

#[box_to_static_reference]
pub(super) fn mulps() -> &'static [IrStatement] {
    // MULPS multiplies four single-precision floating-point values from source to destination
    let size = o1_size();
    let mul = b::mul(o1(), o2());
    let assignment = assign(mul, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Float);
    let type2 = type_specified(o2(), o2_size(), DataType::Float);
    [assignment, type1, type2].into()
}
