//! x86_64 아키텍처 인스트럭션을 IR로 변환하는 함수가 담긴 모듈

mod a;
#[path = "instruction_analyze_shortcuts.rs"]
mod shortcuts;

use crate::{core::Instruction, ir::statements::*};
use iceball::Statement;

/// 어셈블리 인스트럭션을 받아 IR 명령으로 변환한다.
///
/// ### Arguments
/// - `instruction` : 어셈블리 인스트럭션
///
/// ### Returns
/// `Box<[IrStatement]>` : IR 명령 배열
pub fn create_ir_statement(instruction: &Instruction) -> Box<[IrStatement]> {
    let op = if let Ok(Statement::X64(op)) = instruction.inner.statement {
        op
    } else {
        return [IrStatement::Unknown(IrStatementUnknown::Instruction(
            instruction.clone(),
        ))]
        .into();
    };

    use iceball::X64Statement;

    match op {
        X64Statement::Adc => a::adc(),
        X64Statement::Add
        | X64Statement::And
        | X64Statement::Clc
        | X64Statement::Cmc
        | X64Statement::Stc
        | X64Statement::Sahf
        | X64Statement::Fnstsw
        | X64Statement::Bswap
        | X64Statement::Bt
        | X64Statement::Call
        | X64Statement::Cbw
        | X64Statement::Cwde
        | X64Statement::Cdqe
        | X64Statement::Cqo
        | X64Statement::Cld
        | X64Statement::Cmp
        | X64Statement::Cmpxchg
        | X64Statement::Cpuid
        | X64Statement::Dec
        | X64Statement::Div
        | X64Statement::Hlt
        | X64Statement::Idiv
        | X64Statement::Int
        | X64Statement::Imul
        | X64Statement::Mul
        | X64Statement::Inc
        | X64Statement::Ja
        | X64Statement::Jae
        | X64Statement::Jb
        | X64Statement::Jbe
        | X64Statement::Jcxz
        | X64Statement::Jecxz
        | X64Statement::Jrcxz
        | X64Statement::Jz
        | X64Statement::Jg
        | X64Statement::Jge
        | X64Statement::Jl
        | X64Statement::Jle
        | X64Statement::Jnz
        | X64Statement::Jno
        | X64Statement::Jnp
        | X64Statement::Jns
        | X64Statement::Jo
        | X64Statement::Jp
        | X64Statement::Js
        | X64Statement::Jmp
        | X64Statement::Lea
        | X64Statement::Leave
        | X64Statement::Loop
        | X64Statement::Loopcc
        | X64Statement::Mov
        | X64Statement::Movsx
        | X64Statement::Movsxd
        | X64Statement::Movzx
        | X64Statement::Neg
        | X64Statement::Nop
        | X64Statement::Not
        | X64Statement::Or
        | X64Statement::Pop
        | X64Statement::Push
        | X64Statement::Popf
        | X64Statement::Popfd
        | X64Statement::Popfq
        | X64Statement::Pushf
        | X64Statement::Pushfq
        | X64Statement::Ret
        | X64Statement::Shl
        | X64Statement::Sar
        | X64Statement::Sbb
        | X64Statement::Setcc
        | X64Statement::Shr
        | X64Statement::Std
        | X64Statement::Sub
        | X64Statement::Test
        | X64Statement::Xchg
        | X64Statement::Xor
        | X64Statement::Cmovcc
        | X64Statement::Cmpsb
        | X64Statement::Cmpsw
        | X64Statement::Cmpsd
        | X64Statement::Cmpsq
        | X64Statement::Movsb
        | X64Statement::Movsw
        | X64Statement::Movsd
        | X64Statement::Movsq
        | X64Statement::Scasb
        | X64Statement::Scasw
        | X64Statement::Scasd
        | X64Statement::Scas
        | X64Statement::Stosb
        | X64Statement::Stosw
        | X64Statement::Stosd
        | X64Statement::Stosq
        | _ => [IrStatement::Unknown(IrStatementUnknown::Instruction(
            instruction.clone(),
        ))]
        .into(),
    }
}
