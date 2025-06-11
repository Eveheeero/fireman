//! Module containing functions to convert x86_64 architecture instructions into IR
#![allow(unused_imports)]

mod a;
mod atomic_variants;
mod b;
mod c;
mod d;
mod f;
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

    pub(in crate::arch) use crate::ir::analyze::DataType;
    pub(in crate::arch) use crate::ir::{data::*, operator::*, statements::*};
    use crate::utils::Aos;
    pub(in crate::arch) use data::*;
    pub(in crate::arch) use fireman_macro::box_to_static_reference;
    pub(in crate::arch) use macros::*;
    pub(in crate::arch) use statements::*;
    use std::num::NonZeroU8;
}

use super::atomic::{LockPrefix, has_lock_prefix, validate_lock_prefix};
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
///
/// This function also handles LOCK prefix detection for atomic operations.
pub fn create_ir_statement(instruction: &Instruction) -> Option<&'static [IrStatement]> {
    // Check for LOCK prefix
    let lock_prefix = has_lock_prefix(instruction);

    // Validate LOCK prefix usage if present
    if validate_lock_prefix(instruction).is_err() {
        // Invalid LOCK prefix usage - treat as regular instruction
        // In production, we might want to log this
    }

    // If LOCK prefix is present and valid, use atomic variants
    let use_atomic = lock_prefix == LockPrefix::Present;
    let op = if let Ok(Statement::X64(op)) = instruction.inner.statement {
        op
    } else {
        return None;
    };

    use iceball::X64Statement;
    let base_statements = match op {
        X64Statement::Adc => {
            if use_atomic {
                atomic_variants::atomic_adc()
            } else {
                a::adc()
            }
        }
        X64Statement::Add => {
            if use_atomic {
                atomic_variants::atomic_add()
            } else {
                a::add()
            }
        }
        X64Statement::Addps => a::addps(),
        X64Statement::Addpd => a::addpd(),
        X64Statement::Addss => a::addss(),
        X64Statement::Addsd => a::addsd(),
        X64Statement::And => {
            if use_atomic {
                atomic_variants::atomic_and()
            } else {
                a::and()
            }
        }
        X64Statement::Andps => a::andps(),
        X64Statement::Andpd => a::andpd(),
        X64Statement::Andnps => a::andnps(),
        X64Statement::Andnpd => a::andnpd(),
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
        X64Statement::Cmpxchg => {
            if use_atomic {
                atomic_variants::atomic_cmpxchg()
            } else {
                c::cmpxchg()
            }
        }
        X64Statement::Cpuid => None?,
        X64Statement::Cmovcc => c::cmovcc(),
        X64Statement::Cmps => c::cmps(),
        X64Statement::Cmpsb => c::cmpsb(),
        X64Statement::Cmpsw => c::cmpsw(),
        X64Statement::Cmpsd => c::cmpsd(),
        X64Statement::Cmpsq => c::cmpsq(),
        X64Statement::Dec => {
            if use_atomic {
                atomic_variants::atomic_dec()
            } else {
                d::dec()
            }
        }
        X64Statement::Div => d::div(),
        X64Statement::Divps => d::divps(),
        X64Statement::Divpd => d::divpd(),
        X64Statement::Divss => d::divss(),
        X64Statement::Divsd => d::divsd(),
        X64Statement::Hlt => None?,
        X64Statement::Imul => i::imul(),
        X64Statement::Inc => {
            if use_atomic {
                atomic_variants::atomic_inc()
            } else {
                i::inc()
            }
        }
        X64Statement::Idiv => None?,
        X64Statement::Int | X64Statement::Into => None?, // INLINE ASSEMBLY
        X64Statement::Int1 | X64Statement::Int3 => &[],
        X64Statement::Jmp => j::jmp(),
        X64Statement::Ja => j::ja(),
        X64Statement::Jae => j::jae(),
        X64Statement::Jb => j::jb(),
        X64Statement::Jbe => j::jbe(),
        X64Statement::Jcxz => j::jcxz(),
        X64Statement::Jecxz => j::jecxz(),
        X64Statement::Jrcxz => j::jrcxz(),
        X64Statement::Jz => j::je(),
        X64Statement::Jg => j::jg(),
        X64Statement::Jge => j::jge(),
        X64Statement::Jl => j::jl(),
        X64Statement::Jle => j::jle(),
        X64Statement::Jnz => j::jne(),
        X64Statement::Jno => j::jno(),
        X64Statement::Jnp => j::jnp(),
        X64Statement::Jns => j::jns(),
        X64Statement::Jo => j::jo(),
        X64Statement::Jp => j::jp(),
        X64Statement::Js => j::js(),
        X64Statement::Lea => l::lea(),
        X64Statement::Leave => None?,
        X64Statement::Loop => None?,
        X64Statement::Loopcc => None?,
        X64Statement::Mov => m::mov(),
        X64Statement::Movaps => m::movaps(),
        X64Statement::Movapd => m::movapd(),
        X64Statement::Movups => m::movups(),
        X64Statement::Movupd => m::movupd(),
        X64Statement::Movss => m::movss(),
        X64Statement::Mul => m::mul(),
        X64Statement::Mulps => m::mulps(),
        X64Statement::Mulpd => m::mulpd(),
        X64Statement::Mulss => m::mulss(),
        X64Statement::Mulsd => m::mulsd(),
        X64Statement::Minps => m::minps(),
        X64Statement::Minpd => m::minpd(),
        X64Statement::Minss => m::minss(),
        X64Statement::Minsd => m::minsd(),
        X64Statement::Maxps => m::maxps(),
        X64Statement::Maxpd => m::maxpd(),
        X64Statement::Maxss => m::maxss(),
        X64Statement::Maxsd => m::maxsd(),
        X64Statement::Movsx => m::movsx(),
        X64Statement::Movsxd => m::movsxd(),
        X64Statement::Movzx => m::movzx(),
        X64Statement::Movsb => m::movsb(),
        X64Statement::Movsw => m::movsw(),
        X64Statement::Movsd => m::movsd(),
        X64Statement::Movsq => m::movsq(),
        X64Statement::Neg => {
            if use_atomic {
                atomic_variants::atomic_neg()
            } else {
                n::neg()
            }
        }
        X64Statement::Nop => &[],
        X64Statement::Not => {
            if use_atomic {
                atomic_variants::atomic_not()
            } else {
                n::not()
            }
        }
        X64Statement::Or => {
            if use_atomic {
                atomic_variants::atomic_or()
            } else {
                o::or()
            }
        }
        X64Statement::Orps => o::orps(),
        X64Statement::Orpd => o::orpd(),
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
        X64Statement::Sub => {
            if use_atomic {
                atomic_variants::atomic_sub()
            } else {
                s::sub()
            }
        }
        X64Statement::Subps => s::subps(),
        X64Statement::Subpd => s::subpd(),
        X64Statement::Subss => s::subss(),
        X64Statement::Subsd => s::subsd(),
        X64Statement::Sqrtps => s::sqrtps(),
        X64Statement::Sqrtpd => s::sqrtpd(),
        X64Statement::Sqrtss => s::sqrtss(),
        X64Statement::Sqrtsd => s::sqrtsd(),
        X64Statement::Stc => s::stc(),
        X64Statement::Sahf => s::sahf(),
        X64Statement::Sar => s::sar(),
        X64Statement::Sbb => {
            if use_atomic {
                atomic_variants::atomic_sbb()
            } else {
                s::sbb()
            }
        }
        X64Statement::Setcc => None?,
        X64Statement::Std => s::std(),
        X64Statement::Scasb => None?,
        X64Statement::Scasw => None?,
        X64Statement::Scasd => None?,
        X64Statement::Scas => None?,
        X64Statement::Stosb => s::stosb(),
        X64Statement::Stosw => s::stosw(),
        X64Statement::Stosd => s::stosd(),
        X64Statement::Stosq => s::stosq(),
        X64Statement::Test => t::test(),
        X64Statement::Xchg => {
            if use_atomic {
                atomic_variants::atomic_xchg()
            } else {
                x::xchg()
            }
        }
        X64Statement::Xor => {
            if use_atomic {
                atomic_variants::atomic_xor()
            } else {
                x::xor()
            }
        }
        X64Statement::Xorps => x::xorps(),
        X64Statement::Xorpd => x::xorpd(),

        // FPU instructions
        X64Statement::Fabs => f::fabs(),
        X64Statement::Fadd => f::fadd(),
        X64Statement::Fchs => f::fchs(),
        X64Statement::Fcom => f::fcom(),
        X64Statement::Fdiv => f::fdiv(),
        X64Statement::Fild => f::fild(),
        X64Statement::Fist => f::fist(),
        X64Statement::Fistp => f::fistp(),
        X64Statement::Fld => f::fld(),
        X64Statement::Fmul => f::fmul(),
        X64Statement::Fsqrt => f::fsqrt(),
        X64Statement::Fst => f::fst(),
        X64Statement::Fstp => f::fstp(),
        X64Statement::Fsub => f::fsub(),
        X64Statement::Fxch => f::fxch(),

        // TODO: Add more FPU instructions
        X64Statement::Faddp => None?,
        X64Statement::Fbld => None?,
        X64Statement::Fbstp => None?,
        X64Statement::Fclex => None?,
        X64Statement::Fcmovcc => None?,
        X64Statement::Fcomi => None?,
        X64Statement::Fcomip => None?,
        X64Statement::Fcomp => None?,
        X64Statement::Fcompp => None?,
        X64Statement::Fcos => None?,
        X64Statement::Fdecstp => None?,
        X64Statement::Fdivp => None?,
        X64Statement::Fdivr => None?,
        X64Statement::Fdivrp => None?,
        X64Statement::Ffree => None?,
        X64Statement::Fiadd => None?,
        X64Statement::Ficom => None?,
        X64Statement::Ficomp => None?,
        X64Statement::Fidiv => None?,
        X64Statement::Fidivr => None?,
        X64Statement::Fimul => None?,
        X64Statement::Fincstp => None?,
        X64Statement::Finit => None?,
        X64Statement::Fisttp => None?,
        X64Statement::Fisub => None?,
        X64Statement::Fisubr => None?,
        X64Statement::Fld1 => None?,
        X64Statement::Fldcw => None?,
        X64Statement::Fldenv => None?,
        X64Statement::Fldl2e => None?,
        X64Statement::Fldl2t => None?,
        X64Statement::Fldlg2 => None?,
        X64Statement::Fldln2 => None?,
        X64Statement::Fldpi => None?,
        X64Statement::Fldz => None?,
        X64Statement::Fmulp => None?,
        X64Statement::Fnclex => None?,
        X64Statement::Fninit => None?,
        X64Statement::Fnop => None?,
        X64Statement::Fnsave => None?,
        X64Statement::Fnstcw => None?,
        X64Statement::Fnstenv => None?,
        X64Statement::Fpatan => None?,
        X64Statement::Fprem => None?,
        X64Statement::Fprem1 => None?,
        X64Statement::Fptan => None?,
        X64Statement::Frndint => None?,
        X64Statement::Frstor => None?,
        X64Statement::Fsave => None?,
        X64Statement::Fscale => None?,
        X64Statement::Fsin => None?,
        X64Statement::Fsincos => None?,
        X64Statement::Fstcw => None?,
        X64Statement::Fstenv => None?,
        X64Statement::Fstsw => None?,
        X64Statement::Fsubp => None?,
        X64Statement::Fsubr => None?,
        X64Statement::Fsubrp => None?,
        X64Statement::Ftst => None?,
        X64Statement::Fucom => None?,
        X64Statement::Fucomi => None?,
        X64Statement::Fucomip => None?,
        X64Statement::Fucomp => None?,
        X64Statement::Fucompp => None?,
        X64Statement::Fwait => None?,
        X64Statement::Fxam => None?,
        X64Statement::Fxrstor => None?,
        X64Statement::Fxsave => None?,
        X64Statement::Fxtract => None?,
        X64Statement::Fyl2x => None?,
        X64Statement::Fyl2xp1 => None?,

        _ => None?,
    };

    Some(base_statements)
}
