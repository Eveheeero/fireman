use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[dn];
/// bits(VL) operand2 = Z[m];
/// bits(VL) operand3 = Z[a];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     integer element1 = UInt(Elem[operand1, e, esize]);
///     integer element2 = UInt(Elem[operand2, e, esize]);
///     if ElemP[mask, e, esize] == '1' then
///         integer product = element1 * element2;
///         if sub_op then
///             Elem[result, e, esize] = Elem[operand3, e, esize] - product;
///         else
///             Elem[result, e, esize] = Elem[operand3, e, esize] + product;
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn mad() -> &'static [IrStatement] {
    [exception("mad")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// bits(destsize) operand3 = X[a];
/// 
/// integer result;
/// 
/// if sub_op then
///     result = UInt(operand3) - (UInt(operand1) * UInt(operand2));
/// else
///     result = UInt(operand3) + (UInt(operand1) * UInt(operand2));
/// 
/// X[d] = result<destsize-1:0>;
/// ```
#[box_to_static_reference]
pub(super) fn madd() -> &'static [IrStatement] {
    let assignment = assign(b::add(o4(), b::mul(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(idxdsize) operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// bits(esize) product;
/// 
/// element2 = UInt(Elem[operand2, index, esize]);
/// for e = 0 to elements-1
///     element1 = UInt(Elem[operand1, e, esize]);
///     product = (element1 * element2)<esize-1:0>;
///     if sub_op then
///         Elem[result, e, esize] = Elem[operand3, e, esize] - product;
///     else
///         Elem[result, e, esize] = Elem[operand3, e, esize] + product;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn mla() -> &'static [IrStatement] {
    [exception("mla")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(idxdsize) operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// bits(esize) product;
/// 
/// element2 = UInt(Elem[operand2, index, esize]);
/// for e = 0 to elements-1
///     element1 = UInt(Elem[operand1, e, esize]);
///     product = (element1 * element2)<esize-1:0>;
///     if sub_op then
///         Elem[result, e, esize] = Elem[operand3, e, esize] - product;
///     else
///         Elem[result, e, esize] = Elem[operand3, e, esize] + product;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn mls() -> &'static [IrStatement] {
    [exception("mls")].into()
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
pub(super) fn movi() -> &'static [IrStatement] {
    [exception("movi")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// 
/// if opcode == MoveWideOp_K then
///     result = X[d];
/// else
///     result = Zeros();
/// 
/// result<pos+15:pos> = imm;
/// if opcode == MoveWideOp_N then
///     result = NOT(result);
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn movk() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// 
/// if opcode == MoveWideOp_K then
///     result = X[d];
/// else
///     result = Zeros();
/// 
/// result<pos+15:pos> = imm;
/// if opcode == MoveWideOp_N then
///     result = NOT(result);
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn movn() -> &'static [IrStatement] {
    let assignment = assign(u::not(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[n];
/// bits(VL) dest = Z[d];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element = Elem[operand1, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = element;
///     elsif merging then
///         Elem[result, e, esize] = Elem[dest, e, esize];
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn movprfx() -> &'static [IrStatement] {
    [exception("movprfx")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// 
/// if opcode == MoveWideOp_K then
///     result = X[d];
/// else
///     result = Zeros();
/// 
/// result<pos+15:pos> = imm;
/// if opcode == MoveWideOp_N then
///     result = NOT(result);
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn movz() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// if read then
///     X[t] = AArch64.SysRegRead(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2);
/// else
///     AArch64.SysRegWrite(sys_op0, sys_op1, sys_crn, sys_crm, sys_op2, X[t]);
/// ```
#[box_to_static_reference]
pub(super) fn mrs() -> &'static [IrStatement] {
    [exception("mrs")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[dn];
/// bits(VL) operand2 = Z[m];
/// bits(VL) operand3 = Z[a];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     integer element1 = UInt(Elem[operand1, e, esize]);
///     integer element2 = UInt(Elem[operand2, e, esize]);
///     if ElemP[mask, e, esize] == '1' then
///         integer product = element1 * element2;
///         if sub_op then
///             Elem[result, e, esize] = Elem[operand3, e, esize] - product;
///         else
///             Elem[result, e, esize] = Elem[operand3, e, esize] + product;
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn msb() -> &'static [IrStatement] {
    [exception("msb")].into()
}

/// # Pseudocode
/// ```text
/// case field of
///     when PSTATEField_SSBS
///         PSTATE.SSBS = operand<0>;
///     when PSTATEField_SP
///         PSTATE.SP = operand<0>;
///     when PSTATEField_DAIFSet
///         PSTATE.D = PSTATE.D OR operand<3>;
///         PSTATE.A = PSTATE.A OR operand<2>;
///         PSTATE.I = PSTATE.I OR operand<1>;
///         PSTATE.F = PSTATE.F OR operand<0>;
///     when PSTATEField_DAIFClr
///         PSTATE.D = PSTATE.D AND NOT(operand<3>);
///         PSTATE.A = PSTATE.A AND NOT(operand<2>);
///         PSTATE.I = PSTATE.I AND NOT(operand<1>);
///         PSTATE.F = PSTATE.F AND NOT(operand<0>);
///     when PSTATEField_PAN
///         PSTATE.PAN = operand<0>;
///     when PSTATEField_UAO
///         PSTATE.UAO = operand<0>;
///     when PSTATEField_DIT
///         PSTATE.DIT = operand<0>;
///     when PSTATEField_TCO
///         PSTATE.TCO = operand<0>;
/// ```
#[box_to_static_reference]
pub(super) fn msr() -> &'static [IrStatement] {
    [exception("msr")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// bits(destsize) operand3 = X[a];
/// 
/// integer result;
/// 
/// if sub_op then
///     result = UInt(operand3) - (UInt(operand1) * UInt(operand2));
/// else
///     result = UInt(operand3) + (UInt(operand1) * UInt(operand2));
/// 
/// X[d] = result<destsize-1:0>;
/// ```
#[box_to_static_reference]
pub(super) fn msub() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o4(), b::mul(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(idxdsize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// bits(esize) product;
/// 
/// element2 = UInt(Elem[operand2, index, esize]);
/// for e = 0 to elements-1
///     element1 = UInt(Elem[operand1, e, esize]);
///     product = (element1 * element2)<esize-1:0>;
///     Elem[result, e, esize] = product;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn mul() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
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
pub(super) fn mvni() -> &'static [IrStatement] {
    [exception("mvni")].into()
}
