pub(super) use crate::ir::{data::*, operator::*, statements::*};
use std::num::{NonZeroU16, NonZeroU8};

/// return size of register (byte)
#[inline]
pub(super) fn size(data: &crate::ir::Register) -> Option<NonZeroU16> {
    let bit_len = data.bit_len() as u16;
    let byte_len = bit_len / 8;
    NonZeroU16::new(byte_len)
}
/// Register
#[inline]
pub(super) fn r(r: &crate::ir::Register) -> IrData {
    IrData::Register(r.clone())
}
/// Operand
#[inline]
pub(super) const fn o(o: u8) -> IrData {
    IrData::Operand(NonZeroU8::new(o).unwrap())
}
/// Constant
#[inline]
pub(super) const fn c(c: usize) -> IrData {
    IrData::Constant(c)
}
/// Dereference
#[inline]
pub(super) fn d(d: impl Into<Box<IrData>>) -> IrData {
    IrData::Dereference(d.into())
}
#[inline]
pub(super) fn unknown(u: impl Into<Box<IrData>>) -> IrData {
    IrData::Intrinsic(IntrinsicType::Unknown(u.into()))
}
#[inline]
pub(super) fn undefined(u: impl Into<Box<IrData>>) -> IrData {
    IrData::Intrinsic(IntrinsicType::Undefined(u.into()))
}
#[inline]
pub(super) fn ret(r: impl Into<Box<IrData>>) -> IrData {
    IrData::Intrinsic(IntrinsicType::ReturnAddress(r.into()))
}
/// Unary Operation
pub(super) mod u {
    use super::*;

    #[inline]
    fn transform(
        operator: UnaryOperator,
        arg: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        IrData::Operation(IrDataOperation::Unary {
            operator,
            arg: arg.into(),
            size: size.into().0,
        })
    }
    #[inline]
    pub(in super::super) fn not(
        arg: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(UnaryOperator::Not, arg, size)
    }
    #[inline]
    pub(in super::super) fn neg(
        arg: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(UnaryOperator::Negation, arg, size)
    }
    #[inline]
    pub(in super::super) fn sign_extend(
        arg: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(UnaryOperator::SignExtend, arg, size)
    }
    #[inline]
    pub(in super::super) fn zero_extend(
        arg: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(UnaryOperator::ZeroExtend, arg, size)
    }
    #[inline]
    pub(in super::super) fn truncate(
        arg: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(UnaryOperator::Truncate, arg, size)
    }
}
/// Binary Operation
pub(super) mod b {
    use super::*;

    #[inline]
    fn transform(
        operator: BinaryOperator,
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        IrData::Operation(IrDataOperation::Binary {
            operator,
            arg1: arg1.into(),
            arg2: arg2.into(),
            size: size.into().0,
        })
    }
    #[inline]
    pub(in super::super) fn and(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::And, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn or(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Or, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn xor(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Xor, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn shl(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Shl, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn shr(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Shr, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn sar(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Sar, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn add(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Add, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn sub(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Sub, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn mul(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Mul, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn signed_div(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::SignedDiv, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn signed_rem(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::SignedRem, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn unsigned_div(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::UnsignedDiv, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn unsigned_rem(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::UnsignedRem, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn equal(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::Equal, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn signed_less(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::SignedLess, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn signed_less_or_euqla(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::SignedLessOrEqual, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn unsigned_less(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::UnsignedLess, arg1, arg2, size)
    }
    #[inline]
    pub(in super::super) fn unsigned_less_or_equal(
        arg1: impl Into<Box<IrData>>,
        arg2: impl Into<Box<IrData>>,
        size: impl Into<IntoNonZeroU16>,
    ) -> IrData {
        transform(BinaryOperator::UnsignedLessOrEqual, arg1, arg2, size)
    }
}

pub(super) struct IntoNonZeroU16(Option<NonZeroU16>);
impl Into<IntoNonZeroU16> for u16 {
    #[inline]
    fn into(self) -> IntoNonZeroU16 {
        IntoNonZeroU16(NonZeroU16::new(self))
    }
}
impl Into<IntoNonZeroU16> for Option<NonZeroU16> {
    #[inline]
    fn into(self) -> IntoNonZeroU16 {
        IntoNonZeroU16(self)
    }
}

#[test]
fn size_test() {
    let eax_size = size(&super::super::static_register::eax);
    let rax_size = size(&super::super::static_register::rax);
    assert_eq!(eax_size, NonZeroU16::new(4));
    assert_eq!(rax_size, NonZeroU16::new(8));
}
