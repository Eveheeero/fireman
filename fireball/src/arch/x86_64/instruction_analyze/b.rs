use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn bswap() -> &'static [IrStatement] {
    let size = o1_size();
    let swap_32 = [
        assign(o1(), tmp32.clone(), size.clone()),
        assign(tmp32.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp32.clone(), c(8)), tmp32.clone(), size.clone()),
        assign(tmp32.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp32.clone(), c(8)), tmp32.clone(), size.clone()),
        assign(tmp32.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp32.clone(), c(8)), tmp32.clone(), size.clone()),
        assign(tmp32.clone(), o1(), size_result_bit(c(8))),
    ];
    let swap_64 = [
        assign(o1(), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
    ];
    let bswap = condition(
        b::equal(bit_size_of_o1(), c(32), size_unlimited()),
        swap_32,
        swap_64,
    );
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    [bswap, type1].into()
}

#[box_to_static_reference]
pub(super) fn bt() -> &'static [IrStatement] {
    let size = size_relative(cf.clone());
    let shr = b::shr(o1(), o2());
    let assignment = assign(shr.clone(), cf.clone(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    extend_undefined_flags(&[assignment, type1, type2], &[&of, &sf, &af, &pf])
}
