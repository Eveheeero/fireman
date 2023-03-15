use super::Instruction;

/// 어셈블리 패턴 파싱을 위한 인스트럭션 기록
#[derive(Debug, Default)]
pub struct InstructionHistory {
    pub(crate) data: Vec<Instruction>,
}
