use super::{super::static_register::*, shortcuts::*};

pub(super) fn aaa() -> Box<[IrStatement]> {
    let al_and_0fh = b::and(undefined(r(&al)), c(0x0f), size(&al));
    let al_and_0fh_lt_9 = b::unsigned_less(c(9), al_and_0fh, None);
    let then = [
        IrStatement::Assignment {
            from: b::add(undefined(r(&ax)), c(0x106), size(&ax)),
            to: r(&ax),
            size: size(&ax),
        },
        IrStatement::Assignment {
            from: c(1),
            to: r(&af),
            size: size(&af),
        },
        IrStatement::Assignment {
            from: c(1),
            to: r(&cf),
            size: size(&cf),
        },
    ];
    let r#else = [
        IrStatement::Assignment {
            from: c(0),
            to: r(&af),
            size: size(&af),
        },
        IrStatement::Assignment {
            from: c(0),
            to: r(&cf),
            size: size(&cf),
        },
    ];
    let after = IrStatement::Assignment {
        from: b::and(r(&al), c(0x0f), size(&al)),
        to: r(&al),
        size: size(&al),
    };

    [
        IrStatement::Condition {
            condition: al_and_0fh_lt_9,
            true_branch: then.into(),
            false_branch: r#else.into(),
        },
        after,
    ]
    .into()
}

pub(super) fn adc() -> Box<[IrStatement]> {
    let add = b::add(o(1), o(2), None);
    let add = b::add(add, u::zero_extend(r(&cf), size(&cf)), None);
    let set_cf = IrStatement::Assignment {
        from: b::signed_less(add.clone(), o(1), None),
        to: r(&cf),
        size: size(&cf),
    };
    let assignment = IrStatement::Assignment {
        from: add,
        to: o(1),
        size: None,
    };
    let set_sf = IrStatement::Assignment {
        from: b::signed_less(o(1), c(0), None),
        to: r(&sf),
        size: size(&sf),
    };
    let set_zf = IrStatement::Assignment {
        from: b::equal(o(1), c(0), None),
        to: r(&zf),
        size: size(&zf),
    };
    let set_less = IrStatement::Assignment {
        from: u::not(b::equal(r(&sf), r(&of), None), None),
        to: r(&less),
        size: size(&less),
    };
    let set_less_or_equal = IrStatement::Assignment {
        from: b::or(r(&less), r(&zf), None),
        to: r(&less_or_equal),
        size: size(&less_or_equal),
    };
    let set_below_or_equal = IrStatement::Assignment {
        from: b::or(r(&less), r(&zf), None),
        to: r(&below_or_equal),
        size: size(&below_or_equal),
    };
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
