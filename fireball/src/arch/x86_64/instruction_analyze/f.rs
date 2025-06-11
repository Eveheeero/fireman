use super::{super::static_register::*, shortcuts::*};
use crate::arch::x86_64::fpu_stack::{FpuCompareResult, fpu_control_word, fpu_st, fpu_status_word};
use crate::core::LocalId;
use crate::ir::Register;
use crate::utils::Aos;
use std::boxed::Box;
use std::num::NonZeroU8;
use std::ops::Deref;

// FPU Stack registers - x87 uses 8 stack registers ST0-ST7
// ST0 is the top of stack, ST7 is the bottom
// The TOP field in the status word determines which physical register is ST0
//
// This implementation now uses proper FPU stack semantics from fpu_stack.rs

// Helper function to push onto FPU stack
// Decrements TOP and stores value at new ST0
fn fpu_push(value: Aos<IrData>) -> Vec<IrStatement> {
    let mut stmts = Vec::new();

    // Get current TOP value from status word
    let status_word = fpu_status_word();
    let top_mask = c(0x3800); // TOP field mask (bits 11-13)
    let top_shift = c(11);

    // Extract current TOP value
    let current_top = b::and(status_word.clone(), top_mask.clone());
    let current_top = b::shr(current_top, top_shift.clone());

    // Decrement TOP (wraps from 0 to 7)
    let one = c(1);
    let seven = c(7);
    let new_top = b::sub(current_top.clone(), one.clone());
    let new_top = b::and(new_top, seven); // Ensure 3-bit value

    // Update TOP field in status word
    let shifted_new_top = b::shl(new_top, top_shift.clone());
    let clear_mask = u::not(top_mask.clone());
    let cleared_status = b::and(status_word.clone(), clear_mask);
    let updated_status = b::or(cleared_status, shifted_new_top);

    stmts.push(assign(
        updated_status,
        status_word.clone(),
        size_result_byte(c(2)),
    ));

    // Store value at new ST0
    stmts.push(assign(value, fpu_st(0), size_result_byte(c(10))));

    stmts
}

// Helper function to pop from FPU stack
// Increments TOP
fn fpu_pop() -> Vec<IrStatement> {
    let mut stmts = Vec::new();

    // Get current TOP value from status word
    let status_word = fpu_status_word();
    let top_mask = c(0x3800); // TOP field mask (bits 11-13)
    let top_shift = c(11);

    // Extract current TOP value
    let current_top = b::and(status_word.clone(), top_mask.clone());
    let current_top = b::shr(current_top, top_shift.clone());

    // Increment TOP (wraps from 7 to 0)
    let one = c(1);
    let seven = c(7);
    let new_top = b::add(current_top, one);
    let new_top = b::and(new_top, seven); // Ensure 3-bit value

    // Update TOP field in status word
    let shifted_new_top = b::shl(new_top, top_shift.clone());
    let clear_mask = u::not(top_mask.clone());
    let cleared_status = b::and(status_word.clone(), clear_mask);
    let updated_status = b::or(cleared_status, shifted_new_top);

    stmts.push(assign(updated_status, status_word, size_result_byte(c(2))));

    stmts
}

// Helper to set FPU condition codes based on comparison result
fn set_fpu_condition_codes(result: FpuCompareResult) -> Vec<IrStatement> {
    let mut stmts = Vec::new();
    let status_word = fpu_status_word();

    // Condition code bit positions
    let c0_bit = c(1 << 8); // bit 8
    let c2_bit = c(1 << 10); // bit 10
    let c3_bit = c(1 << 14); // bit 14

    // Clear all condition code bits first
    let clear_mask = u::not(b::or(c0_bit.clone(), b::or(c2_bit.clone(), c3_bit.clone())));
    let cleared_status = b::and(status_word.clone(), clear_mask);

    // Set bits based on comparison result
    let new_bits = match result {
        FpuCompareResult::Less => c0_bit,
        FpuCompareResult::Equal => c3_bit,
        FpuCompareResult::Greater => c(0),
        FpuCompareResult::Unordered => b::or(c0_bit, b::or(c2_bit, c3_bit)),
    };

    let updated_status = b::or(cleared_status, new_bits);

    stmts.push(assign(updated_status, status_word, size_result_byte(c(2))));

    stmts
}

// FLD - Load floating point value
#[box_to_static_reference]
pub(super) fn fld() -> &'static [IrStatement] {
    // FLD pushes a floating-point value onto the FPU stack
    let src = o1();

    // Use proper FPU push operation (decrements TOP and stores at new ST0)
    let mut stmts = fpu_push(src.clone());

    // Add type specification for the source
    let size = o1_size();
    stmts.push(type_specified(src, size, DataType::Float));

    stmts.into_boxed_slice()
}

