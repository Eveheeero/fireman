use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// if ConditionHolds(condition) then
///     BranchTo(PC[] + offset, BranchType_DIR);
/// ```
#[box_to_static_reference]
pub(super) fn b() -> &'static [IrStatement] {
    [jump(o1())].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckFPAdvSIMDEnabled();
/// 
/// bits(128) Vm = V[m];
/// bits(128) Vn = V[n];
/// bits(128) Va = V[a];
/// V[d] = Vn EOR (Vm AND NOT(Va));
/// ```
#[box_to_static_reference]
pub(super) fn bcax() -> &'static [IrStatement] {
    [exception("bcax")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(32) operand = V[n];
/// bits(16) result;
/// 
/// result = FPConvertBF(operand, FPCR);
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn bfcvt() -> &'static [IrStatement] {
    let v_0 = unknown_data();
    let stmt_1 = assign(v_0, o1(), o1_size());
    [stmt_1].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(128) operand = V[n];
/// bits(64) result;
/// 
/// for e = 0 to elements-1
///     Elem[result, e, 16] = FPConvertBF(Elem[operand, e, 32], FPCR);
/// 
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn bfcvtn() -> &'static [IrStatement] {
    [exception("bfcvtn")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV 32;
/// bits(PL) mask = P[g];
/// bits(VL) operand  = Z[n];
/// bits(VL) result = Z[d];
/// 
/// for e = 0 to elements-1
///     bits(32) element = Elem[operand, e, 32];
///     if ElemP[mask, e, 32] == '1' then
///         Elem[result, 2*e+1, 16] = FPConvertBF(element, FPCR);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn bfcvtnt() -> &'static [IrStatement] {
    [exception("bfcvtnt")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(128)      operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// 
/// for e = 0 to elements-1
///     bits(16) elt1_a = Elem[operand1, 2 * e + 0, 16];
///     bits(16) elt1_b = Elem[operand1, 2 * e + 1, 16];
///     bits(16) elt2_a = Elem[operand2, 2 * i + 0, 16];
///     bits(16) elt2_b = Elem[operand2, 2 * i + 1, 16];
/// 
///     bits(32) sum = BFAdd(BFMul(elt1_a, elt2_a), BFMul(elt1_b, elt2_b));
///     Elem[result, e, 32] = BFAdd(Elem[operand3, e, 32], sum);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn bfdot() -> &'static [IrStatement] {
    [exception("bfdot")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) dst = if inzero then Zeros() else X[d];
/// bits(datasize) src = X[n];
/// 
/// // perform bitfield move on low bits
/// bits(datasize) bot = (dst AND NOT(wmask)) OR (ROR(src, R) AND wmask);
/// 
/// // determine extension bits (sign, zero or dest register)
/// bits(datasize) top = if extend then Replicate(src<S>) else dst;
/// 
/// // combine extension bits and result bits
/// X[d] = (top AND NOT(tmask)) OR (bot AND tmask);
/// ```
#[box_to_static_reference]
pub(super) fn bfm() -> &'static [IrStatement] {
    [exception("bfm")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(128) operand1 = V[n];
/// bits(128) operand2 = V[m];
/// bits(128) operand3 = V[d];
/// bits(128) result;
/// 
/// bits(32) element2 = Elem[operand2, index, 16] : Zeros(16);
/// 
/// for e = 0 to elements-1
///     bits(32) element1 = Elem[operand1, 2 * e + sel, 16] : Zeros(16);
///     bits(32) addend = Elem[operand3, e, 32];
///     Elem[result, e, 32] = FPMulAdd(addend, element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn bfmlal() -> &'static [IrStatement] {
    [exception("bfmlal")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV 32;
/// bits(VL) operand1 = Z[n];
/// bits(VL) operand2 = Z[m];
/// bits(VL) operand3 = Z[da];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(32) element1 = Elem[operand1, 2 * e + 0, 16] : Zeros(16);
///     bits(32) element2 = Elem[operand2, 2 * e + 0, 16] : Zeros(16);
///     bits(32) element3 = Elem[operand3, e, 32];
///     Elem[result, e, 32] = FPMulAdd(element3, element1, element2, FPCR);
/// 
/// Z[da] = result;
/// ```
#[box_to_static_reference]
pub(super) fn bfmlalb() -> &'static [IrStatement] {
    [exception("bfmlalb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV 32;
/// bits(VL) operand1 = Z[n];
/// bits(VL) operand2 = Z[m];
/// bits(VL) operand3 = Z[da];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(32) element1 = Elem[operand1, 2 * e + 1, 16] : Zeros(16);
///     bits(32) element2 = Elem[operand2, 2 * e + 1, 16] : Zeros(16);
///     bits(32) element3 = Elem[operand3, e, 32];
///     Elem[result, e, 32] = FPMulAdd(element3, element1, element2, FPCR);
/// 
/// Z[da] = result;
/// ```
#[box_to_static_reference]
pub(super) fn bfmlalt() -> &'static [IrStatement] {
    [exception("bfmlalt")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(128) op1 = V[n];
/// bits(128) op2 = V[m];
/// bits(128) acc = V[d];
/// 
/// V[d] = BFMatMulAdd(acc, op1, op2);
/// ```
#[box_to_static_reference]
pub(super) fn bfmmla() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand;
/// bits(datasize) result;
/// 
/// case operation of
///     when ImmediateOp_MOVI
///         result = imm;
///     when ImmediateOp_MVNI
///         result = NOT(imm);
///     when ImmediateOp_ORR
///         operand = V[rd];
///         result = operand OR imm;
///     when ImmediateOp_BIC
///         operand = V[rd];
///         result = operand AND NOT(imm);
/// 
/// V[rd] = result;
/// ```
#[box_to_static_reference]
pub(super) fn bic() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), u::not(o3())), o1(), o1_size());
    [assignment].into()
}

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
pub(super) fn bics() -> &'static [IrStatement] {
    let op = b::and(o2(), u::not(o3()));
    let assignment = assign(op.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(op, o1_size(), &[&pstate_n, &pstate_z, &pstate_c, &pstate_v]);
    [calc_flags, assignment].into()
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
pub(super) fn bif() -> &'static [IrStatement] {
    [exception("bif")].into()
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
pub(super) fn bit() -> &'static [IrStatement] {
    [exception("bit")].into()
}

/// # Pseudocode
/// ```text
/// if branch_type == BranchType_DIRCALL then X[30] = PC[] + 4;
/// 
/// BranchTo(PC[] + offset, branch_type);
/// ```
#[box_to_static_reference]
pub(super) fn bl() -> &'static [IrStatement] {
    let save_lr = assign(b::add(pc.clone(), instruction_byte_size()), x30.clone(), size_relative(x30.clone()));
    let do_jump = jump(o1());
    [save_lr, do_jump].into()
}

/// # Pseudocode
/// ```text
/// bits(64) target = X[n];
/// boolean auth_then_branch = TRUE;
/// 
/// if pac then
///     bits(64) modifier = if source_is_sp then SP[] else X[m];
/// 
///     if use_key_a then
///         target = AuthIA(target, modifier, auth_then_branch);
///     else
///         target = AuthIB(target, modifier, auth_then_branch);
/// 
/// if branch_type == BranchType_INDCALL then X[30] = PC[] + 4;
/// 
/// // Value in BTypeNext will be used to set PSTATE.BTYPE
/// case branch_type of
///     when BranchType_INDIR           // BR, BRAA, BRAB, BRAAZ, BRABZ
///         if InGuardedPage then
///             if n == 16 || n == 17 then
///                 BTypeNext = '01';
///             else
///                 BTypeNext = '11';
///         else
///             BTypeNext = '01';
///     when BranchType_INDCALL         // BLR, BLRAA, BLRAB, BLRAAZ, BLRABZ
///         BTypeNext = '10';
///     when BranchType_RET             // RET, RETAA, RETAB
///         BTypeNext = '00';
/// 
/// BranchTo(target, branch_type);
/// ```
#[box_to_static_reference]
pub(super) fn blr() -> &'static [IrStatement] {
    let save_lr = assign(b::add(pc.clone(), instruction_byte_size()), x30.clone(), size_relative(x30.clone()));
    let do_jump = jump(o1());
    [save_lr, do_jump].into()
}

/// # Pseudocode
/// ```text
/// bits(64) target = X[n];
/// boolean auth_then_branch = TRUE;
/// 
/// if pac then
///     bits(64) modifier = if source_is_sp then SP[] else X[m];
/// 
///     if use_key_a then
///         target = AuthIA(target, modifier, auth_then_branch);
///     else
///         target = AuthIB(target, modifier, auth_then_branch);
/// 
/// if branch_type == BranchType_INDCALL then X[30] = PC[] + 4;
/// 
/// // Value in BTypeNext will be used to set PSTATE.BTYPE
/// case branch_type of
///     when BranchType_INDIR           // BR, BRAA, BRAB, BRAAZ, BRABZ
///         if InGuardedPage then
///             if n == 16 || n == 17 then
///                 BTypeNext = '01';
///             else
///                 BTypeNext = '11';
///         else
///             BTypeNext = '01';
///     when BranchType_INDCALL         // BLR, BLRAA, BLRAB, BLRAAZ, BLRABZ
///         BTypeNext = '10';
///     when BranchType_RET             // RET, RETAA, RETAB
///         BTypeNext = '00';
/// 
/// BranchTo(target, branch_type);
/// ```
#[box_to_static_reference]
pub(super) fn blraa() -> &'static [IrStatement] {
    [exception("blraa")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) target = X[n];
/// boolean auth_then_branch = TRUE;
/// 
/// if pac then
///     bits(64) modifier = if source_is_sp then SP[] else X[m];
/// 
///     if use_key_a then
///         target = AuthIA(target, modifier, auth_then_branch);
///     else
///         target = AuthIB(target, modifier, auth_then_branch);
/// 
/// if branch_type == BranchType_INDCALL then X[30] = PC[] + 4;
/// 
/// // Value in BTypeNext will be used to set PSTATE.BTYPE
/// case branch_type of
///     when BranchType_INDIR           // BR, BRAA, BRAB, BRAAZ, BRABZ
///         if InGuardedPage then
///             if n == 16 || n == 17 then
///                 BTypeNext = '01';
///             else
///                 BTypeNext = '11';
///         else
///             BTypeNext = '01';
///     when BranchType_INDCALL         // BLR, BLRAA, BLRAB, BLRAAZ, BLRABZ
///         BTypeNext = '10';
///     when BranchType_RET             // RET, RETAA, RETAB
///         BTypeNext = '00';
/// 
/// BranchTo(target, branch_type);
/// ```
#[box_to_static_reference]
pub(super) fn br() -> &'static [IrStatement] {
    [jump(o1())].into()
}

/// # Pseudocode
/// ```text
/// bits(64) target = X[n];
/// boolean auth_then_branch = TRUE;
/// 
/// if pac then
///     bits(64) modifier = if source_is_sp then SP[] else X[m];
/// 
///     if use_key_a then
///         target = AuthIA(target, modifier, auth_then_branch);
///     else
///         target = AuthIB(target, modifier, auth_then_branch);
/// 
/// if branch_type == BranchType_INDCALL then X[30] = PC[] + 4;
/// 
/// // Value in BTypeNext will be used to set PSTATE.BTYPE
/// case branch_type of
///     when BranchType_INDIR           // BR, BRAA, BRAB, BRAAZ, BRABZ
///         if InGuardedPage then
///             if n == 16 || n == 17 then
///                 BTypeNext = '01';
///             else
///                 BTypeNext = '11';
///         else
///             BTypeNext = '01';
///     when BranchType_INDCALL         // BLR, BLRAA, BLRAB, BLRAAZ, BLRABZ
///         BTypeNext = '10';
///     when BranchType_RET             // RET, RETAA, RETAB
///         BTypeNext = '00';
/// 
/// BranchTo(target, branch_type);
/// ```
#[box_to_static_reference]
pub(super) fn braa() -> &'static [IrStatement] {
    [exception("braa")].into()
}

