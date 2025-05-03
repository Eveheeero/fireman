use super::*;
use std::sync::LazyLock;

#[inline]
#[must_use]
pub(in crate::arch) fn size_result_bit(data: impl Into<Aos<IrData>>) -> AccessSize {
    let data: Aos<_> = data.into();
    let data_ptr = Aos::as_ptr(&data);
    static O1: LazyLock<Aos<IrData>> = LazyLock::new(bit_size_of_o1);
    static O2: LazyLock<Aos<IrData>> = LazyLock::new(bit_size_of_o2);
    static O3: LazyLock<Aos<IrData>> = LazyLock::new(bit_size_of_o3);
    static O4: LazyLock<Aos<IrData>> = LazyLock::new(bit_size_of_o4);
    let o1_ptr = Aos::as_ptr(&O1);
    let o2_ptr = Aos::as_ptr(&O2);
    let o3_ptr = Aos::as_ptr(&O3);
    let o4_ptr = Aos::as_ptr(&O4);
    match () {
        () if data_ptr == o1_ptr => return o1_size(),
        () if data_ptr == o2_ptr => return o2_size(),
        () if data_ptr == o3_ptr => return o3_size(),
        () if data_ptr == o4_ptr => return o4_size(),
        _ => {}
    }
    AccessSize::ResultOfBit(data)
}
#[inline]
#[must_use]
pub(in crate::arch) fn size_result_byte(data: impl Into<Aos<IrData>>) -> AccessSize {
    let data: Aos<_> = data.into();
    let data_ptr = Aos::as_ptr(&data);
    static O1: LazyLock<Aos<IrData>> = LazyLock::new(byte_size_of_o1);
    static O2: LazyLock<Aos<IrData>> = LazyLock::new(byte_size_of_o2);
    static O3: LazyLock<Aos<IrData>> = LazyLock::new(byte_size_of_o3);
    static O4: LazyLock<Aos<IrData>> = LazyLock::new(byte_size_of_o4);
    let o1_ptr = Aos::as_ptr(&O1);
    let o2_ptr = Aos::as_ptr(&O2);
    let o3_ptr = Aos::as_ptr(&O3);
    let o4_ptr = Aos::as_ptr(&O4);
    match () {
        () if data_ptr == o1_ptr => return o1_size(),
        () if data_ptr == o2_ptr => return o2_size(),
        () if data_ptr == o3_ptr => return o3_size(),
        () if data_ptr == o4_ptr => return o4_size(),
        _ => {}
    }
    AccessSize::ResultOfByte(data)
}
#[test]
fn size_result_singleton_test() {
    let extract_arc = |x: AccessSize| match x {
        AccessSize::RelativeWith(ir_data) => return ir_data,
        _ => unreachable!(),
    };

    let l = Aos::as_ptr(&extract_arc(size_result_bit(bit_size_of_o1()))).addr();
    let r = Aos::as_ptr(&extract_arc(o1_size())).addr();
    dbg!(l, r);
    assert_eq!(l, r);

    let l = Aos::as_ptr(&extract_arc(size_result_bit(bit_size_of_o1()))).addr();
    let r = Aos::as_ptr(&extract_arc(o1_size())).addr();
    dbg!(l, r);
    assert_eq!(l, r);

    let l = Aos::as_ptr(&extract_arc(size_result_bit(bit_size_of_o2()))).addr();
    let r = Aos::as_ptr(&extract_arc(o3_size())).addr();
    dbg!(l, r);
    assert_ne!(l, r);

    let l = Aos::as_ptr(&extract_arc(size_result_bit(bit_size_of_o3()))).addr();
    let r = Aos::as_ptr(&extract_arc(o3_size())).addr();
    dbg!(l, r);
    assert_eq!(l, r);

    let l = Aos::as_ptr(&extract_arc(size_result_byte(byte_size_of_o1()))).addr();
    let r = Aos::as_ptr(&extract_arc(o1_size())).addr();
    dbg!(l, r);
    assert_eq!(l, r);
}
#[inline]
#[must_use]
pub(in crate::arch) fn size_relative(data: impl Into<Aos<IrData>>) -> AccessSize {
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
fn o(o: u8) -> Aos<IrData> {
    IrData::Operand(NonZeroU8::new(o).unwrap()).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn o1() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| o(1));
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
pub(in crate::arch) fn o2() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| o(2));
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
pub(in crate::arch) fn o3() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| o(3));
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
pub(in crate::arch) fn o4() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| o(4));
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
pub(in crate::arch) fn c(c: usize) -> Aos<IrData> {
    static C0: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(0)));
    static C1: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(1)));
    static C2: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(2)));
    static C3: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(3)));
    static C4: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(4)));
    static C5: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(5)));
    static C6: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(6)));
    static C7: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(7)));
    static C8: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(8)));
    static C9: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(9)));
    static C10: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(10)));
    static C11: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(11)));
    static C12: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(12)));
    static C13: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(13)));
    static C14: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(14)));
    static C15: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(15)));
    static C16: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(16)));
    static C32: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(32)));
    static C64: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(64)));
    static C128: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(128)));
    static C256: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(256)));
    static C512: LazyLock<Aos<IrData>> = LazyLock::new(|| Aos::new_static(IrData::Constant(512)));
    match () {
        () if c == 0 => return C0.clone(),
        () if c == 1 => return C1.clone(),
        () if c == 2 => return C2.clone(),
        () if c == 3 => return C3.clone(),
        () if c == 4 => return C4.clone(),
        () if c == 5 => return C5.clone(),
        () if c == 6 => return C6.clone(),
        () if c == 7 => return C7.clone(),
        () if c == 8 => return C8.clone(),
        () if c == 9 => return C9.clone(),
        () if c == 10 => return C10.clone(),
        () if c == 11 => return C11.clone(),
        () if c == 12 => return C12.clone(),
        () if c == 13 => return C13.clone(),
        () if c == 14 => return C14.clone(),
        () if c == 15 => return C15.clone(),
        () if c == 16 => return C16.clone(),
        () if c == 32 => return C32.clone(),
        () if c == 64 => return C64.clone(),
        () if c == 128 => return C128.clone(),
        () if c == 256 => return C256.clone(),
        () if c == 512 => return C512.clone(),
        _ => {}
    }
    IrData::Constant(c).into()
}
/// Dereference
#[inline]
#[must_use]
pub(in crate::arch) fn d(d: impl Into<Aos<IrData>>) -> Aos<IrData> {
    IrData::Dereference(d.into()).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn unknown_data() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::Unknown)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn undefined_data() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::Undefined)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn signed_max(size: impl Into<AccessSize>) -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::SignedMax(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn signed_min(size: impl Into<AccessSize>) -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::SignedMin(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn unsigned_max(size: impl Into<AccessSize>) -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::UnsignedMax(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn unsigned_min(size: impl Into<AccessSize>) -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::UnsignedMin(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_ones(size: impl Into<AccessSize>) -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::BitOnes(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_zeros(size: impl Into<AccessSize>) -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::BitZeros(size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn architecture_bit_size() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::ArchitectureBitSize)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn architecture_byte_size() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::ArchitectureByteSize)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn architecture_bit_per_byte() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::ArchitectureBitPerByte)));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn instruction_byte_size() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::InstructionByteSize)));
    ONCE.clone()
}
#[must_use]
pub(in crate::arch) fn byte_size_of_data(data: impl Into<Aos<IrData>>) -> Aos<IrData> {
    let data: Aos<_> = data.into();
    let data_ptr = Aos::as_ptr(&data);
    static O1: LazyLock<Aos<IrData>> = LazyLock::new(o1);
    static O2: LazyLock<Aos<IrData>> = LazyLock::new(o2);
    static O3: LazyLock<Aos<IrData>> = LazyLock::new(o3);
    static O4: LazyLock<Aos<IrData>> = LazyLock::new(o4);
    let o1_ptr = Aos::as_ptr(&O1);
    let o2_ptr = Aos::as_ptr(&O2);
    let o3_ptr = Aos::as_ptr(&O3);
    let o4_ptr = Aos::as_ptr(&O4);
    match () {
        () if data_ptr == o1_ptr => return byte_size_of_o1(),
        () if data_ptr == o2_ptr => return byte_size_of_o2(),
        () if data_ptr == o3_ptr => return byte_size_of_o3(),
        () if data_ptr == o4_ptr => return byte_size_of_o4(),
        _ => {}
    }
    IrData::Intrinsic(IrIntrinsic::ByteSizeOf(data)).into()
}
#[must_use]
pub(in crate::arch) fn bit_size_of_data(data: impl Into<Aos<IrData>>) -> Aos<IrData> {
    let data: Aos<_> = data.into();
    let data_ptr = Aos::as_ptr(&data);
    static O1: LazyLock<Aos<IrData>> = LazyLock::new(o1);
    static O2: LazyLock<Aos<IrData>> = LazyLock::new(o2);
    static O3: LazyLock<Aos<IrData>> = LazyLock::new(o3);
    static O4: LazyLock<Aos<IrData>> = LazyLock::new(o4);
    let o1_ptr = Aos::as_ptr(&O1);
    let o2_ptr = Aos::as_ptr(&O2);
    let o3_ptr = Aos::as_ptr(&O3);
    let o4_ptr = Aos::as_ptr(&O4);
    match () {
        () if data_ptr == o1_ptr => return bit_size_of_o1(),
        () if data_ptr == o2_ptr => return bit_size_of_o2(),
        () if data_ptr == o3_ptr => return bit_size_of_o3(),
        () if data_ptr == o4_ptr => return bit_size_of_o4(),
        _ => {}
    }
    IrData::Intrinsic(IrIntrinsic::BitSizeOf(data)).into()
}
#[test]
fn byte_size_of_data_singleton_test() {
    let l = Aos::as_ptr(&byte_size_of_data(o1())).addr();
    let r = Aos::as_ptr(&byte_size_of_o1()).addr();
    dbg!(l, r);
    assert_eq!(l, r);

    let l = Aos::as_ptr(&byte_size_of_data(o1())).addr();
    let r = Aos::as_ptr(&byte_size_of_o1()).addr();
    dbg!(l, r);
    assert_eq!(l, r);

    let l = Aos::as_ptr(&byte_size_of_data(o2())).addr();
    let r = Aos::as_ptr(&byte_size_of_o3()).addr();
    dbg!(l, r);
    assert_ne!(l, r);

    let l = Aos::as_ptr(&byte_size_of_data(o3())).addr();
    let r = Aos::as_ptr(&byte_size_of_o3()).addr();
    dbg!(l, r);
    assert_eq!(l, r);
}
#[inline]
#[must_use]
pub(in crate::arch) fn byte_size_of_o1() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::ByteSizeOf(o1()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn byte_size_of_o2() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::ByteSizeOf(o2()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn byte_size_of_o3() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::ByteSizeOf(o3()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn byte_size_of_o4() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::ByteSizeOf(o4()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_size_of_o1() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::BitSizeOf(o1()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_size_of_o2() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::BitSizeOf(o2()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_size_of_o3() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::BitSizeOf(o3()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn bit_size_of_o4() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> =
        LazyLock::new(|| Aos::new_static(IrData::Intrinsic(IrIntrinsic::BitSizeOf(o4()))));
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn sized(
    data: impl Into<Aos<IrData>>,
    size: impl Into<AccessSize>,
) -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::Sized(data.into(), size.into())).into()
}
#[inline]
#[must_use]
pub(in crate::arch) fn is_o1_exists() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| {
        Aos::new_static(IrData::Intrinsic(IrIntrinsic::OperandExists(
            1.try_into().unwrap(),
        )))
    });
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn is_o2_exists() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| {
        Aos::new_static(IrData::Intrinsic(IrIntrinsic::OperandExists(
            2.try_into().unwrap(),
        )))
    });
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn is_o3_exists() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| {
        Aos::new_static(IrData::Intrinsic(IrIntrinsic::OperandExists(
            3.try_into().unwrap(),
        )))
    });
    ONCE.clone()
}
#[inline]
#[must_use]
pub(in crate::arch) fn is_o4_exists() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| {
        Aos::new_static(IrData::Intrinsic(IrIntrinsic::OperandExists(
            4.try_into().unwrap(),
        )))
    });
    ONCE.clone()
}
/// Unary Operation
pub(in crate::arch) mod u {
    use super::*;

