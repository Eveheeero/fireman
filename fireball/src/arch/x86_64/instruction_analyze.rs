//! x86_64 아키텍처 인스트럭션을 IR로 변환하는 함수가 담긴 모듈

mod a;

use crate::{core::Instruction, ir::statements::*};
use iceball::Statement;
use std::num::NonZeroU16;

/// 어셈블리 인스트럭션을 받아 IR 명령으로 변환한다.
///
/// ### Arguments
/// - `instruction` : 어셈블리 인스트럭션
///
/// ### Returns
/// `Box<[IRStatement]>` : IR 명령 배열
pub fn create_ir_statement(instruction: &Instruction) -> Box<[IRStatement]> {
    let op = if let Ok(Statement::X64(op)) = instruction.inner.statement {
        op
    } else {
        return [IRStatement::Unknown(IRStatementUnknown::Instruction(
            instruction.clone(),
        ))]
        .into();
    };

    use iceball::X64Statement;

    match op {
        X64Statement::Aaa => a::aaa(),
        // X64Statement::Aad => [IRStatement::Touch],
        _ => [IRStatement::Unknown(IRStatementUnknown::Instruction(
            instruction.clone(),
        ))]
        .into(),
    }
}

/// return size of register (byte)
fn size(data: &crate::ir::Register) -> NonZeroU16 {
    let bit_len = data.bit_len() as u16;
    let byte_len = bit_len / 8;
    NonZeroU16::new(byte_len).unwrap()
}
#[test]
fn size_test() {
    let eax_size = size(&super::static_register::eax);
    let rax_size = size(&super::static_register::rax);
    assert_eq!(eax_size, NonZeroU16::new(4).unwrap());
    assert_eq!(rax_size, NonZeroU16::new(8).unwrap());
}
fn max() -> NonZeroU16 {
    NonZeroU16::MAX
}
