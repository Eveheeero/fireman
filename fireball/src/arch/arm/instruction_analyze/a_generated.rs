use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

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
pub(super) fn abs() -> &'static [IrStatement] {
    [exception("abs")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// bits(4) nzcv;
/// 
/// if sub_op then
///     operand2 = NOT(operand2);
/// 
/// (result, nzcv) = AddWithCarry(operand1, operand2, PSTATE.C);
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = nzcv;
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn adc() -> &'static [IrStatement] {
    let op = b::add(b::add(o2(), o3()), pstate_c.clone());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// bits(4) nzcv;
/// 
/// if sub_op then
///     operand2 = NOT(operand2);
/// 
/// (result, nzcv) = AddWithCarry(operand1, operand2, PSTATE.C);
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = nzcv;
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn adcs() -> &'static [IrStatement] {
    let op = b::add(b::add(o2(), o3()), pstate_c.clone());
    let assignment = assign(op.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(op, o1_size(), &[&pstate_n, &pstate_z, &pstate_c, &pstate_v]);
    [calc_flags, assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = if n == 31 then SP[] else X[n];
/// bits(datasize) operand2 = ExtendReg(m, extend_type, shift);
/// bits(4) nzcv;
/// bit carry_in;
/// 
/// if sub_op then
///     operand2 = NOT(operand2);
///     carry_in = '1';
/// else
///     carry_in = '0';
/// 
/// (result, nzcv) = AddWithCarry(operand1, operand2, carry_in);
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = nzcv;
/// 
/// if d == 31 && !setflags then
///     SP[] = result;
/// else
///     X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn add() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) operand1 = if n == 31 then SP[] else X[n];
/// bits(4) start_tag = AArch64.AllocationTagFromAddress(operand1);
/// bits(16) exclude = GCR_EL1.Exclude;
/// bits(64) result;
/// bits(4) rtag;
/// 
/// if AArch64.AllocationTagAccessIsEnabled() then
///     rtag = AArch64.ChooseNonExcludedTag(start_tag, tag_offset, exclude);
/// else
///     rtag = '0000';
/// 
/// if ADD then
///     (result, -) = AddWithCarry(operand1, offset, '0');
/// else
///     (result, -) = AddWithCarry(operand1, NOT(offset), '1');
/// 
/// result = AArch64.AddressWithAllocationTag(result, rtag);
/// 
/// if d == 31 then
///     SP[] = result;
/// else
///     X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn addg() -> &'static [IrStatement] {
    [exception("addg")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(2*datasize) operand1 = V[n];
/// bits(2*datasize) operand2 = V[m];
/// bits(datasize)   result;
/// integer round_const = if round then 1 << (esize - 1) else 0;
/// bits(2*esize) element1;
/// bits(2*esize) element2;
/// bits(2*esize) sum;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, 2*esize];
///     element2 = Elem[operand2, e, 2*esize];
///     if sub_op then
///         sum = element1 - element2;
///     else
///         sum = element1 + element2;
///     sum = sum + round_const;
///     Elem[result, e, esize] = sum<2*esize-1:esize>;
/// 
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn addhn() -> &'static [IrStatement] {
    [exception("addhn")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn addp() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(64) operand1 = if n == 31 then SP[] else X[n];
/// bits(64) result = operand1 + (imm * (PL DIV 8));
/// 
/// if d == 31 then
///     SP[] = result;
/// else
///     X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn addpl() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = if n == 31 then SP[] else X[n];
/// bits(datasize) operand2 = ExtendReg(m, extend_type, shift);
/// bits(4) nzcv;
/// bit carry_in;
/// 
/// if sub_op then
///     operand2 = NOT(operand2);
///     carry_in = '1';
/// else
///     carry_in = '0';
/// 
/// (result, nzcv) = AddWithCarry(operand1, operand2, carry_in);
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = nzcv;
/// 
/// if d == 31 && !setflags then
///     SP[] = result;
/// else
///     X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn adds() -> &'static [IrStatement] {
    let op = b::add(o2(), o3());
    let assignment = assign(op.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(op, o1_size(), &[&pstate_n, &pstate_z, &pstate_c, &pstate_v]);
    [calc_flags, assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn addv() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(64) operand1 = if n == 31 then SP[] else X[n];
/// bits(64) result = operand1 + (imm * (VL DIV 8));
/// 
/// if d == 31 then
///     SP[] = result;
/// else
///     X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn addvl() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// bits(64) base = PC[];
/// 
/// if page then
///     base<11:0> = Zeros(12);
/// 
/// X[d] = base + imm;
/// ```
#[box_to_static_reference]
pub(super) fn adr() -> &'static [IrStatement] {
    let assignment = assign(b::add(pc.clone(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) base = PC[];
/// 
/// if page then
///     base<11:0> = Zeros(12);
/// 
/// X[d] = base + imm;
/// ```
#[box_to_static_reference]
pub(super) fn adrp() -> &'static [IrStatement] {
    let page_base = b::and(pc.clone(), u::not(c(0xFFF)));
    let assignment = assign(b::add(page_base, o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckFPAdvSIMDEnabled();
/// 
/// bits(128) operand1 = V[d];
/// bits(128) operand2 = V[n];
/// bits(128) result;
/// result = operand1 EOR operand2;
/// if decrypt then
///     result = AESInvSubBytes(AESInvShiftRows(result));
/// else
///     result = AESSubBytes(AESShiftRows(result));
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn aesd() -> &'static [IrStatement] {
    [exception("aesd")].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckFPAdvSIMDEnabled();
/// 
/// bits(128) operand1 = V[d];
/// bits(128) operand2 = V[n];
/// bits(128) result;
/// result = operand1 EOR operand2;
/// if decrypt then
///     result = AESInvSubBytes(AESInvShiftRows(result));
/// else
///     result = AESSubBytes(AESShiftRows(result));
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn aese() -> &'static [IrStatement] {
    [exception("aese")].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckFPAdvSIMDEnabled();
/// 
/// bits(128) operand = V[n];
/// bits(128) result;
/// if decrypt then
///     result = AESInvMixColumns(operand);
/// else
///     result = AESMixColumns(operand);
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn aesimc() -> &'static [IrStatement] {
    [exception("aesimc")].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckFPAdvSIMDEnabled();
/// 
/// bits(128) operand = V[n];
/// bits(128) result;
/// if decrypt then
///     result = AESInvMixColumns(operand);
/// else
///     result = AESMixColumns(operand);
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn aesmc() -> &'static [IrStatement] {
    [exception("aesmc")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// 
/// if invert then operand2 = NOT(operand2);
/// 
/// case op of
///     when LogicalOp_AND
///         result = operand1 AND operand2;
///     when LogicalOp_ORR
///         result = operand1 OR operand2;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn and() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = imm;
/// 
/// case op of
///     when LogicalOp_AND result = operand1 AND operand2;
///     when LogicalOp_ORR result = operand1 OR  operand2;
///     when LogicalOp_EOR result = operand1 EOR operand2;
/// 
/// if setflags then
///     PSTATE.<N,Z,C,V> = result<datasize-1>:IsZeroBit(result):'00';
/// 
/// if d == 31 && !setflags then
///     SP[] = result;
/// else
///     X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ands() -> &'static [IrStatement] {
    let op = b::and(o2(), o3());
    let assignment = assign(op.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(op, o1_size(), &[&pstate_n, &pstate_z, &pstate_c, &pstate_v]);
    [calc_flags, assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand = Z[n];
/// bits(esize) result = Ones(esize);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         result = result AND Elem[operand, e, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn andv() -> &'static [IrStatement] {
    [exception("andv")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) operand1 = Z[dn];
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element1 = Elem[operand1, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = ASR(element1, shift);
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn asr() -> &'static [IrStatement] {
    let assignment = assign(b::sar(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[dn];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     integer element1 = SInt(Elem[operand1, e, esize]);
///     if ElemP[mask, e, esize] == '1' then
///         if element1 < 0 then
///             element1 = element1 + ((1 << shift) - 1);
///         Elem[result, e, esize] = (element1 >> shift)<esize-1:0>;
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn asrd() -> &'static [IrStatement] {
    [exception("asrd")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[dn];
/// bits(VL) operand2 = Z[m];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     integer shift = Min(UInt(element1), esize);
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = ASR(element2, shift);
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn asrr() -> &'static [IrStatement] {
    [exception("asrr")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand2 = X[m];
/// 
/// result = ShiftReg(n, shift_type, UInt(operand2) MOD datasize);
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn asrv() -> &'static [IrStatement] {
    let assignment = assign(b::sar(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// auth_then_branch = FALSE;
/// 
/// if HavePACExt() then
///     if source_is_sp then
///         X[d] = AuthDA(X[d], SP[], auth_then_branch);
///     else
///         X[d] = AuthDA(X[d], X[n], auth_then_branch);
/// ```
#[box_to_static_reference]
pub(super) fn autda() -> &'static [IrStatement] {
    let v_0 = unknown_data();
    let stmt_1 = assign(unknown_data(), o1(), o1_size());
    [stmt_1].into()
}

/// # Pseudocode
/// ```text
/// auth_then_branch = FALSE;
/// 
/// if HavePACExt() then
///     if source_is_sp then
///         X[d] = AuthDB(X[d], SP[], auth_then_branch);
///     else
///         X[d] = AuthDB(X[d], X[n], auth_then_branch);
/// ```
#[box_to_static_reference]
pub(super) fn autdb() -> &'static [IrStatement] {
    let v_0 = unknown_data();
    let stmt_1 = assign(unknown_data(), o1(), o1_size());
    [stmt_1].into()
}

/// # Pseudocode
/// ```text
/// auth_then_branch = FALSE;
/// 
/// if HavePACExt() then
///     if source_is_sp then
///         X[d] = AuthIA(X[d], SP[], auth_then_branch);
///     else
///         X[d] = AuthIA(X[d], X[n], auth_then_branch);
/// ```
#[box_to_static_reference]
pub(super) fn autia() -> &'static [IrStatement] {
    let v_0 = unknown_data();
    let stmt_1 = assign(unknown_data(), o1(), o1_size());
    [stmt_1].into()
}

/// # Pseudocode
/// ```text
/// auth_then_branch = FALSE;
/// 
/// if HavePACExt() then
///     if source_is_sp then
///         X[d] = AuthIB(X[d], SP[], auth_then_branch);
///     else
///         X[d] = AuthIB(X[d], X[n], auth_then_branch);
/// ```
#[box_to_static_reference]
pub(super) fn autib() -> &'static [IrStatement] {
    let v_0 = unknown_data();
    let stmt_1 = assign(unknown_data(), o1(), o1_size());
    [stmt_1].into()
}

/// # Pseudocode
/// ```text
/// bit N = '0';
/// bit Z = PSTATE.Z OR PSTATE.V;
/// bit C = PSTATE.C AND NOT(PSTATE.V);
/// bit V = '0';
/// 
/// PSTATE.N = N;
/// PSTATE.Z = Z;
/// PSTATE.C = C;
/// PSTATE.V = V;
/// ```
#[box_to_static_reference]
pub(super) fn axflag() -> &'static [IrStatement] {
    let old_z = pstate_z.clone();
    let old_c = pstate_c.clone();
    let old_v = pstate_v.clone();
    let set_n = assign(c(0), pstate_n.clone(), size_relative(pstate_n.clone()));
    let set_z = assign(b::or(old_z.clone(), old_v.clone()), pstate_z.clone(), size_relative(pstate_z.clone()));
    let set_c = assign(b::and(old_c, u::not(old_v)), pstate_c.clone(), size_relative(pstate_c.clone()));
    let set_v = assign(c(0), pstate_v.clone(), size_relative(pstate_v.clone()));
    [set_n, set_z, set_c, set_v].into()
}
