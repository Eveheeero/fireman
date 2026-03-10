use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// START := SRC2[7:0];
/// LEN := SRC2[15:8];
/// TEMP := ZERO_EXTEND_TO_512 (SRC1 );
/// DEST := ZERO_EXTEND(TEMP[START+LEN -1: START]);
/// ZF := (DEST = 0);
/// ```
#[box_to_static_reference]
pub(super) fn bextr() -> &'static [IrStatement] {
    let start = b::and(o3(), c(0xFF));
    let len = b::and(b::shr(o3(), c(8)), c(0xFF));
    let shifted = b::shr(o2(), start);
    let mask = b::sub(b::shl(c(1), len), c(1));
    let op = b::and(shifted, mask);
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BLENDPD (128-bit Legacy SSE Version)
/// IF (IMM8[0] = 0)THEN DEST[63:0] := DEST[63:0]
///     ELSE DEST [63:0] := SRC[63:0] FI
/// IF (IMM8[1] = 0) THEN DEST[127:64] := DEST[127:64]
///     ELSE DEST [127:64] := SRC[127:64] FI
/// DEST[MAXVL-1:128] (Unmodified)
/// VBLENDPD (VEX.128 Encoded Version)
/// IF (IMM8[0] = 0)THEN DEST[63:0] := SRC1[63:0]
///     ELSE DEST [63:0] := SRC2[63:0] FI
/// IF (IMM8[1] = 0) THEN DEST[127:64] := SRC1[127:64]
///     ELSE DEST [127:64] := SRC2[127:64] FI
/// DEST[MAXVL-1:128] := 0
/// VBLENDPD (VEX.256 Encoded Version)
/// IF (IMM8[0] = 0)THEN DEST[63:0] := SRC1[63:0]
///     ELSE DEST [63:0] := SRC2[63:0] FI
/// IF (IMM8[1] = 0) THEN DEST[127:64] := SRC1[127:64]
///     ELSE DEST [127:64] := SRC2[127:64] FI
/// IF (IMM8[2] = 0) THEN DEST[191:128] := SRC1[191:128]
///     ELSE DEST [191:128] := SRC2[191:128] FI
/// IF (IMM8[3] = 0) THEN DEST[255:192] := SRC1[255:192]
///     ELSE DEST [255:192] := SRC2[255:192] FI
/// ```
#[box_to_static_reference]
pub(super) fn blendpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BLENDPS (128-bit Legacy SSE Version)
/// IF (IMM8[0] = 0) THEN DEST[31:0] :=DEST[31:0]
///     ELSE DEST [31:0] := SRC[31:0] FI
/// IF (IMM8[1] = 0) THEN DEST[63:32] := DEST[63:32]
///     ELSE DEST [63:32] := SRC[63:32] FI
/// IF (IMM8[2] = 0) THEN DEST[95:64] := DEST[95:64]
///     ELSE DEST [95:64] := SRC[95:64] FI
/// IF (IMM8[3] = 0) THEN DEST[127:96] := DEST[127:96]
///     ELSE DEST [127:96] := SRC[127:96] FI
/// DEST[MAXVL-1:128] (Unmodified)
/// VBLENDPS (VEX.128 Encoded Version)
/// IF (IMM8[0] = 0) THEN DEST[31:0] :=SRC1[31:0]
///     ELSE DEST [31:0] := SRC2[31:0] FI
/// IF (IMM8[1] = 0) THEN DEST[63:32] := SRC1[63:32]
///     ELSE DEST [63:32] := SRC2[63:32] FI
/// IF (IMM8[2] = 0) THEN DEST[95:64] := SRC1[95:64]
///     ELSE DEST [95:64] := SRC2[95:64] FI
/// IF (IMM8[3] = 0) THEN DEST[127:96] := SRC1[127:96]
///     ELSE DEST [127:96] := SRC2[127:96] FI
/// DEST[MAXVL-1:128] := 0
/// VBLENDPS (VEX.256 Encoded Version)
/// IF (IMM8[0] = 0) THEN DEST[31:0] :=SRC1[31:0]
///     ELSE DEST [31:0] := SRC2[31:0] FI
/// IF (IMM8[1] = 0) THEN DEST[63:32] := SRC1[63:32]
///     ELSE DEST [63:32] := SRC2[63:32] FI
/// IF (IMM8[2] = 0) THEN DEST[95:64] := SRC1[95:64]
///     ELSE DEST [95:64] := SRC2[95:64] FI
/// IF (IMM8[3] = 0) THEN DEST[127:96] := SRC1[127:96]
///     ELSE DEST [127:96] := SRC2[127:96] FI
/// IF (IMM8[4] = 0) THEN DEST[159:128] := SRC1[159:128]
///     ELSE DEST [159:128] := SRC2[159:128] FI
/// IF (IMM8[5] = 0) THEN DEST[191:160] := SRC1[191:160]
///     ELSE DEST [191:160] := SRC2[191:160] FI
/// IF (IMM8[6] = 0) THEN DEST[223:192] := SRC1[223:192]
///     ELSE DEST [223:192] := SRC2[223:192] FI
/// IF (IMM8[7] = 0) THEN DEST[255:224] := SRC1[255:224]
///     ELSE DEST [255:224] := SRC2[255:224] FI.
/// ```
#[box_to_static_reference]
pub(super) fn blendps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BLENDVPD (128-bit Legacy SSE Version)
/// MASK := XMM0
/// IF (MASK[63] = 0) THEN DEST[63:0] := DEST[63:0]
///     ELSE DEST [63:0] := SRC[63:0] FI
/// IF (MASK[127] = 0) THEN DEST[127:64] := DEST[127:64]
///     ELSE DEST [127:64] := SRC[127:64] FI
/// DEST[MAXVL-1:128] (Unmodified)
/// VBLENDVPD (VEX.128 Encoded Version)
/// MASK := SRC3
/// IF (MASK[63] = 0) THEN DEST[63:0] := SRC1[63:0]
///     ELSE DEST [63:0] := SRC2[63:0] FI
/// IF (MASK[127] = 0) THEN DEST[127:64] := SRC1[127:64]
///     ELSE DEST [127:64] := SRC2[127:64] FI
/// DEST[MAXVL-1:128] := 0
/// VBLENDVPD (VEX.256 Encoded Version)
/// MASK := SRC3
/// IF (MASK[63] = 0) THEN DEST[63:0] := SRC1[63:0]
///     ELSE DEST [63:0] := SRC2[63:0] FI
/// IF (MASK[127] = 0) THEN DEST[127:64] := SRC1[127:64]
///     ELSE DEST [127:64] := SRC2[127:64] FI
/// IF (MASK[191] = 0) THEN DEST[191:128] := SRC1[191:128]
///     ELSE DEST [191:128] := SRC2[191:128] FI
/// IF (MASK[255] = 0) THEN DEST[255:192] := SRC1[255:192]
///     ELSE DEST [255:192] := SRC2[255:192] FI
/// ```
#[box_to_static_reference]
pub(super) fn blendvpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BLENDVPS (128-bit Legacy SSE Version)
/// MASK := XMM0
/// IF (MASK[31] = 0) THEN DEST[31:0] := DEST[31:0]
///     ELSE DEST [31:0] := SRC[31:0] FI
/// IF (MASK[63] = 0) THEN DEST[63:32] := DEST[63:32]
///     ELSE DEST [63:32] := SRC[63:32] FI
/// IF (MASK[95] = 0) THEN DEST[95:64] := DEST[95:64]
///     ELSE DEST [95:64] := SRC[95:64] FI
/// IF (MASK[127] = 0) THEN DEST[127:96] := DEST[127:96]
///     ELSE DEST [127:96] := SRC[127:96] FI
/// DEST[MAXVL-1:128] (Unmodified)
/// VBLENDVPS (VEX.128 Encoded Version)
/// MASK := SRC3
/// IF (MASK[31] = 0) THEN DEST[31:0] := SRC1[31:0]
///     ELSE DEST [31:0] := SRC2[31:0] FI
/// IF (MASK[63] = 0) THEN DEST[63:32] := SRC1[63:32]
///     ELSE DEST [63:32] := SRC2[63:32] FI
/// IF (MASK[95] = 0) THEN DEST[95:64] := SRC1[95:64]
///     ELSE DEST [95:64] := SRC2[95:64] FI
/// IF (MASK[127] = 0) THEN DEST[127:96] := SRC1[127:96]
///     ELSE DEST [127:96] := SRC2[127:96] FI
/// DEST[MAXVL-1:128] := 0
/// VBLENDVPS (VEX.256 Encoded Version)
/// MASK := SRC3
/// IF (MASK[31] = 0) THEN DEST[31:0] := SRC1[31:0]
///     ELSE DEST [31:0] := SRC2[31:0] FI
/// IF (MASK[63] = 0) THEN DEST[63:32] := SRC1[63:32]
///     ELSE DEST [63:32] := SRC2[63:32] FI
/// IF (MASK[95] = 0) THEN DEST[95:64] := SRC1[95:64]
///     ELSE DEST [95:64] := SRC2[95:64] FI
/// IF (MASK[127] = 0) THEN DEST[127:96] := SRC1[127:96]
///     ELSE DEST [127:96] := SRC2[127:96] FI
/// IF (MASK[159] = 0) THEN DEST[159:128] := SRC1[159:128]
///     ELSE DEST [159:128] := SRC2[159:128] FI
/// IF (MASK[191] = 0) THEN DEST[191:160] := SRC1[191:160]
///     ELSE DEST [191:160] := SRC2[191:160] FI
/// IF (MASK[223] = 0) THEN DEST[223:192] := SRC1[223:192]
///     ELSE DEST [223:192] := SRC2[223:192] FI
/// IF (MASK[255] = 0) THEN DEST[255:224] := SRC1[255:224]
///     ELSE DEST [255:224] := SRC2[255:224] FI
/// ```
#[box_to_static_reference]
pub(super) fn blendvps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// temp := (-SRC) bitwiseAND (SRC);
/// SF := temp[OperandSize -1];
/// ZF := (temp = 0);
/// IF SRC = 0
///     CF := 0;
/// ELSE
///     CF := 1;
/// FI
/// DEST := temp;
/// ```
#[box_to_static_reference]
pub(super) fn blsi() -> &'static [IrStatement] {
    let op = b::and(u::neg(o2()), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// temp := (SRC-1) XOR (SRC) ;
/// SF := temp[OperandSize -1];
/// ZF := 0;
/// IF SRC = 0
///     CF := 1;
/// ELSE
///     CF := 0;
/// FI
/// DEST := temp;
/// ```
#[box_to_static_reference]
pub(super) fn blsmsk() -> &'static [IrStatement] {
    let op = b::xor(b::sub(o2(), c(1)), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// temp := (SRC-1) bitwiseAND ( SRC );
/// SF := temp[OperandSize -1];
/// ZF := (temp = 0);
/// IF SRC = 0
///     CF := 1;
/// ELSE
///     CF := 0;
/// FI
/// DEST := temp;
/// ```
#[box_to_static_reference]
pub(super) fn blsr() -> &'static [IrStatement] {
    let op = b::and(b::sub(o2(), c(1)), o2());
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BNDCL BND, reg
/// IF reg < BND.LB Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// BNDCL BND, mem
/// TEMP := LEA(mem);
/// IF TEMP < BND.LB Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bndcl() -> &'static [IrStatement] {
    [exception("bndcl")].into()
}

/// # Pseudocode
/// ```text
/// BNDCU BND, reg
/// IF reg > NOT(BND.UB) Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// BNDCU BND, mem
/// TEMP := LEA(mem);
/// IF TEMP > NOT(BND.UB) Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// BNDCN BND, reg
/// IF reg > BND.UB Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// BNDCN BND, mem
/// TEMP := LEA(mem);
/// IF TEMP > BND.UB Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bndcn() -> &'static [IrStatement] {
    [exception("bndcn")].into()
}

