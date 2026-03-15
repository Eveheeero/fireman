use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

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
pub(super) fn raddhn() -> &'static [IrStatement] {
    [exception("raddhn")].into()
}

/// # Pseudocode
/// ```text
/// AArch64.CheckFPAdvSIMDEnabled();
/// 
/// bits(128) Vm = V[m];
/// bits(128) Vn = V[n];
/// V[d] = Vn EOR (ROL(Vm<127:64>,1):ROL(Vm<63:0>, 1));
/// ```
#[box_to_static_reference]
pub(super) fn rax1() -> &'static [IrStatement] {
    [exception("rax1")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// bits(esize) element;
/// bits(esize) rev;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, esize];
///     for i = 0 to esize-1
///         rev<esize-1-i> = element<i>;
///     Elem[result, e, esize] = rev;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn rbit() -> &'static [IrStatement] {
    [exception("rbit")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(PL) ffr = FFR[];
/// P[d] = ffr;
/// ```
#[box_to_static_reference]
pub(super) fn rdffr() -> &'static [IrStatement] {
    [exception("rdffr")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer len = imm * (VL DIV 8);
/// X[d] = len<63:0>;
/// ```
#[box_to_static_reference]
pub(super) fn rdvl() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
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
pub(super) fn ret() -> &'static [IrStatement] {
    [jump(x30.clone())].into()
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
pub(super) fn retaa() -> &'static [IrStatement] {
    [exception("retaa")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand = X[n];
/// bits(datasize) result;
/// 
/// integer containers = datasize DIV container_size;
/// integer elements_per_container = container_size DIV 8;
/// integer index = 0;
/// integer rev_index;
/// for c = 0 to containers-1
///     rev_index = index + ((elements_per_container - 1) * 8);
///     for e = 0 to elements_per_container-1
///         result<rev_index + 7:rev_index> = operand<index + 7:index>;
///         index = index + 8;
///         rev_index = rev_index - 8;
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn rev() -> &'static [IrStatement] {
    [exception("rev")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// integer element = 0;
/// integer rev_element;
/// for c = 0 to containers-1
///     rev_element = element + elements_per_container - 1;
///     for e = 0 to elements_per_container-1
///         Elem[result, rev_element, esize] = Elem[operand, element, esize];
///         element = element + 1;
///         rev_element = rev_element - 1;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn rev16() -> &'static [IrStatement] {
    [exception("rev16")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// integer element = 0;
/// integer rev_element;
/// for c = 0 to containers-1
///     rev_element = element + elements_per_container - 1;
///     for e = 0 to elements_per_container-1
///         Elem[result, rev_element, esize] = Elem[operand, element, esize];
///         element = element + 1;
///         rev_element = rev_element - 1;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn rev32() -> &'static [IrStatement] {
    [exception("rev32")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// integer element = 0;
/// integer rev_element;
/// for c = 0 to containers-1
///     rev_element = element + elements_per_container - 1;
///     for e = 0 to elements_per_container-1
///         Elem[result, rev_element, esize] = Elem[operand, element, esize];
///         element = element + 1;
///         rev_element = rev_element - 1;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn rev64() -> &'static [IrStatement] {
    [exception("rev64")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand = Z[n];
/// bits(VL) result = Z[d];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         bits(esize) element = Elem[operand, e, esize];
///         Elem[result, e, esize] = Reverse(element, swsize);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn revb() -> &'static [IrStatement] {
    [exception("revb")].into()
}

/// # Pseudocode
/// ```text
/// bits(4) tmp;
/// bits(64) tmpreg = X[n];
/// tmp = (tmpreg:tmpreg)<lsb+3:lsb>;
/// if mask<3> == '1' then PSTATE.N = tmp<3>;
/// if mask<2> == '1' then PSTATE.Z = tmp<2>;
/// if mask<1> == '1' then PSTATE.C = tmp<1>;
/// if mask<0> == '1' then PSTATE.V = tmp<0>;
/// ```
#[box_to_static_reference]
pub(super) fn rmif() -> &'static [IrStatement] {
    [exception("rmif")].into()
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
pub(super) fn rorv() -> &'static [IrStatement] {
    let op = b::or(b::shr(o2(), o3()), b::shl(o2(), b::sub(bit_size_of_o2(), o3())));
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize*2) operand = V[n];
/// bits(datasize) result;
/// integer round_const = if round then (1 << (shift - 1)) else 0;
/// integer element;
/// 
/// for e = 0 to elements-1
///     element = (UInt(Elem[operand, e, 2*esize]) + round_const) >> shift;
///     Elem[result, e, esize] = element<esize-1:0>;
/// 
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn rshrn() -> &'static [IrStatement] {
    [exception("rshrn")].into()
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
pub(super) fn rsubhn() -> &'static [IrStatement] {
    [exception("rsubhn")].into()
}
