use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// DCPSInstruction(target_level);
/// ```
#[box_to_static_reference]
pub(super) fn dcps1() -> &'static [IrStatement] {
    [exception("dcps1")].into()
}

/// # Pseudocode
/// ```text
/// DCPSInstruction(target_level);
/// ```
#[box_to_static_reference]
pub(super) fn dcps2() -> &'static [IrStatement] {
    [exception("dcps2")].into()
}

/// # Pseudocode
/// ```text
/// DCPSInstruction(target_level);
/// ```
#[box_to_static_reference]
pub(super) fn dcps3() -> &'static [IrStatement] {
    [exception("dcps3")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(64) operand1 = X[dn];
/// 
/// X[dn] = operand1 - (count * imm);
/// ```
#[box_to_static_reference]
pub(super) fn decb() -> &'static [IrStatement] {
    [exception("decb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// integer count = DecodePredCount(pat, esize);
/// bits(VL) operand1 = Z[dn];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     Elem[result, e, esize] = Elem[operand1, e, esize] - (count * imm);
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn decd() -> &'static [IrStatement] {
    [exception("decd")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) operand1 = X[dn];
/// bits(PL) operand2 = P[m];
/// integer count = 0;
/// 
/// for e = 0 to elements-1
///     if ElemP[operand2, e, esize] == '1' then
///         count = count + 1;
/// 
/// X[dn] = operand1 - count;
/// ```
#[box_to_static_reference]
pub(super) fn decp() -> &'static [IrStatement] {
    [exception("decp")].into()
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
pub(super) fn dgh() -> &'static [IrStatement] {
    [exception("dgh")].into()
}

/// # Pseudocode
/// ```text
/// DataMemoryBarrier(domain, types);
/// ```
#[box_to_static_reference]
pub(super) fn dmb() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// DRPSInstruction();
/// ```
#[box_to_static_reference]
pub(super) fn drps() -> &'static [IrStatement] {
    [exception("drps")].into()
}

/// # Pseudocode
/// ```text
/// DataSynchronizationBarrier(domain, types);
/// ```
#[box_to_static_reference]
pub(super) fn dsb() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(idxdsize) operand = V[n];
/// bits(datasize) result;
/// bits(esize) element;
/// 
/// element = Elem[operand, index, esize];
/// for e = 0 to elements-1
///     Elem[result, e, esize] = element;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn dup() -> &'static [IrStatement] {
    [exception("dup")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(VL) result = Replicate(imm);
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn dupm() -> &'static [IrStatement] {
    [exception("dupm")].into()
}
