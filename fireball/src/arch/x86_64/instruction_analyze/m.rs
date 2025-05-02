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
        b::mul(sized(al.clone(), size_relative(al.clone())), o1()),
        ax.clone(),
        size_relative(ax.clone()),
    );
    todo!()
}
