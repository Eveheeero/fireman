use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) comparevalue;
/// bits(datasize) newvalue;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// comparevalue = X[s];
/// newvalue = X[t];
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomicCompareAndSwap(address, comparevalue, newvalue, ldacctype, stacctype);
/// 
/// X[s] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn cas() -> &'static [IrStatement] {
    [exception("cas")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) comparevalue;
/// bits(datasize) newvalue;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// comparevalue = X[s];
/// newvalue = X[t];
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomicCompareAndSwap(address, comparevalue, newvalue, ldacctype, stacctype);
/// 
/// X[s] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn casb() -> &'static [IrStatement] {
    [exception("casb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) comparevalue;
/// bits(datasize) newvalue;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// comparevalue = X[s];
/// newvalue = X[t];
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomicCompareAndSwap(address, comparevalue, newvalue, ldacctype, stacctype);
/// 
/// X[s] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn cash() -> &'static [IrStatement] {
    [exception("cash")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(2*datasize) comparevalue;
/// bits(2*datasize) newvalue;
/// bits(2*datasize) data;
/// 
/// bits(datasize) s1 = X[s];
/// bits(datasize) s2 = X[s+1];
/// bits(datasize) t1 = X[t];
/// bits(datasize) t2 = X[t+1];
/// comparevalue = if BigEndian() then s1:s2 else s2:s1;
/// newvalue     = if BigEndian() then t1:t2 else t2:t1;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomicCompareAndSwap(address, comparevalue, newvalue, ldacctype, stacctype);
/// 
/// if BigEndian() then
///     X[s]   = ZeroExtend(data<2*datasize-1:datasize>, regsize);
///     X[s+1] = ZeroExtend(data<datasize-1:0>, regsize);
/// else
///     X[s]   = ZeroExtend(data<datasize-1:0>, regsize);
///     X[s+1] = ZeroExtend(data<2*datasize-1:datasize>, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn casp() -> &'static [IrStatement] {
    [exception("casp")].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[t];
/// 
/// if IsZero(operand1) == iszero then
///     BranchTo(PC[] + offset, BranchType_DIR);
/// ```
#[box_to_static_reference]
pub(super) fn cbnz() -> &'static [IrStatement] {
    let fallthrough = b::add(pc.clone(), instruction_byte_size());
    let is_zero = b::equal(o1(), c(0), o1_size());
    [condition(u::not(is_zero), [jump(o2())], [jump(fallthrough)])].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[t];
/// 
/// if IsZero(operand1) == iszero then
///     BranchTo(PC[] + offset, BranchType_DIR);
/// ```
#[box_to_static_reference]
pub(super) fn cbz() -> &'static [IrStatement] {
    let fallthrough = b::add(pc.clone(), instruction_byte_size());
    [condition(b::equal(o1(), c(0), o1_size()), [jump(o2())], [jump(fallthrough)])].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = imm;
/// bit carry_in = '0';
/// 
/// if ConditionHolds(condition) then
///     if sub_op then
///         operand2 = NOT(operand2);
///         carry_in = '1';
///     (-, flags) = AddWithCarry(operand1, operand2, carry_in);
/// PSTATE.<N,Z,C,V> = flags;
/// ```
#[box_to_static_reference]
pub(super) fn ccmn() -> &'static [IrStatement] {
    let add = b::add(o1(), o2());
    let calc_flags = calc_flags_automatically(add, o1_size(), &[&pstate_n, &pstate_z, &pstate_c, &pstate_v]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = imm;
/// bit carry_in = '0';
/// 
/// if ConditionHolds(condition) then
///     if sub_op then
///         operand2 = NOT(operand2);
///         carry_in = '1';
///     (-, flags) = AddWithCarry(operand1, operand2, carry_in);
/// PSTATE.<N,Z,C,V> = flags;
/// ```
#[box_to_static_reference]
pub(super) fn ccmp() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&pstate_n, &pstate_z, &pstate_c, &pstate_v]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// PSTATE.C = NOT(PSTATE.C);
/// ```
#[box_to_static_reference]
pub(super) fn cfinv() -> &'static [IrStatement] {
    let stmt_0 = assign(u::not(pstate_c.clone()), pstate_c.clone(), size_relative(pstate_c.clone()));
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(esize) operand1 = X[dn];
/// bits(VL) operand2 = Z[m];
/// bits(csize) result;
/// integer last = LastActiveElement(mask, esize);
/// 
/// if last < 0 then
///     result = ZeroExtend(operand1);
/// else
///     if !isBefore then
///         last = last + 1;
///         if last >= elements then last = 0;
///     result = ZeroExtend(Elem[operand2, last, esize]);
/// 
/// X[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn clasta() -> &'static [IrStatement] {
    [exception("clasta")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(esize) operand1 = X[dn];
/// bits(VL) operand2 = Z[m];
/// bits(csize) result;
/// integer last = LastActiveElement(mask, esize);
/// 
/// if last < 0 then
///     result = ZeroExtend(operand1);
/// else
///     if !isBefore then
///         last = last + 1;
///         if last >= elements then last = 0;
///     result = ZeroExtend(Elem[operand2, last, esize]);
/// 
/// X[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn clastb() -> &'static [IrStatement] {
    [exception("clastb")].into()
}

/// # Pseudocode
/// ```text
/// ClearExclusiveLocal(ProcessorID());
/// ```
#[box_to_static_reference]
pub(super) fn clrex() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// 
/// integer count;
/// for e = 0 to elements-1
///     if countop == CountOp_CLS then
///         count = CountLeadingSignBits(Elem[operand, e, esize]);
///     else
///         count = CountLeadingZeroBits(Elem[operand, e, esize]);
///     Elem[result, e, esize] = count<esize-1:0>;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cls() -> &'static [IrStatement] {
    [exception("cls")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// 
/// integer count;
/// for e = 0 to elements-1
///     if countop == CountOp_CLS then
///         count = CountLeadingSignBits(Elem[operand, e, esize]);
///     else
///         count = CountLeadingZeroBits(Elem[operand, e, esize]);
///     Elem[result, e, esize] = count<esize-1:0>;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn clz() -> &'static [IrStatement] {
    [exception("clz")].into()
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
///     if and_test then
///         test_passed = !IsZero(element1 AND element2);
///     else
///         test_passed = (element1 == element2);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmeq() -> &'static [IrStatement] {
    [exception("cmeq")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     test_passed = if cmp_eq then element1 >= element2 else element1 > element2;
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmge() -> &'static [IrStatement] {
    [exception("cmge")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     test_passed = if cmp_eq then element1 >= element2 else element1 > element2;
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmgt() -> &'static [IrStatement] {
    [exception("cmgt")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     test_passed = if cmp_eq then element1 >= element2 else element1 > element2;
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmhi() -> &'static [IrStatement] {
    [exception("cmhi")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand1 = V[n];
/// bits(datasize) operand2 = V[m];
/// bits(datasize) result;
/// integer element1;
/// integer element2;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element1 = Int(Elem[operand1, e, esize], unsigned);
///     element2 = Int(Elem[operand2, e, esize], unsigned);
///     test_passed = if cmp_eq then element1 >= element2 else element1 > element2;
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmhs() -> &'static [IrStatement] {
    [exception("cmhs")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// integer element;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element = SInt(Elem[operand, e, esize]);
///     case comparison of
///         when CompareOp_GT test_passed = element > 0;
///         when CompareOp_GE test_passed = element >= 0;
///         when CompareOp_EQ test_passed = element == 0;
///         when CompareOp_LE test_passed = element <= 0;
///         when CompareOp_LT test_passed = element < 0;
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmle() -> &'static [IrStatement] {
    [exception("cmle")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// integer element;
/// boolean test_passed;
/// 
/// for e = 0 to elements-1
///     element = SInt(Elem[operand, e, esize]);
///     case comparison of
///         when CompareOp_GT test_passed = element > 0;
///         when CompareOp_GE test_passed = element >= 0;
///         when CompareOp_EQ test_passed = element == 0;
///         when CompareOp_LE test_passed = element <= 0;
///         when CompareOp_LT test_passed = element < 0;
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmlt() -> &'static [IrStatement] {
    [exception("cmlt")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[n];
/// bits(PL) result;
/// 
/// for e = 0 to elements-1
///     integer element1 = Int(Elem[operand1, e, esize], unsigned);
///     if ElemP[mask, e, esize] == '1' then
///         boolean cond;
///         case op of
///             when Cmp_EQ cond = element1 == imm;
///             when Cmp_NE cond = element1 != imm;
///             when Cmp_GE cond = element1 >= imm;
///             when Cmp_LT cond = element1 <  imm;
///             when Cmp_GT cond = element1 >  imm;
///             when Cmp_LE cond = element1 <= imm;
///         ElemP[result, e, esize] = if cond then '1' else '0';
///     else
///         ElemP[result, e, esize] = '0';
/// 
/// PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
/// P[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmpcc() -> &'static [IrStatement] {
    [exception("cmpcc")].into()
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
///     if and_test then
///         test_passed = !IsZero(element1 AND element2);
///     else
///         test_passed = (element1 == element2);
///     Elem[result, e, esize] = if test_passed then Ones() else Zeros();
/// 
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cmtst() -> &'static [IrStatement] {
    [exception("cmtst")].into()
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
///         Elem[result, e, esize] = ZeroExtend(IsZeroBit(element), esize);
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cnot() -> &'static [IrStatement] {
    [exception("cnot")].into()
}

/// # Pseudocode
/// ```text
/// CheckFPAdvSIMDEnabled64();
/// bits(datasize) operand = V[n];
/// bits(datasize) result;
/// 
/// integer count;
/// for e = 0 to elements-1
///     count = BitCount(Elem[operand, e, esize]);
///     Elem[result, e, esize] = count<esize-1:0>;
/// V[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cnt() -> &'static [IrStatement] {
    [exception("cnt")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer count = DecodePredCount(pat, esize);
/// 
/// X[d] = (count * imm)<63:0>;
/// ```
#[box_to_static_reference]
pub(super) fn cntb() -> &'static [IrStatement] {
    let stmt_0 = assign(b::mul(unknown_data(), unknown_data()), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(PL) operand = P[n];
/// bits(64) sum = Zeros();
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' && ElemP[operand, e, esize] == '1' then
///         sum = sum + 1;
/// X[d] = sum;
/// ```
#[box_to_static_reference]
pub(super) fn cntp() -> &'static [IrStatement] {
    [exception("cntp")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand1 = Z[n];
/// bits(VL) result;
/// integer x = 0;
/// 
/// for e = 0 to elements-1
///     Elem[result, e, esize] = Zeros();
///     if ElemP[mask, e, esize] == '1' then
///         bits(esize) element = Elem[operand1, e, esize];
///         Elem[result, x, esize] = element;
///         x = x + 1;
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn compact() -> &'static [IrStatement] {
    [exception("compact")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) dest = Z[d];
/// bits(VL) result;
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = imm<esize-1:0>;
///     elsif merging then
///         Elem[result, e, esize] = Elem[dest, e, esize];
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn cpy() -> &'static [IrStatement] {
    [exception("cpy")].into()
}

/// # Pseudocode
/// ```text
/// bits(32)      acc     = X[n];   // accumulator
/// bits(size)    val     = X[m];   // input value
/// bits(32)      poly    = (if crc32c then 0x1EDC6F41 else 0x04C11DB7)<31:0>;
/// 
/// bits(32+size) tempacc = BitReverse(acc) : Zeros(size);
/// bits(size+32) tempval = BitReverse(val) : Zeros(32);
/// 
/// // Poly32Mod2 on a bitstring does a polynomial Modulus over {0,1} operation
/// X[d] = BitReverse(Poly32Mod2(tempacc EOR tempval, poly));
/// ```
#[box_to_static_reference]
pub(super) fn crc32b() -> &'static [IrStatement] {
    [exception("crc32b")].into()
}

/// # Pseudocode
/// ```text
/// bits(32)      acc     = X[n];   // accumulator
/// bits(size)    val     = X[m];   // input value
/// bits(32)      poly    = (if crc32c then 0x1EDC6F41 else 0x04C11DB7)<31:0>;
/// 
/// bits(32+size) tempacc = BitReverse(acc) : Zeros(size);
/// bits(size+32) tempval = BitReverse(val) : Zeros(32);
/// 
/// // Poly32Mod2 on a bitstring does a polynomial Modulus over {0,1} operation
/// X[d] = BitReverse(Poly32Mod2(tempacc EOR tempval, poly));
/// ```
#[box_to_static_reference]
pub(super) fn crc32cb() -> &'static [IrStatement] {
    [exception("crc32cb")].into()
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
pub(super) fn csdb() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// 
/// if ConditionHolds(condition) then
///     result = operand1;
/// else
///     result = operand2;
///     if else_inv then result = NOT(result);
///     if else_inc then result = result + 1;
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn csel() -> &'static [IrStatement] {
    [condition(o3(), [assign(o1(), o1(), o1_size())], [assign(o2(), o1(), o1_size())])].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// 
/// if ConditionHolds(condition) then
///     result = operand1;
/// else
///     result = operand2;
///     if else_inv then result = NOT(result);
///     if else_inc then result = result + 1;
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn csinc() -> &'static [IrStatement] {
    [condition(o3(), [assign(o1(), o1(), o1_size())], [assign(b::add(o2(), c(1)), o1(), o1_size())])].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// 
/// if ConditionHolds(condition) then
///     result = operand1;
/// else
///     result = operand2;
///     if else_inv then result = NOT(result);
///     if else_inc then result = result + 1;
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn csinv() -> &'static [IrStatement] {
    [condition(o3(), [assign(o1(), o1(), o1_size())], [assign(u::not(o2()), o1(), o1_size())])].into()
}

/// # Pseudocode
/// ```text
/// bits(datasize) result;
/// bits(datasize) operand1 = X[n];
/// bits(datasize) operand2 = X[m];
/// 
/// if ConditionHolds(condition) then
///     result = operand1;
/// else
///     result = operand2;
///     if else_inv then result = NOT(result);
///     if else_inc then result = result + 1;
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn csneg() -> &'static [IrStatement] {
    [condition(o3(), [assign(o1(), o1(), o1_size())], [assign(u::neg(o2()), o1(), o1_size())])].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// bits(esize) operand1 = X[n];
/// bits(esize) operand2 = X[m];
/// integer element1 = UInt(operand1);
/// integer element2 = UInt(operand2);
/// boolean term;
/// 
/// case op of
///     when Cmp_EQ term = element1 == element2;
///     when Cmp_NE term = element1 != element2;
/// if term then
///     PSTATE.N = '1';
///     PSTATE.V = '0';
/// else
///     PSTATE.N = '0';
///     PSTATE.V = (NOT PSTATE.C);
/// ```
#[box_to_static_reference]
pub(super) fn ctermeq() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), pstate_n.clone(), size_relative(pstate_n.clone()));
    let stmt_1 = assign(unknown_data(), pstate_v.clone(), size_relative(pstate_v.clone()));
    [stmt_0, stmt_1].into()
}
