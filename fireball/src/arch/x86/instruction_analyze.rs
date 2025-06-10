//! x86 (32-bit) instruction analysis and IR generation
//!
//! Many instructions are shared with x86_64, but operate in 32-bit mode.

use crate::core::Instruction;
use crate::ir::data::{AccessSize, IrData};
use crate::ir::statements::IrStatement;
use crate::utils::Aos;
use iceball::Statement;

// Import shortcuts for IR construction
#[path = "../x86_64/instruction_analyze_shortcuts"]
mod shortcuts {
    #![allow(non_upper_case_globals, unused_imports)]

    pub mod data;
    pub mod macros;
    pub mod statements;

    pub(in crate::arch) use crate::ir::analyze::DataType;
    pub(in crate::arch) use crate::ir::{data::*, statements::*};
    pub(in crate::arch) use data::*;
    pub(in crate::arch) use fireman_macro::box_to_static_reference;
    pub(in crate::arch) use statements::*;
}

use shortcuts::*;

use std::sync::LazyLock;

// Import the x86_64 register helpers for flags
use crate::arch::x86_64::str_to_x64_register;

// Create static flag register references
static CF: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("cf"));
static ZF: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("zf"));
static SF: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("sf"));
static OF: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("of"));
static PF: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("pf"));
static AF: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("af"));
static EIP: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("eip"));

// x86-specific register data helpers
fn esp_data() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("esp"));
    ONCE.clone()
}

fn eax_data() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("eax"));
    ONCE.clone()
}

fn edx_data() -> Aos<IrData> {
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| str_to_x64_register("edx"));
    ONCE.clone()
}

fn edx_eax_data() -> Aos<IrData> {
    // Combined EDX:EAX for 64-bit results - using a temporary 64-bit register
    static ONCE: LazyLock<Aos<IrData>> = LazyLock::new(|| {
        Aos::new_static(IrData::Register(crate::ir::Register::new("edx:eax", 0..64)))
    });
    ONCE.clone()
}

// Helper functions - these are already defined in shortcuts
// Just need to import them properly

// Additional helper for size_relative
fn size_relative(data: Aos<IrData>) -> AccessSize {
    AccessSize::RelativeWith(data)
}

// Binary and unary operators are already available through shortcuts::b and shortcuts::u
// For rotate operations, we need to implement them differently
// ROL and ROR are not basic binary operators in the IR

/// Converts x86 (32-bit) assembly instructions into IR statements.
///
/// ### Arguments
/// - `instruction: &Instruction` : x86 assembly instruction
///
/// ### Returns
/// `Option<&'static [IrStatement]>` : IR statements corresponding to the x86 instruction
/// or `None` if the instruction is not supported.
pub fn create_ir_statement(instruction: &Instruction) -> Option<&'static [IrStatement]> {
    // Extract x86 statement
    let op = if let Ok(Statement::X86(op)) = instruction.inner.statement {
        op
    } else {
        return None;
    };

    use iceball::X86Statement;
    match op {
        // Data transfer
        X86Statement::Mov => Some(mov()),
        X86Statement::Push => Some(push()),
        X86Statement::Pop => Some(pop()),
        X86Statement::Pushad => x86_specific::pushad(),
        X86Statement::Popad => x86_specific::popad(),
        X86Statement::Lea => Some(lea()),
        X86Statement::Xchg => Some(xchg()),

        // Arithmetic
        X86Statement::Add => Some(add()),
        X86Statement::Sub => Some(sub()),
        X86Statement::Adc => Some(adc()),
        X86Statement::Sbb => Some(sbb()),
        X86Statement::Inc => Some(inc()),
        X86Statement::Dec => Some(dec()),
        X86Statement::Neg => Some(neg()),
        X86Statement::Cmp => Some(cmp()),
        X86Statement::Mul => Some(mul()),
        X86Statement::Imul => Some(imul()),
        X86Statement::Div => Some(div()),
        X86Statement::Idiv => Some(idiv()),

        // BCD arithmetic (x86-specific)
        X86Statement::Daa => x86_specific::daa(),
        X86Statement::Das => x86_specific::das(),
        X86Statement::Aaa => x86_specific::aaa(),
        X86Statement::Aas => x86_specific::aas(),
        X86Statement::Aam => x86_specific::aam(),
        X86Statement::Aad => x86_specific::aad(),

        // Logical
        X86Statement::And => Some(and()),
        X86Statement::Or => Some(or()),
        X86Statement::Xor => Some(xor()),
        X86Statement::Not => Some(not()),
        X86Statement::Test => Some(test()),

        // Shift and rotate
        X86Statement::Shl => Some(shl()),
        X86Statement::Shr => Some(shr()),
        X86Statement::Sar => Some(sar()),
        X86Statement::Rol => Some(rol()),
        X86Statement::Ror => Some(ror()),

        // Control flow
        X86Statement::Jmp => Some(jmp()),
        X86Statement::Je => Some(je()),
        X86Statement::Jne => Some(jne()),
        X86Statement::Jb => Some(jb()),
        X86Statement::Jbe => Some(jbe()),
        X86Statement::Ja => Some(ja()),
        X86Statement::Jae => Some(jae()),
        X86Statement::Jl => Some(jl()),
        X86Statement::Jle => Some(jle()),
        X86Statement::Jg => Some(jg()),
        X86Statement::Jge => Some(jge()),
        X86Statement::Call => Some(call()),
        X86Statement::Ret => Some(ret()),

        // Other
        X86Statement::Nop => Some(nop()),

        _ => None,
    }
}

