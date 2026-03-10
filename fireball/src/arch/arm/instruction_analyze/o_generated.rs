use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

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
pub(super) fn orn() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), u::not(o3())), o1(), o1_size());
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
pub(super) fn orr() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
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
///         result = result OR Elem[operand, e, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn orv() -> &'static [IrStatement] {
    [exception("orv")].into()
}
