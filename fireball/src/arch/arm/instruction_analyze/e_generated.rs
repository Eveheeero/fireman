use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = ShiftReg(m, shift_type, shift_amount);
/// 
/// if invert then operand2 = NOT(operand2);
/// 
/// case op of
///     when LogicalOp_AND result = operand1 AND operand2;
///     when LogicalOp_ORR result = operand1 OR  operand2;
///     when LogicalOp_EOR result = operand1 EOR operand2;
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = result<datasize-1>:IsZeroBit(result):'00';
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn eon() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), u::not(o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1;
/// bits(datasize) operand2;
/// bits(datasize) operand3;
/// bits(datasize) operand4 = V[n];
/// 
/// case op of
///     when VBitOp_VEOR
///         operand1 = V[m];
///         operand2 = Zeros();
///         operand3 = Ones();
///     when VBitOp_VBSL
///         operand1 = V[m];
///         operand2 = operand1;
///         operand3 = V[d];
///     when VBitOp_VBIT
///         operand1 = V[d];
///         operand2 = operand1;
///         operand3 = V[m];
///     when VBitOp_VBIF
///         operand1 = V[d];
///         operand2 = operand1;
///         operand3 = NOT(V[m]);
/// 
/// V[d] = operand1 EOR ((operand2 EOR operand4) AND operand3);
/// ```
#[box_to_static_reference]
pub(super) fn eor() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckFPAdvSIMDEnabled();
/// 
/// bits(128) Vm = V[m];
/// bits(128) Vn = V[n];
/// bits(128) Va = V[a];
/// V[d] = Vn EOR Vm EOR Va;
/// ```
#[box_to_static_reference]
pub(super) fn eor3() -> &'static [IrStatement] {
    [exception("eor3")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand = Z[n];
/// bits(esize) result = Zeros(esize);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         result = result EOR Elem[operand, e, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn eorv() -> &'static [IrStatement] {
    [exception("eorv")].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckForERetTrap(pac, use_key_a);
/// bits(64) target = ELR[];
/// boolean auth_then_branch = TRUE;
/// 
/// if pac then
///     if use_key_a then
///         target = AuthIA(ELR[], SP[], auth_then_branch);
///     else
///         target = AuthIB(ELR[], SP[], auth_then_branch);
/// 
/// AArch64.ExceptionReturn(target, SPSR[]);
/// ```
#[box_to_static_reference]
pub(super) fn eret() -> &'static [IrStatement] {
    [exception("eret")].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckForERetTrap(pac, use_key_a);
/// bits(64) target = ELR[];
/// boolean auth_then_branch = TRUE;
/// 
/// if pac then
///     if use_key_a then
///         target = AuthIA(ELR[], SP[], auth_then_branch);
///     else
///         target = AuthIB(ELR[], SP[], auth_then_branch);
/// 
/// AArch64.ExceptionReturn(target, SPSR[]);
/// ```
#[box_to_static_reference]
pub(super) fn eretaa() -> &'static [IrStatement] {
    [exception("eretaa")].into()
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
pub(super) fn esb() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) hi = V[m];
/// bits(datasize) lo = V[n];
/// bits(datasize*2) concat = hi : lo;
/// 
/// V[d] = concat<position+datasize-1:position>;
/// ```
#[box_to_static_reference]
pub(super) fn ext() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// bits(2*datasize) concat = operand1:operand2;
/// 
/// result = concat<lsb+datasize-1:lsb>;
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn extr() -> &'static [IrStatement] {
    let assignment = assign(o3(), o1(), o1_size());
    [assignment].into()
}
