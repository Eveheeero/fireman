//! IR의 각 명령이 담겨져 있는 모듈

use crate::core::Instruction;
use crate::ir::data::{AccessType, IRData};
use std::rc::Rc;

/// IR의 각 명령에 대한 Enum
///
/// ### Note
/// snowman's expressions.h, StatementBase based classes, or snowman's ir::statement.h classes
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
    Touch {
        data: IRData,
        access_type: AccessType,
    },
    /// 조건문
    Condition {
        condition: IRData,
        true_branch: Rc<[IRStatement]>,
        false_branch: Rc<[IRStatement]>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRStatementUnknown {
    Instruction(Instruction),
    Bytecode(Box<[u8]>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRStatementJump {
    Conditional { ok: IRData, fail: IRData },
    Unconditional(IRData),
}