// Module for x86-specific instruction implementations
mod x86_specific {
    use crate::ir::statements::IrStatement;

    /// Instructions that behave differently in 32-bit mode
    pub fn pushad() -> Option<&'static [IrStatement]> {
        // PUSHAD - Push all 32-bit registers
        // This instruction doesn't exist in 64-bit mode
        // TODO: Complete implementation for all registers
        None
    }

    pub fn popad() -> Option<&'static [IrStatement]> {
        // POPAD - Pop all 32-bit registers
        // This instruction doesn't exist in 64-bit mode
        // TODO: Complete implementation
        None
    }

    pub fn bound() -> Option<&'static [IrStatement]> {
        // BOUND - Check array bounds
        // This instruction is not available in 64-bit mode
        None // Complex instruction, needs proper implementation
    }

    pub fn into() -> Option<&'static [IrStatement]> {
        // INTO - Interrupt on overflow
        // This instruction is not available in 64-bit mode
        None // Complex instruction, needs proper implementation
    }

    pub fn aam() -> Option<&'static [IrStatement]> {
        // AAM - ASCII adjust after multiply
        // This instruction is not available in 64-bit mode
        None // Complex BCD arithmetic
    }

    pub fn aad() -> Option<&'static [IrStatement]> {
        // AAD - ASCII adjust before division
        // This instruction is not available in 64-bit mode
        None // Complex BCD arithmetic
    }

    pub fn aaa() -> Option<&'static [IrStatement]> {
        // AAA - ASCII adjust after addition
        // This instruction is not available in 64-bit mode
        None // Complex BCD arithmetic
    }

    pub fn aas() -> Option<&'static [IrStatement]> {
        // AAS - ASCII adjust after subtraction
        // This instruction is not available in 64-bit mode
        None // Complex BCD arithmetic
    }

    pub fn daa() -> Option<&'static [IrStatement]> {
        // DAA - Decimal adjust after addition
        // This instruction is not available in 64-bit mode
        None // Complex BCD arithmetic
    }

    pub fn das() -> Option<&'static [IrStatement]> {
        // DAS - Decimal adjust after subtraction
        // This instruction is not available in 64-bit mode
        None // Complex BCD arithmetic
    }
}

// Common instruction implementations
// Many of these can be adapted from x86_64 with 32-bit operand sizes

#[box_to_static_reference]
fn mov() -> &'static [IrStatement] {
    let _size = o1_size();
    let assignment = assign(o2(), o1(), &_size);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [assignment, type1, type2].into()
}

#[box_to_static_reference]
fn push() -> &'static [IrStatement] {
    // ESP = ESP - 4
    let esp_sub = assign(b::sub(esp_data(), c(4)), esp_data(), size_result_byte(c(4)));
    // [ESP] = operand
    let store = assign(o1(), d(esp_data()), o1_size());
    [esp_sub, store].into()
}

#[box_to_static_reference]
fn pop() -> &'static [IrStatement] {
    // operand = [ESP]
    let load = assign(d(esp_data()), o1(), o1_size());
    // ESP = ESP + 4
    let esp_add = assign(b::add(esp_data(), c(4)), esp_data(), size_result_byte(c(4)));
    [load, esp_add].into()
}

#[box_to_static_reference]
fn lea() -> &'static [IrStatement] {
    let size = o1_size();
    // LEA loads the address, not the value
    let assignment = assign(o2(), o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Address);
    [assignment, type1].into()
}