/// # Pseudocode
/// ```text
/// BNDCU BND, reg
/// IF reg > NOT(BND.UB) Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// BNDCU BND, mem
/// TEMP := LEA(mem);
/// IF TEMP > NOT(BND.UB) Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// BNDCN BND, reg
/// IF reg > BND.UB Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// BNDCN BND, mem
/// TEMP := LEA(mem);
/// IF TEMP > BND.UB Then
///     BNDSTATUS := 01H;
///     #BR;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bndcu() -> &'static [IrStatement] {
    [exception("bndcu")].into()
}

/// # Pseudocode
/// ```text
/// base := mib.SIB.base ? mib.SIB.base + Disp: 0;
/// ptr_value := mib.SIB.index ? mib.SIB.index : 0;
/// Outside 64-bit Mode
/// A_BDE[31:0] := (Zero_extend32(base[31:12] << 2) + (BNDCFG[31:12] <<12 );
/// A_BT[31:0] := LoadFrom(A_BDE );
/// IF A_BT[0] equal 0 Then
///     BNDSTATUS := A_BDE | 02H;
///     #BR;
/// FI;
/// A_BTE[31:0] := (Zero_extend32(base[11:2] << 4) + (A_BT[31:2] << 2 );
/// Temp_lb[31:0] := LoadFrom(A_BTE);
/// Temp_ub[31:0] := LoadFrom(A_BTE + 4);
/// Temp_ptr[31:0] := LoadFrom(A_BTE + 8);
/// IF Temp_ptr equal ptr_value Then
///     BND.LB := Temp_lb;
/// ELSE
///     BND.LB := 0;
///     BND.UB := 0;
/// FI;
/// In 64-bit Mode
/// A_BDE[63:0] := (Zero_extend64(base[47+MAWA:20] << 3) + (BNDCFG[63:12] <<12 );
/// A_BT[63:0] := LoadFrom(A_BDE);
/// IF A_BT[0] equal 0 Then
///     BNDSTATUS := A_BDE | 02H;
///     #BR;
/// FI;
/// A_BTE[63:0] := (Zero_extend64(base[19:3] << 5) + (A_BT[63:3] << 3 );
/// Temp_lb[63:0] := LoadFrom(A_BTE);
/// Temp_ub[63:0] := LoadFrom(A_BTE + 8);
/// Temp_ptr[63:0] := LoadFrom(A_BTE + 16);
/// IF Temp_ptr equal ptr_value Then
///     BND.LB := Temp_lb;
///     BND.UB := Temp_ub;
/// ELSE
///     BND.LB := 0;
///     BND.UB := 0;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bndldx() -> &'static [IrStatement] {
    [exception("bndldx")].into()
}

