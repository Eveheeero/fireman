use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand = Z[n];
/// bits(rsize) result;
/// integer last = LastActiveElement(mask, esize);
/// 
/// if isBefore then
///     if last < 0 then last = elements - 1;
/// else
///     last = last + 1;
///     if last >= elements then last = 0;
/// result = ZeroExtend(Elem[operand, last, esize]);
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn lasta() -> &'static [IrStatement] {
    [exception("lasta")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(PL) mask = P[g];
/// bits(VL) operand = Z[n];
/// bits(rsize) result;
/// integer last = LastActiveElement(mask, esize);
/// 
/// if isBefore then
///     if last < 0 then last = elements - 1;
/// else
///     last = last + 1;
///     if last >= elements then last = 0;
/// result = ZeroExtend(Elem[operand, last, esize]);
/// 
/// X[d] = result;
/// ```
#[box_to_static_reference]
pub(super) fn lastb() -> &'static [IrStatement] {
    [exception("lastb")].into()
}

/// # Pseudocode
/// ```text
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = if Q == '1' then 128 else 64;
/// integer esize = 8 << UInt(size);
/// integer elements = datasize DIV esize;
/// 
/// integer rpt;    // number of iterations
/// integer selem;  // structure elements
/// 
/// case opcode of
///     when '0000' rpt = 1; selem = 4;     // LD/ST4 (4 registers)
///     when '0010' rpt = 4; selem = 1;     // LD/ST1 (4 registers)
///     when '0100' rpt = 1; selem = 3;     // LD/ST3 (3 registers)
///     when '0110' rpt = 3; selem = 1;     // LD/ST1 (3 registers)
///     when '0111' rpt = 1; selem = 1;     // LD/ST1 (1 register)
///     when '1000' rpt = 1; selem = 2;     // LD/ST2 (2 registers)
///     when '1010' rpt = 2; selem = 1;     // LD/ST1 (2 registers)
///     otherwise UNDEFINED;
/// 
/// // .1D format only permitted with LD1 & ST1
/// if size:Q == '110' && selem != 1 then UNDEFINED;CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(64) offs;
/// bits(datasize) rval;
/// integer tt;
/// constant integer ebytes = esize DIV 8;
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
/// offs = Zeros();
/// for r = 0 to rpt-1
///     for e = 0 to elements-1
///         tt = (t + r) MOD 32;
///         for s = 0 to selem-1
///             rval = V[tt];
///             if memop == MemOp_LOAD then
///                 Elem[rval, e, esize] = Mem[address + offs, ebytes, AccType_VEC];
///                 V[tt] = rval;
///             else // memop == MemOp_STORE
///                 Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, e, esize];
///             offs = offs + ebytes;
///             tt = (tt + 1) MOD 32;
/// 
/// if wback then
///     if m != 31 then
///         offs = X[m];
///     if n == 31 then
///         SP[] = address + offs;
///     else
///         X[n] = address + offs;
/// ```
#[box_to_static_reference]
pub(super) fn ld1() -> &'static [IrStatement] {
    [exception("ld1")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         data = Mem[addr, mbytes, AccType_NORMAL];
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1b() -> &'static [IrStatement] {
    [exception("ld1b")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         data = Mem[addr, mbytes, AccType_NORMAL];
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1d() -> &'static [IrStatement] {
    [exception("ld1d")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         data = Mem[addr, mbytes, AccType_NORMAL];
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1h() -> &'static [IrStatement] {
    [exception("ld1h")].into()
}

/// # Pseudocode
/// ```text
/// integer scale = UInt(opcode<2:1>);
/// integer selem = UInt(opcode<0>:R) + 1;
/// boolean replicate = FALSE;
/// integer index;
/// 
/// case scale of
///     when 3
///         // load and replicate
///         if L == '0' || S == '1' then UNDEFINED;
///         scale = UInt(size);
///         replicate = TRUE;
///     when 0
///         index = UInt(Q:S:size);         // B[0-15]
///     when 1
///         if size<0> == '1' then UNDEFINED;
///         index = UInt(Q:S:size<1>);      // H[0-7]
///     when 2
///         if size<1> == '1' then UNDEFINED;
///         if size<0> == '0' then
///             index = UInt(Q:S);          // S[0-3]
///         else
///             if S == '1' then UNDEFINED;
///             index = UInt(Q);            // D[0-1]
///             scale = 3;
/// 
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = if Q == '1' then 128 else 64;
/// integer esize = 8 << scale;if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(64) offs;
/// bits(128) rval;
/// bits(esize) element;
/// constant integer ebytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// offs = Zeros();
/// if replicate then
///     // load and replicate to all elements
///     for s = 0 to selem-1
///         element = Mem[address + offs, ebytes, AccType_VEC];
///         // replicate to fill 128- or 64-bit register
///         V[t] = Replicate(element, datasize DIV esize);
///         offs = offs + ebytes;
///         t = (t + 1) MOD 32;
/// else
///     // load/store one element per register
///     for s = 0 to selem-1
///         rval = V[t];
///         if memop == MemOp_LOAD then
///             // insert into one lane of 128-bit register
///             Elem[rval, index, esize] = Mem[address + offs, ebytes, AccType_VEC];
///             V[t] = rval;
///         else // memop == MemOp_STORE
///             // extract from one lane of 128-bit register
///             Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, index, esize];
///         offs = offs + ebytes;
///         t = (t + 1) MOD 32;
/// 
/// if wback then
///     if m != 31 then
///         offs = X[m];
///     if n == 31 then
///         SP[] = address + offs;
///     else
///         X[n] = address + offs;
/// ```
#[box_to_static_reference]
pub(super) fn ld1r() -> &'static [IrStatement] {
    [exception("ld1r")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     base = SP[];
/// else
///     base = X[n];
/// 
/// integer last = LastActiveElement(mask, esize);
/// if last >= 0 then
///     addr = base + offset * mbytes;
///     data = Mem[addr, mbytes, AccType_NORMAL];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1rb() -> &'static [IrStatement] {
    [exception("ld1rb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     base = SP[];
/// else
///     base = X[n];
/// 
/// integer last = LastActiveElement(mask, esize);
/// if last >= 0 then
///     addr = base + offset * mbytes;
///     data = Mem[addr, mbytes, AccType_NORMAL];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1rd() -> &'static [IrStatement] {
    [exception("ld1rd")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     base = SP[];
/// else
///     base = X[n];
/// 
/// integer last = LastActiveElement(mask, esize);
/// if last >= 0 then
///     addr = base + offset * mbytes;
///     data = Mem[addr, mbytes, AccType_NORMAL];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1rh() -> &'static [IrStatement] {
    [exception("ld1rh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// if VL < 256 then UNDEFINED;
/// integer elements = 256 DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g]; // low bits only
/// bits(256) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * 32;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = ZeroExtend(Replicate(result, VL DIV 256), VL);
/// ```
#[box_to_static_reference]
pub(super) fn ld1rob() -> &'static [IrStatement] {
    [exception("ld1rob")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// if VL < 256 then UNDEFINED;
/// integer elements = 256 DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g]; // low bits only
/// bits(256) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * 32;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = ZeroExtend(Replicate(result, VL DIV 256), VL);
/// ```
#[box_to_static_reference]
pub(super) fn ld1rod() -> &'static [IrStatement] {
    [exception("ld1rod")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// if VL < 256 then UNDEFINED;
/// integer elements = 256 DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g]; // low bits only
/// bits(256) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * 32;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = ZeroExtend(Replicate(result, VL DIV 256), VL);
/// ```
#[box_to_static_reference]
pub(super) fn ld1roh() -> &'static [IrStatement] {
    [exception("ld1roh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// if VL < 256 then UNDEFINED;
/// integer elements = 256 DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g]; // low bits only
/// bits(256) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * 32;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = ZeroExtend(Replicate(result, VL DIV 256), VL);
/// ```
#[box_to_static_reference]
pub(super) fn ld1row() -> &'static [IrStatement] {
    [exception("ld1row")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = 128 DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g]; // low 16 bits only
/// bits(128) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * 16;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = Replicate(result, VL DIV 128);
/// ```
#[box_to_static_reference]
pub(super) fn ld1rqb() -> &'static [IrStatement] {
    [exception("ld1rqb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = 128 DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g]; // low 16 bits only
/// bits(128) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * 16;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = Replicate(result, VL DIV 128);
/// ```
#[box_to_static_reference]
pub(super) fn ld1rqd() -> &'static [IrStatement] {
    [exception("ld1rqd")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = 128 DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g]; // low 16 bits only
/// bits(128) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * 16;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = Replicate(result, VL DIV 128);
/// ```
#[box_to_static_reference]
pub(super) fn ld1rqh() -> &'static [IrStatement] {
    [exception("ld1rqh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = 128 DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g]; // low 16 bits only
/// bits(128) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * 16;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = Replicate(result, VL DIV 128);
/// ```
#[box_to_static_reference]
pub(super) fn ld1rqw() -> &'static [IrStatement] {
    [exception("ld1rqw")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     base = SP[];
/// else
///     base = X[n];
/// 
/// integer last = LastActiveElement(mask, esize);
/// if last >= 0 then
///     addr = base + offset * mbytes;
///     data = Mem[addr, mbytes, AccType_NORMAL];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1rsb() -> &'static [IrStatement] {
    [exception("ld1rsb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     base = SP[];
/// else
///     base = X[n];
/// 
/// integer last = LastActiveElement(mask, esize);
/// if last >= 0 then
///     addr = base + offset * mbytes;
///     data = Mem[addr, mbytes, AccType_NORMAL];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1rsh() -> &'static [IrStatement] {
    [exception("ld1rsh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     base = SP[];
/// else
///     base = X[n];
/// 
/// integer last = LastActiveElement(mask, esize);
/// if last >= 0 then
///     addr = base + offset * mbytes;
///     data = Mem[addr, mbytes, AccType_NORMAL];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1rsw() -> &'static [IrStatement] {
    [exception("ld1rsw")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     base = SP[];
/// else
///     base = X[n];
/// 
/// integer last = LastActiveElement(mask, esize);
/// if last >= 0 then
///     addr = base + offset * mbytes;
///     data = Mem[addr, mbytes, AccType_NORMAL];
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1rw() -> &'static [IrStatement] {
    [exception("ld1rw")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         data = Mem[addr, mbytes, AccType_NORMAL];
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1sb() -> &'static [IrStatement] {
    [exception("ld1sb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         data = Mem[addr, mbytes, AccType_NORMAL];
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1sh() -> &'static [IrStatement] {
    [exception("ld1sh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         data = Mem[addr, mbytes, AccType_NORMAL];
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1sw() -> &'static [IrStatement] {
    [exception("ld1sw")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         data = Mem[addr, mbytes, AccType_NORMAL];
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
///     else
///         Elem[result, e, esize] = Zeros();
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ld1w() -> &'static [IrStatement] {
    [exception("ld1w")].into()
}

/// # Pseudocode
/// ```text
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = if Q == '1' then 128 else 64;
/// integer esize = 8 << UInt(size);
/// integer elements = datasize DIV esize;
/// 
/// integer rpt;    // number of iterations
/// integer selem;  // structure elements
/// 
/// case opcode of
///     when '0000' rpt = 1; selem = 4;     // LD/ST4 (4 registers)
///     when '0010' rpt = 4; selem = 1;     // LD/ST1 (4 registers)
///     when '0100' rpt = 1; selem = 3;     // LD/ST3 (3 registers)
///     when '0110' rpt = 3; selem = 1;     // LD/ST1 (3 registers)
///     when '0111' rpt = 1; selem = 1;     // LD/ST1 (1 register)
///     when '1000' rpt = 1; selem = 2;     // LD/ST2 (2 registers)
///     when '1010' rpt = 2; selem = 1;     // LD/ST1 (2 registers)
///     otherwise UNDEFINED;
/// 
/// // .1D format only permitted with LD1 & ST1
/// if size:Q == '110' && selem != 1 then UNDEFINED;CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(64) offs;
/// bits(datasize) rval;
/// integer tt;
/// constant integer ebytes = esize DIV 8;
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
/// offs = Zeros();
/// for r = 0 to rpt-1
///     for e = 0 to elements-1
///         tt = (t + r) MOD 32;
///         for s = 0 to selem-1
///             rval = V[tt];
///             if memop == MemOp_LOAD then
///                 Elem[rval, e, esize] = Mem[address + offs, ebytes, AccType_VEC];
///                 V[tt] = rval;
///             else // memop == MemOp_STORE
///                 Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, e, esize];
///             offs = offs + ebytes;
///             tt = (tt + 1) MOD 32;
/// 
/// if wback then
///     if m != 31 then
///         offs = X[m];
///     if n == 31 then
///         SP[] = address + offs;
///     else
///         X[n] = address + offs;
/// ```
#[box_to_static_reference]
pub(super) fn ld2() -> &'static [IrStatement] {
    [exception("ld2")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..1] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld2b() -> &'static [IrStatement] {
    [exception("ld2b")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..1] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld2d() -> &'static [IrStatement] {
    [exception("ld2d")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..1] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld2h() -> &'static [IrStatement] {
    [exception("ld2h")].into()
}

/// # Pseudocode
/// ```text
/// integer scale = UInt(opcode<2:1>);
/// integer selem = UInt(opcode<0>:R) + 1;
/// boolean replicate = FALSE;
/// integer index;
/// 
/// case scale of
///     when 3
///         // load and replicate
///         if L == '0' || S == '1' then UNDEFINED;
///         scale = UInt(size);
///         replicate = TRUE;
///     when 0
///         index = UInt(Q:S:size);         // B[0-15]
///     when 1
///         if size<0> == '1' then UNDEFINED;
///         index = UInt(Q:S:size<1>);      // H[0-7]
///     when 2
///         if size<1> == '1' then UNDEFINED;
///         if size<0> == '0' then
///             index = UInt(Q:S);          // S[0-3]
///         else
///             if S == '1' then UNDEFINED;
///             index = UInt(Q);            // D[0-1]
///             scale = 3;
/// 
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = if Q == '1' then 128 else 64;
/// integer esize = 8 << scale;if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(64) offs;
/// bits(128) rval;
/// bits(esize) element;
/// constant integer ebytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// offs = Zeros();
/// if replicate then
///     // load and replicate to all elements
///     for s = 0 to selem-1
///         element = Mem[address + offs, ebytes, AccType_VEC];
///         // replicate to fill 128- or 64-bit register
///         V[t] = Replicate(element, datasize DIV esize);
///         offs = offs + ebytes;
///         t = (t + 1) MOD 32;
/// else
///     // load/store one element per register
///     for s = 0 to selem-1
///         rval = V[t];
///         if memop == MemOp_LOAD then
///             // insert into one lane of 128-bit register
///             Elem[rval, index, esize] = Mem[address + offs, ebytes, AccType_VEC];
///             V[t] = rval;
///         else // memop == MemOp_STORE
///             // extract from one lane of 128-bit register
///             Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, index, esize];
///         offs = offs + ebytes;
///         t = (t + 1) MOD 32;
/// 
/// if wback then
///     if m != 31 then
///         offs = X[m];
///     if n == 31 then
///         SP[] = address + offs;
///     else
///         X[n] = address + offs;
/// ```
#[box_to_static_reference]
pub(super) fn ld2r() -> &'static [IrStatement] {
    [exception("ld2r")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..1] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld2w() -> &'static [IrStatement] {
    [exception("ld2w")].into()
}

/// # Pseudocode
/// ```text
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = if Q == '1' then 128 else 64;
/// integer esize = 8 << UInt(size);
/// integer elements = datasize DIV esize;
/// 
/// integer rpt;    // number of iterations
/// integer selem;  // structure elements
/// 
/// case opcode of
///     when '0000' rpt = 1; selem = 4;     // LD/ST4 (4 registers)
///     when '0010' rpt = 4; selem = 1;     // LD/ST1 (4 registers)
///     when '0100' rpt = 1; selem = 3;     // LD/ST3 (3 registers)
///     when '0110' rpt = 3; selem = 1;     // LD/ST1 (3 registers)
///     when '0111' rpt = 1; selem = 1;     // LD/ST1 (1 register)
///     when '1000' rpt = 1; selem = 2;     // LD/ST2 (2 registers)
///     when '1010' rpt = 2; selem = 1;     // LD/ST1 (2 registers)
///     otherwise UNDEFINED;
/// 
/// // .1D format only permitted with LD1 & ST1
/// if size:Q == '110' && selem != 1 then UNDEFINED;CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(64) offs;
/// bits(datasize) rval;
/// integer tt;
/// constant integer ebytes = esize DIV 8;
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
/// offs = Zeros();
/// for r = 0 to rpt-1
///     for e = 0 to elements-1
///         tt = (t + r) MOD 32;
///         for s = 0 to selem-1
///             rval = V[tt];
///             if memop == MemOp_LOAD then
///                 Elem[rval, e, esize] = Mem[address + offs, ebytes, AccType_VEC];
///                 V[tt] = rval;
///             else // memop == MemOp_STORE
///                 Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, e, esize];
///             offs = offs + ebytes;
///             tt = (tt + 1) MOD 32;
/// 
/// if wback then
///     if m != 31 then
///         offs = X[m];
///     if n == 31 then
///         SP[] = address + offs;
///     else
///         X[n] = address + offs;
/// ```
#[box_to_static_reference]
pub(super) fn ld3() -> &'static [IrStatement] {
    [exception("ld3")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..2] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld3b() -> &'static [IrStatement] {
    [exception("ld3b")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..2] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld3d() -> &'static [IrStatement] {
    [exception("ld3d")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..2] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld3h() -> &'static [IrStatement] {
    [exception("ld3h")].into()
}

/// # Pseudocode
/// ```text
/// integer scale = UInt(opcode<2:1>);
/// integer selem = UInt(opcode<0>:R) + 1;
/// boolean replicate = FALSE;
/// integer index;
/// 
/// case scale of
///     when 3
///         // load and replicate
///         if L == '0' || S == '1' then UNDEFINED;
///         scale = UInt(size);
///         replicate = TRUE;
///     when 0
///         index = UInt(Q:S:size);         // B[0-15]
///     when 1
///         if size<0> == '1' then UNDEFINED;
///         index = UInt(Q:S:size<1>);      // H[0-7]
///     when 2
///         if size<1> == '1' then UNDEFINED;
///         if size<0> == '0' then
///             index = UInt(Q:S);          // S[0-3]
///         else
///             if S == '1' then UNDEFINED;
///             index = UInt(Q);            // D[0-1]
///             scale = 3;
/// 
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = if Q == '1' then 128 else 64;
/// integer esize = 8 << scale;if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(64) offs;
/// bits(128) rval;
/// bits(esize) element;
/// constant integer ebytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// offs = Zeros();
/// if replicate then
///     // load and replicate to all elements
///     for s = 0 to selem-1
///         element = Mem[address + offs, ebytes, AccType_VEC];
///         // replicate to fill 128- or 64-bit register
///         V[t] = Replicate(element, datasize DIV esize);
///         offs = offs + ebytes;
///         t = (t + 1) MOD 32;
/// else
///     // load/store one element per register
///     for s = 0 to selem-1
///         rval = V[t];
///         if memop == MemOp_LOAD then
///             // insert into one lane of 128-bit register
///             Elem[rval, index, esize] = Mem[address + offs, ebytes, AccType_VEC];
///             V[t] = rval;
///         else // memop == MemOp_STORE
///             // extract from one lane of 128-bit register
///             Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, index, esize];
///         offs = offs + ebytes;
///         t = (t + 1) MOD 32;
/// 
/// if wback then
///     if m != 31 then
///         offs = X[m];
///     if n == 31 then
///         SP[] = address + offs;
///     else
///         X[n] = address + offs;
/// ```
#[box_to_static_reference]
pub(super) fn ld3r() -> &'static [IrStatement] {
    [exception("ld3r")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..2] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld3w() -> &'static [IrStatement] {
    [exception("ld3w")].into()
}

/// # Pseudocode
/// ```text
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = if Q == '1' then 128 else 64;
/// integer esize = 8 << UInt(size);
/// integer elements = datasize DIV esize;
/// 
/// integer rpt;    // number of iterations
/// integer selem;  // structure elements
/// 
/// case opcode of
///     when '0000' rpt = 1; selem = 4;     // LD/ST4 (4 registers)
///     when '0010' rpt = 4; selem = 1;     // LD/ST1 (4 registers)
///     when '0100' rpt = 1; selem = 3;     // LD/ST3 (3 registers)
///     when '0110' rpt = 3; selem = 1;     // LD/ST1 (3 registers)
///     when '0111' rpt = 1; selem = 1;     // LD/ST1 (1 register)
///     when '1000' rpt = 1; selem = 2;     // LD/ST2 (2 registers)
///     when '1010' rpt = 2; selem = 1;     // LD/ST1 (2 registers)
///     otherwise UNDEFINED;
/// 
/// // .1D format only permitted with LD1 & ST1
/// if size:Q == '110' && selem != 1 then UNDEFINED;CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(64) offs;
/// bits(datasize) rval;
/// integer tt;
/// constant integer ebytes = esize DIV 8;
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
/// offs = Zeros();
/// for r = 0 to rpt-1
///     for e = 0 to elements-1
///         tt = (t + r) MOD 32;
///         for s = 0 to selem-1
///             rval = V[tt];
///             if memop == MemOp_LOAD then
///                 Elem[rval, e, esize] = Mem[address + offs, ebytes, AccType_VEC];
///                 V[tt] = rval;
///             else // memop == MemOp_STORE
///                 Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, e, esize];
///             offs = offs + ebytes;
///             tt = (tt + 1) MOD 32;
/// 
/// if wback then
///     if m != 31 then
///         offs = X[m];
///     if n == 31 then
///         SP[] = address + offs;
///     else
///         X[n] = address + offs;
/// ```
#[box_to_static_reference]
pub(super) fn ld4() -> &'static [IrStatement] {
    [exception("ld4")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..3] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld4b() -> &'static [IrStatement] {
    [exception("ld4b")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..3] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld4d() -> &'static [IrStatement] {
    [exception("ld4d")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..3] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld4h() -> &'static [IrStatement] {
    [exception("ld4h")].into()
}

/// # Pseudocode
/// ```text
/// integer scale = UInt(opcode<2:1>);
/// integer selem = UInt(opcode<0>:R) + 1;
/// boolean replicate = FALSE;
/// integer index;
/// 
/// case scale of
///     when 3
///         // load and replicate
///         if L == '0' || S == '1' then UNDEFINED;
///         scale = UInt(size);
///         replicate = TRUE;
///     when 0
///         index = UInt(Q:S:size);         // B[0-15]
///     when 1
///         if size<0> == '1' then UNDEFINED;
///         index = UInt(Q:S:size<1>);      // H[0-7]
///     when 2
///         if size<1> == '1' then UNDEFINED;
///         if size<0> == '0' then
///             index = UInt(Q:S);          // S[0-3]
///         else
///             if S == '1' then UNDEFINED;
///             index = UInt(Q);            // D[0-1]
///             scale = 3;
/// 
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = if Q == '1' then 128 else 64;
/// integer esize = 8 << scale;if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(64) offs;
/// bits(128) rval;
/// bits(esize) element;
/// constant integer ebytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// offs = Zeros();
/// if replicate then
///     // load and replicate to all elements
///     for s = 0 to selem-1
///         element = Mem[address + offs, ebytes, AccType_VEC];
///         // replicate to fill 128- or 64-bit register
///         V[t] = Replicate(element, datasize DIV esize);
///         offs = offs + ebytes;
///         t = (t + 1) MOD 32;
/// else
///     // load/store one element per register
///     for s = 0 to selem-1
///         rval = V[t];
///         if memop == MemOp_LOAD then
///             // insert into one lane of 128-bit register
///             Elem[rval, index, esize] = Mem[address + offs, ebytes, AccType_VEC];
///             V[t] = rval;
///         else // memop == MemOp_STORE
///             // extract from one lane of 128-bit register
///             Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, index, esize];
///         offs = offs + ebytes;
///         t = (t + 1) MOD 32;
/// 
/// if wback then
///     if m != 31 then
///         offs = X[m];
///     if n == 31 then
///         SP[] = address + offs;
///     else
///         X[n] = address + offs;
/// ```
#[box_to_static_reference]
pub(super) fn ld4r() -> &'static [IrStatement] {
    [exception("ld4r")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// constant integer mbytes = esize DIV 8;
/// array [0..3] of bits(VL) values;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * nreg * mbytes;
/// for e = 0 to elements-1
///     for r = 0 to nreg-1
///         if ElemP[mask, e, esize] == '1' then
///             Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
///         else
///             Elem[values[r], e, esize] = Zeros();
///         addr = addr + mbytes;
/// 
/// for r = 0 to nreg-1
///     Z[(t+r) MOD 32] = values[r];
/// ```
#[box_to_static_reference]
pub(super) fn ld4w() -> &'static [IrStatement] {
    [exception("ld4w")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldadd() -> &'static [IrStatement] {
    [exception("ldadd")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldaddb() -> &'static [IrStatement] {
    [exception("ldaddb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldaddh() -> &'static [IrStatement] {
    [exception("ldaddh")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// data = Mem[address, dbytes, acctype];
/// X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldapr() -> &'static [IrStatement] {
    [exception("ldapr")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// data = Mem[address, dbytes, acctype];
/// X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldaprb() -> &'static [IrStatement] {
    [exception("ldaprb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// data = Mem[address, dbytes, acctype];
/// X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldaprh() -> &'static [IrStatement] {
    [exception("ldaprh")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_ORDERED;
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
pub(super) fn ldapur() -> &'static [IrStatement] {
    [exception("ldapur")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_ORDERED;
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
pub(super) fn ldapurb() -> &'static [IrStatement] {
    [exception("ldapurb")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_ORDERED;
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
pub(super) fn ldapurh() -> &'static [IrStatement] {
    [exception("ldapurh")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_ORDERED;
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
pub(super) fn ldapursb() -> &'static [IrStatement] {
    [exception("ldapursb")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_ORDERED;
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
pub(super) fn ldapursh() -> &'static [IrStatement] {
    [exception("ldapursh")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_ORDERED;
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
pub(super) fn ldapursw() -> &'static [IrStatement] {
    [exception("ldapursw")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// case memop of
///     when MemOp_STORE
///         data = X[t];
///         Mem[address, dbytes, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, dbytes, acctype];
///         X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldar() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// case memop of
///     when MemOp_STORE
///         data = X[t];
///         Mem[address, dbytes, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, dbytes, acctype];
///         X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldarb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// case memop of
///     when MemOp_STORE
///         data = X[t];
///         Mem[address, dbytes, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, dbytes, acctype];
///         X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldarh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// boolean rn_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && pair && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE then
///     if s == t || (pair && s == t2) then
///         Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
///             when Constraint_NONE       rt_unknown = FALSE;   // store original value
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
///     if s == n && n != 31 then
///         Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
///             when Constraint_NONE       rn_unknown = FALSE;   // address is original base
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// elsif rn_unknown then
///     address = bits(64) UNKNOWN;
/// else
///     address = X[n];
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         elsif pair then
///             bits(datasize DIV 2) el1 = X[t];
///             bits(datasize DIV 2) el2 = X[t2];
///             data = if BigEndian() then el1 : el2 else el2 : el1;
///         else
///             data = X[t];
/// 
///         bit status = '1';
///         // Check whether the Exclusives monitors are set to include the
///         // physical memory locations corresponding to virtual address
///         // range [address, address+dbytes-1].
///         if AArch64.ExclusiveMonitorsPass(address, dbytes) then
///             // This atomic write will be rejected if it does not refer
///             // to the same physical locations after address translation.
///             Mem[address, dbytes, acctype] = data;
///             status = ExclusiveMonitorsStatus();
///         X[s] = ZeroExtend(status, 32);
/// 
///     when MemOp_LOAD
///         // Tell the Exclusives monitors to record a sequence of one or more atomic
///         // memory reads from virtual address range [address, address+dbytes-1].
///         // The Exclusives monitor will only be set if all the reads are from the
///         // same dbytes-aligned physical address, to allow for the possibility of
///         // an atomicity break if the translation is changed between reads.
///         AArch64.SetExclusiveMonitors(address, dbytes);
/// 
///         if pair then
///             if rt_unknown then
///                 // ConstrainedUNPREDICTABLE case
///                 X[t]  = bits(datasize) UNKNOWN;        // In this case t = t2
///             elsif elsize == 32 then
///                 // 32-bit load exclusive pair (atomic)
///                 data = Mem[address, dbytes, acctype];
///                 if BigEndian() then
///                     X[t]  = data<datasize-1:elsize>;
///                     X[t2] = data<elsize-1:0>;
///                 else
///                     X[t]  = data<elsize-1:0>;
///                     X[t2] = data<datasize-1:elsize>;
///             else // elsize == 64
///                 // 64-bit load exclusive pair (not atomic),
///                 // but must be 128-bit aligned
///                 if address != Align(address, dbytes) then
///                     iswrite = FALSE;
///                     secondstage = FALSE;
///                     AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
///                 X[t]  = Mem[address + 0, 8, acctype];
///                 X[t2] = Mem[address + 8, 8, acctype];
///         else
///             data = Mem[address, dbytes, acctype];
///             X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldaxp() -> &'static [IrStatement] {
    [exception("ldaxp")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// boolean rn_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && pair && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE then
///     if s == t || (pair && s == t2) then
///         Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
///             when Constraint_NONE       rt_unknown = FALSE;   // store original value
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
///     if s == n && n != 31 then
///         Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
///             when Constraint_NONE       rn_unknown = FALSE;   // address is original base
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// elsif rn_unknown then
///     address = bits(64) UNKNOWN;
/// else
///     address = X[n];
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         elsif pair then
///             bits(datasize DIV 2) el1 = X[t];
///             bits(datasize DIV 2) el2 = X[t2];
///             data = if BigEndian() then el1 : el2 else el2 : el1;
///         else
///             data = X[t];
/// 
///         bit status = '1';
///         // Check whether the Exclusives monitors are set to include the
///         // physical memory locations corresponding to virtual address
///         // range [address, address+dbytes-1].
///         if AArch64.ExclusiveMonitorsPass(address, dbytes) then
///             // This atomic write will be rejected if it does not refer
///             // to the same physical locations after address translation.
///             Mem[address, dbytes, acctype] = data;
///             status = ExclusiveMonitorsStatus();
///         X[s] = ZeroExtend(status, 32);
/// 
///     when MemOp_LOAD
///         // Tell the Exclusives monitors to record a sequence of one or more atomic
///         // memory reads from virtual address range [address, address+dbytes-1].
///         // The Exclusives monitor will only be set if all the reads are from the
///         // same dbytes-aligned physical address, to allow for the possibility of
///         // an atomicity break if the translation is changed between reads.
///         AArch64.SetExclusiveMonitors(address, dbytes);
/// 
///         if pair then
///             if rt_unknown then
///                 // ConstrainedUNPREDICTABLE case
///                 X[t]  = bits(datasize) UNKNOWN;        // In this case t = t2
///             elsif elsize == 32 then
///                 // 32-bit load exclusive pair (atomic)
///                 data = Mem[address, dbytes, acctype];
///                 if BigEndian() then
///                     X[t]  = data<datasize-1:elsize>;
///                     X[t2] = data<elsize-1:0>;
///                 else
///                     X[t]  = data<elsize-1:0>;
///                     X[t2] = data<datasize-1:elsize>;
///             else // elsize == 64
///                 // 64-bit load exclusive pair (not atomic),
///                 // but must be 128-bit aligned
///                 if address != Align(address, dbytes) then
///                     iswrite = FALSE;
///                     secondstage = FALSE;
///                     AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
///                 X[t]  = Mem[address + 0, 8, acctype];
///                 X[t2] = Mem[address + 8, 8, acctype];
///         else
///             data = Mem[address, dbytes, acctype];
///             X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldaxr() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// boolean rn_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && pair && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE then
///     if s == t || (pair && s == t2) then
///         Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
///             when Constraint_NONE       rt_unknown = FALSE;   // store original value
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
///     if s == n && n != 31 then
///         Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
///             when Constraint_NONE       rn_unknown = FALSE;   // address is original base
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// elsif rn_unknown then
///     address = bits(64) UNKNOWN;
/// else
///     address = X[n];
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         elsif pair then
///             bits(datasize DIV 2) el1 = X[t];
///             bits(datasize DIV 2) el2 = X[t2];
///             data = if BigEndian() then el1 : el2 else el2 : el1;
///         else
///             data = X[t];
/// 
///         bit status = '1';
///         // Check whether the Exclusives monitors are set to include the
///         // physical memory locations corresponding to virtual address
///         // range [address, address+dbytes-1].
///         if AArch64.ExclusiveMonitorsPass(address, dbytes) then
///             // This atomic write will be rejected if it does not refer
///             // to the same physical locations after address translation.
///             Mem[address, dbytes, acctype] = data;
///             status = ExclusiveMonitorsStatus();
///         X[s] = ZeroExtend(status, 32);
/// 
///     when MemOp_LOAD
///         // Tell the Exclusives monitors to record a sequence of one or more atomic
///         // memory reads from virtual address range [address, address+dbytes-1].
///         // The Exclusives monitor will only be set if all the reads are from the
///         // same dbytes-aligned physical address, to allow for the possibility of
///         // an atomicity break if the translation is changed between reads.
///         AArch64.SetExclusiveMonitors(address, dbytes);
/// 
///         if pair then
///             if rt_unknown then
///                 // ConstrainedUNPREDICTABLE case
///                 X[t]  = bits(datasize) UNKNOWN;        // In this case t = t2
///             elsif elsize == 32 then
///                 // 32-bit load exclusive pair (atomic)
///                 data = Mem[address, dbytes, acctype];
///                 if BigEndian() then
///                     X[t]  = data<datasize-1:elsize>;
///                     X[t2] = data<elsize-1:0>;
///                 else
///                     X[t]  = data<elsize-1:0>;
///                     X[t2] = data<datasize-1:elsize>;
///             else // elsize == 64
///                 // 64-bit load exclusive pair (not atomic),
///                 // but must be 128-bit aligned
///                 if address != Align(address, dbytes) then
///                     iswrite = FALSE;
///                     secondstage = FALSE;
///                     AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
///                 X[t]  = Mem[address + 0, 8, acctype];
///                 X[t2] = Mem[address + 8, 8, acctype];
///         else
///             data = Mem[address, dbytes, acctype];
///             X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldaxrb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// boolean rn_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && pair && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE then
///     if s == t || (pair && s == t2) then
///         Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
///             when Constraint_NONE       rt_unknown = FALSE;   // store original value
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
///     if s == n && n != 31 then
///         Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
///             when Constraint_NONE       rn_unknown = FALSE;   // address is original base
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// elsif rn_unknown then
///     address = bits(64) UNKNOWN;
/// else
///     address = X[n];
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         elsif pair then
///             bits(datasize DIV 2) el1 = X[t];
///             bits(datasize DIV 2) el2 = X[t2];
///             data = if BigEndian() then el1 : el2 else el2 : el1;
///         else
///             data = X[t];
/// 
///         bit status = '1';
///         // Check whether the Exclusives monitors are set to include the
///         // physical memory locations corresponding to virtual address
///         // range [address, address+dbytes-1].
///         if AArch64.ExclusiveMonitorsPass(address, dbytes) then
///             // This atomic write will be rejected if it does not refer
///             // to the same physical locations after address translation.
///             Mem[address, dbytes, acctype] = data;
///             status = ExclusiveMonitorsStatus();
///         X[s] = ZeroExtend(status, 32);
/// 
///     when MemOp_LOAD
///         // Tell the Exclusives monitors to record a sequence of one or more atomic
///         // memory reads from virtual address range [address, address+dbytes-1].
///         // The Exclusives monitor will only be set if all the reads are from the
///         // same dbytes-aligned physical address, to allow for the possibility of
///         // an atomicity break if the translation is changed between reads.
///         AArch64.SetExclusiveMonitors(address, dbytes);
/// 
///         if pair then
///             if rt_unknown then
///                 // ConstrainedUNPREDICTABLE case
///                 X[t]  = bits(datasize) UNKNOWN;        // In this case t = t2
///             elsif elsize == 32 then
///                 // 32-bit load exclusive pair (atomic)
///                 data = Mem[address, dbytes, acctype];
///                 if BigEndian() then
///                     X[t]  = data<datasize-1:elsize>;
///                     X[t2] = data<elsize-1:0>;
///                 else
///                     X[t]  = data<elsize-1:0>;
///                     X[t2] = data<datasize-1:elsize>;
///             else // elsize == 64
///                 // 64-bit load exclusive pair (not atomic),
///                 // but must be 128-bit aligned
///                 if address != Align(address, dbytes) then
///                     iswrite = FALSE;
///                     secondstage = FALSE;
///                     AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
///                 X[t]  = Mem[address + 0, 8, acctype];
///                 X[t2] = Mem[address + 8, 8, acctype];
///         else
///             data = Mem[address, dbytes, acctype];
///             X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldaxrh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldclr() -> &'static [IrStatement] {
    [exception("ldclr")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldclrb() -> &'static [IrStatement] {
    [exception("ldclrb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldclrh() -> &'static [IrStatement] {
    [exception("ldclrh")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldeor() -> &'static [IrStatement] {
    [exception("ldeor")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldeorb() -> &'static [IrStatement] {
    [exception("ldeorb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldeorh() -> &'static [IrStatement] {
    [exception("ldeorh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean first = TRUE;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         if first then
///             // Mem[] will not return if a fault is detected for the first active element
///             data = Mem[addr, mbytes, AccType_NORMAL];
///             first = FALSE;
///         else
///             // MemNF[] will return fault=TRUE if access is not performed for any reason
///             (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldff1b() -> &'static [IrStatement] {
    [exception("ldff1b")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean first = TRUE;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         if first then
///             // Mem[] will not return if a fault is detected for the first active element
///             data = Mem[addr, mbytes, AccType_NORMAL];
///             first = FALSE;
///         else
///             // MemNF[] will return fault=TRUE if access is not performed for any reason
///             (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldff1d() -> &'static [IrStatement] {
    [exception("ldff1d")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean first = TRUE;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         if first then
///             // Mem[] will not return if a fault is detected for the first active element
///             data = Mem[addr, mbytes, AccType_NORMAL];
///             first = FALSE;
///         else
///             // MemNF[] will return fault=TRUE if access is not performed for any reason
///             (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldff1h() -> &'static [IrStatement] {
    [exception("ldff1h")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean first = TRUE;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         if first then
///             // Mem[] will not return if a fault is detected for the first active element
///             data = Mem[addr, mbytes, AccType_NORMAL];
///             first = FALSE;
///         else
///             // MemNF[] will return fault=TRUE if access is not performed for any reason
///             (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldff1sb() -> &'static [IrStatement] {
    [exception("ldff1sb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean first = TRUE;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         if first then
///             // Mem[] will not return if a fault is detected for the first active element
///             data = Mem[addr, mbytes, AccType_NORMAL];
///             first = FALSE;
///         else
///             // MemNF[] will return fault=TRUE if access is not performed for any reason
///             (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldff1sh() -> &'static [IrStatement] {
    [exception("ldff1sh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean first = TRUE;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         if first then
///             // Mem[] will not return if a fault is detected for the first active element
///             data = Mem[addr, mbytes, AccType_NORMAL];
///             first = FALSE;
///         else
///             // MemNF[] will return fault=TRUE if access is not performed for any reason
///             (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldff1sw() -> &'static [IrStatement] {
    [exception("ldff1sw")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(VL) base = Z[n];
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean first = TRUE;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
/// 
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
///         if first then
///             // Mem[] will not return if a fault is detected for the first active element
///             data = Mem[addr, mbytes, AccType_NORMAL];
///             first = FALSE;
///         else
///             // MemNF[] will return fault=TRUE if access is not performed for any reason
///             (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldff1w() -> &'static [IrStatement] {
    [exception("ldff1w")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(4) tag;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// address = address + offset;
/// address = Align(address, TAG_GRANULE);
/// 
/// tag = AArch64.MemTag[address, AccType_NORMAL];
/// X[t] = AArch64.AddressWithAllocationTag(X[t], tag);
/// ```
#[box_to_static_reference]
pub(super) fn ldg() -> &'static [IrStatement] {
    [exception("ldg")].into()
}

/// # Pseudocode
/// ```text
/// if PSTATE.EL == EL0 then
///     UNDEFINED;
/// 
/// bits(64) data = Zeros(64);
/// bits(64) address;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// integer size = 4*(2^(UInt(GMID_EL1.BS)));
/// address = Align(address,size);
/// integer count = size >> LOG2_TAG_GRANULE;
/// integer index = UInt(address<LOG2_TAG_GRANULE+3:LOG2_TAG_GRANULE>);
/// 
/// for i = 0 to count-1
///     bits(4) tag = AArch64.MemTag[address, AccType_NORMAL];
///     data<(index*4)+3:index*4> = tag;
///     address = address + TAG_GRANULE;
///     index = index + 1;
/// 
/// X[t] = data;
/// ```
#[box_to_static_reference]
pub(super) fn ldgm() -> &'static [IrStatement] {
    [exception("ldgm")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// case memop of
///     when MemOp_STORE
///         data = X[t];
///         Mem[address, dbytes, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, dbytes, acctype];
///         X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldlar() -> &'static [IrStatement] {
    [exception("ldlar")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// case memop of
///     when MemOp_STORE
///         data = X[t];
///         Mem[address, dbytes, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, dbytes, acctype];
///         X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldlarb() -> &'static [IrStatement] {
    [exception("ldlarb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
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
/// case memop of
///     when MemOp_STORE
///         data = X[t];
///         Mem[address, dbytes, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, dbytes, acctype];
///         X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldlarh() -> &'static [IrStatement] {
    [exception("ldlarh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         // MemNF[] will return fault=TRUE if access is not performed for any reason
///         (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnf1b() -> &'static [IrStatement] {
    [exception("ldnf1b")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         // MemNF[] will return fault=TRUE if access is not performed for any reason
///         (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnf1d() -> &'static [IrStatement] {
    [exception("ldnf1d")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         // MemNF[] will return fault=TRUE if access is not performed for any reason
///         (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnf1h() -> &'static [IrStatement] {
    [exception("ldnf1h")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         // MemNF[] will return fault=TRUE if access is not performed for any reason
///         (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnf1sb() -> &'static [IrStatement] {
    [exception("ldnf1sb")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         // MemNF[] will return fault=TRUE if access is not performed for any reason
///         (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnf1sh() -> &'static [IrStatement] {
    [exception("ldnf1sh")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         // MemNF[] will return fault=TRUE if access is not performed for any reason
///         (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnf1sw() -> &'static [IrStatement] {
    [exception("ldnf1sw")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// bits(VL) orig = Z[t];
/// bits(msize) data;
/// constant integer mbytes = msize DIV 8;
/// boolean fault = FALSE;
/// boolean faulted = FALSE;
/// boolean unknown = FALSE;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         // MemNF[] will return fault=TRUE if access is not performed for any reason
///         (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
///     else
///         (data, fault) = (Zeros(msize), FALSE);
/// 
///     // FFR elements set to FALSE following a supressed access/fault
///     faulted = faulted || fault;
///     if faulted then
///         ElemFFR[e, esize] = '0';
/// 
///     // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
///     unknown = unknown || ElemFFR[e, esize] == '0';
///     if unknown then
///         if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
///             Elem[result, e, esize] = Extend(data, esize, unsigned);
///         elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
///             Elem[result, e, esize] = Zeros();
///         else  // merge
///             Elem[result, e, esize] = Elem[orig, e, esize];
///     else
///         Elem[result, e, esize] = Extend(data, esize, unsigned);
/// 
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnf1w() -> &'static [IrStatement] {
    [exception("ldnf1w")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// integer t2 = UInt(Rt2);
/// AccType acctype = AccType_VECSTREAM;
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// if opc == '11' then UNDEFINED;
/// integer scale = 2 + UInt(opc);
/// integer datasize = 8 << scale;
/// bits(64) offset = LSL(SignExtend(imm7, 64), scale);
/// boolean tag_checked = wback || n != 31;CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(datasize) data1;
/// bits(datasize) data2;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// if ! postindex then
///     address = address + offset;
/// 
/// case memop of
///     when MemOp_STORE
///         data1 = V[t];
///         data2 = V[t2];
///         Mem[address + 0     , dbytes, acctype] = data1;
///         Mem[address + dbytes, dbytes, acctype] = data2;
/// 
///     when MemOp_LOAD
///         data1 = Mem[address + 0     , dbytes, acctype];
///         data2 = Mem[address + dbytes, dbytes, acctype];
///         if rt_unknown then
///             data1 = bits(datasize) UNKNOWN;
///             data2 = bits(datasize) UNKNOWN;
///         V[t]  = data1;
///         V[t2] = data2;
/// 
/// if wback then
///     if postindex then
///         address = address + offset;
///     if n == 31 then
///         SP[] = address;
///     else
///         X[n] = address;
/// ```
#[box_to_static_reference]
pub(super) fn ldnp() -> &'static [IrStatement] {
    [exception("ldnp")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_STREAM];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnt1b() -> &'static [IrStatement] {
    [exception("ldnt1b")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_STREAM];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnt1d() -> &'static [IrStatement] {
    [exception("ldnt1d")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_STREAM];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnt1h() -> &'static [IrStatement] {
    [exception("ldnt1h")].into()
}

/// # Pseudocode
/// ```text
/// CheckSVEEnabled();
/// integer elements = VL DIV esize;
/// bits(64) base;
/// bits(64) addr;
/// bits(PL) mask = P[g];
/// bits(VL) result;
/// constant integer mbytes = esize DIV 8;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     if HaveMTEExt() then SetTagCheckedInstruction(FALSE);
///     base = SP[];
/// else
///     if HaveMTEExt() then SetTagCheckedInstruction(TRUE);
///     base = X[n];
/// 
/// addr = base + offset * elements * mbytes;
/// for e = 0 to elements-1
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = Mem[addr, mbytes, AccType_STREAM];
///     else
///         Elem[result, e, esize] = Zeros();
///     addr = addr + mbytes;
/// 
/// Z[t] = result;
/// ```
#[box_to_static_reference]
pub(super) fn ldnt1w() -> &'static [IrStatement] {
    [exception("ldnt1w")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// integer t2 = UInt(Rt2);
/// AccType acctype = AccType_VEC;
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// if opc == '11' then UNDEFINED;
/// integer scale = 2 + UInt(opc);
/// integer datasize = 8 << scale;
/// bits(64) offset = LSL(SignExtend(imm7, 64), scale);
/// boolean tag_checked = wback || n != 31;CheckFPAdvSIMDEnabled64();
/// 
/// bits(64) address;
/// bits(datasize) data1;
/// bits(datasize) data2;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// if ! postindex then
///     address = address + offset;
/// 
/// case memop of
///     when MemOp_STORE
///         data1 = V[t];
///         data2 = V[t2];
///         Mem[address + 0     , dbytes, acctype] = data1;
///         Mem[address + dbytes, dbytes, acctype] = data2;
/// 
///     when MemOp_LOAD
///         data1 = Mem[address + 0     , dbytes, acctype];
///         data2 = Mem[address + dbytes, dbytes, acctype];
///         if rt_unknown then
///             data1 = bits(datasize) UNKNOWN;
///             data2 = bits(datasize) UNKNOWN;
///         V[t]  = data1;
///         V[t2] = data2;
/// 
/// if wback then
///     if postindex then
///         address = address + offset;
///     if n == 31 then
///         SP[] = address;
///     else
///         X[n] = address;
/// ```
#[box_to_static_reference]
pub(super) fn ldp() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// integer t2 = UInt(Rt2);
/// AccType acctype = AccType_NORMAL;
/// MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
/// if L:opc<0> == '01' || opc == '11' then UNDEFINED;
/// boolean signed = (opc<0> != '0');
/// integer scale = 2 + UInt(opc<1>);
/// integer datasize = 8 << scale;
/// bits(64) offset = LSL(SignExtend(imm7, 64), scale);
/// boolean tag_checked = wback || n != 31;bits(64) address;
/// bits(datasize) data1;
/// bits(datasize) data2;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// boolean wb_unknown = FALSE;
/// 
/// if memop == MemOp_LOAD && wback && (t == n || t2 == n) && n != 31 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_WBOVERLAPLD);
///     assert c IN {Constraint_WBSUPPRESS, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_WBSUPPRESS wback = FALSE;        // writeback is suppressed
///         when Constraint_UNKNOWN    wb_unknown = TRUE;    // writeback is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE && wback && (t == n || t2 == n) && n != 31 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_WBOVERLAPST);
///     assert c IN {Constraint_NONE, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_NONE       rt_unknown = FALSE;   // value stored is pre-writeback
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // value stored is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_LOAD && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// if ! postindex then
///     address = address + offset;
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown && t == n then
///             data1 = bits(datasize) UNKNOWN;
///         else
///             data1 = X[t];
///         if rt_unknown && t2 == n then
///             data2 = bits(datasize) UNKNOWN;
///         else
///             data2 = X[t2];
///         Mem[address + 0     , dbytes, acctype] = data1;
///         Mem[address + dbytes, dbytes, acctype] = data2;
/// 
///     when MemOp_LOAD
///         data1 = Mem[address + 0     , dbytes, acctype];
///         data2 = Mem[address + dbytes, dbytes, acctype];
///         if rt_unknown then
///             data1 = bits(datasize) UNKNOWN;
///             data2 = bits(datasize) UNKNOWN;
///         if signed then
///             X[t]  = SignExtend(data1, 64);
///             X[t2] = SignExtend(data2, 64);
///         else
///             X[t]  = data1;
///             X[t2] = data2;
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
pub(super) fn ldpsw() -> &'static [IrStatement] {
    [exception("ldpsw")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_VEC;
/// MemOp memop = if opc<0> == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = 8 << scale;
/// boolean tag_checked = memop != MemOp_PREFETCH && (wback || n != 31);if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// CheckFPAdvSIMDEnabled64();
/// bits(64) address;
/// bits(datasize) data;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// if ! postindex then
///     address = address + offset;
/// 
/// case memop of
///     when MemOp_STORE
///         data = V[t];
///         Mem[address, datasize DIV 8, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, datasize DIV 8, acctype];
///         V[t] = data;
/// 
/// if wback then
///     if postindex then
///         address = address + offset;
///     if n == 31 then
///         SP[] = address;
///     else
///         X[n] = address;
/// ```
#[box_to_static_reference]
pub(super) fn ldr() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(64) data;
/// boolean wb_unknown = FALSE;
/// boolean auth_then_branch = TRUE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if wback && n == t && n != 31 then
///     c = ConstrainUnpredictable(Unpredictable_WBOVERLAPLD);
///     assert c IN {Constraint_WBSUPPRESS, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_WBSUPPRESS wback = FALSE;       // writeback is suppressed
///         when Constraint_UNKNOWN    wb_unknown = TRUE;   // writeback is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     address = SP[];
/// else
///     address = X[n];
/// 
/// if use_key_a then
///     address = AuthDA(address, X[31], auth_then_branch);
/// else
///     address = AuthDB(address, X[31], auth_then_branch);
/// 
/// if n == 31 then
///     CheckSPAlignment();
/// 
/// address = address + offset;
/// data = Mem[address, 8, AccType_NORMAL];
/// X[t] = data;
/// 
/// if wback then
///     if wb_unknown then
///         address = bits(64) UNKNOWN;
///     if n == 31 then
///         SP[] = address;
///     else
///         X[n] = address;
/// ```
#[box_to_static_reference]
pub(super) fn ldraa() -> &'static [IrStatement] {
    [exception("ldraa")].into()
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
///         UNDEFINED;
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
pub(super) fn ldrb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///         UNDEFINED;
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
pub(super) fn ldrh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
///         UNDEFINED;
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
pub(super) fn ldrsb() -> &'static [IrStatement] {
    let assignment = sign_extend(o2(), o1(), o1_size());
    [assignment].into()
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
///         UNDEFINED;
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
pub(super) fn ldrsh() -> &'static [IrStatement] {
    let assignment = sign_extend(o2(), o1(), o1_size());
    [assignment].into()
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
///         UNDEFINED;
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
pub(super) fn ldrsw() -> &'static [IrStatement] {
    let assignment = sign_extend(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldset() -> &'static [IrStatement] {
    [exception("ldset")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldsetb() -> &'static [IrStatement] {
    [exception("ldsetb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldseth() -> &'static [IrStatement] {
    [exception("ldseth")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldsmax() -> &'static [IrStatement] {
    [exception("ldsmax")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldsmaxb() -> &'static [IrStatement] {
    [exception("ldsmaxb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldsmaxh() -> &'static [IrStatement] {
    [exception("ldsmaxh")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldsmin() -> &'static [IrStatement] {
    [exception("ldsmin")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldsminb() -> &'static [IrStatement] {
    [exception("ldsminb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldsminh() -> &'static [IrStatement] {
    [exception("ldsminh")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// 
/// unpriv_at_el1 = PSTATE.EL == EL1 && !(EL2Enabled() && HaveNVExt() && HCR_EL2.<NV,NV1> == '11');
/// unpriv_at_el2 = PSTATE.EL == EL2 && HaveVirtHostExt() && HCR_EL2.<E2H,TGE> == '11';
/// 
/// user_access_override = HaveUAOExt() && PSTATE.UAO == '1';
/// if !user_access_override && (unpriv_at_el1 || unpriv_at_el2) then
///     acctype = AccType_UNPRIV;
/// else
///     acctype = AccType_NORMAL;
/// 
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
///         UNDEFINED;
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
pub(super) fn ldtr() -> &'static [IrStatement] {
    [exception("ldtr")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// 
/// unpriv_at_el1 = PSTATE.EL == EL1 && !(EL2Enabled() && HaveNVExt() && HCR_EL2.<NV,NV1> == '11');
/// unpriv_at_el2 = PSTATE.EL == EL2 && HaveVirtHostExt() && HCR_EL2.<E2H,TGE> == '11';
/// 
/// user_access_override = HaveUAOExt() && PSTATE.UAO == '1';
/// if !user_access_override && (unpriv_at_el1 || unpriv_at_el2) then
///     acctype = AccType_UNPRIV;
/// else
///     acctype = AccType_NORMAL;
/// 
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
///         UNDEFINED;
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
pub(super) fn ldtrb() -> &'static [IrStatement] {
    [exception("ldtrb")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// 
/// unpriv_at_el1 = PSTATE.EL == EL1 && !(EL2Enabled() && HaveNVExt() && HCR_EL2.<NV,NV1> == '11');
/// unpriv_at_el2 = PSTATE.EL == EL2 && HaveVirtHostExt() && HCR_EL2.<E2H,TGE> == '11';
/// 
/// user_access_override = HaveUAOExt() && PSTATE.UAO == '1';
/// if !user_access_override && (unpriv_at_el1 || unpriv_at_el2) then
///     acctype = AccType_UNPRIV;
/// else
///     acctype = AccType_NORMAL;
/// 
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
///         UNDEFINED;
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
pub(super) fn ldtrh() -> &'static [IrStatement] {
    [exception("ldtrh")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// 
/// unpriv_at_el1 = PSTATE.EL == EL1 && !(EL2Enabled() && HaveNVExt() && HCR_EL2.<NV,NV1> == '11');
/// unpriv_at_el2 = PSTATE.EL == EL2 && HaveVirtHostExt() && HCR_EL2.<E2H,TGE> == '11';
/// 
/// user_access_override = HaveUAOExt() && PSTATE.UAO == '1';
/// if !user_access_override && (unpriv_at_el1 || unpriv_at_el2) then
///     acctype = AccType_UNPRIV;
/// else
///     acctype = AccType_NORMAL;
/// 
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
///         UNDEFINED;
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
pub(super) fn ldtrsb() -> &'static [IrStatement] {
    [exception("ldtrsb")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// 
/// unpriv_at_el1 = PSTATE.EL == EL1 && !(EL2Enabled() && HaveNVExt() && HCR_EL2.<NV,NV1> == '11');
/// unpriv_at_el2 = PSTATE.EL == EL2 && HaveVirtHostExt() && HCR_EL2.<E2H,TGE> == '11';
/// 
/// user_access_override = HaveUAOExt() && PSTATE.UAO == '1';
/// if !user_access_override && (unpriv_at_el1 || unpriv_at_el2) then
///     acctype = AccType_UNPRIV;
/// else
///     acctype = AccType_NORMAL;
/// 
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
///         UNDEFINED;
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
pub(super) fn ldtrsh() -> &'static [IrStatement] {
    [exception("ldtrsh")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// 
/// unpriv_at_el1 = PSTATE.EL == EL1 && !(EL2Enabled() && HaveNVExt() && HCR_EL2.<NV,NV1> == '11');
/// unpriv_at_el2 = PSTATE.EL == EL2 && HaveVirtHostExt() && HCR_EL2.<E2H,TGE> == '11';
/// 
/// user_access_override = HaveUAOExt() && PSTATE.UAO == '1';
/// if !user_access_override && (unpriv_at_el1 || unpriv_at_el2) then
///     acctype = AccType_UNPRIV;
/// else
///     acctype = AccType_NORMAL;
/// 
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
///         UNDEFINED;
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
pub(super) fn ldtrsw() -> &'static [IrStatement] {
    [exception("ldtrsw")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldumax() -> &'static [IrStatement] {
    [exception("ldumax")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldumaxb() -> &'static [IrStatement] {
    [exception("ldumaxb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldumaxh() -> &'static [IrStatement] {
    [exception("ldumaxh")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldumin() -> &'static [IrStatement] {
    [exception("ldumin")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn lduminb() -> &'static [IrStatement] {
    [exception("lduminb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) value;
/// bits(datasize) data;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// value = X[s];
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// data = MemAtomic(address, op, value, ldacctype, stacctype);
/// 
/// if t != 31 then
///     X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn lduminh() -> &'static [IrStatement] {
    [exception("lduminh")].into()
}

/// # Pseudocode
/// ```text
/// integer n = UInt(Rn);
/// integer t = UInt(Rt);
/// AccType acctype = AccType_VEC;
/// MemOp memop = if opc<0> == '1' then MemOp_LOAD else MemOp_STORE;
/// integer datasize = 8 << scale;
/// boolean tag_checked = memop != MemOp_PREFETCH && (wback || n != 31);if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// CheckFPAdvSIMDEnabled64();
/// bits(64) address;
/// bits(datasize) data;
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// else
///     address = X[n];
/// 
/// if ! postindex then
///     address = address + offset;
/// 
/// case memop of
///     when MemOp_STORE
///         data = V[t];
///         Mem[address, datasize DIV 8, acctype] = data;
/// 
///     when MemOp_LOAD
///         data = Mem[address, datasize DIV 8, acctype];
///         V[t] = data;
/// 
/// if wback then
///     if postindex then
///         address = address + offset;
///     if n == 31 then
///         SP[] = address;
///     else
///         X[n] = address;
/// ```
#[box_to_static_reference]
pub(super) fn ldur() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
pub(super) fn ldurb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
pub(super) fn ldurh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
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
pub(super) fn ldursb() -> &'static [IrStatement] {
    let assignment = sign_extend(o2(), o1(), o1_size());
    [assignment].into()
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
pub(super) fn ldursh() -> &'static [IrStatement] {
    let assignment = sign_extend(o2(), o1(), o1_size());
    [assignment].into()
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
pub(super) fn ldursw() -> &'static [IrStatement] {
    let assignment = sign_extend(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// boolean rn_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && pair && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE then
///     if s == t || (pair && s == t2) then
///         Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
///             when Constraint_NONE       rt_unknown = FALSE;   // store original value
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
///     if s == n && n != 31 then
///         Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
///             when Constraint_NONE       rn_unknown = FALSE;   // address is original base
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// elsif rn_unknown then
///     address = bits(64) UNKNOWN;
/// else
///     address = X[n];
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         elsif pair then
///             bits(datasize DIV 2) el1 = X[t];
///             bits(datasize DIV 2) el2 = X[t2];
///             data = if BigEndian() then el1 : el2 else el2 : el1;
///         else
///             data = X[t];
/// 
///         bit status = '1';
///         // Check whether the Exclusives monitors are set to include the
///         // physical memory locations corresponding to virtual address
///         // range [address, address+dbytes-1].
///         if AArch64.ExclusiveMonitorsPass(address, dbytes) then
///             // This atomic write will be rejected if it does not refer
///             // to the same physical locations after address translation.
///             Mem[address, dbytes, acctype] = data;
///             status = ExclusiveMonitorsStatus();
///         X[s] = ZeroExtend(status, 32);
/// 
///     when MemOp_LOAD
///         // Tell the Exclusives monitors to record a sequence of one or more atomic
///         // memory reads from virtual address range [address, address+dbytes-1].
///         // The Exclusives monitor will only be set if all the reads are from the
///         // same dbytes-aligned physical address, to allow for the possibility of
///         // an atomicity break if the translation is changed between reads.
///         AArch64.SetExclusiveMonitors(address, dbytes);
/// 
///         if pair then
///             if rt_unknown then
///                 // ConstrainedUNPREDICTABLE case
///                 X[t]  = bits(datasize) UNKNOWN;        // In this case t = t2
///             elsif elsize == 32 then
///                 // 32-bit load exclusive pair (atomic)
///                 data = Mem[address, dbytes, acctype];
///                 if BigEndian() then
///                     X[t]  = data<datasize-1:elsize>;
///                     X[t2] = data<elsize-1:0>;
///                 else
///                     X[t]  = data<elsize-1:0>;
///                     X[t2] = data<datasize-1:elsize>;
///             else // elsize == 64
///                 // 64-bit load exclusive pair (not atomic),
///                 // but must be 128-bit aligned
///                 if address != Align(address, dbytes) then
///                     iswrite = FALSE;
///                     secondstage = FALSE;
///                     AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
///                 X[t]  = Mem[address + 0, 8, acctype];
///                 X[t2] = Mem[address + 8, 8, acctype];
///         else
///             data = Mem[address, dbytes, acctype];
///             X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldxp() -> &'static [IrStatement] {
    [exception("ldxp")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// boolean rn_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && pair && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE then
///     if s == t || (pair && s == t2) then
///         Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
///             when Constraint_NONE       rt_unknown = FALSE;   // store original value
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
///     if s == n && n != 31 then
///         Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
///             when Constraint_NONE       rn_unknown = FALSE;   // address is original base
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// elsif rn_unknown then
///     address = bits(64) UNKNOWN;
/// else
///     address = X[n];
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         elsif pair then
///             bits(datasize DIV 2) el1 = X[t];
///             bits(datasize DIV 2) el2 = X[t2];
///             data = if BigEndian() then el1 : el2 else el2 : el1;
///         else
///             data = X[t];
/// 
///         bit status = '1';
///         // Check whether the Exclusives monitors are set to include the
///         // physical memory locations corresponding to virtual address
///         // range [address, address+dbytes-1].
///         if AArch64.ExclusiveMonitorsPass(address, dbytes) then
///             // This atomic write will be rejected if it does not refer
///             // to the same physical locations after address translation.
///             Mem[address, dbytes, acctype] = data;
///             status = ExclusiveMonitorsStatus();
///         X[s] = ZeroExtend(status, 32);
/// 
///     when MemOp_LOAD
///         // Tell the Exclusives monitors to record a sequence of one or more atomic
///         // memory reads from virtual address range [address, address+dbytes-1].
///         // The Exclusives monitor will only be set if all the reads are from the
///         // same dbytes-aligned physical address, to allow for the possibility of
///         // an atomicity break if the translation is changed between reads.
///         AArch64.SetExclusiveMonitors(address, dbytes);
/// 
///         if pair then
///             if rt_unknown then
///                 // ConstrainedUNPREDICTABLE case
///                 X[t]  = bits(datasize) UNKNOWN;        // In this case t = t2
///             elsif elsize == 32 then
///                 // 32-bit load exclusive pair (atomic)
///                 data = Mem[address, dbytes, acctype];
///                 if BigEndian() then
///                     X[t]  = data<datasize-1:elsize>;
///                     X[t2] = data<elsize-1:0>;
///                 else
///                     X[t]  = data<elsize-1:0>;
///                     X[t2] = data<datasize-1:elsize>;
///             else // elsize == 64
///                 // 64-bit load exclusive pair (not atomic),
///                 // but must be 128-bit aligned
///                 if address != Align(address, dbytes) then
///                     iswrite = FALSE;
///                     secondstage = FALSE;
///                     AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
///                 X[t]  = Mem[address + 0, 8, acctype];
///                 X[t2] = Mem[address + 8, 8, acctype];
///         else
///             data = Mem[address, dbytes, acctype];
///             X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldxr() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// boolean rn_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && pair && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE then
///     if s == t || (pair && s == t2) then
///         Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
///             when Constraint_NONE       rt_unknown = FALSE;   // store original value
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
///     if s == n && n != 31 then
///         Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
///             when Constraint_NONE       rn_unknown = FALSE;   // address is original base
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// elsif rn_unknown then
///     address = bits(64) UNKNOWN;
/// else
///     address = X[n];
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         elsif pair then
///             bits(datasize DIV 2) el1 = X[t];
///             bits(datasize DIV 2) el2 = X[t2];
///             data = if BigEndian() then el1 : el2 else el2 : el1;
///         else
///             data = X[t];
/// 
///         bit status = '1';
///         // Check whether the Exclusives monitors are set to include the
///         // physical memory locations corresponding to virtual address
///         // range [address, address+dbytes-1].
///         if AArch64.ExclusiveMonitorsPass(address, dbytes) then
///             // This atomic write will be rejected if it does not refer
///             // to the same physical locations after address translation.
///             Mem[address, dbytes, acctype] = data;
///             status = ExclusiveMonitorsStatus();
///         X[s] = ZeroExtend(status, 32);
/// 
///     when MemOp_LOAD
///         // Tell the Exclusives monitors to record a sequence of one or more atomic
///         // memory reads from virtual address range [address, address+dbytes-1].
///         // The Exclusives monitor will only be set if all the reads are from the
///         // same dbytes-aligned physical address, to allow for the possibility of
///         // an atomicity break if the translation is changed between reads.
///         AArch64.SetExclusiveMonitors(address, dbytes);
/// 
///         if pair then
///             if rt_unknown then
///                 // ConstrainedUNPREDICTABLE case
///                 X[t]  = bits(datasize) UNKNOWN;        // In this case t = t2
///             elsif elsize == 32 then
///                 // 32-bit load exclusive pair (atomic)
///                 data = Mem[address, dbytes, acctype];
///                 if BigEndian() then
///                     X[t]  = data<datasize-1:elsize>;
///                     X[t2] = data<elsize-1:0>;
///                 else
///                     X[t]  = data<elsize-1:0>;
///                     X[t2] = data<datasize-1:elsize>;
///             else // elsize == 64
///                 // 64-bit load exclusive pair (not atomic),
///                 // but must be 128-bit aligned
///                 if address != Align(address, dbytes) then
///                     iswrite = FALSE;
///                     secondstage = FALSE;
///                     AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
///                 X[t]  = Mem[address + 0, 8, acctype];
///                 X[t2] = Mem[address + 8, 8, acctype];
///         else
///             data = Mem[address, dbytes, acctype];
///             X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldxrb() -> &'static [IrStatement] {
    [exception("ldxrb")].into()
}

/// # Pseudocode
/// ```text
/// bits(64) address;
/// bits(datasize) data;
/// constant integer dbytes = datasize DIV 8;
/// boolean rt_unknown = FALSE;
/// boolean rn_unknown = FALSE;
/// 
/// if HaveMTEExt() then
///     SetTagCheckedInstruction(tag_checked);
/// 
/// if memop == MemOp_LOAD && pair && t == t2 then
///     Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
///     assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
///     case c of
///         when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
///         when Constraint_UNDEF      UNDEFINED;
///         when Constraint_NOP        EndOfInstruction();
/// 
/// if memop == MemOp_STORE then
///     if s == t || (pair && s == t2) then
///         Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
///             when Constraint_NONE       rt_unknown = FALSE;   // store original value
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
///     if s == n && n != 31 then
///         Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
///         assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
///         case c of
///             when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
///             when Constraint_NONE       rn_unknown = FALSE;   // address is original base
///             when Constraint_UNDEF      UNDEFINED;
///             when Constraint_NOP        EndOfInstruction();
/// 
/// if n == 31 then
///     CheckSPAlignment();
///     address = SP[];
/// elsif rn_unknown then
///     address = bits(64) UNKNOWN;
/// else
///     address = X[n];
/// 
/// case memop of
///     when MemOp_STORE
///         if rt_unknown then
///             data = bits(datasize) UNKNOWN;
///         elsif pair then
///             bits(datasize DIV 2) el1 = X[t];
///             bits(datasize DIV 2) el2 = X[t2];
///             data = if BigEndian() then el1 : el2 else el2 : el1;
///         else
///             data = X[t];
/// 
///         bit status = '1';
///         // Check whether the Exclusives monitors are set to include the
///         // physical memory locations corresponding to virtual address
///         // range [address, address+dbytes-1].
///         if AArch64.ExclusiveMonitorsPass(address, dbytes) then
///             // This atomic write will be rejected if it does not refer
///             // to the same physical locations after address translation.
///             Mem[address, dbytes, acctype] = data;
///             status = ExclusiveMonitorsStatus();
///         X[s] = ZeroExtend(status, 32);
/// 
///     when MemOp_LOAD
///         // Tell the Exclusives monitors to record a sequence of one or more atomic
///         // memory reads from virtual address range [address, address+dbytes-1].
///         // The Exclusives monitor will only be set if all the reads are from the
///         // same dbytes-aligned physical address, to allow for the possibility of
///         // an atomicity break if the translation is changed between reads.
///         AArch64.SetExclusiveMonitors(address, dbytes);
/// 
///         if pair then
///             if rt_unknown then
///                 // ConstrainedUNPREDICTABLE case
///                 X[t]  = bits(datasize) UNKNOWN;        // In this case t = t2
///             elsif elsize == 32 then
///                 // 32-bit load exclusive pair (atomic)
///                 data = Mem[address, dbytes, acctype];
///                 if BigEndian() then
///                     X[t]  = data<datasize-1:elsize>;
///                     X[t2] = data<elsize-1:0>;
///                 else
///                     X[t]  = data<elsize-1:0>;
///                     X[t2] = data<datasize-1:elsize>;
///             else // elsize == 64
///                 // 64-bit load exclusive pair (not atomic),
///                 // but must be 128-bit aligned
///                 if address != Align(address, dbytes) then
///                     iswrite = FALSE;
///                     secondstage = FALSE;
///                     AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
///                 X[t]  = Mem[address + 0, 8, acctype];
///                 X[t2] = Mem[address + 8, 8, acctype];
///         else
///             data = Mem[address, dbytes, acctype];
///             X[t] = ZeroExtend(data, regsize);
/// ```
#[box_to_static_reference]
pub(super) fn ldxrh() -> &'static [IrStatement] {
    [exception("ldxrh")].into()
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
///         Elem[result, e, esize] = LSL(element1, shift);
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn lsl() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
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
///     integer shift = Min(UInt(element1), esize);
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = LSL(element2, shift);
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn lslr() -> &'static [IrStatement] {
    [exception("lslr")].into()
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
pub(super) fn lslv() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
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
///         Elem[result, e, esize] = LSR(element1, shift);
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn lsr() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
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
///     integer shift = Min(UInt(element1), esize);
///     if ElemP[mask, e, esize] == '1' then
///         Elem[result, e, esize] = LSR(element2, shift);
///     else
///         Elem[result, e, esize] = Elem[operand1, e, esize];
/// 
/// Z[dn] = result;
/// ```
#[box_to_static_reference]
pub(super) fn lsrr() -> &'static [IrStatement] {
    [exception("lsrr")].into()
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
pub(super) fn lsrv() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}
