use super::*;

#[inline]
#[must_use]
pub(in crate::arch) fn assign(
    from: impl Into<Arc<IrData>>,
    to: impl Into<Arc<IrData>>,
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
    condition: impl Into<Arc<IrData>>,
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
pub(in crate::arch) fn jump(target: impl Into<Arc<IrData>>) -> IrStatement {
    IrStatement::Jump {
        target: target.into(),
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn call(target: impl Into<Arc<IrData>>) -> IrStatement {
    IrStatement::Call {
        target: target.into(),
    }
}
#[inline]
#[must_use]
pub(in crate::arch) fn type_specified(
    location: impl Into<Arc<IrData>>,
    size: impl Into<AccessSize>,
    data_type: crate::ir::analyze::DataType,
) -> IrStatement {
    IrStatement::Special(IrStatementSpecial::TypeSpecified {
        location: location.into(),
        size: size.into(),
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
/// calc flags should be placed before the assignment
/// ```rust,ignore
/// let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
/// ```
#[inline]
#[must_use]
pub(in crate::arch) fn calc_flags_automatically(
    operation: impl Into<Arc<IrData>>,
    size: impl Into<AccessSize>,
    affected_registers: &[&Arc<IrData>],
) -> IrStatement {
    use crate::arch::x86_64::static_register::*;
    IrStatement::Special(IrStatementSpecial::CalcFlagsAutomatically {
        operation: operation.into(),
        size: size.into(),
        of: affected_registers.contains(&&*of),
        sf: affected_registers.contains(&&*sf),
        zf: affected_registers.contains(&&*zf),
        af: affected_registers.contains(&&*af),
        cf: affected_registers.contains(&&*cf),
        pf: affected_registers.contains(&&*pf),
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
