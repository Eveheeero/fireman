use super::super::static_register::*;
use super::size;
use crate::ir::{data::*, operator::*, statements::*};

pub(super) fn aaa() -> Box<[IrStatement]> {
    let al_and_0fh = IrData::Operation(IrDataOperation::Binary {
        operator: BinaryOperator::And,
        arg1: IrData::Intrinsic(IntrinsicType::Undefined(IrData::register(&al).b())).b(),
        arg2: IrData::Constant(0x0f).b(),
        size: size(&al),
    });
    let al_and_0fh_lt_9 = IrData::Operation(IrDataOperation::Binary {
        operator: BinaryOperator::UnsignedLess,
        arg1: IrData::Constant(9).b(),
        arg2: al_and_0fh.b(),
        size: None,
    });
    let then = [
        IrStatement::Assignment {
            from: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Add,
                arg1: IrData::Intrinsic(IntrinsicType::Undefined(IrData::register(&ax).b())).b(),
                arg2: IrData::Constant(0x106).b(),
                size: size(&ax),
            }),
            to: IrData::register(&ax),
            size: size(&ax),
        },
        IrStatement::Assignment {
            from: IrData::Constant(1),
            to: IrData::register(&af),
            size: size(&af),
        },
        IrStatement::Assignment {
            from: IrData::Constant(1),
            to: IrData::register(&cf),
            size: size(&cf),
        },
    ];
    let r#else = [
        IrStatement::Assignment {
            from: IrData::Constant(0),
            to: IrData::register(&af),
            size: size(&af),
        },
        IrStatement::Assignment {
            from: IrData::Constant(0),
            to: IrData::register(&cf),
            size: size(&cf),
        },
    ];
    let after = IrStatement::Assignment {
        from: IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::And,
            arg1: IrData::register(&al).b(),
            arg2: IrData::Constant(0x0f).b(),
            size: size(&al),
        }),
        to: IrData::register(&al),
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
    let add = IrData::Operation(IrDataOperation::Binary {
        operator: BinaryOperator::Add,
        arg1: IrData::operand(1).b(),
        arg2: IrData::operand(2).b(),
        size: None,
    });
    let add = IrData::Operation(IrDataOperation::Binary {
        operator: BinaryOperator::Add,
        arg1: add.b(),
        arg2: IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::ZeroExtend,
            arg: IrData::register(&cf).b(),
            size: size(&cf),
        })
        .b(),
        size: None,
    });
    let set_cf = IrStatement::Assignment {
        from: IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::SignedLess,
            arg1: add.clone().b(),
            arg2: IrData::operand(1).b(),
            size: None,
        }),
        to: IrData::register(&cf),
        size: size(&cf),
    };
    let assignment = IrStatement::Assignment {
        from: add,
        to: IrData::operand(1),
        size: None,
    };
    let set_sf = IrStatement::Assignment {
        from: IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::SignedLess,
            arg1: IrData::operand(1).b(),
            arg2: IrData::Constant(0).b(),
            size: None,
        }),
        to: IrData::register(&sf),
        size: size(&sf),
    };
    let set_zf = IrStatement::Assignment {
        from: IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Equal,
            arg1: IrData::operand(1).b(),
            arg2: IrData::Constant(0).b(),
            size: None,
        }),
        to: IrData::register(&zf),
        size: size(&zf),
    };
    let set_less = IrStatement::Assignment {
        from: IrData::Operation(IrDataOperation::Unary {
            operator: UnaryOperator::Not,
            arg: IrData::Operation(IrDataOperation::Binary {
                operator: BinaryOperator::Equal,
                arg1: IrData::register(&sf).b(),
                arg2: IrData::register(&of).b(),
                size: None,
            })
            .b(),
            size: None,
        }),
        to: IrData::register(&less),
        size: size(&less),
    };
    let set_less_or_equal = IrStatement::Assignment {
        from: IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Or,
            arg1: IrData::register(&less).b(),
            arg2: IrData::register(&zf).b(),
            size: None,
        }),
        to: IrData::register(&less_or_equal),
        size: size(&less_or_equal),
    };
    let set_below_or_equal = IrStatement::Assignment {
        from: IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Or,
            arg1: IrData::register(&less).b(),
            arg2: IrData::register(&zf).b(),
            size: None,
        }),
        to: IrData::register(&below_or_equal),
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
