use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn neg() -> &'static [IrStatement] {
    // NEG instruction: dest = -dest (two's complement)
    let size = o1_size();
    let negated = u::neg(o1());
    let assignment = assign(negated.clone(), o1(), &size);

    // NEG affects CF, OF, SF, ZF, AF, PF flags
    let calc_flags = calc_flags_automatically(negated, &size, &[&cf, &of, &sf, &zf, &af, &pf]);

    [assignment, calc_flags].into()
}

#[box_to_static_reference]
pub(super) fn not() -> &'static [IrStatement] {
    // NOT instruction: dest = ~dest (one's complement)
    let size = o1_size();
    let inverted = u::not(o1());
    let assignment = assign(inverted, o1(), &size);

    // NOT does not affect any flags
    [assignment].into()
}
