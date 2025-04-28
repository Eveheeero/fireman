#![allow(non_upper_case_globals, unused_imports)]

pub(super) use crate::ir::{data::*, operator::*, statements::*};
pub(super) use fireman_macro::box_to_static_reference;
use std::num::{NonZeroU16, NonZeroU8};

#[inline]
#[must_use]
pub(super) fn size_fix(data: &crate::ir::Register) -> AccessSize {
    let bit_len = data.bit_len() as u16;
    AccessSize::Fixed {
        bit_len: NonZeroU16::new(bit_len).unwrap(),
    }
}
pub(super) use size_fix as s_fix;
#[inline]
#[must_use]
pub(super) fn size_relative_register(data: &crate::ir::Register) -> AccessSize {
    AccessSize::Relative {
        with: IrData::Register(data.clone()).into(),
    }
}
pub(super) use size_relative_register as s_relative_register;
#[inline]
#[must_use]
pub(super) fn size_relative(data: impl Into<Box<IrData>>) -> AccessSize {
    AccessSize::Relative { with: data.into() }
}
pub(super) use size_relative as s_relative;
#[inline]
#[must_use]
pub(super) fn assign(
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
pub(super) fn condition_jump(
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
pub(super) fn uncondition_jump(target: impl Into<IrData>) -> IrStatement {
    IrStatement::Jump {
        target: target.into(),
    }
}
#[inline]
#[must_use]
pub(super) fn call(target: impl Into<IrData>) -> IrStatement {
    IrStatement::Call {
        target: target.into(),
    }
}
#[inline]
#[must_use]
pub(super) fn type_specified(
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
pub(super) fn architecture_byte_size_condition(
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
pub(super) fn halt() -> IrStatement {
    IrStatement::Halt
}
#[inline]
#[must_use]
pub(super) const fn undefined_behavior() -> IrStatement {
    IrStatement::Undefined
}
#[inline]
#[must_use]
pub(super) const fn exception(msg: &'static str) -> IrStatement {
    IrStatement::Exception(msg)
}

/// Register
#[inline]
#[must_use]
pub(super) fn r(r: &crate::ir::Register) -> IrData {
    IrData::Register(*r)
}
/// Operand
#[inline]
#[must_use]
pub(super) const fn o(o: u8) -> IrData {
    IrData::Operand(NonZeroU8::new(o).unwrap())
}
pub(super) const o1: IrData = o(1);
/// Relative size
#[inline]
#[must_use]
pub(super) fn o1_size() -> AccessSize {
    s_relative(o1)
}
pub(super) const o2: IrData = o(2);
/// Relative size
#[inline]
#[must_use]
pub(super) fn o2_size() -> AccessSize {
    s_relative(o2)
}
pub(super) const o3: IrData = o(3);
/// Relative size
#[inline]
#[must_use]
pub(super) fn o3_size() -> AccessSize {
    s_relative(o3)
}
/// Constant
#[inline]
#[must_use]
pub(super) const fn c(c: usize) -> IrData {
    IrData::Constant(c)
}
/// Dereference
#[inline]
#[must_use]
pub(super) fn d(d: impl Into<Box<IrData>>) -> IrData {
    IrData::Dereference(d.into())
}
#[inline]
#[must_use]
pub(super) const fn unknown_data() -> IrData {
    IrData::Intrinsic(IntrinsicType::Unknown)
}
#[inline]
#[must_use]
pub(super) const fn undefined_data() -> IrData {
    IrData::Intrinsic(IntrinsicType::Undefined)
}
/// Unary Operation
pub(super) mod u {
    use super::*;

    #[inline]
    #[must_use]
    fn transform(operator: UnaryOperator, arg: impl Into<Box<IrData>>, size: AccessSize) -> IrData {
        IrData::Operation(IrDataOperation::Unary {
            operator,
            arg: arg.into(),
            size,
        })
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn not(arg: impl Into<Box<IrData>>, size: AccessSize) -> IrData {
        transform(UnaryOperator::Not, arg, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn neg(arg: impl Into<Box<IrData>>, size: AccessSize) -> IrData {
        transform(UnaryOperator::Negation, arg, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn sign_extend(arg: impl Into<Box<IrData>>, size: AccessSize) -> IrData {
        transform(UnaryOperator::SignExtend, arg, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn zero_extend(arg: impl Into<Box<IrData>>, size: AccessSize) -> IrData {
        transform(UnaryOperator::ZeroExtend, arg, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn truncate(arg: impl Into<Box<IrData>>, size: AccessSize) -> IrData {
        transform(UnaryOperator::Truncate, arg, size)
    }
}
/// Binary Operation
pub(super) mod b {
    use super::*;

    #[inline]
    #[must_use]
    fn transform(
        operator: BinaryOperator,
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        IrData::Operation(IrDataOperation::Binary {
            operator,
            arg1: arg1.into(),
            arg2: arg2.into(),
            size,
        })
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn and(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::And, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn or(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Or, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn xor(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Xor, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn shl(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Shl, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn shr(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Shr, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn sar(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Sar, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn add(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Add, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn sub(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Sub, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn mul(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Mul, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn signed_div(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::SignedDiv, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn signed_rem(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::SignedRem, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn unsigned_div(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::UnsignedDiv, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn unsigned_rem(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::UnsignedRem, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn equal(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::Equal, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn signed_less(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::SignedLess, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn signed_less_or_euqla(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::SignedLessOrEqual, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn unsigned_less(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::UnsignedLess, arg1, arg2, size)
    }
    #[inline]
    #[must_use]
    pub(in super::super) fn unsigned_less_or_equal(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: AccessSize,
    ) -> IrData {
        transform(BinaryOperator::UnsignedLessOrEqual, arg1, arg2, size)
    }
}

#[test]
fn size_test() {
    let eax_size = size_fix(&super::super::static_register::eax);
    let rax_size = size_fix(&super::super::static_register::rax);
    assert_eq!(
        eax_size,
        AccessSize::Fixed {
            bit_len: NonZeroU16::new(4 * 8).unwrap()
        }
    );
    assert_eq!(
        rax_size,
        AccessSize::Fixed {
            bit_len: NonZeroU16::new(8 * 8).unwrap()
        }
    );
}
