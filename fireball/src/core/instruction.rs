/// Capstone엔진의 Instruction은 Clone을 사용할 수 없어, 복사할 수 있는 Instruction을 만들어 사용합니다.
#[derive(Debug, Clone)]
pub(crate) struct Instruction {
    pub(crate) address: u64,
    pub(crate) len: u8,
    pub(crate) op: String,
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
