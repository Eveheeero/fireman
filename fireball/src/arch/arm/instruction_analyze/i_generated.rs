use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(64) operand1 = X[dn];
/// 
/// X[dn] = operand1 + (count * imm);
/// ```
#[box_to_static_reference]
pub(super) fn incb() -> &'static [IrStatement] {
    [exception("incb")].into()
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
///     Elem[result, e, esize] = Elem[operand1, e, esize] + (count * imm);
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn incd() -> &'static [IrStatement] {
    [exception("incd")].into()
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
/// X[dn] = operand1 + count;
/// ```
#[box_to_static_reference]
pub(super) fn incp() -> &'static [IrStatement] {
    [exception("incp")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     integer index = imm1 + e * imm2;
///     Elem[result, e, esize] = index<esize-1:0>;
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn index() -> &'static [IrStatement] {
    [exception("index")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(idxdsize) operand = V[n];
/// bits(128) result;
/// 
/// result = V[d];
/// Elem[result, dst_index, esize] = Elem[operand, src_index, esize];
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ins() -> &'static [IrStatement] {
    [exception("ins")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(VL) dest = Z[dn];
/// bits(esize) src = X[m];
/// Z[dn] = dest<VL-esize-1:0> : src;
/// ```
#[box_to_static_reference]
pub(super) fn insr() -> &'static [IrStatement] {
    [exception("insr")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) operand = if n == 31 then SP[] else X[n];
/// bits(64) exclude_reg = X[m];
/// bits(16) exclude = exclude_reg<15:0> OR GCR_EL1.Exclude;
/// 
/// if AArch64.AllocationTagAccessIsEnabled() then
///     if GCR_EL1.RRND == '1' then
///         RGSR_EL1 = bits(32) UNKNOWN;
///         rtag = _ChooseRandomNonExcludedTag(exclude);
///     else
///         bits(4) start = RGSR_EL1.TAG;
///         bits(4) offset = AArch64.RandomTag();
/// 
///         rtag = AArch64.ChooseNonExcludedTag(start, offset, exclude);
/// 
///         RGSR_EL1.TAG = rtag;
/// else
///     rtag = '0000';
/// 
/// bits(64) result = AArch64.AddressWithAllocationTag(operand, rtag);
/// 
/// if d == 31 then
///     SP[] = result;
/// else
///     X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn irg() -> &'static [IrStatement] {
    [exception("irg")].into()
}

/// # Pseudocode
/// ```text
/// InstructionSynchronizationBarrier();
/// ```
#[box_to_static_reference]
pub(super) fn isb() -> &'static [IrStatement] {
    [].into()
}
