//! x86_64 아키텍처 인스트럭션을 IR로 변환하는 함수가 담긴 모듈

use crate::{core::Instruction, ir::statements::*};
use iceball::Statement;
use std::rc::Rc;

/// 어셈블리 인스트럭션을 받아 IR 명령으로 변환한다.
///
/// ### Arguments
/// - `instruction` : 어셈블리 인스트럭션
///
/// ### Returns
/// `Rc<[IRStatement]>` : IR 명령 배열
#[allow(unused)]
fn create_ir_statement(instruction: &Instruction) -> Rc<[IRStatement]> {
    let op = if let Ok(Statement::X64(op)) = instruction.inner.statement {
        op
    } else {
        return Rc::new([IRStatement::Unknown(IRStatementUnknown::Instruction(
            Box::new(instruction.clone()),
        ))]);
    };

    Rc::new(match op {
        // X64Statement::Aaa => [IRStatement::Touch],
        // X64Statement::Aad => [IRStatement::Touch],
        _ => [IRStatement::Unknown(IRStatementUnknown::Instruction(
            Box::new(instruction.clone()),
        ))],
    })
}