/// # Pseudocode
/// ```text
/// BND.LB := SRCMEM.base;
/// IF 64-bit mode Then
///     BND.UB := NOT(LEA.64_bits(SRCMEM));
/// ELSE
///     BND.UB := Zero_Extend.64_bits(NOT(LEA.32_bits(SRCMEM)));
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bndmk() -> &'static [IrStatement] {
    [exception("bndmk")].into()
}

/// # Pseudocode
/// ```text
/// BNDMOV register to register
/// DEST.LB := SRC.LB;
/// DEST.UB := SRC.UB;
/// BNDMOV from memory
/// IF 64-bit mode THEN
///         DEST.LB := LOAD_QWORD(SRC);
///         DEST.UB := LOAD_QWORD(SRC+8);
///     ELSE
///         DEST.LB := LOAD_DWORD_ZERO_EXT(SRC);
///         DEST.UB := LOAD_DWORD_ZERO_EXT(SRC+4);
/// FI;
/// BNDMOV to memory
/// IF 64-bit mode THEN
///         DEST[63:0] := SRC.LB;
///         DEST[127:64] := SRC.UB;
///     ELSE
///         DEST[31:0] := SRC.LB;
///         DEST[63:32] := SRC.UB;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bndmov() -> &'static [IrStatement] {
    [exception("bndmov")].into()
}

