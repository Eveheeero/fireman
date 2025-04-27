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
