//! IR의 각 명령이 담겨져 있는 모듈

use super::data::IRData;
use crate::core::Instruction;

/// IR의 각 명령에 대한 Enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRStatement {
    /// 해석할 수 없는 명령, 인라인 어셈블리로 처리됩니다.
    Unknown(IRStatementUnknown),
    /// 변수 할당
    Assignment { from: IRData, to: IRData },
    /// 명령 라인 변경
    Jump(IRStatementJump),
    /// 함수 호출
    Call { target: IRData },
    /// 함수 호출 후 반환
    Halt,
    /// 값 접근
    Touch,
    /// 콜백
    Callback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRStatementUnknown {
    Instruction(Box<Instruction>),
    Bytecode(Box<[u8]>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRStatementJump {
    Conditional { ok: IRData, fail: IRData },
    Unconditional(IRData),
}
