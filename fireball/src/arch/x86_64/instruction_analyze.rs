//! Module containing functions to convert x86_64 architecture instructions into IR
#![allow(unused_imports)]

mod a;
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
        X64Statement::Call => c::call(),
        X64Statement::Cmp => c::cmp(),
        X64Statement::Dec => d::dec(),
        X64Statement::Div => d::div(),
        X64Statement::Imul => i::imul(),
        X64Statement::Inc => i::inc(),
        X64Statement::Jmp => j::jmp(),
        X64Statement::Lea => l::lea(),
        X64Statement::Mov => m::mov(),
        X64Statement::Mul => m::mul(),
        X64Statement::Or => o::or(),
        X64Statement::Pop => p::pop(),
        X64Statement::Push => p::push(),
        X64Statement::Ret => r::ret(),
        X64Statement::Shl => s::shl(),
        X64Statement::Shr => s::shr(),
        X64Statement::Sub => s::sub(),

        X64Statement::Clc
        | X64Statement::Cmc
        | X64Statement::Stc
        | X64Statement::Sahf
        | X64Statement::Fnstsw
        | X64Statement::Bswap
        | X64Statement::Bt
        | X64Statement::Cbw
        | X64Statement::Cwde
        | X64Statement::Cdqe
        | X64Statement::Cqo
        | X64Statement::Cld
        | X64Statement::Cmpxchg
        | X64Statement::Cpuid
        | X64Statement::Hlt
        | X64Statement::Idiv
        | X64Statement::Int
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
        | X64Statement::Leave
        | X64Statement::Loop
        | X64Statement::Loopcc
        | X64Statement::Movsx
        | X64Statement::Movsxd
        | X64Statement::Movzx
        | X64Statement::Neg
        | X64Statement::Nop
        | X64Statement::Not
        | X64Statement::Popf
        | X64Statement::Popfd
        | X64Statement::Popfq
        | X64Statement::Pushf
        | X64Statement::Pushfq
        | X64Statement::Sar
        | X64Statement::Sbb
        | X64Statement::Setcc
        | X64Statement::Std
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
        | _ => None?,
    })
}
