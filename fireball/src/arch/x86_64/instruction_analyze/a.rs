use super::super::static_register::*;
use super::{max, size};
use crate::ir::{data::*, operator::*, statements::*, x86_64::X64Range as X64, VirtualMachine};

pub(super) fn aaa() -> Box<[IrStatement]> {
    let al_and_0fh = IrData::Operator(IrDataOperator::Binary {
        operator: BinaryOperator::And,
        arg1: Box::new(IrData::Intrinsic(IntrinsicType::Undefined(Box::new(
            IrData::Register(al.clone()),
        )))),
        arg2: Box::new(IrData::Constant(0x0f)),
        size: size(&al),
    });
    let al_and_0fh_lt_9 = IrData::Operator(IrDataOperator::Binary {
        operator: BinaryOperator::UnsignedLess,
        arg1: Box::new(IrData::Constant(9)),
        arg2: Box::new(al_and_0fh),
        size: max(),
    });
    let then = [
        IrStatement::Assignment {
            from: IrData::Operator(IrDataOperator::Binary {
                operator: BinaryOperator::Add,
                arg1: Box::new(IrData::Intrinsic(IntrinsicType::Undefined(Box::new(
                    IrData::Register(ax.clone()),
                )))),
                arg2: Box::new(IrData::Constant(0x106)),
                size: size(&ax),
            }),
            to: IrData::Register(ax.clone()),
            size: size(&ax),
        },
        IrStatement::Assignment {
            from: IrData::Constant(1),
            to: IrData::Register(af.clone()),
            size: size(&af),
        },
        IrStatement::Assignment {
            from: IrData::Constant(1),
            to: IrData::Register(cf.clone()),
            size: size(&cf),
        },
    ];
    let r#else = [
        IrStatement::Assignment {
            from: IrData::Constant(0),
            to: IrData::Register(af.clone()),
            size: size(&af),
        },
        IrStatement::Assignment {
            from: IrData::Constant(0),
            to: IrData::Register(cf.clone()),
            size: size(&cf),
        },
    ];
    let after = IrStatement::Assignment {
        from: IrData::Operator(IrDataOperator::Binary {
            operator: BinaryOperator::And,
            arg1: Box::new(IrData::Register(al.clone())),
            arg2: Box::new(IrData::Constant(0x0f)),
            size: size(&al),
        }),
        to: IrData::Register(al.clone()),
        size: size(&al),
    };

    [
        IrStatement::Condition {
            condition: al_and_0fh_lt_9,
            size: max(),
            true_branch: then.into(),
            false_branch: r#else.into(),
        },
        after,
    ]
    .into()
}