/// # Pseudocode
/// ```text
/// base := mib.SIB.base ? mib.SIB.base + Disp: 0;
/// ptr_value := mib.SIB.index ? mib.SIB.index : 0;
/// Outside 64-bit Mode
/// A_BDE[31:0] := (Zero_extend32(base[31:12] << 2) + (BNDCFG[31:12] <<12 );
/// A_BT[31:0] := LoadFrom(A_BDE);
/// IF A_BT[0] equal 0 Then
///     BNDSTATUS := A_BDE | 02H;
///     #BR;
/// FI;
/// A_DEST[31:0] := (Zero_extend32(base[11:2] << 4) + (A_BT[31:2] << 2 ); // address of Bound table entry
/// A_DEST[8][31:0] := ptr_value;
/// A_DEST[0][31:0] := BND.LB;
/// A_DEST[4][31:0] := BND.UB;
/// In 64-bit Mode
/// A_BDE[63:0] := (Zero_extend64(base[47+MAWA:20] << 3) + (BNDCFG[63:12] <<12 );
/// A_BT[63:0] := LoadFrom(A_BDE);
/// IF A_BT[0] equal 0 Then
///     BNDSTATUS := A_BDE | 02H;
///     #BR;
/// FI;
/// A_DEST[63:0] := (Zero_extend64(base[19:3] << 5) + (A_BT[63:3] << 3 ); // address of Bound table entry
/// A_DEST[16][63:0] := ptr_value;
/// A_DEST[0][63:0] := BND.LB;
/// A_DEST[8][63:0] := BND.UB;
/// ```
#[box_to_static_reference]
pub(super) fn bndstx() -> &'static [IrStatement] {
    [exception("bndstx")].into()
}