// FST - Store floating point value
#[box_to_static_reference]
pub(super) fn fst() -> &'static [IrStatement] {
    // FST stores ST0 to memory without popping the stack
    let src = fpu_st(0);
    let dst = o1();
    let size = o1_size();

    let assignment = assign(src.clone(), dst, &size);
    // FPU registers are 80-bit (10 bytes)
    let fpu_size = size_result_byte(c(10));
    let type_spec = type_specified(src, fpu_size, DataType::Float);

    [assignment, type_spec].into()
}

// FSTP - Store floating point value and pop
#[box_to_static_reference]
pub(super) fn fstp() -> &'static [IrStatement] {
    // FSTP stores ST0 to memory and pops the stack
    let src = fpu_st(0);
    let dst = o1();
    let size = o1_size();

    let mut stmts = Vec::new();

    // Store ST0 to destination
    stmts.push(assign(src.clone(), dst, &size));

    // Type specification
    let fpu_size = size_result_byte(c(10));
    stmts.push(type_specified(src, fpu_size, DataType::Float));

    // Pop the FPU stack (increments TOP)
    stmts.extend(fpu_pop());

    stmts.into_boxed_slice()
}

// FADD - Add floating point values
#[box_to_static_reference]
pub(super) fn fadd() -> &'static [IrStatement] {
    // FADD has multiple forms:
    // - No operands: ST0 = ST0 + ST1
    // - One operand: ST0 = ST0 + operand
    // - Two operands: STi = STi + ST0

    // For now, implement the simple one-operand form
    let dst = fpu_st(0);
    let src = if has_operand(1) { o1() } else { fpu_st(1) };

    let add = b::add(dst.clone(), src.clone());
    let fpu_size = size_result_byte(c(10));
    let assignment = assign(add, dst.clone(), &fpu_size);
    let type1 = type_specified(dst, fpu_size.clone(), DataType::Float);
    let type2 = type_specified(src, o1_size(), DataType::Float);

    [assignment, type1, type2].into()
}

// FSUB - Subtract floating point values
#[box_to_static_reference]
pub(super) fn fsub() -> &'static [IrStatement] {
    let dst = fpu_st(0);
    let src = if has_operand(1) { o1() } else { fpu_st(1) };

    let sub = b::sub(dst.clone(), src.clone());
    let fpu_size = size_result_byte(c(10));
    let assignment = assign(sub, dst.clone(), &fpu_size);
    let type1 = type_specified(dst, fpu_size.clone(), DataType::Float);
    let type2 = type_specified(src, o1_size(), DataType::Float);

    [assignment, type1, type2].into()
}

// FMUL - Multiply floating point values
#[box_to_static_reference]
pub(super) fn fmul() -> &'static [IrStatement] {
    let dst = fpu_st(0);
    let src = if has_operand(1) { o1() } else { fpu_st(1) };

    let mul = b::mul(dst.clone(), src.clone());
    let fpu_size = size_result_byte(c(10));
    let assignment = assign(mul, dst.clone(), &fpu_size);
    let type1 = type_specified(dst, fpu_size.clone(), DataType::Float);
    let type2 = type_specified(src, o1_size(), DataType::Float);

    [assignment, type1, type2].into()
}

// FDIV - Divide floating point values
#[box_to_static_reference]
pub(super) fn fdiv() -> &'static [IrStatement] {
    let dst = fpu_st(0);
    let src = if has_operand(1) { o1() } else { fpu_st(1) };

    // Use signed_div for floating point division
    let div = b::signed_div(dst.clone(), src.clone());
    let fpu_size = size_result_byte(c(10));
    let assignment = assign(div, dst.clone(), &fpu_size);
    let type1 = type_specified(dst, fpu_size.clone(), DataType::Float);
    let type2 = type_specified(src, o1_size(), DataType::Float);

    [assignment, type1, type2].into()
}

// FABS - Absolute value
#[box_to_static_reference]
pub(super) fn fabs() -> &'static [IrStatement] {
    let dst = fpu_st(0);
    let src = dst.clone();

    // TODO: Add absolute value unary operator
    // For now, use a conditional approach
    // if (src < 0) then -src else src
    let zero = c(0);
    let fpu_size = size_result_byte(c(10));
    let cond = b::signed_less(src.clone(), zero, fpu_size.clone());
    let neg_src = u::neg(src.clone());

    // Create condition statement
    let true_branch = vec![assign(neg_src, dst.clone(), &fpu_size)].into_boxed_slice();
    let false_branch = vec![assign(src.clone(), dst.clone(), &fpu_size)].into_boxed_slice();

    let abs_statement = condition(cond, true_branch, false_branch);
    let type_spec = type_specified(dst, fpu_size, DataType::Float);

    [abs_statement, type_spec].into()
}

// FCHS - Change sign
#[box_to_static_reference]
pub(super) fn fchs() -> &'static [IrStatement] {
    let dst = fpu_st(0);
    let src = dst.clone();

    let neg = u::neg(src);
    let fpu_size = size_result_byte(c(10));
    let assignment = assign(neg, dst.clone(), &fpu_size);
    let type_spec = type_specified(dst, fpu_size, DataType::Float);

    [assignment, type_spec].into()
}

