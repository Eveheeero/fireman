//! x86_64 아키텍처 인스트럭션을 IR로 변환하는 함수가 담긴 모듈

use crate::{core::Instruction, ir::statements::IRStatement};

/// 어셈블리 인스트럭션을 받아 IR 명령으로 변환한다.
///
/// ### Arguments
/// - `instruction` : 어셈블리 인스트럭션
///
/// ### Returns
/// `&'static [IRStatement]` : IR 명령 배열
#[allow(unused)]
fn create_ir_statement(instruction: &Instruction) -> &'static [IRStatement] {
    todo!()
}
