use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// if source_is_sp then
///     X[d] = AddPACDA(X[d], SP[]);
/// else
///     X[d] = AddPACDA(X[d], X[n]);
/// ```
#[box_to_static_reference]
pub(super) fn pacda() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// if source_is_sp then
///     X[d] = AddPACDB(X[d], SP[]);
/// else
///     X[d] = AddPACDB(X[d], X[n]);
/// ```
#[box_to_static_reference]
pub(super) fn pacdb() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// if source_is_sp then
///     X[d] = AddPACGA(X[n], SP[]);
/// else
///     X[d] = AddPACGA(X[n], X[m]);
/// ```
#[box_to_static_reference]
pub(super) fn pacga() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// if HavePACExt() then
///     if source_is_sp then
///         X[d] = AddPACIA(X[d], SP[]);
///     else
///         X[d] = AddPACIA(X[d], X[n]);
/// ```
#[box_to_static_reference]
pub(super) fn pacia() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// if HavePACExt() then
///     if source_is_sp then
///         X[d] = AddPACIB(X[d], SP[]);
///     else
///         X[d] = AddPACIB(X[d], X[n]);
/// ```
#[box_to_static_reference]
pub(super) fn pacib() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// P[d] = Zeros(PL);
/// ```
#[box_to_static_reference]
pub(super) fn pfalse() -> &'static [IrStatement] {
    [exception("pfalse")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) result = P[dn];