// FCOM - Compare floating point values
#[box_to_static_reference]
pub(super) fn fcom() -> &'static [IrStatement] {
    let left = fpu_st(0);
    let right = if has_operand(1) { o1() } else { fpu_st(1) };

    let mut stmts = Vec::new();

    // Type specifications
    let fpu_size = size_result_byte(c(10));
    stmts.push(type_specified(
        left.clone(),
        fpu_size.clone(),
        DataType::Float,
    ));
    stmts.push(type_specified(right.clone(), o1_size(), DataType::Float));

    // FPU comparison sets condition codes in the FPU status word
    // C3, C2, C0 bits indicate the comparison result:
    // Less:    C0=1, C2=0, C3=0
    // Equal:   C0=0, C2=0, C3=1
    // Greater: C0=0, C2=0, C3=0
    // Unordered: C0=1, C2=1, C3=1

    // For now, we'll generate a simplified comparison
    // In a real implementation, this would handle NaN and set proper flags
    let comparison = b::sub(left, right);

    // Store comparison result in a temporary
    let cmp_temp: Aos<IrData> = IrData::Register(Register::new("fpu_cmp_temp", 0..80)).into();
    stmts.push(assign(comparison, cmp_temp.clone(), &fpu_size));

    // TODO: Convert comparison result to proper condition codes
    // For now, just update status word with a placeholder
    stmts.extend(set_fpu_condition_codes(FpuCompareResult::Equal));

    stmts.into_boxed_slice()
}

// FILD - Load integer as floating point
#[box_to_static_reference]
pub(super) fn fild() -> &'static [IrStatement] {
    let src = o1();
    let src_size = o1_size();

    // Convert integer to float and push onto FPU stack
    // TODO: Add proper integer-to-float conversion
    let mut stmts = fpu_push(src.clone());

    // Add type specification for the source
    stmts.push(type_specified(src, src_size, DataType::Int));

    stmts.into_boxed_slice()
}

// FIST - Store integer from floating point
#[box_to_static_reference]
pub(super) fn fist() -> &'static [IrStatement] {
    let src = fpu_st(0);
    let dst = o1();
    let dst_size = o1_size();

    // Convert float to integer
    // For now, just copy the value (proper conversion needed)
    let assignment = assign(src.clone(), dst, &dst_size);
    let type_spec = type_specified(src, size_result_byte(c(10)), DataType::Float);

    [assignment, type_spec].into()
}

// FISTP - Store integer from floating point and pop
#[box_to_static_reference]
pub(super) fn fistp() -> &'static [IrStatement] {
    // Same as FIST but also pops the stack
    let src = fpu_st(0);
    let dst = o1();
    let dst_size = o1_size();

    let mut stmts = Vec::new();

    // Convert float to integer and store
    stmts.push(assign(src.clone(), dst, &dst_size));
    stmts.push(type_specified(
        src,
        size_result_byte(c(10)),
        DataType::Float,
    ));

    // Pop the FPU stack (increments TOP)
    stmts.extend(fpu_pop());

    stmts.into_boxed_slice()
}

// FSQRT - Square root
#[box_to_static_reference]
pub(super) fn fsqrt() -> &'static [IrStatement] {
    let dst = fpu_st(0);
    let src = dst.clone();

    // TODO: Add square root unary operator
    // For now, just copy the value unchanged
    let fpu_size = size_result_byte(c(10));
    let assignment = assign(src, dst.clone(), &fpu_size);
    let type_spec = type_specified(dst, fpu_size, DataType::Float);

    [assignment, type_spec].into()
}

// FXCH - Exchange FPU stack registers
#[box_to_static_reference]
pub(super) fn fxch() -> &'static [IrStatement] {
    // Exchange ST0 with STi (default ST1)
    let st0_reg = fpu_st(0);
    let sti_reg = if has_operand(1) {
        // TODO: Parse ST register operand
        fpu_st(1)
    } else {
        fpu_st(1)
    };

    // Use tmp64 register for swap
    let size = size_result_byte(c(10));

    let save_st0 = assign(st0_reg.clone(), tmp64.clone(), &size);
    let move_sti_to_st0 = assign(sti_reg.clone(), st0_reg.clone(), &size);
    let move_temp_to_sti = assign(tmp64.clone(), sti_reg, &size);

    [save_st0, move_sti_to_st0, move_temp_to_sti].into()
}

// Helper function to check if an operand exists
// TODO: This needs to be implemented properly
#[inline]
fn has_operand(_index: usize) -> bool {
    // For now, assume operand exists
    // This should check the actual instruction operands
    true
}

// Helper to get operand count
// TODO: This needs to be implemented properly based on actual instruction
#[inline]
fn operand_count() -> usize {
    // For now, assume 1 operand
    // This should check the actual instruction operands
    1
}
