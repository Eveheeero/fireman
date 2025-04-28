use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn adc() -> &'static [IrStatement] {
    let add = b::add(o(1), o(2), None);
    let add = b::add(add, u::zero_extend(r(&cf), size(&cf)), None);
    let set_cf = assign(b::signed_less(add.clone(), o(1), None), r(&cf), size(&cf));
    let assignment = assign(add, o(1), None);
    let set_sf = assign(b::signed_less(o(1), c(0), None), r(&sf), size(&sf));
    let set_zf = assign(b::equal(o(1), c(0), None), r(&zf), size(&zf));
    let set_less = assign(
        u::not(b::equal(r(&sf), r(&of), None), None),
        r(&less),
        size(&less),
    );
    let set_less_or_equal = assign(
        b::or(r(&less), r(&zf), None),
        r(&less_or_equal),
        size(&less_or_equal),
    );
    let set_below_or_equal = assign(
        b::or(r(&less), r(&zf), None),
        r(&below_or_equal),
        size(&below_or_equal),
    );
    [
        set_cf,
        assignment,
        set_sf,
        set_zf,
        set_less,
        set_less_or_equal,
        set_below_or_equal,
    ]
    .into()
}
