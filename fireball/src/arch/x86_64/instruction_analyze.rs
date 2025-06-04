//! Module containing functions to convert x86_64 architecture instructions into IR
#![allow(unused_imports)]

mod a;
mod b;
mod c;
mod d;
mod i;
mod j;
mod l;
mod m;
mod o;
mod p;
mod r;
mod s;

#[path = "instruction_analyze_shortcuts"]
mod shortcuts {
    #![allow(non_upper_case_globals, unused_imports)]

    mod data;
    mod macros;
    mod statements;

    pub(in crate::arch) use crate::ir::{data::*, operator::*, statements::*};
    use crate::utils::Aos;
    pub(in crate::arch) use data::*;
    pub(in crate::arch) use fireman_macro::box_to_static_reference;
    pub(in crate::arch) use macros::*;
    pub(in crate::arch) use statements::*;
    use std::num::{NonZeroU16, NonZeroU8};
}

use crate::{core::Instruction, ir::statements::*};
use iceball::Statement;

/// Converts x86_64 assembly instructions into IR statements.
///
/// ### Arguments
/// - `instruction: &Instruction` : x86_64 assembly instruction
///
/// ### Returns
/// `Option<&'static [IrStatement]>` : IR statements corresponding to the x86_64 instruction
/// or `None` if the instruction is not supported.
pub fn create_ir_statement(instruction: &Instruction) -> Option<&'static [IrStatement]> {
    let op = if let Ok(Statement::X64(op)) = instruction.inner.statement {
        op
    } else {
        return None;
    };

    use iceball::X64Statement;
    Some(match op {
        X64Statement::Adc => a::adc(),
        X64Statement::Add => a::add(),
        X64Statement::And => a::and(),
        X64Statement::Bswap => b::bswap(),
        X64Statement::Bt => b::bt(),
        X64Statement::Call => c::call(),
        X64Statement::Cmp => c::cmp(),
        X64Statement::Clc => c::clc(),
        X64Statement::Cmc => c::cmc(),
        X64Statement::Cbw => c::cbw(),
        X64Statement::Cwde => c::cwde(),
        X64Statement::Cdqe => c::cdqe(),
        X64Statement::Cwd => c::cwd(),
        X64Statement::Cdq => c::cdq(),
        X64Statement::Cqo => c::cqo(),
        X64Statement::Cld => c::cld(),
        X64Statement::Cmpxchg => c::cmpxchg(),
        X64Statement::Cpuid => None?,
        X64Statement::Cmovcc => None?,
        X64Statement::Cmps => c::cmps(),
        X64Statement::Cmpsb => c::cmpsb(),
        X64Statement::Cmpsw => c::cmpsw(),
        X64Statement::Cmpsd => c::cmpsd(),
        X64Statement::Cmpsq => c::cmpsq(),
        X64Statement::Dec => d::dec(),
        X64Statement::Div => d::div(),
        X64Statement::Fnstsw => None?,
        X64Statement::Hlt => None?,
        X64Statement::Imul => i::imul(),
        X64Statement::Inc => i::inc(),
        X64Statement::Idiv => None?,
        X64Statement::Int => None?,
        X64Statement::Jmp => j::jmp(),
        X64Statement::Ja => None?,
        X64Statement::Jae => None?,
        X64Statement::Jb => None?,
        X64Statement::Jbe => None?,
        X64Statement::Jcxz => None?,
        X64Statement::Jecxz => None?,
        X64Statement::Jrcxz => None?,
        X64Statement::Jz => None?,
        X64Statement::Jg => None?,
        X64Statement::Jge => None?,
        X64Statement::Jl => None?,
        X64Statement::Jle => None?,
        X64Statement::Jnz => None?,
        X64Statement::Jno => None?,
        X64Statement::Jnp => None?,
        X64Statement::Jns => None?,
        X64Statement::Jo => None?,
        X64Statement::Jp => None?,
        X64Statement::Js => None?,
        X64Statement::Lea => l::lea(),
        X64Statement::Leave => None?,
        X64Statement::Loop => None?,
        X64Statement::Loopcc => None?,
        X64Statement::Mov => m::mov(),
        X64Statement::Mul => m::mul(),
        X64Statement::Movsx => None?,
        X64Statement::Movsxd => None?,
        X64Statement::Movzx => None?,
        X64Statement::Movsb => None?,
        X64Statement::Movsw => None?,
        X64Statement::Movsd => None?,
        X64Statement::Movsq => None?,
        X64Statement::Neg => None?,
        X64Statement::Nop => None?,
        X64Statement::Not => None?,
        X64Statement::Or => o::or(),
        X64Statement::Pop => p::pop(),
        X64Statement::Push => p::push(),
        X64Statement::Popf => None?,
        X64Statement::Popfd => None?,
        X64Statement::Popfq => None?,
        X64Statement::Pushf => None?,
        X64Statement::Pushfq => None?,
        X64Statement::Ret => r::ret(),
        X64Statement::Shl => s::shl(),
        X64Statement::Shr => s::shr(),
        X64Statement::Sub => s::sub(),
        X64Statement::Stc => None?,
        X64Statement::Sahf => None?,
        X64Statement::Sar => None?,
        X64Statement::Sbb => None?,
        X64Statement::Setcc => None?,
        X64Statement::Std => None?,
        X64Statement::Scasb => None?,
        X64Statement::Scasw => None?,
        X64Statement::Scasd => None?,
        X64Statement::Scas => None?,
        X64Statement::Stosb => None?,
        X64Statement::Stosw => None?,
        X64Statement::Stosd => None?,
        X64Statement::Stosq => None?,
        X64Statement::Test => None?,
        X64Statement::Xchg => None?,
        X64Statement::Xor => None?,

        _ => None?,
    })
}
