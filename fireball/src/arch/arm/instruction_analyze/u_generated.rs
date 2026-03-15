use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// bits(esize) absdiff;
/// 
/// result = if accumulate then V[d] else Zeros();
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     absdiff = Abs(element1 - element2)<esize-1:0>;
///     Elem[result, e, esize] = Elem[result, e, esize] + absdiff;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uaba() -> &'static [IrStatement] {
    [exception("uaba")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize)   operand1 = Vpart[n, part];
/// bits(datasize)   operand2 = Vpart[m, part];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// bits(2*esize) absdiff;
/// 
/// result = if accumulate then V[d] else Zeros();
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     absdiff = Abs(element1 - element2)<2*esize-1:0>;
///     Elem[result, e, 2*esize] = Elem[result, e, 2*esize] + absdiff;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uabal() -> &'static [IrStatement] {
    [exception("uabal")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// bits(esize) absdiff;
/// 
/// result = if accumulate then V[d] else Zeros();
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     absdiff = Abs(element1 - element2)<esize-1:0>;
///     Elem[result, e, esize] = Elem[result, e, esize] + absdiff;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uabd() -> &'static [IrStatement] {
    [exception("uabd")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize)   operand1 = Vpart[n, part];
/// bits(datasize)   operand2 = Vpart[m, part];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// bits(2*esize) absdiff;
/// 
/// result = if accumulate then V[d] else Zeros();
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     absdiff = Abs(element1 - element2)<2*esize-1:0>;
///     Elem[result, e, 2*esize] = Elem[result, e, 2*esize] + absdiff;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uabdl() -> &'static [IrStatement] {
    [exception("uabdl")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// 
/// bits(2*esize) sum;
/// integer op1;
/// integer op2;
/// 
/// if acc then result = V[d];
/// for e = 0 to elements-1
///     op1 = Int(Elem[operand, 2*e+0, esize], unsigned);
///     op2 = Int(Elem[operand, 2*e+1, esize], unsigned);
///     sum = (op1 + op2)<2*esize-1:0>;
///     if acc then
///         Elem[result, e, 2*esize] = Elem[result, e, 2*esize] + sum;
///     else
///         Elem[result, e, 2*esize] = sum;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uadalp() -> &'static [IrStatement] {
    [exception("uadalp")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize)   operand1 = Vpart[n, part];
/// bits(datasize)   operand2 = Vpart[m, part];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// integer sum;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     if sub_op then
///         sum = element1 - element2;
///     else
///         sum = element1 + element2;
///     Elem[result, e, 2*esize] = sum<2*esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uaddl() -> &'static [IrStatement] {
    [exception("uaddl")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// 
/// bits(2*esize) sum;
/// integer op1;
/// integer op2;
/// 
/// if acc then result = V[d];
/// for e = 0 to elements-1
///     op1 = Int(Elem[operand, 2*e+0, esize], unsigned);
///     op2 = Int(Elem[operand, 2*e+1, esize], unsigned);
///     sum = (op1 + op2)<2*esize-1:0>;
///     if acc then
///         Elem[result, e, 2*esize] = Elem[result, e, 2*esize] + sum;
///     else
///         Elem[result, e, 2*esize] = sum;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uaddlp() -> &'static [IrStatement] {
    [exception("uaddlp")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// integer sum;
/// 
/// sum = Int(Elem[operand, 0, esize], unsigned);
/// for e = 1 to elements-1
///     sum = sum + Int(Elem[operand, e, esize], unsigned);
/// 
/// V[d] = sum<2*esize-1:0>;
/// ```
#[box_to_static_reference]
pub(super) fn uaddlv() -> &'static [IrStatement] {
    [exception("uaddlv")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand = Z[n];
/// integer sum = 0;
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         integer element = UInt(Elem[operand, e, esize]);
///         sum = sum + element;
/// 
/// V[d] = sum<63:0>;
/// ```
#[box_to_static_reference]
pub(super) fn uaddv() -> &'static [IrStatement] {
    [exception("uaddv")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(2*datasize) operand1 = V[n];
/// bits(datasize)   operand2 = Vpart[m, part];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// integer sum;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, 2*esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     if sub_op then
///         sum = element1 - element2;
///     else
///         sum = element1 + element2;
///     Elem[result, e, 2*esize] = sum<2*esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uaddw() -> &'static [IrStatement] {
    [exception("uaddw")].into()
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
pub(super) fn ubfm() -> &'static [IrStatement] {
    [exception("ubfm")].into()
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
///     Elem[result, e, esize] = FixedToFP(element, fracbits, unsigned, FPCR, rounding);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ucvtf() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// // No operation.
/// ```
#[box_to_static_reference]
pub(super) fn udf() -> &'static [IrStatement] {
    [exception("udf")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// integer result;
/// 
/// if IsZero(operand2) then
///     result = 0;
/// else
///     result = RoundTowardsZero(Real(Int(operand1, unsigned)) / Real(Int(operand2, unsigned)));
/// 
/// X[d] = result<datasize-1:0>;
/// ```
#[box_to_static_reference]
pub(super) fn udiv() -> &'static [IrStatement] {
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
///     integer element1 = Int(Elem[operand1, e, esize], unsigned);
///     integer element2 = Int(Elem[operand2, e, esize], unsigned);
///     if ElemP[mask, e, esize] == '1' then
///         integer quotient;
///         if element1 == 0 then
///             quotient = 0;
///         else
///             quotient = RoundTowardsZero(Real(element2) / Real(element1));
///         Elem[result, e, esize] = quotient<esize-1:0>;
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn udivr() -> &'static [IrStatement] {
    [exception("udivr")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(128) operand2 = V[m];
/// bits(datasize) result = V[d];
/// for e = 0 to elements-1
///     integer res = 0;
///     integer element1, element2;
///     for i = 0 to 3
///         if signed then
///             element1 = SInt(Elem[operand1, 4 * e + i, esize DIV 4]);
///             element2 = SInt(Elem[operand2, 4 * index + i, esize DIV 4]);
///         else
///             element1 = UInt(Elem[operand1, 4 * e + i, esize DIV 4]);
///             element2 = UInt(Elem[operand2, 4 * index + i, esize DIV 4]);
///         res = res + element1 * element2;
///     Elem[result, e, esize] = Elem[result, e, esize] + res;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn udot() -> &'static [IrStatement] {
    [exception("udot")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// integer sum;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     sum = element1 + element2;
///     Elem[result, e, esize] = sum<esize:1>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uhadd() -> &'static [IrStatement] {
    [exception("uhadd")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// integer diff;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     diff = element1 - element2;
///     Elem[result, e, esize] = diff<esize:1>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uhsub() -> &'static [IrStatement] {
    [exception("uhsub")].into()
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
///     result = Int(operand3, unsigned) - (Int(operand1, unsigned) * Int(operand2, unsigned));
/// else
///     result = Int(operand3, unsigned) + (Int(operand1, unsigned) * Int(operand2, unsigned));
/// 
/// X[d] = result<63:0>;
/// ```
#[box_to_static_reference]
pub(super) fn umaddl() -> &'static [IrStatement] {
    let assignment = assign(b::add(o4(), b::mul(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// integer maxmin;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     maxmin = if minimum then Min(element1, element2) else Max(element1, element2);
///     Elem[result, e, esize] = maxmin<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn umax() -> &'static [IrStatement] {
    [exception("umax")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(2*datasize) concat = operand2:operand1;
/// integer element1;
/// integer element2;
/// integer maxmin;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[concat, 2*e, esize], unsigned);
///     element2 = Int(Elem[concat, (2*e)+1, esize], unsigned);
///     maxmin = if minimum then Min(element1, element2) else Max(element1, element2);
///     Elem[result, e, esize] = maxmin<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn umaxp() -> &'static [IrStatement] {
    [exception("umaxp")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// integer maxmin;
/// integer element;
/// 
/// maxmin = Int(Elem[operand, 0, esize], unsigned);
/// for e = 1 to elements-1
///     element = Int(Elem[operand, e, esize], unsigned);
///     maxmin = if min then Min(maxmin, element) else Max(maxmin, element);
/// 
/// V[d] = maxmin<esize-1:0>;
/// ```
#[box_to_static_reference]
pub(super) fn umaxv() -> &'static [IrStatement] {
    [exception("umaxv")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// integer maxmin;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     maxmin = if minimum then Min(element1, element2) else Max(element1, element2);
///     Elem[result, e, esize] = maxmin<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn umin() -> &'static [IrStatement] {
    [exception("umin")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// bits(2*datasize) concat = operand2:operand1;
/// integer element1;
/// integer element2;
/// integer maxmin;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[concat, 2*e, esize], unsigned);
///     element2 = Int(Elem[concat, (2*e)+1, esize], unsigned);
///     maxmin = if minimum then Min(element1, element2) else Max(element1, element2);
///     Elem[result, e, esize] = maxmin<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uminp() -> &'static [IrStatement] {
    [exception("uminp")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// integer maxmin;
/// integer element;
/// 
/// maxmin = Int(Elem[operand, 0, esize], unsigned);
/// for e = 1 to elements-1
///     element = Int(Elem[operand, e, esize], unsigned);
///     maxmin = if min then Min(maxmin, element) else Max(maxmin, element);
/// 
/// V[d] = maxmin<esize-1:0>;
/// ```
#[box_to_static_reference]
pub(super) fn uminv() -> &'static [IrStatement] {
    [exception("uminv")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize)   operand1 = Vpart[n, part];
/// bits(idxdsize)   operand2 = V[m];
/// bits(2*datasize) operand3 = V[d];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// bits(2*esize) product;
/// 
/// element2 = Int(Elem[operand2, index, esize], unsigned);
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     product = (element1 * element2)<2*esize-1:0>;
///     if sub_op then
///         Elem[result, e, 2*esize] = Elem[operand3, e, 2*esize] - product;
///     else
///         Elem[result, e, 2*esize] = Elem[operand3, e, 2*esize] + product;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn umlal() -> &'static [IrStatement] {
    [exception("umlal")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize)   operand1 = Vpart[n, part];
/// bits(idxdsize)   operand2 = V[m];
/// bits(2*datasize) operand3 = V[d];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// bits(2*esize) product;
/// 
/// element2 = Int(Elem[operand2, index, esize], unsigned);
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     product = (element1 * element2)<2*esize-1:0>;
///     if sub_op then
///         Elem[result, e, 2*esize] = Elem[operand3, e, 2*esize] - product;
///     else
///         Elem[result, e, 2*esize] = Elem[operand3, e, 2*esize] + product;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn umlsl() -> &'static [IrStatement] {
    [exception("umlsl")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(128) operand1 = V[n];
/// bits(128) operand2 = V[m];
/// bits(128) addend   = V[d];
/// 
/// V[d] = MatMulAdd(addend, operand1, operand2, op1_unsigned, op2_unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn ummla() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(idxdsize) operand = V[n];
/// 
/// X[d] = ZeroExtend(Elem[operand, index, esize], datasize);
/// ```
#[box_to_static_reference]
pub(super) fn umov() -> &'static [IrStatement] {
    [exception("umov")].into()
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
///     result = Int(operand3, unsigned) - (Int(operand1, unsigned) * Int(operand2, unsigned));
/// else
///     result = Int(operand3, unsigned) + (Int(operand1, unsigned) * Int(operand2, unsigned));
/// 
/// X[d] = result<63:0>;
/// ```
#[box_to_static_reference]
pub(super) fn umsubl() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o4(), b::mul(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// 
/// integer result;
/// 
/// result = Int(operand1, unsigned) * Int(operand2, unsigned);
/// 
/// X[d] = result<127:64>;
/// ```
#[box_to_static_reference]
pub(super) fn umulh() -> &'static [IrStatement] {
    [exception("umulh")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize)   operand1 = Vpart[n, part];
/// bits(idxdsize)   operand2 = V[m];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// bits(2*esize) product;
/// 
/// element2 = Int(Elem[operand2, index, esize], unsigned);
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     product = (element1 * element2)<2*esize-1:0>;
///     Elem[result, e, 2*esize] = product;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn umull() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// integer sum;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     sum = element1 + element2;
///     (Elem[result, e, esize], sat) = SatQ(sum, esize, unsigned);
///     if sat then FPSR.QC = '1';
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uqadd() -> &'static [IrStatement] {
    [exception("uqadd")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(ssize) operand1 = X[dn];
/// bits(ssize) result;
/// 
/// integer element1 = Int(operand1, unsigned);
/// (result, -) = SatQ(element1 - (count * imm), ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqdecb() -> &'static [IrStatement] {
    [exception("uqdecb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(ssize) operand1 = X[dn];
/// bits(ssize) result;
/// 
/// integer element1 = Int(operand1, unsigned);
/// (result, -) = SatQ(element1 - (count * imm), ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqdecd() -> &'static [IrStatement] {
    [exception("uqdecd")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(ssize) operand1 = X[dn];
/// bits(ssize) result;
/// 
/// integer element1 = Int(operand1, unsigned);
/// (result, -) = SatQ(element1 - (count * imm), ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqdech() -> &'static [IrStatement] {
    [exception("uqdech")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(ssize) operand1 = X[dn];
/// bits(PL) operand2 = P[m];
/// bits(ssize) result;
/// integer count = 0;
/// 
/// for e = 0 to elements-1
///     if ElemP[operand2, e, esize] == '1' then
///         count = count + 1;
/// 
/// integer element = Int(operand1, unsigned);
/// (result, -) = SatQ(element - count, ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqdecp() -> &'static [IrStatement] {
    [exception("uqdecp")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(ssize) operand1 = X[dn];
/// bits(ssize) result;
/// 
/// integer element1 = Int(operand1, unsigned);
/// (result, -) = SatQ(element1 - (count * imm), ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqdecw() -> &'static [IrStatement] {
    [exception("uqdecw")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(ssize) operand1 = X[dn];
/// bits(ssize) result;
/// 
/// integer element1 = Int(operand1, unsigned);
/// (result, -) = SatQ(element1 + (count * imm), ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqincb() -> &'static [IrStatement] {
    [exception("uqincb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(ssize) operand1 = X[dn];
/// bits(ssize) result;
/// 
/// integer element1 = Int(operand1, unsigned);
/// (result, -) = SatQ(element1 + (count * imm), ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqincd() -> &'static [IrStatement] {
    [exception("uqincd")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(ssize) operand1 = X[dn];
/// bits(ssize) result;
/// 
/// integer element1 = Int(operand1, unsigned);
/// (result, -) = SatQ(element1 + (count * imm), ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqinch() -> &'static [IrStatement] {
    [exception("uqinch")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(ssize) operand1 = X[dn];
/// bits(PL) operand2 = P[m];
/// bits(ssize) result;
/// integer count = 0;
/// 
/// for e = 0 to elements-1
///     if ElemP[operand2, e, esize] == '1' then
///         count = count + 1;
/// 
/// integer element = Int(operand1, unsigned);
/// (result, -) = SatQ(element + count, ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqincp() -> &'static [IrStatement] {
    [exception("uqincp")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// bits(ssize) operand1 = X[dn];
/// bits(ssize) result;
/// 
/// integer element1 = Int(operand1, unsigned);
/// (result, -) = SatQ(element1 + (count * imm), ssize, unsigned);
/// X[dn] = Extend(result, 64, unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn uqincw() -> &'static [IrStatement] {
    [exception("uqincw")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// 
/// integer round_const = 0;
/// integer shift;
/// integer element;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     shift = SInt(Elem[operand2, e, esize]<7:0>);
///     if rounding then
///         round_const = 1 << (-shift - 1); // 0 for left shift, 2^(n-1) for right shift
///     element = (Int(Elem[operand1, e, esize], unsigned) + round_const) << shift;
///     if saturating then
///         (Elem[result, e, esize], sat) = SatQ(element, esize, unsigned);
///         if sat then FPSR.QC = '1';
///     else
///         Elem[result, e, esize] = element<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uqrshl() -> &'static [IrStatement] {
    [exception("uqrshl")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize*2) operand = V[n];
/// bits(datasize) result;
/// integer round_const = if round then (1 << (shift - 1)) else 0;
/// integer element;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     element = (Int(Elem[operand, e, 2*esize], unsigned) + round_const) >> shift;
///     (Elem[result, e, esize], sat) = SatQ(element, esize, unsigned);
///     if sat then FPSR.QC = '1';
/// 
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uqrshrn() -> &'static [IrStatement] {
    [exception("uqrshrn")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand  = V[n];
/// bits(datasize) result;
/// integer element;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     element = Int(Elem[operand, e, esize], src_unsigned) << shift;
///     (Elem[result, e, esize], sat) = SatQ(element, esize, dst_unsigned);
///     if sat then FPSR.QC = '1';
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uqshl() -> &'static [IrStatement] {
    [exception("uqshl")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize*2) operand = V[n];
/// bits(datasize) result;
/// integer round_const = if round then (1 << (shift - 1)) else 0;
/// integer element;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     element = (Int(Elem[operand, e, 2*esize], unsigned) + round_const) >> shift;
///     (Elem[result, e, esize], sat) = SatQ(element, esize, unsigned);
///     if sat then FPSR.QC = '1';
/// 
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uqshrn() -> &'static [IrStatement] {
    [exception("uqshrn")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// integer diff;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     diff = element1 - element2;
///     (Elem[result, e, esize], sat) = SatQ(diff, esize, unsigned);
///     if sat then FPSR.QC = '1';
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uqsub() -> &'static [IrStatement] {
    [exception("uqsub")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(2*datasize) operand = V[n];
/// bits(datasize) result;
/// bits(2*esize) element;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, 2*esize];
///     (Elem[result, e, esize], sat) = SatQ(Int(element, unsigned), esize, unsigned);
///     if sat then FPSR.QC = '1';
/// 
/// Vpart[d, part] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uqxtn() -> &'static [IrStatement] {
    [exception("uqxtn")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// bits(32) element;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, 32];
///     Elem[result, e, 32] = UnsignedRecipEstimate(element);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn urecpe() -> &'static [IrStatement] {
    [exception("urecpe")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     Elem[result, e, esize] = (element1 + element2 + 1)<esize:1>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn urhadd() -> &'static [IrStatement] {
    [exception("urhadd")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// 
/// integer round_const = 0;
/// integer shift;
/// integer element;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     shift = SInt(Elem[operand2, e, esize]<7:0>);
///     if rounding then
///         round_const = 1 << (-shift - 1); // 0 for left shift, 2^(n-1) for right shift
///     element = (Int(Elem[operand1, e, esize], unsigned) + round_const) << shift;
///     if saturating then
///         (Elem[result, e, esize], sat) = SatQ(element, esize, unsigned);
///         if sat then FPSR.QC = '1';
///     else
///         Elem[result, e, esize] = element<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn urshl() -> &'static [IrStatement] {
    [exception("urshl")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand  = V[n];
/// bits(datasize) operand2;
/// bits(datasize) result;
/// integer round_const = if round then (1 << (shift - 1)) else 0;
/// integer element;
/// 
/// operand2 = if accumulate then V[d] else Zeros();
/// for e = 0 to elements-1
///     element = (Int(Elem[operand, e, esize], unsigned) + round_const) >> shift;
///     Elem[result, e, esize] = Elem[operand2, e, esize] + element<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn urshr() -> &'static [IrStatement] {
    [exception("urshr")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// bits(32) element;
/// 
/// for e = 0 to elements-1
///     element = Elem[operand, e, 32];
///     Elem[result, e, 32] = UnsignedRSqrtEstimate(element);
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ursqrte() -> &'static [IrStatement] {
    [exception("ursqrte")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand  = V[n];
/// bits(datasize) operand2;
/// bits(datasize) result;
/// integer round_const = if round then (1 << (shift - 1)) else 0;
/// integer element;
/// 
/// operand2 = if accumulate then V[d] else Zeros();
/// for e = 0 to elements-1
///     element = (Int(Elem[operand, e, esize], unsigned) + round_const) >> shift;
///     Elem[result, e, esize] = Elem[operand2, e, esize] + element<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ursra() -> &'static [IrStatement] {
    [exception("ursra")].into()
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
///     bits(32) res = Elem[operand3, e, 32];
///     for b = 0 to 3
///         integer element1 = Int(Elem[operand1, 4 * e + b, 8], op1_unsigned);
///         integer element2 = Int(Elem[operand2, 4 * i + b, 8], op2_unsigned);
///         res = res + element1 * element2;
///     Elem[result, e, 32] = res;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn usdot() -> &'static [IrStatement] {
    [exception("usdot")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// 
/// integer round_const = 0;
/// integer shift;
/// integer element;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     shift = SInt(Elem[operand2, e, esize]<7:0>);
///     if rounding then
///         round_const = 1 << (-shift - 1); // 0 for left shift, 2^(n-1) for right shift
///     element = (Int(Elem[operand1, e, esize], unsigned) + round_const) << shift;
///     if saturating then
///         (Elem[result, e, esize], sat) = SatQ(element, esize, unsigned);
///         if sat then FPSR.QC = '1';
///     else
///         Elem[result, e, esize] = element<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ushl() -> &'static [IrStatement] {
    [exception("ushl")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = Vpart[n, part];
/// bits(datasize*2) result;
/// integer element;
/// 
/// for e = 0 to elements-1
///     element = Int(Elem[operand, e, esize], unsigned) << shift;
///     Elem[result, e, 2*esize] = element<2*esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ushll() -> &'static [IrStatement] {
    [exception("ushll")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand  = V[n];
/// bits(datasize) operand2;
/// bits(datasize) result;
/// integer round_const = if round then (1 << (shift - 1)) else 0;
/// integer element;
/// 
/// operand2 = if accumulate then V[d] else Zeros();
/// for e = 0 to elements-1
///     element = (Int(Elem[operand, e, esize], unsigned) + round_const) >> shift;
///     Elem[result, e, esize] = Elem[operand2, e, esize] + element<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ushr() -> &'static [IrStatement] {
    [exception("ushr")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(128) operand1 = V[n];
/// bits(128) operand2 = V[m];
/// bits(128) addend   = V[d];
/// 
/// V[d] = MatMulAdd(addend, operand1, operand2, op1_unsigned, op2_unsigned);
/// ```
#[box_to_static_reference]
pub(super) fn usmmla() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// 
/// bits(datasize) operand2 = V[d];
/// integer op1;
/// integer op2;
/// boolean sat;
/// 
/// for e = 0 to elements-1
///     op1 = Int(Elem[operand, e, esize], !unsigned);
///     op2 = Int(Elem[operand2, e, esize], unsigned);
///     (Elem[result, e, esize], sat) = SatQ(op1 + op2, esize, unsigned);
///     if sat then FPSR.QC = '1';
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn usqadd() -> &'static [IrStatement] {
    [exception("usqadd")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand  = V[n];
/// bits(datasize) operand2;
/// bits(datasize) result;
/// integer round_const = if round then (1 << (shift - 1)) else 0;
/// integer element;
/// 
/// operand2 = if accumulate then V[d] else Zeros();
/// for e = 0 to elements-1
///     element = (Int(Elem[operand, e, esize], unsigned) + round_const) >> shift;
///     Elem[result, e, esize] = Elem[operand2, e, esize] + element<esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn usra() -> &'static [IrStatement] {
    [exception("usra")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize)   operand1 = Vpart[n, part];
/// bits(datasize)   operand2 = Vpart[m, part];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// integer sum;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     if sub_op then
///         sum = element1 - element2;
///     else
///         sum = element1 + element2;
///     Elem[result, e, 2*esize] = sum<2*esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn usubl() -> &'static [IrStatement] {
    [exception("usubl")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(2*datasize) operand1 = V[n];
/// bits(datasize)   operand2 = Vpart[m, part];
/// bits(2*datasize) result;
/// integer element1;
/// integer element2;
/// integer sum;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, 2*esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     if sub_op then
///         sum = element1 - element2;
///     else
///         sum = element1 + element2;
///     Elem[result, e, 2*esize] = sum<2*esize-1:0>;
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn usubw() -> &'static [IrStatement] {
    [exception("usubw")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// integer hsize = esize DIV 2;
/// bits(VL) operand = Z[n];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     bits(hsize) element = if hi then Elem[operand, e + elements, hsize] else Elem[operand, e, hsize];
///     Elem[result, e, esize] = Extend(element, esize, unsigned);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uunpkhi() -> &'static [IrStatement] {
    [exception("uunpkhi")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand  = Z[n];
/// bits(VL) result = Z[d];
/// 
/// for e = 0 to elements-1
///     bits(esize) element = Elem[operand, e, esize];
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Extend(element<s_esize-1:0>, esize, unsigned);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uxtb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operandl = V[n];
/// bits(datasize) operandh = V[m];
/// bits(datasize) result;
/// 
/// bits(datasize*2) zipped = operandh:operandl;
/// for e = 0 to elements-1
///     Elem[result, e, esize] = Elem[zipped, 2*e+part, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uzp1() -> &'static [IrStatement] {
    [exception("uzp1")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operandl = V[n];
/// bits(datasize) operandh = V[m];
/// bits(datasize) result;
/// 
/// bits(datasize*2) zipped = operandh:operandl;
/// for e = 0 to elements-1
///     Elem[result, e, esize] = Elem[zipped, 2*e+part, esize];
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn uzp2() -> &'static [IrStatement] {
    [exception("uzp2")].into()
}
