use super::super::static_register::*;
use crate::ir::{data::*, operator::*, statements::*, x86_64::X64Range as X64, VirtualMachine};
use std::rc::Rc;

pub(super) fn aaa() -> Rc<[IRStatement]> {
    let al_and_0fh = IRData::Operator(IRDataOperator::Binary(
        BinaryOperator::And,
        Box::new(IRData::Intrinsic(IntrinsicType::Undefined(Box::new(
            IRData::Register(al.clone()),
        )))),
        Box::new(IRData::Constant(0x0f)),
    ));
    let al_and_0fh_lt_9 = IRData::Operator(IRDataOperator::Binary(
        BinaryOperator::UnsignedLess,
        Box::new(IRData::Constant(9)),
        Box::new(al_and_0fh),
    ));
    let then = [
        IRStatement::Assignment {
            from: IRData::Operator(IRDataOperator::Binary(
                BinaryOperator::Add,
                Box::new(IRData::Intrinsic(IntrinsicType::Undefined(Box::new(
                    IRData::Register(ax.clone()),
                )))),
                Box::new(IRData::Constant(0x106)),
            )),
            to: IRData::Register(ax.clone()),
        },
        IRStatement::Assignment {
            from: IRData::Constant(1),
            to: IRData::Register(af.clone()),
        },
        IRStatement::Assignment {
            from: IRData::Constant(1),
            to: IRData::Register(cf.clone()),
        },
    ];
    let r#else = [
        IRStatement::Assignment {
            from: IRData::Constant(0),
            to: IRData::Register(af.clone()),
        },
        IRStatement::Assignment {
            from: IRData::Constant(0),
            to: IRData::Register(cf.clone()),
        },
    ];
    let after = IRStatement::Assignment {
        from: IRData::Operator(IRDataOperator::Binary(
            BinaryOperator::And,
            Box::new(IRData::Register(al.clone())),
            Box::new(IRData::Constant(0)),
        )),
        to: IRData::Register(al.clone()),
    };

    [
        IRStatement::Condition {
            condition: al_and_0fh_lt_9,
            true_branch: then.into(),
            false_branch: r#else.into(),
        },
        after,
    ]
    .into()
}