#[box_to_static_reference]
fn xchg() -> &'static [IrStatement] {
    let size = o1_size();
    let tmp: Aos<IrData> =
        IrData::Register(crate::ir::Register::new("tmp", 176 * 64..176 * 64 + 32)).into();
    let save_o1 = assign(o1(), tmp.clone(), &size);
    let move_o2_to_o1 = assign(o2(), o1(), &size);
    let move_tmp_to_o2 = assign(tmp, o2(), &size);
    [save_o1, move_o2_to_o1, move_tmp_to_o2].into()
}

#[box_to_static_reference]
fn add() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&OF, &SF, &ZF, &AF, &CF, &PF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn sub() -> &'static [IrStatement] {
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let assignment = assign(sub.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(sub, size, &[&OF, &SF, &ZF, &AF, &CF, &PF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn adc() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let add = b::add(add, u::zero_extend(CF.clone()));
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&OF, &SF, &ZF, &AF, &CF, &PF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(CF.clone(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1, type2, type3].into()
}

#[box_to_static_reference]
fn sbb() -> &'static [IrStatement] {
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let sub = b::sub(sub, u::zero_extend(CF.clone()));
    let assignment = assign(sub.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(sub, size, &[&OF, &SF, &ZF, &AF, &CF, &PF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(CF.clone(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1, type2, type3].into()
}

#[box_to_static_reference]
fn inc() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), c(1));
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&OF, &SF, &ZF, &AF, &PF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1].into()
}

#[box_to_static_reference]
fn dec() -> &'static [IrStatement] {
    let size = o1_size();
    let sub = b::sub(o1(), c(1));
    let assignment = assign(sub.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(sub, size, &[&OF, &SF, &ZF, &AF, &PF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1].into()
}

#[box_to_static_reference]
fn neg() -> &'static [IrStatement] {
    let size = o1_size();
    let neg = u::neg(o1());
    let assignment = assign(neg.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(neg, size, &[&OF, &SF, &ZF, &AF, &CF, &PF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1].into()
}

#[box_to_static_reference]
fn cmp() -> &'static [IrStatement] {
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, size, &[&OF, &SF, &ZF, &AF, &CF, &PF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, type1, type2].into()
}

#[box_to_static_reference]
fn mul() -> &'static [IrStatement] {
    // Unsigned multiply - result in EDX:EAX
    let _size = o1_size();
    let mul = b::mul(eax_data(), o1());
    let assignment = assign(mul.clone(), edx_eax_data(), size_result_byte(c(8)));
    let calc_flags = calc_flags_automatically(mul, size_result_byte(c(8)), &[&OF, &CF]);
    let set_sf = assign(undefined_data(), SF.clone(), size_relative(SF.clone()));
    let set_zf = assign(undefined_data(), ZF.clone(), size_relative(ZF.clone()));
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let set_pf = assign(undefined_data(), PF.clone(), size_relative(PF.clone()));
    [calc_flags, assignment, set_sf, set_zf, set_af, set_pf].into()
}

#[box_to_static_reference]
fn imul() -> &'static [IrStatement] {
    let size = o1_size();
    let mul = b::mul(o1(), o2());
    let assignment = assign(mul.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(mul, size, &[&OF, &CF]);
    let set_sf = assign(undefined_data(), SF.clone(), size_relative(SF.clone()));
    let set_zf = assign(undefined_data(), ZF.clone(), size_relative(ZF.clone()));
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let set_pf = assign(undefined_data(), PF.clone(), size_relative(PF.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [
        calc_flags, assignment, set_sf, set_zf, set_af, set_pf, type1, type2,
    ]
    .into()
}

#[box_to_static_reference]
fn div() -> &'static [IrStatement] {
    // Unsigned divide - EDX:EAX / operand -> quotient in EAX, remainder in EDX
    let size = size_result_byte(c(4));
    let div = b::unsigned_div(edx_eax_data(), o1());
    let rem = b::unsigned_rem(edx_eax_data(), o1());
    let assign_quo = assign(div, eax_data(), size.clone());
    let assign_rem = assign(rem, edx_data(), size.clone());
    // All flags are undefined after DIV
    let set_of = assign(undefined_data(), OF.clone(), size_relative(OF.clone()));
    let set_sf = assign(undefined_data(), SF.clone(), size_relative(SF.clone()));
    let set_zf = assign(undefined_data(), ZF.clone(), size_relative(ZF.clone()));
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let set_cf = assign(undefined_data(), CF.clone(), size_relative(CF.clone()));
    let set_pf = assign(undefined_data(), PF.clone(), size_relative(PF.clone()));
    [
        assign_quo, assign_rem, set_of, set_sf, set_zf, set_af, set_cf, set_pf,
    ]
    .into()
}

#[box_to_static_reference]
fn idiv() -> &'static [IrStatement] {
    // Signed divide - EDX:EAX / operand -> quotient in EAX, remainder in EDX
    let size = size_result_byte(c(4));
    let div = b::signed_div(edx_eax_data(), o1());
    let rem = b::signed_rem(edx_eax_data(), o1());
    let assign_quo = assign(div, eax_data(), size.clone());
    let assign_rem = assign(rem, edx_data(), size.clone());
    // All flags are undefined after IDIV
    let set_of = assign(undefined_data(), OF.clone(), size_relative(OF.clone()));
    let set_sf = assign(undefined_data(), SF.clone(), size_relative(SF.clone()));
    let set_zf = assign(undefined_data(), ZF.clone(), size_relative(ZF.clone()));
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let set_cf = assign(undefined_data(), CF.clone(), size_relative(CF.clone()));
    let set_pf = assign(undefined_data(), PF.clone(), size_relative(PF.clone()));
    [
        assign_quo, assign_rem, set_of, set_sf, set_zf, set_af, set_cf, set_pf,
    ]
    .into()
}

#[box_to_static_reference]
fn and() -> &'static [IrStatement] {
    let size = o1_size();
    let and = b::and(o1(), o2());
    let assignment = assign(and.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(and, size, &[&SF, &ZF, &PF]);
    let set_of = assign(c(0), OF.clone(), size_relative(OF.clone()));
    let set_cf = assign(c(0), CF.clone(), size_relative(CF.clone()));
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_of, set_cf, set_af, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn or() -> &'static [IrStatement] {
    let size = o1_size();
    let or = b::or(o1(), o2());
    let assignment = assign(or.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(or, size, &[&SF, &ZF, &PF]);
    let set_of = assign(c(0), OF.clone(), size_relative(OF.clone()));
    let set_cf = assign(c(0), CF.clone(), size_relative(CF.clone()));
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_of, set_cf, set_af, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn xor() -> &'static [IrStatement] {
    let size = o1_size();
    let xor = b::xor(o1(), o2());
    let assignment = assign(xor.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(xor, size, &[&SF, &ZF, &PF]);
    let set_of = assign(c(0), OF.clone(), size_relative(OF.clone()));
    let set_cf = assign(c(0), CF.clone(), size_relative(CF.clone()));
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_of, set_cf, set_af, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn not() -> &'static [IrStatement] {
    let size = o1_size();
    let not = u::not(o1());
    let assignment = assign(not, o1(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    [assignment, type1].into()
}

#[box_to_static_reference]
fn test() -> &'static [IrStatement] {
    let size = o1_size();
    let and = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and, size, &[&SF, &ZF, &PF]);
    let set_of = assign(c(0), OF.clone(), size_relative(OF.clone()));
    let set_cf = assign(c(0), CF.clone(), size_relative(CF.clone()));
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_of, set_cf, set_af, type1, type2].into()
}

#[box_to_static_reference]
fn shl() -> &'static [IrStatement] {
    let size = o1_size();
    let shl = b::shl(o1(), o2());
    let assignment = assign(shl.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(shl, size, &[&CF, &OF, &SF, &ZF, &PF]);
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_af, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn shr() -> &'static [IrStatement] {
    let size = o1_size();
    let shr = b::shr(o1(), o2());
    let assignment = assign(shr.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(shr, size, &[&CF, &OF, &SF, &ZF, &PF]);
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_af, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn sar() -> &'static [IrStatement] {
    let size = o1_size();
    let sar = b::sar(o1(), o2());
    let assignment = assign(sar.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(sar, size, &[&CF, &OF, &SF, &ZF, &PF]);
    let set_af = assign(undefined_data(), AF.clone(), size_relative(AF.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_af, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn rol() -> &'static [IrStatement] {
    let size = o1_size();
    // ROL is implemented as: (value << count) | (value >> (32 - count))
    let val = o1();
    let count = o2();
    let bits = c(32);
    let left_shift = b::shl(val.clone(), count.clone());
    let right_shift = b::shr(val.clone(), b::sub(bits, count.clone()));
    let rol = b::or(left_shift, right_shift);
    let assignment = assign(rol.clone(), o1(), &size);
    // ROL only affects CF and OF
    let calc_cf = calc_flags_automatically(rol.clone(), size.clone(), &[&CF]);
    let calc_of = calc_flags_automatically(rol, size, &[&OF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_cf, calc_of, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn ror() -> &'static [IrStatement] {
    let size = o1_size();
    // ROR is implemented as: (value >> count) | (value << (32 - count))
    let val = o1();
    let count = o2();
    let bits = c(32);
    let right_shift = b::shr(val.clone(), count.clone());
    let left_shift = b::shl(val.clone(), b::sub(bits, count.clone()));
    let ror = b::or(right_shift, left_shift);
    let assignment = assign(ror.clone(), o1(), &size);
    // ROR only affects CF and OF
    let calc_cf = calc_flags_automatically(ror.clone(), size.clone(), &[&CF]);
    let calc_of = calc_flags_automatically(ror, size, &[&OF]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_cf, calc_of, assignment, type1, type2].into()
}

#[box_to_static_reference]
fn jmp() -> &'static [IrStatement] {
    let j = jump(o1());
    [j].into()
}

#[box_to_static_reference]
fn je() -> &'static [IrStatement] {
    let cond = condition(ZF.clone(), vec![jump(o1())], vec![]);
    [cond].into()
}

#[box_to_static_reference]
fn jne() -> &'static [IrStatement] {
    let cond = condition(u::not(ZF.clone()), vec![jump(o1())], vec![]);
    [cond].into()
}

#[box_to_static_reference]
fn jb() -> &'static [IrStatement] {
    let cond = condition(CF.clone(), vec![jump(o1())], vec![]);
    [cond].into()
}

#[box_to_static_reference]
fn jbe() -> &'static [IrStatement] {
    let cond = condition(b::or(CF.clone(), ZF.clone()), vec![jump(o1())], vec![]);
    [cond].into()
}

#[box_to_static_reference]
fn ja() -> &'static [IrStatement] {
    let cond = condition(
        b::and(u::not(CF.clone()), u::not(ZF.clone())),
        vec![jump(o1())],
        vec![],
    );
    [cond].into()
}

#[box_to_static_reference]
fn jae() -> &'static [IrStatement] {
    let cond = condition(u::not(CF.clone()), vec![jump(o1())], vec![]);
    [cond].into()
}

#[box_to_static_reference]
fn jl() -> &'static [IrStatement] {
    let cond = condition(b::xor(SF.clone(), OF.clone()), vec![jump(o1())], vec![]);
    [cond].into()
}

#[box_to_static_reference]
fn jle() -> &'static [IrStatement] {
    let cond = condition(
        b::or(ZF.clone(), b::xor(SF.clone(), OF.clone())),
        vec![jump(o1())],
        vec![],
    );
    [cond].into()
}

