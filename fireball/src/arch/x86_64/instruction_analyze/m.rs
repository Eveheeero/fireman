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
    let mul_8 = assign(
        b::mul(sized(r(&al), size_relative(r(&al))), o1()),
        r(&ax),
        size_relative(r(&ax)),
    );
    todo!()
}