/// # Pseudocode
/// ```text
/// IF 64bit Mode
///     THEN
///         #UD;
///     ELSE
///         IF (ArrayIndex < LowerBound OR ArrayIndex > UpperBound) THEN
///         (* Below lower bound or above upper bound *)
///             IF <equation for PL enabled> THEN BNDSTATUS := 0
///             #BR;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bound() -> &'static [IrStatement] {
    [exception("bound")].into()
}

/// # Pseudocode
/// ```text
/// IF SRC = 0
///     THEN
///         ZF := 1;
///         DEST is undefined;
///     ELSE
///         ZF := 0;
///         temp := 0;
///         WHILE Bit(SRC, temp) = 0
///         DO
///             temp := temp + 1;
///         OD;
///         DEST := temp;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bsf() -> &'static [IrStatement] {
    let set_zf = condition(b::equal(o2(), c(0), o2_size()), [assign(c(1), zf.clone(), size_relative(zf.clone()))], [assign(c(0), zf.clone(), size_relative(zf.clone())), assign(o2(), o1(), o1_size())]);
    [set_zf].into()
}

/// # Pseudocode
/// ```text
/// IF SRC = 0
///     THEN
///         ZF := 1;
///         DEST is undefined;
///     ELSE
///         ZF := 0;
///         temp := OperandSize - 1;
///         WHILE Bit(SRC, temp) = 0
///         DO
///             temp := temp - 1;
///         OD;
///         DEST := temp;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bsr() -> &'static [IrStatement] {
    let set_zf = condition(b::equal(o2(), c(0), o2_size()), [assign(c(1), zf.clone(), size_relative(zf.clone()))], [assign(c(0), zf.clone(), size_relative(zf.clone())), assign(o2(), o1(), o1_size())]);
    [set_zf].into()
}