/// integer first = -1;
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' && first == -1 then
///         first = e;
/// 
/// if first >= 0 then
///     ElemP[result, first, esize] = '1';
/// 
/// PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn pfirst() -> &'static [IrStatement] {
    [exception("pfirst")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// bits(esize) product;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     if poly then
///         product = PolynomialMult(element1, element2)<esize-1:0>;
///     else
///         product = (UInt(element1) * UInt(element2))<esize-1:0>;
///     Elem[result, e, esize] = product;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn pmul() -> &'static [IrStatement] {
    [exception("pmul")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize)   operand1 = Vpart[n, part];
/// bits(datasize)   operand2 = Vpart[m, part];
/// bits(2*datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     Elem[result, e, 2*esize] = PolynomialMult(element1, element2);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn pmull() -> &'static [IrStatement] {
    [exception("pmull")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) operand = P[dn];
/// bits(PL) result;
/// 
/// integer next = LastActiveElement(operand, esize) + 1;
/// 
/// while next < elements && (ElemP[mask, next, esize] == '0') do
///     next = next + 1;
/// 
/// result = Zeros();
/// if next < elements then
///     ElemP[result, next, esize] = '1';
/// 
/// PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn pnext() -> &'static [IrStatement] {
    [exception("pnext")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) base;
/// bits(64) addr;
/// base = Z[n];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + (offset << scale);
///         Hint_Prefetch(addr, pref_hint, level, stream);
/// ```
#[box_to_static_reference]
pub(super) fn prfb() -> &'static [IrStatement] {
    [exception("prfb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) base;
/// bits(64) addr;
/// base = Z[n];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + (offset << scale);
///         Hint_Prefetch(addr, pref_hint, level, stream);
/// ```
#[box_to_static_reference]
pub(super) fn prfd() -> &'static [IrStatement] {
    [exception("prfd")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) base;
/// bits(64) addr;
/// base = Z[n];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + (offset << scale);
///         Hint_Prefetch(addr, pref_hint, level, stream);
/// ```
#[box_to_static_reference]
pub(super) fn prfh() -> &'static [IrStatement] {
    [exception("prfh")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_NORMAL;
/// MemOp memop;
/// boolean signed;
/// integer regsize;
/// 
/// if opc<1> == '0' then
///     // store or zero-extending load
///     memop = if opc<0> == '1' then MemOp_LOAD else MemOp_STORE;
///     regsize = if size == '11' then 64 else 32;
///     signed = FALSE;
/// else
///     if size == '11' then
///         memop = MemOp_PREFETCH;
///         if opc<0> == '1' then UNDEFINED;
///     else
///         // sign-extending load
///         memop = MemOp_LOAD;
///         if size == '10' && opc<0> == '1' then UNDEFINED;
///         regsize = if opc<0> == '1' then 32 else 64;
///         signed = TRUE;
/// 
/// integer datasize = 8 << scale;
/// boolean tag_checked = memop != MemOp_PREFETCH && (wback || n != 31);if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// bits(64) address;
/// bits(datasize) data;
/// 
/// boolean wb_unknown = FALSE;
/// boolean rt_unknown = FALSE;
/// 
/// if memop == MemOp_LOAD && wback && n == t && n != 31 then
///     c = ConstrainUnpredictable(Unpredictable_WBOVERLAPLD);
///     assert c IN {Constraint_WBSUPPRESS, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_WBSUPPRESS wback = FALSE;       // writeback is suppressed
///         when Constraint_UNKNOWN    wb_unknown = TRUE;   // writeback is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE && wback && n == t && n != 31 then
///     c = ConstrainUnpredictable(Unpredictable_WBOVERLAPST);
///     assert c IN {Constraint_NONE, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_NONE       rt_unknown = FALSE;  // value stored is original value
///         when Constraint_UNKNOWN    rt_unknown = TRUE;   // value stored is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     if memop != MemOp_PREFETCH then CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// if ! postindex then
///     address = address + offset;
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         else
///             data = X[t];
///         Mem[address, datasize DIV 8, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, datasize DIV 8, acctype];
///         if signed then
///             X[t] = SignExtend(data, regsize);
///         else
///             X[t] = ZeroExtend(data, regsize);
/// 
///     when MemOp_PREFETCH
///         Prefetch(address, t<4:0>);
/// 
/// if wback then
///     if wb_unknown then
///         address = bits(64) UNKNOWN;
///     elsif postindex then
///         address = address + offset;
///     if n == 31 then
///         SP[] = address;
///     else
///         X[n] = address;
/// ```
#[box_to_static_reference]
pub(super) fn prfm() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_NORMAL;
/// MemOp memop;
/// boolean signed;
/// integer regsize;
/// 
/// if opc<1> == '0' then
///     // store or zero-extending load
///     memop = if opc<0> == '1' then MemOp_LOAD else MemOp_STORE;
///     regsize = if size == '11' then 64 else 32;
///     signed = FALSE;
/// else
///     if size == '11' then
///         memop = MemOp_PREFETCH;
///         if opc<0> == '1' then UNDEFINED;
///     else
///         // sign-extending load
///         memop = MemOp_LOAD;
///         if size == '10' && opc<0> == '1' then UNDEFINED;
///         regsize = if opc<0> == '1' then 32 else 64;
///         signed = TRUE;
/// 
/// integer datasize = 8 << scale;
/// boolean tag_checked = memop != MemOp_PREFETCH && (wback || n != 31);if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// bits(64) address;
/// bits(datasize) data;
/// 
/// boolean wb_unknown = FALSE;
/// boolean rt_unknown = FALSE;
/// 
/// if memop == MemOp_LOAD && wback && n == t && n != 31 then
///     c = ConstrainUnpredictable(Unpredictable_WBOVERLAPLD);
///     assert c IN {Constraint_WBSUPPRESS, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_WBSUPPRESS wback = FALSE;       // writeback is suppressed
///         when Constraint_UNKNOWN    wb_unknown = TRUE;   // writeback is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE && wback && n == t && n != 31 then
///     c = ConstrainUnpredictable(Unpredictable_WBOVERLAPST);
///     assert c IN {Constraint_NONE, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_NONE       rt_unknown = FALSE;  // value stored is original value
///         when Constraint_UNKNOWN    rt_unknown = TRUE;   // value stored is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     if memop != MemOp_PREFETCH then CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// if ! postindex then
///     address = address + offset;
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         else
///             data = X[t];
///         Mem[address, datasize DIV 8, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, datasize DIV 8, acctype];
///         if signed then
///             X[t] = SignExtend(data, regsize);
///         else
///             X[t] = ZeroExtend(data, regsize);
/// 
///     when MemOp_PREFETCH
///         Prefetch(address, t<4:0>);
/// 
/// if wback then
///     if wb_unknown then
///         address = bits(64) UNKNOWN;
///     elsif postindex then
///         address = address + offset;
///     if n == 31 then
///         SP[] = address;
///     else
///         X[n] = address;
/// ```
#[box_to_static_reference]
pub(super) fn prfum() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) base;
/// bits(64) addr;
/// base = Z[n];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + (offset << scale);
///         Hint_Prefetch(addr, pref_hint, level, stream);
/// ```
#[box_to_static_reference]
pub(super) fn prfw() -> &'static [IrStatement] {
    [exception("prfw")].into()
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
pub(super) fn psb() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// SpeculativeStoreBypassBarrierToPA();
/// ```
#[box_to_static_reference]
pub(super) fn pssbb() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(PL) mask = P[g];
/// bits(PL) result = P[n];
/// 
/// PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// ```
#[box_to_static_reference]
pub(super) fn ptest() -> &'static [IrStatement] {
    [exception("ptest")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// integer count = DecodePredCount(pat, esize);
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     ElemP[result, e, esize] = if e < count then '1' else '0';
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = PredTest(result, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ptrue() -> &'static [IrStatement] {
    [exception("ptrue")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) operand = P[n];
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     ElemP[result, e, esize] = ElemP[operand, if hi then e + elements else e, esize DIV 2];
/// 
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn punpkhi() -> &'static [IrStatement] {
    [exception("punpkhi")].into()
}
