//! 기본적으로 사용되는 use문 등이 들어가는 모듈

#![allow(unused_imports)]

pub(crate) use crate::utils::error::decompile_error::DecompileError;
pub(crate) use crate::utils::error::disassemble_error::DisassembleError;
pub(crate) use crate::utils::error::io_error::IoError;
pub(crate) use crate::utils::error::FireballError;
pub(crate) use tracing::{debug, error, info, trace, warn};

pub(crate) type BitBox = bitvec::prelude::BitBox<usize>;
pub(crate) type BitSlice = bitvec::prelude::BitSlice<usize>;

#[cfg(test)]
pub(crate) fn test_init() {
    use tracing_subscriber::{
        prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    };

    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false),
            )
            .with(
                tracing_subscriber::filter::Targets::new()
                    .with_target("fireball", tracing::Level::TRACE),
            )
            .try_init();
    });
}

#[cfg(test)]
mod tests {
    /// bitvec 내부 구현이 Bit 어떤 순서대로 저장되는지 테스트
    /// 데이터 조작 후 캐스팅을 위한 방법 포함.
    #[test]
    fn bit_align_test_msb0() {
        /* BitPtr형태로 변환하여 Cast를 사용하는 것은 불안정하기 떄문에, 해당 연산을 사용해야 합니다. */
        let mut bits = bitvec::prelude::BitVec::<usize, bitvec::order::Msb0>::new();
        bits.resize(8, false);
        let mut bits = bits.into_boxed_bitslice();
        bits.set(7, true);
        let data = bits[0..8].iter().fold(0, |acc, x| (acc << 1) | *x as u8);
        assert_eq!(data, 0b00000001)
    }

    /// bitvec 내부 구현이 Bit 어떤 순서대로 저장되는지 테스트
    /// 데이터 조작 후 캐스팅을 위한 방법 포함.
    #[test]
    fn bit_align_test_lsb0() {
        /* BitPtr형태로 변환하여 Cast를 사용하는 것은 불안정하기 떄문에, 해당 연산을 사용해야 합니다. */
        let mut bits = bitvec::prelude::BitVec::<usize>::new();
        bits.resize(8, false);
        let mut bits = bits.into_boxed_bitslice();
        bits.set(7, true);
        let data = bits[0..8]
            .iter()
            .rev()
            .fold(0, |acc, x| (acc << 1) | *x as u8);
        assert_eq!(data, 0b10000000)
    }
}
