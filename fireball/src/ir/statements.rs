//! IR의 각 명령이 담겨져 있는 모듈

use crate::core::Instruction;

/// IR의 각 명령에 대한 Enum
#[derive(Debug, Clone)]
pub enum IRStatement {
    /// 해석할 수 없는 명령, 인라인 어셈블리로 처리됩니다.
    Unknown(IRStatementUnknown),
    /// 변수 할당
    Assignment(IRStatementAssignment),
    /// 명령 라인 변경
    Jump(IRStatementJump),
    /// 함수 호출
    Call(IRStatementJump),
    /// 함수 호출 후 반환
    Halt(IRStatementHalt),
    /// 메모리 접근
    Touch(IRStatementTouch),
    // Callback,
    // RememberReachingDefinitions
    // User
}

#[derive(Debug, Clone)]
pub enum IRStatementUnknown {
    Instruction(Box<Instruction>),
    Bytecode(Box<[u8]>),
}

#[derive(Debug, Clone)]
pub enum IRStatementAssignment {}

#[derive(Debug, Clone)]
pub enum IRStatementJump {}

#[derive(Debug, Clone)]
pub enum IRStatementHalt {}

#[derive(Debug, Clone)]
pub enum IRStatementTouch {
    ReadRelative(i64),
    ReadAbsolute(u64),
    WriteRelative(i64),
    WriteAbsolute(u64),
}
