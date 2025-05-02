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
