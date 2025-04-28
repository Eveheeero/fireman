use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

#[box_to_static_reference]
pub(super) fn adc() -> &'static [IrStatement] {
    let add = b::add(o1, o2, o1_size());
    let add = b::add(add, u::zero_extend(r(&cf), o1_size()), o1_size());
    let set_cf = assign(
        b::signed_less(add.clone(), o1, o1_size()),
        r(&cf),
        size_fix(&cf),
    );
    let assignment = assign(add, o1, o1_size());
    let set_sf = assign(b::signed_less(o1, c(0), o1_size()), r(&sf), size_fix(&sf));
    let set_zf = assign(b::equal(o1, c(0), o1_size()), r(&zf), size_fix(&zf));
    let set_less = assign(
        u::not(b::equal(r(&sf), r(&of), s_fix(&of)), s_fix(&of)),
        r(&less),
        size_fix(&less),
    );
    let set_less_or_equal = assign(
        b::or(r(&less), r(&zf), s_fix(&of)),
        r(&less_or_equal),
        size_fix(&less_or_equal),
    );
    let set_below_or_equal = assign(
        b::or(r(&less), r(&zf), s_fix(&of)),
        r(&below_or_equal),
        size_fix(&below_or_equal),
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
