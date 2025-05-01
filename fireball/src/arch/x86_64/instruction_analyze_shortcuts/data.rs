use super::*;
use std::sync::LazyLock;

#[inline]
#[must_use]
pub(in crate::arch) fn size_fix(data: &crate::ir::Register) -> AccessSize {
    let bit_len = data.bit_len() as u16;
    AccessSize::Fixed {
        bit_len: NonZeroU16::new(bit_len).unwrap(),
    }
}
pub(in crate::arch) use size_fix as s_fix;
#[inline]
#[must_use]
pub(in crate::arch) fn size_relative_register(data: &crate::ir::Register) -> AccessSize {
    AccessSize::Relative {
        with: IrData::Register(data.clone()).into(),
    }
}
pub(in crate::arch) use size_relative_register as s_relative_register;
#[inline]
#[must_use]
pub(in crate::arch) fn size_relative(data: impl Into<Arc<IrData>>) -> AccessSize {
    AccessSize::Relative { with: data.into() }
}
pub(in crate::arch) use size_relative as s_relative;
#[inline]
#[must_use]
pub(in crate::arch) fn size_unlimited() -> AccessSize {
    AccessSize::Unlimited
}

/// Register
#[inline]
#[must_use]
pub(in crate::arch) fn r(r: &crate::ir::Register) -> Arc<IrData> {
    IrData::Register(*r).into()
}
/// Operand
#[inline]
#[must_use]
pub(in crate::arch) fn o(o: u8) -> Arc<IrData> {
    IrData::Operand(NonZeroU8::new(o).unwrap()).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn o1() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Operand(NonZeroU8::new(1).unwrap())));
    ONCE.clone()
}
/// Relative size
#[inline]
#[must_use]
pub(in crate::arch) fn o1_size() -> AccessSize {
    s_relative(o1())
}
#[inline]
#[must_use]
pub(in crate::arch) fn o2() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Operand(NonZeroU8::new(2).unwrap())));
    ONCE.clone()
}
/// Relative size
#[inline]
#[must_use]
pub(in crate::arch) fn o2_size() -> AccessSize {
    s_relative(o2())
}
#[inline]
#[must_use]
pub(in crate::arch) fn o3() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Operand(NonZeroU8::new(3).unwrap())));
    ONCE.clone()
}
/// Relative size
#[inline]
#[must_use]
pub(in crate::arch) fn o3_size() -> AccessSize {
    s_relative(o3())
}
/// Constant
#[inline]
#[must_use]
pub(in crate::arch) fn c(c: usize) -> Arc<IrData> {
    IrData::Constant(c).into()
}
/// Dereference
#[inline]
#[must_use]
pub(in crate::arch) fn d(d: impl Into<Arc<IrData>>) -> Arc<IrData> {
    IrData::Dereference(d.into()).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn unknown_data() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::Unknown)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn undefined_data() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::Undefined)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn signed_max(size: impl Into<AccessSize>) -> Arc<IrData> {
    IrData::Intrinsic(IntrinsicType::SignedMax(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn signed_min(size: impl Into<AccessSize>) -> Arc<IrData> {
    IrData::Intrinsic(IntrinsicType::SignedMin(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn unsigned_max(size: impl Into<AccessSize>) -> Arc<IrData> {
    IrData::Intrinsic(IntrinsicType::UnsignedMax(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn unsigned_min(size: impl Into<AccessSize>) -> Arc<IrData> {
    IrData::Intrinsic(IntrinsicType::UnsignedMin(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_ones(size: impl Into<AccessSize>) -> Arc<IrData> {
    IrData::Intrinsic(IntrinsicType::BitOnes(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_zeros(size: impl Into<AccessSize>) -> Arc<IrData> {
    IrData::Intrinsic(IntrinsicType::BitZeros(size.into())).into()
}
/// Unary Operation
pub(in crate::arch) mod u {
    use super::*;

    #[inline]
    #[must_use]
    fn transform(operator: UnaryOperator, arg: impl Into<Arc<IrData>>) -> Arc<IrData> {
        IrData::Operation(IrDataOperation::Unary {
            operator,
            arg: arg.into(),
        })
        .into()
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn not(arg: impl Into<Arc<IrData>>) -> Arc<IrData> {
        transform(UnaryOperator::Not, arg)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn neg(arg: impl Into<Arc<IrData>>) -> Arc<IrData> {
        transform(UnaryOperator::Negation, arg)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn sign_extend(arg: impl Into<Arc<IrData>>) -> Arc<IrData> {
        transform(UnaryOperator::SignExtend, arg)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn zero_extend(arg: impl Into<Arc<IrData>>) -> Arc<IrData> {
        transform(UnaryOperator::ZeroExtend, arg)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn truncate(arg: impl Into<Arc<IrData>>) -> Arc<IrData> {
        transform(UnaryOperator::Truncate, arg)
    }
}
/// Binary Operation
pub(in crate::arch) mod b {
    use super::*;

    #[inline]
    #[must_use]
    fn transform(
        operator: BinaryOperator,
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        IrData::Operation(IrDataOperation::Binary {
            operator,
            arg1: arg1.into(),
            arg2: arg2.into(),
        })
        .into()
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn and(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::And, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn or(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Or, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn xor(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Xor, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn shl(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Shl, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn shr(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Shr, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn sar(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Sar, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn add(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Add, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn sub(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Sub, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn mul(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Mul, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn signed_div(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::SignedDiv, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn signed_rem(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::SignedRem, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn unsigned_div(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::UnsignedDiv, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn unsigned_rem(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::UnsignedRem, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn equal(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::Equal(size.into()), arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn signed_less(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::SignedLess(size.into()), arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn signed_less_or_euqla(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::SignedLessOrEqual(size.into()), arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn unsigned_less(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::UnsignedLess(size.into()), arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn unsigned_less_or_equal(
        arg1: impl Into<Arc<IrData>>,
        arg2: impl Into<Arc<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Arc<IrData> {
        transform(BinaryOperator::UnsignedLessOrEqual(size.into()), arg1, arg2)
    }
}

#[test]
fn size_test() {
    let eax_size = size_fix(&crate::arch::x86_64::static_register::eax);
    let rax_size = size_fix(&crate::arch::x86_64::static_register::rax);
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