#[box_to_static_reference]
fn jg() -> &'static [IrStatement] {
    let cond = condition(
        b::and(u::not(ZF.clone()), u::not(b::xor(SF.clone(), OF.clone()))),
        vec![jump(o1())],
        vec![],
    );
    [cond].into()
}

#[box_to_static_reference]
fn jge() -> &'static [IrStatement] {
    let cond = condition(
        u::not(b::xor(SF.clone(), OF.clone())),
        vec![jump(o1())],
        vec![],
    );
    [cond].into()
}

#[box_to_static_reference]
fn call() -> &'static [IrStatement] {
    let size = size_result_byte(c(4));
    // ESP = ESP - 4
    let esp_sub = assign(b::sub(esp_data(), c(4)), esp_data(), size.clone());
    // Calculate return address: EIP + instruction_byte_size
    let ret_address = b::add(EIP.clone(), instruction_byte_size());
    // [ESP] = return address
    let push_ret = assign(ret_address, d(esp_data()), size);
    // Jump to target
    let j = jump_by_call(o1());
    let type1 = type_specified(o1(), o1_size(), DataType::Address);
    let type2 = type_specified(esp_data(), size_result_byte(c(4)), DataType::Address);
    let type3 = type_specified(EIP.clone(), size_result_byte(c(4)), DataType::Address);
    [esp_sub, push_ret, j, type1, type2, type3].into()
}

#[box_to_static_reference]
fn ret() -> &'static [IrStatement] {
    let size = size_result_byte(c(4));
    // Jump to [ESP]
    let j = jump(d(esp_data()));
    // ESP = ESP + 4
    let esp_add = assign(b::add(esp_data(), c(4)), esp_data(), size);
    [j, esp_add].into()
}

#[box_to_static_reference]
fn nop() -> &'static [IrStatement] {
    [].into()
}
