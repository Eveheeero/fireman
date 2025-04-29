use super::*;

#[inline]
#[must_use]
pub(in crate::arch) fn assign(
    from: impl Into<IrData>,
    to: impl Into<IrData>,
    size: AccessSize,
) -> IrStatement {
    IrStatement::Assignment {
        from: from.into(),
        to: to.into(),
        size,
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn condition_jump(
    condition: impl Into<IrData>,
    true_branch: impl Into<Box<[IrStatement]>>,
    false_branch: impl Into<Box<[IrStatement]>>,
) -> IrStatement {
    IrStatement::Condition {
        condition: condition.into(),
        true_branch: true_branch.into(),
        false_branch: false_branch.into(),
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn uncondition_jump(target: impl Into<IrData>) -> IrStatement {
    IrStatement::Jump {
        target: target.into(),
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn call(target: impl Into<IrData>) -> IrStatement {
    IrStatement::Call {
        target: target.into(),
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn type_specified(
    location: impl Into<IrData>,
    size: AccessSize,
    data_type: crate::ir::analyze::DataType,
) -> IrStatement {
    IrStatement::Special(IrStatementSpecial::TypeSpecified {
        location: location.into(),
        size,
        data_type,
    })
}
#[inline]
#[must_use]
pub(in crate::arch) fn architecture_byte_size_condition(
    condition: NumCondition,
    true_branch: impl Into<Box<[IrStatement]>>,
    false_branch: impl Into<Box<[IrStatement]>>,
) -> IrStatement {
    IrStatement::Special(IrStatementSpecial::ArchitectureByteSizeCondition {
        condition,
        true_branch: true_branch.into(),
        false_branch: false_branch.into(),
    })
}
#[inline]
#[must_use]
pub(in crate::arch) fn halt() -> IrStatement {
    IrStatement::Halt
}
#[inline]
#[must_use]
pub(in crate::arch) const fn undefined_behavior() -> IrStatement {
    IrStatement::Undefined
}
#[inline]
#[must_use]
pub(in crate::arch) const fn exception(msg: &'static str) -> IrStatement {
    IrStatement::Exception(msg)
}
