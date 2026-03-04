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
mod n;
mod o;
mod p;
mod r;
mod s;
mod t;
mod x;

#[path = "instruction_analyze_shortcuts"]
mod shortcuts {
    #![allow(non_upper_case_globals, unused_imports)]

    mod data;
    mod macros;
    mod statements;

    pub(in crate::arch) use crate::ir::{analyze::DataType, data::*, operator::*, statements::*};
    use crate::utils::Aos;
    pub(in crate::arch) use data::*;
    pub(in crate::arch) use fireman_macro::box_to_static_reference;
    pub(in crate::arch) use macros::*;
    pub(in crate::arch) use statements::*;
    use std::num::{NonZeroU8, NonZeroU16};
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
        X64Statement::Cmova => c::cmova(),
        X64Statement::Cmovae => c::cmovae(),
        X64Statement::Cmovb => c::cmovb(),
        X64Statement::Cmovbe => c::cmovbe(),
        X64Statement::Cmove => c::cmove(),
        X64Statement::Cmovg => c::cmovg(),
        X64Statement::Cmovge => c::cmovge(),
        X64Statement::Cmovl => c::cmovl(),
        X64Statement::Cmovle => c::cmovle(),
        X64Statement::Cmovne => c::cmovne(),
        X64Statement::Cmovno => c::cmovno(),
        X64Statement::Cmovnp => c::cmovnp(),
        X64Statement::Cmovns => c::cmovns(),
        X64Statement::Cmovo => c::cmovo(),
        X64Statement::Cmovp => c::cmovp(),
        X64Statement::Cmovs => c::cmovs(),
        X64Statement::Cmovnb => c::cmovnb(),
        X64Statement::Cmovnbe => c::cmovnbe(),
        X64Statement::Cmovnl => c::cmovnl(),
        X64Statement::Cmovnle => c::cmovnle(),
        X64Statement::Cmovnz => c::cmovnz(),
        X64Statement::Cmovz => c::cmovz(),
        X64Statement::Cmovnc => c::cmovnc(),
        X64Statement::Cmovc => c::cmovc(),
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
        X64Statement::Idiv => d::idiv(),
        X64Statement::Int | X64Statement::Into => None?, // INLINE ASSEMBLY
        X64Statement::Int1 | X64Statement::Int3 => &[],
        X64Statement::Jmp => j::jmp(),
        X64Statement::Ja => j::ja(),
        X64Statement::Jae => j::jae(),
        X64Statement::Jb => j::jb(),
        X64Statement::Jbe => j::jbe(),
        X64Statement::Jcxz => None?,
        X64Statement::Jecxz => None?,
        X64Statement::Jrcxz => None?,
        X64Statement::Je => j::je(),
        X64Statement::Jz => j::jz(),
        X64Statement::Jg => j::jg(),
        X64Statement::Jge => j::jge(),
        X64Statement::Jl => j::jl(),
        X64Statement::Jle => j::jle(),
        X64Statement::Jne => j::jne(),
        X64Statement::Jnz => j::jnz(),
        X64Statement::Jno => j::jno(),
        X64Statement::Jnp => j::jnp(),
        X64Statement::Jns => j::jns(),
        X64Statement::Jo => j::jo(),
        X64Statement::Jp => j::jp(),
        X64Statement::Js => j::js(),
        X64Statement::Lea => l::lea(),
        X64Statement::Leave => l::leave(),
        X64Statement::Loop => None?,
        X64Statement::Loopcc => None?,
        X64Statement::Mov => m::mov(),
        X64Statement::Mul => m::mul(),
        X64Statement::Movsx => m::movsx(),
        X64Statement::Movsxd => m::movsxd(),
        X64Statement::Movzx => m::movzx(),
        X64Statement::Movsb => None?,
        X64Statement::Movsw => None?,
        X64Statement::Movsd => None?,
        X64Statement::Movsq => None?,
        X64Statement::Neg => n::neg(),
        X64Statement::Nop => &[],
        X64Statement::Not => n::not(),
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
        X64Statement::Stc => s::stc(),
        X64Statement::Sahf => s::sahf(),
        X64Statement::Sar => s::sar(),
        X64Statement::Sbb => s::sbb(),
        X64Statement::Setcc => None?,
        X64Statement::Seta => s::seta(),
        X64Statement::Setae => s::setae(),
        X64Statement::Setb => s::setb(),
        X64Statement::Setbe => s::setbe(),
        X64Statement::Sete => s::sete(),
        X64Statement::Setg => s::setg(),
        X64Statement::Setge => s::setge(),
        X64Statement::Setl => s::setl(),
        X64Statement::Setle => s::setle(),
        X64Statement::Setne => s::setne(),
        X64Statement::Setno => s::setno(),
        X64Statement::Setnp => s::setnp(),
        X64Statement::Setns => s::setns(),
        X64Statement::Seto => s::seto(),
        X64Statement::Setp => s::setp(),
        X64Statement::Sets => s::sets(),
        X64Statement::Setnb => s::setnb(),
        X64Statement::Setnbe => s::setnbe(),
        X64Statement::Setnl => s::setnl(),
        X64Statement::Setnle => s::setnle(),
        X64Statement::Setnz => s::setnz(),
        X64Statement::Setz => s::setz(),
        X64Statement::Setnc => s::setnc(),
        X64Statement::Setc => s::setc(),
        X64Statement::Std => s::std_(),
        X64Statement::Scasb => None?,
        X64Statement::Scasw => None?,
        X64Statement::Scasd => None?,
        X64Statement::Scas => None?,
        X64Statement::Stosb => None?,
        X64Statement::Stosw => None?,
        X64Statement::Stosd => None?,
        X64Statement::Stosq => None?,
        X64Statement::Test => t::test(),
        X64Statement::Xchg => x::xchg(),
        X64Statement::Xor => x::xor(),

        _ => None?,
    })
}