    #[inline]
    #[must_use]
    fn transform(operator: UnaryOperator, arg: impl Into<Aos<IrData>>) -> Aos<IrData> {
        IrData::Operation(IrDataOperation::Unary {
            operator,
            arg: arg.into(),
        })
        .into()
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn not(arg: impl Into<Aos<IrData>>) -> Aos<IrData> {
        transform(UnaryOperator::Not, arg)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn neg(arg: impl Into<Aos<IrData>>) -> Aos<IrData> {
        transform(UnaryOperator::Negation, arg)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn sign_extend(arg: impl Into<Aos<IrData>>) -> Aos<IrData> {
        transform(UnaryOperator::SignExtend, arg)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn zero_extend(arg: impl Into<Aos<IrData>>) -> Aos<IrData> {
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
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
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
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::And, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn or(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Or, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn xor(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Xor, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn shl(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Shl, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn shr(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Shr, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn sar(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Sar, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn add(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Add, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn sub(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Sub, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn mul(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Mul, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn signed_div(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::SignedDiv, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn signed_rem(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::SignedRem, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn unsigned_div(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::UnsignedDiv, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn unsigned_rem(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::UnsignedRem, arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn equal(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::Equal(size.into()), arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn signed_less(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::SignedLess(size.into()), arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn signed_less_or_euqla(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::SignedLessOrEqual(size.into()), arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn unsigned_less(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::UnsignedLess(size.into()), arg1, arg2)
    }
    #[inline]
    #[must_use]
    pub(in crate::arch) fn unsigned_less_or_equal(
        arg1: impl Into<Aos<IrData>>,
        arg2: impl Into<Aos<IrData>>,
        size: impl Into<AccessSize>,
    ) -> Aos<IrData> {
        transform(BinaryOperator::UnsignedLessOrEqual(size.into()), arg1, arg2)
    }
}
