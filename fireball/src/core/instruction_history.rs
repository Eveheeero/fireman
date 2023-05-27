//! 분석 중, 여태까지 실행했던 인스트럭션에 대한 기록을 가지고 있는 구조체를 정의하는 모듈

use super::Instruction;

/// 어셈블리 패턴 파싱을 위한 인스트럭션 기록
#[derive(Debug, Default)]
pub struct InstructionHistory {
    pub(crate) data: Vec<Instruction>,
}
