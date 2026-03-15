//! Module containing functions to convert x86_64 architecture instructions into IR
#![allow(unused_imports)]

mod a;
mod b;
mod c;
mod d;
mod e_generated;
mod f_generated;
mod h_generated;
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
mod u_generated;
mod w_generated;
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
        X64Statement::Emms => e_generated::emms(),
        X64Statement::Enter => e_generated::enter(),
        X64Statement::Cdqe => c::cdqe(),
        X64Statement::Cwd => c::cwd(),
        X64Statement::Cdq => c::cdq(),
        X64Statement::Cqo => c::cqo(),
        X64Statement::Cld => c::cld(),
        X64Statement::Cmpxchg => c::cmpxchg(),
        X64Statement::Cpuid => None?,
        X64Statement::Cmps => c::cmps(),
        X64Statement::Cmpsb => c::cmpsb(),
        X64Statement::Cmpsw => c::cmpsw(),
        X64Statement::Cmpsd => c::cmpsd(),
        X64Statement::Cmpsq => c::cmpsq(),
        X64Statement::Dec => d::dec(),
        X64Statement::Div => d::div(),
        X64Statement::F2xm1 => f_generated::f2xm1(),
        X64Statement::Fabs => f_generated::fabs(),
        X64Statement::Fadd => f_generated::fadd(),
        X64Statement::Fchs => f_generated::fchs(),
        X64Statement::Fclex => f_generated::fclex(),
        X64Statement::Finit => f_generated::finit(),
        X64Statement::Fld => f_generated::fld(),
        X64Statement::Fnstsw => f_generated::fnstsw(),
        X64Statement::Fstp => f_generated::fstp(),
        X64Statement::Fxsave => f_generated::fxsave(),
        X64Statement::Haddpd => h_generated::haddpd(),
        X64Statement::Hlt => h_generated::hlt(),
        X64Statement::Imul => i::imul(),
        X64Statement::Inc => i::inc(),
        X64Statement::Idiv => d::idiv(),
        X64Statement::Jmp => j::jmp(),
        X64Statement::Lea => l::lea(),
        X64Statement::Leave => l::leave(),
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
        X64Statement::Ucomisd => u_generated::ucomisd(),
        X64Statement::Ucomiss => u_generated::ucomiss(),
        X64Statement::Ud => u_generated::ud(),
        X64Statement::Uiret => u_generated::uiret(),
        X64Statement::Umonitor => u_generated::umonitor(),
        X64Statement::Umwait => u_generated::umwait(),
        X64Statement::Unpckhpd => u_generated::unpckhpd(),
        X64Statement::Unpckhps => u_generated::unpckhps(),
        X64Statement::Unpcklpd => u_generated::unpcklpd(),
        X64Statement::Unpcklps => u_generated::unpcklps(),
        X64Statement::Wait => w_generated::wait(),
        X64Statement::Wbinvd => w_generated::wbinvd(),
        X64Statement::Wbnoinvd => w_generated::wbnoinvd(),
        X64Statement::Wrfsbase => w_generated::wrfsbase(),
        X64Statement::Wrgsbase => w_generated::wrgsbase(),
        X64Statement::Wrmsr => w_generated::wrmsr(),
        X64Statement::Wrpkru => w_generated::wrpkru(),
        X64Statement::Wrssd => w_generated::wrssd(),
        X64Statement::Wrssq => w_generated::wrssq(),
        X64Statement::Wrussd => w_generated::wrussd(),
        X64Statement::Wrussq => w_generated::wrussq(),
        X64Statement::Xchg => x::xchg(),
        X64Statement::Xor => x::xor(),

        _ => None?,
    })
}
