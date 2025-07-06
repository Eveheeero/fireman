use super::*;

#[inline]
#[must_use]
pub(in crate::arch) fn assign(
    from: impl Into<Aos<IrData>>,
    to: impl Into<Aos<IrData>>,
    size: impl Into<AccessSize>,
) -> IrStatement {
    IrStatement::Assignment {
        from: from.into(),
        to: to.into(),
        size: size.into(),
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn condition(
    condition: impl Into<Aos<IrData>>,
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
pub(in crate::arch) fn jump(target: impl Into<Aos<IrData>>) -> IrStatement {
    IrStatement::Jump {
        target: target.into(),
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn jump_by_call(target: impl Into<Aos<IrData>>) -> IrStatement {
    IrStatement::JumpByCall {
        target: target.into(),
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn type_specified(
    location: impl Into<Aos<IrData>>,
    size: impl Into<AccessSize>,
    data_type: crate::ir::low_ir::analyze::DataType,
) -> IrStatement {
    IrStatement::Special(IrStatementSpecial::TypeSpecified {
        location: location.into(),
        size: size.into(),
        data_type,
    })
}
/// calc flags should be placed before the assignment
/// ```rust,ignore
/// let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
/// ```
#[inline]
#[must_use]
pub(in crate::arch) fn calc_flags_automatically(
    operation: impl Into<Aos<IrData>>,
    size: impl Into<AccessSize>,
    affected_registers: &[&Aos<IrData>],
) -> IrStatement {
    use crate::arch::x86_64::static_register::*;
    IrStatement::Special(IrStatementSpecial::CalcFlagsAutomatically {
        operation: operation.into(),
        size: size.into(),
        flags: affected_registers
            .into_iter()
            .map(|x| (*x).clone())
            .collect(),
    })
}
#[inline]
#[must_use]
pub(in crate::arch) const fn halt() -> IrStatement {
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
#[inline]
#[must_use]
pub(in crate::arch) fn assertion(condition: impl Into<Aos<IrData>>) -> IrStatement {
    IrStatement::Special(IrStatementSpecial::Assertion {
        condition: condition.into(),
    })
}