/// # Pseudocode
/// ```text
/// TEMP := DEST
/// IF 64-bit mode AND OperandSize = 64
///     THEN
///         DEST[7:0] := TEMP[63:56];
///         DEST[15:8] := TEMP[55:48];
///         DEST[23:16] := TEMP[47:40];
///         DEST[31:24] := TEMP[39:32];
///         DEST[39:32] := TEMP[31:24];
///         DEST[47:40] := TEMP[23:16];
///         DEST[55:48] := TEMP[15:8];
///         DEST[63:56] := TEMP[7:0];
///     ELSE
///         DEST[7:0] := TEMP[31:24];
///         DEST[15:8] := TEMP[23:16];
///         DEST[23:16] := TEMP[15:8];
///         DEST[31:24] := TEMP[7:0];
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn bswap() -> &'static [IrStatement] {
    let size = o1_size();
    let swap_32 = [
        assign(o1(), tmp32.clone(), size.clone()),
        assign(tmp32.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp32.clone(), c(8)), tmp32.clone(), size.clone()),
        assign(tmp32.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp32.clone(), c(8)), tmp32.clone(), size.clone()),
        assign(tmp32.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp32.clone(), c(8)), tmp32.clone(), size.clone()),
        assign(tmp32.clone(), o1(), size_result_bit(c(8))),
    ];
    let swap_64 = [
        assign(o1(), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
        assign(b::shl(o1(), c(8)), o1(), size.clone()),
        assign(b::shr(tmp64.clone(), c(8)), tmp64.clone(), size.clone()),
        assign(tmp64.clone(), o1(), size_result_bit(c(8))),
    ];
    let bswap = condition(b::equal(bit_size_of_o1(), c(32), size_unlimited()), swap_32, swap_64);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    [bswap, type1].into()
}

/// # Pseudocode
/// ```text
/// CF := Bit(BitBase, BitOffset);
/// ```
#[box_to_static_reference]
pub(super) fn bt() -> &'static [IrStatement] {
    let size = size_relative(cf.clone());
    let shr = b::shr(o1(), o2());
    let assignment = assign(shr.clone(), cf.clone(), &size);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    extend_undefined_flags(&[assignment, type1, type2], &[&of, &sf, &af, &pf])
}

/// # Pseudocode
/// ```text
/// CF := Bit(BitBase, BitOffset);
/// Bit(BitBase, BitOffset) := NOT Bit(BitBase, BitOffset);
/// ```
#[box_to_static_reference]
pub(super) fn btc() -> &'static [IrStatement] {
    let shr = b::shr(o1(), o2());
    let save_cf = assign(b::and(shr, c(1)), cf.clone(), size_relative(cf.clone()));
    let mask = b::shl(c(1), o2());
    let flip_bit = assign(b::xor(o1(), mask), o1(), o1_size());
    [save_cf, flip_bit].into()
}

/// # Pseudocode
/// ```text
/// CF := Bit(BitBase, BitOffset);
/// Bit(BitBase, BitOffset) := 0;
/// ```
#[box_to_static_reference]
pub(super) fn btr() -> &'static [IrStatement] {
    let shr = b::shr(o1(), o2());
    let save_cf = assign(b::and(shr, c(1)), cf.clone(), size_relative(cf.clone()));
    let mask = u::not(b::shl(c(1), o2()));
    let clear_bit = assign(b::and(o1(), mask), o1(), o1_size());
    [save_cf, clear_bit].into()
}

/// # Pseudocode
/// ```text
/// CF := Bit(BitBase, BitOffset);
/// Bit(BitBase, BitOffset) := 1;
/// ```
#[box_to_static_reference]
pub(super) fn bts() -> &'static [IrStatement] {
    let shr = b::shr(o1(), o2());
    let save_cf = assign(b::and(shr, c(1)), cf.clone(), size_relative(cf.clone()));
    let mask = b::shl(c(1), o2());
    let set_bit = assign(b::or(o1(), mask), o1(), o1_size());
    [save_cf, set_bit].into()
}

/// # Pseudocode
/// ```text
/// N := SRC2[7:0]
/// DEST := SRC1
/// IF (N < OperandSize)
///     DEST[OperandSize-1:N] := 0
/// FI
/// IF (N > OperandSize - 1)
///     CF := 1
/// ELSE
///     CF := 0
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn bzhi() -> &'static [IrStatement] {
    let mask = b::sub(b::shl(c(1), o3()), c(1));
    let op = b::and(o2(), mask);
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}
