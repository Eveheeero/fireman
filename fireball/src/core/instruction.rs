//! 복사할 수 있는 인스트럭션을 정의하는 모듈

/// 어셈블리 인스트럭션 정보
///
/// Capstone엔진의 Instruction은 Clone을 사용할 수 없어, 복사할 수 있는 Instruction을 만들어 사용한다.
#[derive(Debug, Clone)]
pub(crate) struct Instruction {
    /// 인스트럭션의 주소
    pub(crate) address: u64,
    /// 인스트럭션의 길이
    pub(crate) len: u8,
    /// 인스트럭션의 명령어
    pub(crate) op: String,
    /// 인스트럭션의 추가 정보
    pub(crate) mnemonic: String,
}

impl From<&capstone::Insn<'_>> for Instruction {
    fn from(insn: &capstone::Insn<'_>) -> Self {
        Instruction {
            address: insn.address(),
            len: insn.len() as u8,
            op: insn.op_str().unwrap().to_string(),
            mnemonic: insn.mnemonic().unwrap().to_string(),
        }
    }
}

impl From<&&capstone::Insn<'_>> for Instruction {
    fn from(insn: &&capstone::Insn<'_>) -> Self {
        Instruction {
            address: insn.address(),
            len: insn.len() as u8,
            op: insn.op_str().unwrap().to_string(),
            mnemonic: insn.mnemonic().unwrap().to_string(),
        }
    }
}
