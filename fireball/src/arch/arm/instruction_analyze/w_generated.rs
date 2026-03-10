use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

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
pub(super) fn wfe() -> &'static [IrStatement] {
    [].into()
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
pub(super) fn wfi() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = Ones(PL);
/// bits(rsize) operand1 = X[n];
/// bits(rsize) operand2 = X[m];
/// bits(PL) result;
/// boolean last = TRUE;
/// 
/// for e = 0 to elements-1
///     boolean cond;
///     case op of
///         when Cmp_LT cond = (Int(operand1, unsigned) <  Int(operand2, unsigned));
///         when Cmp_LE cond = (Int(operand1, unsigned) <= Int(operand2, unsigned));
/// 
///     last = last && cond;
///     ElemP[result, e, esize] = if last then '1' else '0';
///     operand1 = operand1 + 1;
/// 
/// PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn whilele() -> &'static [IrStatement] {
    [exception("whilele")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = Ones(PL);
/// bits(rsize) operand1 = X[n];
/// bits(rsize) operand2 = X[m];
/// bits(PL) result;
/// boolean last = TRUE;
/// 
/// for e = 0 to elements-1
///     boolean cond;
///     case op of
///         when Cmp_LT cond = (Int(operand1, unsigned) <  Int(operand2, unsigned));
///         when Cmp_LE cond = (Int(operand1, unsigned) <= Int(operand2, unsigned));
/// 
///     last = last && cond;
///     ElemP[result, e, esize] = if last then '1' else '0';
///     operand1 = operand1 + 1;
/// 
/// PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn whilelo() -> &'static [IrStatement] {
    [exception("whilelo")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = Ones(PL);
/// bits(rsize) operand1 = X[n];
/// bits(rsize) operand2 = X[m];
/// bits(PL) result;
/// boolean last = TRUE;
/// 
/// for e = 0 to elements-1
///     boolean cond;
///     case op of
///         when Cmp_LT cond = (Int(operand1, unsigned) <  Int(operand2, unsigned));
///         when Cmp_LE cond = (Int(operand1, unsigned) <= Int(operand2, unsigned));
/// 
///     last = last && cond;
///     ElemP[result, e, esize] = if last then '1' else '0';
///     operand1 = operand1 + 1;
/// 
/// PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn whilels() -> &'static [IrStatement] {
    [exception("whilels")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = Ones(PL);
/// bits(rsize) operand1 = X[n];
/// bits(rsize) operand2 = X[m];
/// bits(PL) result;
/// boolean last = TRUE;
/// 
/// for e = 0 to elements-1
///     boolean cond;
///     case op of
///         when Cmp_LT cond = (Int(operand1, unsigned) <  Int(operand2, unsigned));
///         when Cmp_LE cond = (Int(operand1, unsigned) <= Int(operand2, unsigned));
/// 
///     last = last && cond;
///     ElemP[result, e, esize] = if last then '1' else '0';
///     operand1 = operand1 + 1;
/// 
/// PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn whilelt() -> &'static [IrStatement] {
    [exception("whilelt")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(PL) operand = P[n];
/// 
/// hsb = HighestSetBit(operand);
/// if hsb < 0 || IsOnes(operand<hsb:0>) then
///     FFR[] = operand;
/// else // not a monotonic predicate
///     FFR[] = bits(PL) UNKNOWN;
/// ```
#[box_to_static_reference]
pub(super) fn wrffr() -> &'static [IrStatement] {
    [exception("wrffr")].into()
}
