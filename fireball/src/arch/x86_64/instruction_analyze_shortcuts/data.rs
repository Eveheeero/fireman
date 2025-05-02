use super::*;
use std::sync::LazyLock;

#[inline]
#[must_use]
pub(in crate::arch) fn size_result_bit(data: impl Into<Arc<IrData>>) -> AccessSize {
    AccessSize::ResultOfBit(data.into())
}
#[inline]
#[must_use]
pub(in crate::arch) fn size_result_byte(data: impl Into<Arc<IrData>>) -> AccessSize {
    AccessSize::ResultOfByte(data.into())
}
#[inline]
#[must_use]
pub(in crate::arch) fn size_relative(data: impl Into<Arc<IrData>>) -> AccessSize {
    AccessSize::RelativeWith(data.into())
}
#[inline]
#[must_use]
pub(in crate::arch) fn size_architecture() -> AccessSize {
    AccessSize::ArchitectureSize
}
#[inline]
#[must_use]
pub(in crate::arch) fn size_unlimited() -> AccessSize {
    AccessSize::Unlimited
}

/// Operand
#[inline]
#[must_use]
fn o(o: u8) -> Arc<IrData> {
    IrData::Operand(NonZeroU8::new(o).unwrap()).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn o1() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> = LazyLock::new(|| o(1));
    ONCE.clone()
}
/// Relative size
#[inline]
#[must_use]
pub(in crate::arch) fn o1_size() -> AccessSize {
    size_relative(o1())
}
#[inline]
#[must_use]
pub(in crate::arch) fn o2() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> = LazyLock::new(|| o(2));
    ONCE.clone()
}
/// Relative size
#[inline]
#[must_use]
pub(in crate::arch) fn o2_size() -> AccessSize {
    size_relative(o2())
}
#[inline]
#[must_use]
pub(in crate::arch) fn o3() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> = LazyLock::new(|| o(3));
    ONCE.clone()
}
/// Relative size
#[inline]
#[must_use]
pub(in crate::arch) fn o3_size() -> AccessSize {
    size_relative(o3())
}
#[inline]
#[must_use]
pub(in crate::arch) fn o4() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> = LazyLock::new(|| o(4));
    ONCE.clone()
}
/// Relative size
#[inline]
#[must_use]
pub(in crate::arch) fn o4_size() -> AccessSize {
    size_relative(o4())
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
#[inline]
#[must_use]
pub(in crate::arch) fn architecture_bit_size() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::ArchitectureBitSize)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn architecture_byte_size() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::ArchitectureByteSize)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn architecture_bit_per_byte() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::ArchitectureBitPerByte)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn instruction_byte_size() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::InstructionByteSize)));
    ONCE.clone()
}
#[must_use]
pub(in crate::arch) fn byte_size_of_data(data: impl Into<Arc<IrData>>) -> Arc<IrData> {
    let data: Arc<_> = data.into();
    let data_ptr = Arc::as_ptr(&data);
    static O1: LazyLock<Arc<IrData>> = LazyLock::new(|| o1());
    static O2: LazyLock<Arc<IrData>> = LazyLock::new(|| o2());
    static O3: LazyLock<Arc<IrData>> = LazyLock::new(|| o3());
    static O4: LazyLock<Arc<IrData>> = LazyLock::new(|| o4());
    let o1_ptr = Arc::as_ptr(&O1);
    let o2_ptr = Arc::as_ptr(&O2);
    let o3_ptr = Arc::as_ptr(&O3);
    let o4_ptr = Arc::as_ptr(&O4);
    match () {
        () if data_ptr == o1_ptr => return byte_size_of_o1(),
        () if data_ptr == o2_ptr => return byte_size_of_o2(),
        () if data_ptr == o3_ptr => return byte_size_of_o3(),
        () if data_ptr == o4_ptr => return byte_size_of_o4(),
        _ => {}
    }
    IrData::Intrinsic(IntrinsicType::ByteSizeOf(data)).into()
}
#[must_use]
pub(in crate::arch) fn bit_size_of_data(data: impl Into<Arc<IrData>>) -> Arc<IrData> {
    let data: Arc<_> = data.into();
    let data_ptr = Arc::as_ptr(&data);
    static O1: LazyLock<Arc<IrData>> = LazyLock::new(|| o1());
    static O2: LazyLock<Arc<IrData>> = LazyLock::new(|| o2());
    static O3: LazyLock<Arc<IrData>> = LazyLock::new(|| o3());
    static O4: LazyLock<Arc<IrData>> = LazyLock::new(|| o4());
    let o1_ptr = Arc::as_ptr(&O1);
    let o2_ptr = Arc::as_ptr(&O2);
    let o3_ptr = Arc::as_ptr(&O3);
    let o4_ptr = Arc::as_ptr(&O4);
    match () {
        () if data_ptr == o1_ptr => return bit_size_of_o1(),
        () if data_ptr == o2_ptr => return bit_size_of_o2(),
        () if data_ptr == o3_ptr => return bit_size_of_o3(),
        () if data_ptr == o4_ptr => return bit_size_of_o4(),
        _ => {}
    }
    IrData::Intrinsic(IntrinsicType::BitSizeOf(data)).into()
}
#[test]
fn byte_size_of_data_test() {
    let l = Arc::as_ptr(&byte_size_of_data(o1())).addr();
    let r = Arc::as_ptr(&byte_size_of_o1()).addr();
    dbg!(l, r);
    assert_eq!(l, r);

    let l = Arc::as_ptr(&byte_size_of_data(o1())).addr();
    let r = Arc::as_ptr(&byte_size_of_o1()).addr();
    dbg!(l, r);
    assert_eq!(l, r);

    let l = Arc::as_ptr(&byte_size_of_data(o2())).addr();
    let r = Arc::as_ptr(&byte_size_of_o3()).addr();
    dbg!(l, r);
    assert_ne!(l, r);

    let l = Arc::as_ptr(&byte_size_of_data(o3())).addr();
    let r = Arc::as_ptr(&byte_size_of_o3()).addr();
    dbg!(l, r);
    assert_eq!(l, r);
}
#[inline]
#[must_use]
pub(in crate::arch) fn byte_size_of_o1() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::ByteSizeOf(o1()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn byte_size_of_o2() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::ByteSizeOf(o2()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn byte_size_of_o3() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::ByteSizeOf(o3()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn byte_size_of_o4() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::ByteSizeOf(o4()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_size_of_o1() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::BitSizeOf(o1()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_size_of_o2() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::BitSizeOf(o2()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_size_of_o3() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::BitSizeOf(o3()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_size_of_o4() -> Arc<IrData> {
    static ONCE: LazyLock<Arc<IrData>> =
        LazyLock::new(|| Arc::new(IrData::Intrinsic(IntrinsicType::BitSizeOf(o4()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn sized(
    data: impl Into<Arc<IrData>>,
    size: impl Into<AccessSize>,
) -> Arc<IrData> {
    IrData::Intrinsic(IntrinsicType::Sized(data.into(), size.into())).into()
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
