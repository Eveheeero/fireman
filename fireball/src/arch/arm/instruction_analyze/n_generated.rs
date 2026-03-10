use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) operand1 = P[n];
/// bits(PL) operand2 = P[m];
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     bit element1 = ElemP[operand1, e, esize];
///     bit element2 = ElemP[operand2, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         ElemP[result, e, esize] = NOT(element1 AND element2);
///     else
///         ElemP[result, e, esize] = '0';
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn nand() -> &'static [IrStatement] {
    [exception("nand")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// integer element;
/// 
/// for e = 0 to elements-1
///     element = SInt(Elem[operand, e, esize]);
///     if neg then
///         element = -element;
///     else
///         element = Abs(element);
///     Elem[result, e, esize] = element<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn neg() -> &'static [IrStatement] {
    let assignment = assign(u::neg(o2()), o1(), o1_size());
    [assignment].into()
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
pub(super) fn nop() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) operand1 = P[n];
/// bits(PL) operand2 = P[m];
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     bit element1 = ElemP[operand1, e, esize];
///     bit element2 = ElemP[operand2, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         ElemP[result, e, esize] = NOT(element1 OR element2);
///     else
///         ElemP[result, e, esize] = '0';
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn nor() -> &'static [IrStatement] {
    [exception("nor")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// bits(esize) element;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, esize];
///     Elem[result, e, esize] = NOT(element);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn not() -> &'static [IrStatement] {
    [exception("not")].into()
}