/// # Pseudocode
/// ```text
/// AArch64.SoftwareBreakpoint(comment);
/// ```
#[box_to_static_reference]
pub(super) fn brk() -> &'static [IrStatement] {
    [exception("brk")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) operand  = P[n];
/// bits(PL) operand2 = P[d];
/// boolean break = FALSE;
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     boolean element = ElemP[operand, e, esize] == '1';
///     if ElemP[mask, e, esize] == '1' then
///         ElemP[result, e, esize] = if !break then '1' else '0';
///         break = break || element;
///     elsif merging then
///         ElemP[result, e, esize] = ElemP[operand2, e, esize];
///     else
///         ElemP[result, e, esize] = '0';
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn brka() -> &'static [IrStatement] {
    [exception("brka")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) operand  = P[n];
/// bits(PL) operand2 = P[d];
/// boolean break = FALSE;
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     boolean element = ElemP[operand, e, esize] == '1';
///     if ElemP[mask, e, esize] == '1' then
///         break = break || element;
///         ElemP[result, e, esize] = if !break then '1' else '0';
///     elsif merging then
///         ElemP[result, e, esize] = ElemP[operand2, e, esize];
///     else
///         ElemP[result, e, esize] = '0';
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn brkb() -> &'static [IrStatement] {
    [exception("brkb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(PL) mask = P[g];
/// bits(PL) operand1 = P[n];
/// bits(PL) operand2 = P[dm];
/// bits(PL) result;
/// 
/// if LastActive(mask, operand1, 8) == '1' then
///     result = operand2;
/// else
///     result = Zeros();
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = PredTest(Ones(PL), result, 8);
/// P[dm] = result;
/// ```
#[box_to_static_reference]
pub(super) fn brkn() -> &'static [IrStatement] {
    [exception("brkn")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) operand1 = P[n];
/// bits(PL) operand2 = P[m];
/// bits(PL) result;
/// boolean last = (LastActive(mask, operand1, 8) == '1');
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, 8] == '1' then
///         ElemP[result, e, 8] = if last then '1' else '0';
///         last = last && (ElemP[operand2, e, 8] == '0');
///     else
///         ElemP[result, e, 8] = '0';
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn brkpa() -> &'static [IrStatement] {
    [exception("brkpa")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) operand1 = P[n];
/// bits(PL) operand2 = P[m];
/// bits(PL) result;
/// boolean last = (LastActive(mask, operand1, 8) == '1');
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, 8] == '1' then
///         last = last && (ElemP[operand2, e, 8] == '0');
///         ElemP[result, e, 8] = if last then '1' else '0';
///     else
///         ElemP[result, e, 8] = '0';
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn brkpb() -> &'static [IrStatement] {
    [exception("brkpb")].into()
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
pub(super) fn bsl() -> &'static [IrStatement] {
    [exception("bsl")].into()
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
pub(super) fn bti() -> &'static [IrStatement] {
    [exception("bti")].into()
}
