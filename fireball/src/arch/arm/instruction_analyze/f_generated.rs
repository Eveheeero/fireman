use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// bits(esize) diff;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     diff = FPSub(element1, element2, FPCR);
///     Elem[result, e, esize] = if abs then FPAbs(diff) else diff;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fabd() -> &'static [IrStatement] {
    [exception("fabd")].into()
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
///     if neg then
///         element = FPNeg(element);
///     else
///         element = FPAbs(element);
///     Elem[result, e, esize] = element;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fabs() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[n];
/// bits(VL) operand2 = Z[m];
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         case op of
///             when Cmp_GE res = FPCompareGE(FPAbs(element1), FPAbs(element2), FPCR);
///             when Cmp_GT res = FPCompareGT(FPAbs(element1), FPAbs(element2), FPCR);
///         ElemP[result, e, esize] = if res then '1' else '0';
///     else
///         ElemP[result, e, esize] = '0';
/// 
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn faccc() -> &'static [IrStatement] {
    [exception("faccc")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     if abs then
///         element1 = FPAbs(element1);
///         element2 = FPAbs(element2);
///     case cmp of
///         when CompareOp_EQ test_passed = FPCompareEQ(element1, element2, FPCR);
///         when CompareOp_GE test_passed = FPCompareGE(element1, element2, FPCR);
///         when CompareOp_GT test_passed = FPCompareGT(element1, element2, FPCR);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn facge() -> &'static [IrStatement] {
    [exception("facge")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     if abs then
///         element1 = FPAbs(element1);
///         element2 = FPAbs(element2);
///     case cmp of
///         when CompareOp_EQ test_passed = FPCompareEQ(element1, element2, FPCR);
///         when CompareOp_GE test_passed = FPCompareGE(element1, element2, FPCR);
///         when CompareOp_GT test_passed = FPCompareGT(element1, element2, FPCR);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn facgt() -> &'static [IrStatement] {
    [exception("facgt")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(2*datasize) concat = operand2:operand1;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     if pair then
///         element1 = Elem[concat, 2*e, esize];
///         element2 = Elem[concat, (2*e)+1, esize];
///     else
///         element1 = Elem[operand1, e, esize];
///         element2 = Elem[operand2, e, esize];
///     Elem[result, e, esize] = FPAdd(element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fadd() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(esize) operand1 = V[dn];
/// bits(VL) operand2 = Z[m];
/// bits(esize) result = operand1;
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         bits(esize) element = Elem[operand2, e, esize];
///         result = FPAdd(result, element, FPCR);
/// 
/// V[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fadda() -> &'static [IrStatement] {
    [exception("fadda")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn faddp() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(PL) mask = P[g];
/// bits(VL) operand = Z[n];
/// bits(esize) identity = FPZero('0');
/// 
/// V[d] = ReducePredicated(ReduceOp_FADD, operand, mask, identity);
/// ```
#[box_to_static_reference]
pub(super) fn faddv() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element3;
/// 
/// for e = 0 to (elements DIV 2) -1
///     case rot of
///         when '0'
///             element1 = FPNeg(Elem[operand2, e*2+1, esize]);
///             element3 = Elem[operand2, e*2, esize];
///         when '1'
///             element1 = Elem[operand2, e*2+1, esize];
///             element3 = FPNeg(Elem[operand2, e*2, esize]);
///     Elem[result, e*2,   esize] = FPAdd(Elem[operand1, e*2, esize], element1, FPCR);
///     Elem[result, e*2+1, esize] = FPAdd(Elem[operand1, e*2+1, esize], element3, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcadd() -> &'static [IrStatement] {
    [exception("fcadd")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2;
/// 
/// operand2 = V[m];
/// 
/// if ConditionHolds(condition) then
///     flags = FPCompare(operand1, operand2, signal_all_nans, FPCR);
/// PSTATE.<N,Z,C,V> = flags;
/// ```
#[box_to_static_reference]
pub(super) fn fccmp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&pstate_n, &pstate_z, &pstate_c, &pstate_v]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2;
/// 
/// operand2 = V[m];
/// 
/// if ConditionHolds(condition) then
///     flags = FPCompare(operand1, operand2, signal_all_nans, FPCR);
/// PSTATE.<N,Z,C,V> = flags;
/// ```
#[box_to_static_reference]
pub(super) fn fccmpe() -> &'static [IrStatement] {
    [exception("fccmpe")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand = Z[n];
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element = Elem[operand, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         case op of
///             when Cmp_EQ res = FPCompareEQ(element, 0<esize-1:0>, FPCR);
///             when Cmp_GE res = FPCompareGE(element, 0<esize-1:0>, FPCR);
///             when Cmp_GT res = FPCompareGT(element, 0<esize-1:0>, FPCR);
///             when Cmp_NE res = FPCompareNE(element, 0<esize-1:0>, FPCR);
///             when Cmp_LT res = FPCompareGT(0<esize-1:0>, element, FPCR);
///             when Cmp_LE res = FPCompareGE(0<esize-1:0>, element, FPCR);
///         ElemP[result, e, esize] = if res then '1' else '0';
///     else
///         ElemP[result, e, esize] = '0';
/// 
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcmcc() -> &'static [IrStatement] {
    [exception("fcmcc")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     if abs then
///         element1 = FPAbs(element1);
///         element2 = FPAbs(element2);
///     case cmp of
///         when CompareOp_EQ test_passed = FPCompareEQ(element1, element2, FPCR);
///         when CompareOp_GE test_passed = FPCompareGE(element1, element2, FPCR);
///         when CompareOp_GT test_passed = FPCompareGT(element1, element2, FPCR);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcmeq() -> &'static [IrStatement] {
    [exception("fcmeq")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     if abs then
///         element1 = FPAbs(element1);
///         element2 = FPAbs(element2);
///     case cmp of
///         when CompareOp_EQ test_passed = FPCompareEQ(element1, element2, FPCR);
///         when CompareOp_GE test_passed = FPCompareGE(element1, element2, FPCR);
///         when CompareOp_GT test_passed = FPCompareGT(element1, element2, FPCR);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcmge() -> &'static [IrStatement] {
    [exception("fcmge")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     if abs then
///         element1 = FPAbs(element1);
///         element2 = FPAbs(element2);
///     case cmp of
///         when CompareOp_EQ test_passed = FPCompareEQ(element1, element2, FPCR);
///         when CompareOp_GE test_passed = FPCompareGE(element1, element2, FPCR);
///         when CompareOp_GT test_passed = FPCompareGT(element1, element2, FPCR);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcmgt() -> &'static [IrStatement] {
    [exception("fcmgt")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// 
/// for e = 0 to (elements DIV 2) -1
///     case rot of
///         when '00'
///             element1 = Elem[operand2, index*2, esize];
///             element2 = Elem[operand1, e*2, esize];
///             element3 = Elem[operand2, index*2+1, esize];
///             element4 = Elem[operand1, e*2, esize];
///         when '01'
///             element1 = FPNeg(Elem[operand2, index*2+1, esize]);
///             element2 = Elem[operand1, e*2+1, esize];
///             element3 = Elem[operand2, index*2, esize];
///             element4 = Elem[operand1, e*2+1, esize];
///         when '10'
///             element1 = FPNeg(Elem[operand2, index*2,esize]);
///             element2 = Elem[operand1, e*2, esize];
///             element3 = FPNeg(Elem[operand2, index*2+1, esize]);
///             element4 = Elem[operand1, e*2, esize];
///         when '11'
///             element1 = Elem[operand2, index*2+1, esize];
///             element2 = Elem[operand1, e*2+1, esize];
///             element3 = FPNeg(Elem[operand2, index*2, esize]);
///             element4 = Elem[operand1, e*2+1, esize];
/// 
///     Elem[result, e*2,   esize] = FPMulAdd(Elem[operand3, e*2, esize], element2, element1, FPCR);
///     Elem[result, e*2+1, esize] = FPMulAdd(Elem[operand3, e*2+1, esize], element4, element3, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcmla() -> &'static [IrStatement] {
    [exception("fcmla")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// bits(esize) zero = FPZero('0');
/// bits(esize) element;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, esize];
///     case comparison of
///         when CompareOp_GT test_passed = FPCompareGT(element, zero, FPCR);
///         when CompareOp_GE test_passed = FPCompareGE(element, zero, FPCR);
///         when CompareOp_EQ test_passed = FPCompareEQ(element, zero, FPCR);
///         when CompareOp_LE test_passed = FPCompareGE(zero, element, FPCR);
///         when CompareOp_LT test_passed = FPCompareGT(zero, element, FPCR);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcmle() -> &'static [IrStatement] {
    [exception("fcmle")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// bits(esize) zero = FPZero('0');
/// bits(esize) element;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, esize];
///     case comparison of
///         when CompareOp_GT test_passed = FPCompareGT(element, zero, FPCR);
///         when CompareOp_GE test_passed = FPCompareGE(element, zero, FPCR);
///         when CompareOp_EQ test_passed = FPCompareEQ(element, zero, FPCR);
///         when CompareOp_LE test_passed = FPCompareGE(zero, element, FPCR);
///         when CompareOp_LT test_passed = FPCompareGT(zero, element, FPCR);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcmlt() -> &'static [IrStatement] {
    [exception("fcmlt")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2;
/// 
/// operand2 = if cmp_with_zero then FPZero('0') else V[m];
/// 
/// PSTATE.<N,Z,C,V> = FPCompare(operand1, operand2, signal_all_nans, FPCR);
/// ```
#[box_to_static_reference]
pub(super) fn fcmp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&pstate_n, &pstate_z, &pstate_c, &pstate_v]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2;
/// 
/// operand2 = if cmp_with_zero then FPZero('0') else V[m];
/// 
/// PSTATE.<N,Z,C,V> = FPCompare(operand1, operand2, signal_all_nans, FPCR);
/// ```
#[box_to_static_reference]
pub(super) fn fcmpe() -> &'static [IrStatement] {
    [exception("fcmpe")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) result = Z[d];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = imm;
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcpy() -> &'static [IrStatement] {
    [exception("fcpy")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) result;
/// 
/// result = if ConditionHolds(condition) then V[n] else V[m];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcsel() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(dstsize) result;
/// bits(srcsize) operand = V[n];
/// 
/// result = FPConvert(operand, FPCR);
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvt() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     Elem[result, e, esize] = FPToFixed(element, 0, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtas() -> &'static [IrStatement] {
    [exception("fcvtas")].into()
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
///     Elem[result, e, esize] = FPToFixed(element, 0, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtau() -> &'static [IrStatement] {
    [exception("fcvtau")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = Vpart[n, part];
/// bits(2*datasize) result;
/// 
/// for e = 0 to elements-1
///     Elem[result, e, 2*esize] = FPConvert(Elem[operand, e, esize], FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtl() -> &'static [IrStatement] {
    [exception("fcvtl")].into()
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
///     Elem[result, e, esize] = FPToFixed(element, 0, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtms() -> &'static [IrStatement] {
    [exception("fcvtms")].into()
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
///     Elem[result, e, esize] = FPToFixed(element, 0, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtmu() -> &'static [IrStatement] {
    [exception("fcvtmu")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(2*datasize) operand = V[n];
/// bits(datasize) result;
/// 
/// for e = 0 to elements-1
///     Elem[result, e, esize] = FPConvert(Elem[operand, e, 2*esize], FPCR);
/// 
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtn() -> &'static [IrStatement] {
    [exception("fcvtn")].into()
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
///     Elem[result, e, esize] = FPToFixed(element, 0, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtns() -> &'static [IrStatement] {
    [exception("fcvtns")].into()
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
///     Elem[result, e, esize] = FPToFixed(element, 0, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtnu() -> &'static [IrStatement] {
    [exception("fcvtnu")].into()
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
///     Elem[result, e, esize] = FPToFixed(element, 0, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtps() -> &'static [IrStatement] {
    [exception("fcvtps")].into()
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
///     Elem[result, e, esize] = FPToFixed(element, 0, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtpu() -> &'static [IrStatement] {
    [exception("fcvtpu")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(2*datasize) operand = V[n];
/// bits(datasize) result;
/// 
/// for e = 0 to elements-1
///     Elem[result, e, esize] = FPConvert(Elem[operand, e, 2*esize], FPCR, FPRounding_ODD);
/// 
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtxn() -> &'static [IrStatement] {
    [exception("fcvtxn")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand  = V[n];
/// bits(datasize) result;
/// bits(esize) element;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, esize];
///     Elem[result, e, esize] = FPToFixed(element, fracbits, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtzs() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand  = V[n];
/// bits(datasize) result;
/// bits(esize) element;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, esize];
///     Elem[result, e, esize] = FPToFixed(element, fracbits, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fcvtzu() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     Elem[result, e, esize] = FPDiv(element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fdiv() -> &'static [IrStatement] {
    let assignment = assign(b::unsigned_div(o2(), o3()), o1(), o1_size());
    [assignment].into()
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
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = FPDiv(element2, element1, FPCR);
///     else
///         Elem[result, e, esize] = element1;
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fdivr() -> &'static [IrStatement] {
    [exception("fdivr")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     Elem[result, e, esize] = imm;
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fdup() -> &'static [IrStatement] {
    [exception("fdup")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) operand  = Z[n];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element = Elem[operand, e, esize];
///     Elem[result, e, esize] = FPExpA(element);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fexpa() -> &'static [IrStatement] {
    [exception("fexpa")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(fltsize) fltval;
/// bits(intsize) intval;
/// 
/// case op of
///     when FPConvOp_CVT_FtoI
///         fltval = V[n];
///         intval = FPToFixed(fltval, 0, unsigned, FPCR, rounding);
///         X[d] = intval;
///     when FPConvOp_CVT_ItoF
///         intval = X[n];
///         fltval = FixedToFP(intval, 0, unsigned, FPCR, rounding);
///         V[d] = fltval;
///     when FPConvOp_MOV_FtoI
///         fltval = Vpart[n,part];
///         intval = ZeroExtend(fltval, intsize);
///         X[d] = intval;
///     when FPConvOp_MOV_ItoF
///         intval = X[n];
///         fltval = intval<fltsize-1:0>;
///         Vpart[d,part] = fltval;
///     when FPConvOp_CVT_FtoI_JS
///         bit Z;
///         fltval = V[n];
///         (intval, Z) = FPToFixedJS(fltval, FPCR, TRUE);
///         PSTATE.<N,Z,C,V> = '0':Z:'00';
///         X[d] = intval;
/// ```
#[box_to_static_reference]
pub(super) fn fjcvtzs() -> &'static [IrStatement] {
    [exception("fjcvtzs")].into()
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
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     bits(esize) element3 = Elem[operand3, e, esize];
/// 
///     if ElemP[mask, e, esize] == '1' then
///         if op1_neg then element1 = FPNeg(element1);
///         if op3_neg then element3 = FPNeg(element3);
///         Elem[result, e, esize] = FPMulAdd(element3, element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = element1;
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmad() -> &'static [IrStatement] {
    [exception("fmad")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) result;
/// bits(datasize) operanda = V[a];
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// 
/// if opa_neg then operanda = FPNeg(operanda);
/// if op1_neg then operand1 = FPNeg(operand1);
/// result = FPMulAdd(operanda, operand1, operand2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmadd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o4()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(2*datasize) concat = operand2:operand1;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     if pair then
///         element1 = Elem[concat, 2*e, esize];
///         element2 = Elem[concat, (2*e)+1, esize];
///     else
///         element1 = Elem[operand1, e, esize];
///         element2 = Elem[operand2, e, esize];
/// 
///     if minimum then
///         Elem[result, e, esize] = FPMin(element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = FPMax(element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmax() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(2*datasize) concat = operand2:operand1;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     if pair then
///         element1 = Elem[concat, 2*e, esize];
///         element2 = Elem[concat, (2*e)+1, esize];
///     else
///         element1 = Elem[operand1, e, esize];
///         element2 = Elem[operand2, e, esize];
/// 
///     if minimum then
///         Elem[result, e, esize] = FPMinNum(element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = FPMaxNum(element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmaxnm() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn fmaxnmp() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn fmaxnmv() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn fmaxp() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn fmaxv() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(2*datasize) concat = operand2:operand1;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     if pair then
///         element1 = Elem[concat, 2*e, esize];
///         element2 = Elem[concat, (2*e)+1, esize];
///     else
///         element1 = Elem[operand1, e, esize];
///         element2 = Elem[operand2, e, esize];
/// 
///     if minimum then
///         Elem[result, e, esize] = FPMin(element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = FPMax(element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmin() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(2*datasize) concat = operand2:operand1;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     if pair then
///         element1 = Elem[concat, 2*e, esize];
///         element2 = Elem[concat, (2*e)+1, esize];
///     else
///         element1 = Elem[operand1, e, esize];
///         element2 = Elem[operand2, e, esize];
/// 
///     if minimum then
///         Elem[result, e, esize] = FPMinNum(element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = FPMaxNum(element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fminnm() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn fminnmp() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn fminnmv() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn fminp() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// V[d] = Reduce(op, operand, esize);
/// ```
#[box_to_static_reference]
pub(super) fn fminv() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(idxdsize) operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2 = Elem[operand2, index, esize];
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     if sub_op then element1 = FPNeg(element1);
///     Elem[result, e, esize] = FPMulAdd(Elem[operand3, e, esize], element1, element2, FPCR);
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmla() -> &'static [IrStatement] {
    [exception("fmla")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize DIV 2) operand1 = Vpart[n,part];
/// bits(128) operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// bits(esize DIV 2) element1;
/// bits(esize DIV 2) element2 = Elem[operand2, index, esize DIV 2];
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize DIV 2];
///     if sub_op then element1 = FPNeg(element1);
///     Elem[result, e, esize] = FPMulAddH(Elem[operand3, e, esize], element1, element2, FPCR);
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmlal() -> &'static [IrStatement] {
    [exception("fmlal")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(idxdsize) operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2 = Elem[operand2, index, esize];
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     if sub_op then element1 = FPNeg(element1);
///     Elem[result, e, esize] = FPMulAdd(Elem[operand3, e, esize], element1, element2, FPCR);
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmls() -> &'static [IrStatement] {
    [exception("fmls")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize DIV 2) operand1 = Vpart[n,part];
/// bits(128) operand2 = V[m];
/// bits(datasize) operand3 = V[d];
/// bits(datasize) result;
/// bits(esize DIV 2) element1;
/// bits(esize DIV 2) element2 = Elem[operand2, index, esize DIV 2];
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize DIV 2];
///     if sub_op then element1 = FPNeg(element1);
///     Elem[result, e, esize] = FPMulAddH(Elem[operand3, e, esize], element1, element2, FPCR);
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmlsl() -> &'static [IrStatement] {
    [exception("fmlsl")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// if VL < esize * 4 then UNDEFINED;
/// integer segments = VL DIV (4 * esize);
/// bits(VL) operand1 = Z[n];
/// bits(VL) operand2 = Z[m];
/// bits(VL) operand3 = Z[da];
/// bits(VL) result = Zeros();
/// bits(4*esize) op1, op2;
/// bits(4*esize) res, addend;
/// 
/// for s = 0 to segments-1
///     op1    = Elem[operand1, s, 4*esize];
///     op2    = Elem[operand2, s, 4*esize];
///     addend = Elem[operand3, s, 4*esize];
///     res    = FPMatMulAdd(addend, op1, op2, esize, FPCR);
///     Elem[result, s, 4*esize] = res;
/// 
/// Z[da] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmmla() -> &'static [IrStatement] {
    [exception("fmmla")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// 
/// V[rd] = imm;
/// ```
#[box_to_static_reference]
pub(super) fn fmov() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     bits(esize) element3 = Elem[operand3, e, esize];
/// 
///     if ElemP[mask, e, esize] == '1' then
///         if op1_neg then element1 = FPNeg(element1);
///         if op3_neg then element3 = FPNeg(element3);
///         Elem[result, e, esize] = FPMulAdd(element3, element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = element1;
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmsb() -> &'static [IrStatement] {
    [exception("fmsb")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) result;
/// bits(datasize) operanda = V[a];
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// 
/// if opa_neg then operanda = FPNeg(operanda);
/// if op1_neg then operand1 = FPNeg(operand1);
/// result = FPMulAdd(operanda, operand1, operand2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmsub() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o4()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(idxdsize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2 = Elem[operand2, index, esize];
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     if mulx_op then
///         Elem[result, e, esize] = FPMulX(element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = FPMul(element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmul() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(idxdsize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2 = Elem[operand2, index, esize];
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     if mulx_op then
///         Elem[result, e, esize] = FPMulX(element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = FPMul(element1, element2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fmulx() -> &'static [IrStatement] {
    [exception("fmulx")].into()
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
///     if neg then
///         element = FPNeg(element);
///     else
///         element = FPAbs(element);
///     Elem[result, e, esize] = element;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fneg() -> &'static [IrStatement] {
    let assignment = assign(u::neg(o2()), o1(), o1_size());
    [assignment].into()
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
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     bits(esize) element3 = Elem[operand3, e, esize];
/// 
///     if ElemP[mask, e, esize] == '1' then
///         if op1_neg then element1 = FPNeg(element1);
///         if op3_neg then element3 = FPNeg(element3);
///         Elem[result, e, esize] = FPMulAdd(element3, element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = element1;
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fnmad() -> &'static [IrStatement] {
    [exception("fnmad")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) result;
/// bits(datasize) operanda = V[a];
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// 
/// if opa_neg then operanda = FPNeg(operanda);
/// if op1_neg then operand1 = FPNeg(operand1);
/// result = FPMulAdd(operanda, operand1, operand2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fnmadd() -> &'static [IrStatement] {
    let assignment = assign(u::neg(b::add(b::mul(o2(), o3()), o4())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[n];
/// bits(VL) operand2 = Z[m];
/// bits(VL) operand3 = Z[da];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     bits(esize) element3 = Elem[operand3, e, esize];
/// 
///     if ElemP[mask, e, esize] == '1' then
///         if op1_neg then element1 = FPNeg(element1);
///         if op3_neg then element3 = FPNeg(element3);
///         Elem[result, e, esize] = FPMulAdd(element3, element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = element3;
/// 
/// Z[da] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fnmla() -> &'static [IrStatement] {
    [exception("fnmla")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[n];
/// bits(VL) operand2 = Z[m];
/// bits(VL) operand3 = Z[da];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     bits(esize) element3 = Elem[operand3, e, esize];
/// 
///     if ElemP[mask, e, esize] == '1' then
///         if op1_neg then element1 = FPNeg(element1);
///         if op3_neg then element3 = FPNeg(element3);
///         Elem[result, e, esize] = FPMulAdd(element3, element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = element3;
/// 
/// Z[da] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fnmls() -> &'static [IrStatement] {
    [exception("fnmls")].into()
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
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     bits(esize) element3 = Elem[operand3, e, esize];
/// 
///     if ElemP[mask, e, esize] == '1' then
///         if op1_neg then element1 = FPNeg(element1);
///         if op3_neg then element3 = FPNeg(element3);
///         Elem[result, e, esize] = FPMulAdd(element3, element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = element1;
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fnmsb() -> &'static [IrStatement] {
    [exception("fnmsb")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) result;
/// bits(datasize) operanda = V[a];
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// 
/// if opa_neg then operanda = FPNeg(operanda);
/// if op1_neg then operand1 = FPNeg(operand1);
/// result = FPMulAdd(operanda, operand1, operand2, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fnmsub() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o4(), b::mul(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) result;
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// 
/// result = FPMul(operand1, operand2, FPCR);
/// 
/// if negated then result = FPNeg(result);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fnmul() -> &'static [IrStatement] {
    let v_0 = unknown_data();
    let stmt_1 = assign(v_0, o1(), o1_size());
    [stmt_1].into()
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
///     Elem[result, e, esize] = FPRecipEstimate(element, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frecpe() -> &'static [IrStatement] {
    [exception("frecpe")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     Elem[result, e, esize] = FPRecipStepFused(element1, element2);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frecps() -> &'static [IrStatement] {
    [exception("frecps")].into()
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
///     Elem[result, e, esize] = FPRecpX(element, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frecpx() -> &'static [IrStatement] {
    [exception("frecpx")].into()
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
///     Elem[result, e, esize] = FPRoundIntN(element, FPCR, rounding, intsize);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frint32x() -> &'static [IrStatement] {
    [exception("frint32x")].into()
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
///     Elem[result, e, esize] = FPRoundIntN(element, FPCR, rounding, intsize);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frint32z() -> &'static [IrStatement] {
    [exception("frint32z")].into()
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
///     Elem[result, e, esize] = FPRoundIntN(element, FPCR, rounding, intsize);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frint64x() -> &'static [IrStatement] {
    [exception("frint64x")].into()
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
///     Elem[result, e, esize] = FPRoundIntN(element, FPCR, rounding, intsize);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frint64z() -> &'static [IrStatement] {
    [exception("frint64z")].into()
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
///     bits(esize) element = Elem[operand, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frintr() -> &'static [IrStatement] {
    [exception("frintr")].into()
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
///     Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frinta() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frinti() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frintm() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frintn() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frintp() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frintx() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     Elem[result, e, esize] = FPRoundInt(element, FPCR, rounding, exact);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frintz() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///     Elem[result, e, esize] = FPRSqrtEstimate(element, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frsqrte() -> &'static [IrStatement] {
    [exception("frsqrte")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     Elem[result, e, esize] = FPRSqrtStepFused(element1, element2);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn frsqrts() -> &'static [IrStatement] {
    [exception("frsqrts")].into()
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
///     integer element2 = SInt(Elem[operand2, e, esize]);
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = FPScale(element1, element2, FPCR);
///     else
///         Elem[result, e, esize] = element1;
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fscale() -> &'static [IrStatement] {
    [exception("fscale")].into()
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
///     Elem[result, e, esize] = FPSqrt(element, FPCR);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fsqrt() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(esize) element1;
/// bits(esize) element2;
/// bits(esize) diff;
/// 
/// for e = 0 to elements-1
///     element1 = Elem[operand1, e, esize];
///     element2 = Elem[operand2, e, esize];
///     diff = FPSub(element1, element2, FPCR);
///     Elem[result, e, esize] = if abs then FPAbs(diff) else diff;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fsub() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
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
///     bits(esize) element1 = Elem[operand1, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = FPSub(imm, element1, FPCR);
///     else
///         Elem[result, e, esize] = element1;
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn fsubr() -> &'static [IrStatement] {
    [exception("fsubr")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) operand1 = Z[dn];
/// bits(VL) operand2 = Z[m];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     Elem[result, e, esize] = FPTrigMAdd(imm, element1, element2, FPCR);
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ftmad() -> &'static [IrStatement] {
    [exception("ftmad")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) operand1 = Z[n];
/// bits(VL) operand2 = Z[m];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     Elem[result, e, esize] = FPTrigSMul(element1, element2, FPCR);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ftsmul() -> &'static [IrStatement] {
    [exception("ftsmul")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) operand1 = Z[n];
/// bits(VL) operand2 = Z[m];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(esize) element1 = Elem[operand1, e, esize];
///     bits(esize) element2 = Elem[operand2, e, esize];
///     Elem[result, e, esize] = FPTrigSSel(element1, element2);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ftssel() -> &'static [IrStatement] {
    [exception("ftssel")].into()
}
