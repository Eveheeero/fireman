use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) indices = V[m];
/// bits(128*regs) table = Zeros();
/// bits(datasize) result;
/// integer index;
/// 
/// // Create table from registers
/// for i = 0 to regs - 1
///     table<128*i+127:128*i> = V[n];
///     n = (n + 1) MOD 32;
/// 
/// result = if is_tbl then Zeros() else V[d];
/// for i = 0 to elements - 1
///     index = UInt(Elem[indices, i, 8]);
///     if index < 16 * regs then
///         Elem[result, i, 8] = Elem[table, index, 8];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn tbl() -> &'static [IrStatement] {
    [exception("tbl")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand = X[t];
/// 
/// if operand<bit_pos> == bit_val then
///     BranchTo(PC[] + offset, BranchType_DIR);
/// ```
#[box_to_static_reference]
pub(super) fn tbnz() -> &'static [IrStatement] {
    let fallthrough = b::add(pc.clone(), instruction_byte_size());
    let bit = b::and(b::shr(o1(), o2()), c(1));
    [condition(bit, [jump(o3())], [jump(fallthrough)])].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) indices = V[m];
/// bits(128*regs) table = Zeros();
/// bits(datasize) result;
/// integer index;
/// 
/// // Create table from registers
/// for i = 0 to regs - 1
///     table<128*i+127:128*i> = V[n];
///     n = (n + 1) MOD 32;
/// 
/// result = if is_tbl then Zeros() else V[d];
/// for i = 0 to elements - 1
///     index = UInt(Elem[indices, i, 8]);
///     if index < 16 * regs then
///         Elem[result, i, 8] = Elem[table, index, 8];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn tbx() -> &'static [IrStatement] {
    [exception("tbx")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand = X[t];
/// 
/// if operand<bit_pos> == bit_val then
///     BranchTo(PC[] + offset, BranchType_DIR);
/// ```
#[box_to_static_reference]
pub(super) fn tbz() -> &'static [IrStatement] {
    let fallthrough = b::add(pc.clone(), instruction_byte_size());
    let bit = b::and(b::shr(o1(), o2()), c(1));
    [condition(b::equal(bit, c(0), o1_size()), [jump(o3())], [jump(fallthrough)])].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// 
/// for p = 0 to pairs-1
///     Elem[result, 2*p+0, esize] = Elem[operand1, 2*p+part, esize];
///     Elem[result, 2*p+1, esize] = Elem[operand2, 2*p+part, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn trn1() -> &'static [IrStatement] {
    [exception("trn1")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// 
/// for p = 0 to pairs-1
///     Elem[result, 2*p+0, esize] = Elem[operand1, 2*p+part, esize];
///     Elem[result, 2*p+1, esize] = Elem[operand2, 2*p+part, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn trn2() -> &'static [IrStatement] {
    [exception("trn2")].into()
}

/// # Pseudocode
/// ```text
/// case op of
///     when SystemHintOp_YIELD
///         Hint_Yield();
/// 
///     when SystemHintOp_DGH
///         Hint_DGH();
/// 
///     when SystemHintOp_WFE
///         if IsEventRegisterSet() then
///             ClearEventRegister();
///         else
///             if PSTATE.EL == EL0 then
///                 // Check for traps described by the OS which may be EL1 or EL2.
///                 AArch64.CheckForWFxTrap(EL1, TRUE);
///             if PSTATE.EL IN {EL0, EL1} && EL2Enabled() && !IsInHost() then
///                 // Check for traps described by the Hypervisor.
///                 AArch64.CheckForWFxTrap(EL2, TRUE);
///             if HaveEL(EL3) && PSTATE.EL != EL3 then
///                 // Check for traps described by the Secure Monitor.
///                 AArch64.CheckForWFxTrap(EL3, TRUE);
///             WaitForEvent();
/// 
///     when SystemHintOp_WFI
///         if !InterruptPending() then
///             if PSTATE.EL == EL0 then
///                 // Check for traps described by the OS which may be EL1 or EL2.
///                 AArch64.CheckForWFxTrap(EL1, FALSE);
///             if PSTATE.EL IN {EL0, EL1} && EL2Enabled() && !IsInHost() then
///                 // Check for traps described by the Hypervisor.
///                 AArch64.CheckForWFxTrap(EL2, FALSE);
///             if HaveEL(EL3) && PSTATE.EL != EL3 then
///                 // Check for traps described by the Secure Monitor.
///                 AArch64.CheckForWFxTrap(EL3, FALSE);
///             WaitForInterrupt();
/// 
///     when SystemHintOp_SEV
///         SendEvent();
/// 
///     when SystemHintOp_SEVL
///         SendEventLocal();
/// 
///     when SystemHintOp_ESB
///         SynchronizeErrors();
///         AArch64.ESBOperation();
///         if PSTATE.EL IN {EL0, EL1} && EL2Enabled() then AArch64.vESBOperation();
///         TakeUnmaskedSErrorInterrupts();
/// 
///     when SystemHintOp_PSB
///         ProfilingSynchronizationBarrier();
/// 
///     when SystemHintOp_TSB
///         TraceSynchronizationBarrier();
/// 
///     when SystemHintOp_CSDB
///         ConsumptionOfSpeculativeDataBarrier();
/// 
///     when SystemHintOp_BTI
///         SetBTypeNext('00');
/// 
///     otherwise // do nothing
/// ```
#[box_to_static_reference]
pub(super) fn tsb() -> &'static [IrStatement] {
    [].into()
}
