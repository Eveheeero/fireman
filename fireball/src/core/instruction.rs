//! 복사할 수 있는 인스트럭션을 정의하는 모듈

/// 어셈블리 인스트럭션 정보
///
/// Capstone엔진의 Instruction은 Clone을 사용할 수 없어, 복사할 수 있는 Instruction을 만들어 사용한다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    /// 인스트럭션의 주소
    pub(crate) address: u64,
    /// 인스트럭션의 길이
    pub(crate) inner: iceball::Instruction,
}

impl Instruction {
    pub fn inner(&self) -> &iceball::Instruction {
        &self.inner
    }
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X} {}", self.address, self.inner)
    }
}
