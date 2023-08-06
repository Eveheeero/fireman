//! IR의 각 명령이 담겨져 있는 모듈

use super::term::Term;
use crate::core::Instruction;

/// IR의 각 명령에 대한 Enum
#[derive(Debug, Clone)]
pub enum IRStatement {
    /// 해석할 수 없는 명령, 인라인 어셈블리로 처리됩니다.
    Unknown(IRStatementUnknown),
    /// 변수 할당
    Assignment { from: Term, to: Term },
    /// 명령 라인 변경
    Jump(IRStatementJump),
    /// 함수 호출
    Call { target: Term },
    /// 함수 호출 후 반환
    Halt,
    /// 값 접근
    Touch,
    /// 콜백
    Callback,
}

#[derive(Debug, Clone)]
pub enum IRStatementUnknown {
    Instruction(Box<Instruction>),
    Bytecode(Box<[u8]>),
}

#[derive(Debug, Clone)]
pub enum IRStatementJump {
    Conditional(Term, Term),
    Unconditional(Term),
}
