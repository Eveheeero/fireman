use super::super::static_register::*;
use super::{max, size};
use crate::ir::{data::*, operator::*, statements::*, x86_64::X64Range as X64, VirtualMachine};
use std::rc::Rc;

pub(super) fn aaa() -> Rc<[IRStatement]> {
    let al_and_0fh = IRData::Operator(IRDataOperator::Binary {
        operator: BinaryOperator::And,
        arg1: Box::new(IRData::Intrinsic(IntrinsicType::Undefined(Box::new(
            IRData::Register(al.clone()),
        )))),
        arg2: Box::new(IRData::Constant(0x0f)),
        size: size(&al),
    });
    let al_and_0fh_lt_9 = IRData::Operator(IRDataOperator::Binary {
        operator: BinaryOperator::UnsignedLess,
        arg1: Box::new(IRData::Constant(9)),
        arg2: Box::new(al_and_0fh),
        size: max(),
    });
    let then = [
        IRStatement::Assignment {
            from: IRData::Operator(IRDataOperator::Binary {
                operator: BinaryOperator::Add,
                arg1: Box::new(IRData::Intrinsic(IntrinsicType::Undefined(Box::new(
                    IRData::Register(ax.clone()),
                )))),
                arg2: Box::new(IRData::Constant(0x106)),
                size: size(&ax),
            }),
            to: IRData::Register(ax.clone()),
            size: size(&ax),
        },
        IRStatement::Assignment {
            from: IRData::Constant(1),
            to: IRData::Register(af.clone()),
            size: size(&af),
        },
        IRStatement::Assignment {
            from: IRData::Constant(1),
            to: IRData::Register(cf.clone()),
            size: size(&cf),
        },
    ];
    let r#else = [
        IRStatement::Assignment {
            from: IRData::Constant(0),
            to: IRData::Register(af.clone()),
            size: size(&af),
        },
        IRStatement::Assignment {
            from: IRData::Constant(0),
            to: IRData::Register(cf.clone()),
            size: size(&cf),
        },
    ];
    let after = IRStatement::Assignment {
        from: IRData::Operator(IRDataOperator::Binary {
            operator: BinaryOperator::And,
            arg1: Box::new(IRData::Register(al.clone())),
            arg2: Box::new(IRData::Constant(0x0f)),
            size: size(&al),
        }),
        to: IRData::Register(al.clone()),
        size: size(&al),
    };

    [
        IRStatement::Condition {
            condition: al_and_0fh_lt_9,
            size: max(),
            true_branch: then.into(),
            false_branch: r#else.into(),
        },
        after,
    ]
    .into()
}
