use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// VADDPH (EVEX Encoded Versions) When SRC2 Operand is a Register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.fp16[j] := SRC1.fp16[j] + SRC2.fp16[j]
///     ELSEIF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VADDPH (EVEX Encoded Versions) When SRC2 Operand is a Memory Source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             DEST.fp16[j] := SRC1.fp16[j] + SRC2.fp16[0]
///         ELSE:
///             DEST.fp16[j] := SRC1.fp16[j] + SRC2.fp16[j]
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vaddph() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VADDSH (EVEX Encoded Versions)
/// IF EVEX.b = 1 and SRC2 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     DEST.fp16[0] := SRC1.fp16[0] + SRC2.fp16[0]
/// ELSEIF *zeroing*:
///     DEST.fp16[0] := 0
/// // else dest.fp16[0] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vaddsh() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VALIGND (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (SRC2 *is memory*) (AND EVEX.b = 1)
///     THEN
///         FOR j := 0 TO KL-1
///                 i := j * 32
///                 src[i+31:i] := SRC2[31:0]
///         ENDFOR;
///     ELSE src := SRC2
/// FI
/// ; Concatenate sources
/// tmp[VL-1:0] := src[VL-1:0]
/// tmp[2VL-1:VL] := SRC1[VL-1:0]
/// ; Shift right doubleword elements
/// IF VL = 128
///     THEN SHIFT = imm8[1:0]
///     ELSE
///         IF VL = 256
///                 THEN SHIFT = imm8[2:0]
///                 ELSE SHIFT = imm8[3:0]
///         FI
/// FI;
/// tmp[2VL-1:0] := tmp[2VL-1:0] >> (32*SHIFT)
/// ; Apply writemask
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := tmp[i+31:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VALIGNQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256),(8, 512)
/// IF (SRC2 *is memory*) (AND EVEX.b = 1)
///     THEN
///         FOR j := 0 TO KL-1
///                 i := j * 64
///                 src[i+63:i] := SRC2[63:0]
///         ENDFOR;
///     ELSE src := SRC2
/// FI
/// ; Concatenate sources
/// tmp[VL-1:0] := src[VL-1:0]
/// tmp[2VL-1:VL] := SRC1[VL-1:0]
/// IF VL = 128
///     THEN SHIFT = imm8[0]
///     ELSE
///         IF VL = 256
///                 THEN SHIFT = imm8[1:0]
///                 ELSE SHIFT = imm8[2:0]
///         FI
/// FI;
/// tmp[2VL-1:0] := tmp[2VL-1:0] >> (64*SHIFT)
/// ; Apply writemask
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := tmp[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn valignd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VALIGND (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (SRC2 *is memory*) (AND EVEX.b = 1)
///     THEN
///         FOR j := 0 TO KL-1
///                 i := j * 32
///                 src[i+31:i] := SRC2[31:0]
///         ENDFOR;
///     ELSE src := SRC2
/// FI
/// ; Concatenate sources
/// tmp[VL-1:0] := src[VL-1:0]
/// tmp[2VL-1:VL] := SRC1[VL-1:0]
/// ; Shift right doubleword elements
/// IF VL = 128
///     THEN SHIFT = imm8[1:0]
///     ELSE
///         IF VL = 256
///                 THEN SHIFT = imm8[2:0]
///                 ELSE SHIFT = imm8[3:0]
///         FI
/// FI;
/// tmp[2VL-1:0] := tmp[2VL-1:0] >> (32*SHIFT)
/// ; Apply writemask
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := tmp[i+31:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VALIGNQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256),(8, 512)
/// IF (SRC2 *is memory*) (AND EVEX.b = 1)
///     THEN
///         FOR j := 0 TO KL-1
///                 i := j * 64
///                 src[i+63:i] := SRC2[63:0]
///         ENDFOR;
///     ELSE src := SRC2
/// FI
/// ; Concatenate sources
/// tmp[VL-1:0] := src[VL-1:0]
/// tmp[2VL-1:VL] := SRC1[VL-1:0]
/// IF VL = 128
///     THEN SHIFT = imm8[0]
///     ELSE
///         IF VL = 256
///                 THEN SHIFT = imm8[1:0]
///                 ELSE SHIFT = imm8[2:0]
///         FI
/// FI;
/// tmp[2VL-1:0] := tmp[2VL-1:0] >> (64*SHIFT)
/// ; Apply writemask
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := tmp[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn valignq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VBLENDMPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no controlmask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC2[63:0]
///                     ELSE
///                         DEST[i+63:i] := SRC2[i+63:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+63:i] := SRC1[i+63:i]
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBLENDMPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no controlmask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC2[i+31:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+31:i] := SRC1[i+31:i]
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vblendmpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VBLENDMPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no controlmask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC2[63:0]
///                     ELSE
///                         DEST[i+63:i] := SRC2[i+63:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+63:i] := SRC1[i+63:i]
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBLENDMPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no controlmask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC2[i+31:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+31:i] := SRC1[i+31:i]
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vblendmps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VBROADCASTSS (128-bit Version VEX and Legacy)
/// temp := SRC[31:0]
/// DEST[31:0] := temp
/// DEST[63:32] := temp
/// DEST[95:64] := temp
/// DEST[127:96] := temp
/// DEST[MAXVL-1:128] := 0
/// VBROADCASTSS (VEX.256 Encoded Version)
/// temp := SRC[31:0]
/// DEST[31:0] := temp
/// DEST[63:32] := temp
/// DEST[95:64] := temp
/// DEST[127:96] := temp
/// DEST[159:128] := temp
/// DEST[191:160] := temp
/// DEST[223:192] := temp
/// DEST[255:224] := temp
/// DEST[MAXVL-1:256] := 0
/// VBROADCASTSS (EVEX Encoded Versions)
/// (KL, VL) (4, 128), (8, 256),= (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[31:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTSD (VEX.256 Encoded Version)
/// temp := SRC[63:0]
/// DEST[63:0] := temp
/// DEST[127:64] := temp
/// DEST[191:128] := temp
/// DEST[255:192] := temp
/// DEST[MAXVL-1:256] := 0
/// VBROADCASTSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[63:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTF32x2 (EVEX Encoded Versions)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     n := (j mod 2) * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[n+31:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTF128 (VEX.256 Encoded Version)
/// temp := SRC[127:0]
/// DEST[127:0] := temp
/// DEST[255:128] := temp
/// DEST[MAXVL-1:256] := 0
/// VBROADCASTF32X4 (EVEX Encoded Versions)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j* 32
///     n := (j modulo 4) * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[n+31:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTF64X2 (EVEX Encoded Versions)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     n := (j modulo 2) * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[n+63:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] = 0
///                 FI
///     FI;
/// ENDFOR;
/// VBROADCASTF32X8 (EVEX.U1.512 Encoded Version)
/// FOR j := 0 TO 15
///     i := j * 32
///     n := (j modulo 8) * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[n+31:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTF64X4 (EVEX.512 Encoded Version)
/// FOR j := 0 TO 7
///     i := j * 64
///     n := (j modulo 4) * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[n+63:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vbroadcast() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE (imm8 & 0x1F) OF
/// 0: CMP_OPERATOR := EQ_OQ;
/// 1: CMP_OPERATOR := LT_OS;
/// 2: CMP_OPERATOR := LE_OS;
/// 3: CMP_OPERATOR := UNORD_Q;
/// 4: CMP_OPERATOR := NEQ_UQ;
/// 5: CMP_OPERATOR := NLT_US;
/// 6: CMP_OPERATOR := NLE_US;
/// 7: CMP_OPERATOR := ORD_Q;
/// 8: CMP_OPERATOR := EQ_UQ;
/// 9: CMP_OPERATOR := NGE_US;
/// 10: CMP_OPERATOR := NGT_US;
/// 11: CMP_OPERATOR := FALSE_OQ;
/// 12: CMP_OPERATOR := NEQ_OQ;
/// 13: CMP_OPERATOR := GE_OS;
/// 14: CMP_OPERATOR := GT_OS;
/// 15: CMP_OPERATOR := TRUE_UQ;
/// 16: CMP_OPERATOR := EQ_OS;
/// 17: CMP_OPERATOR := LT_OQ;
/// 18: CMP_OPERATOR := LE_OQ;
/// 19: CMP_OPERATOR := UNORD_S;
/// 20: CMP_OPERATOR := NEQ_US;
/// 21: CMP_OPERATOR := NLT_UQ;
/// 22: CMP_OPERATOR := NLE_UQ;
/// 24: CMP_OPERATOR := EQ_US;
/// 25: CMP_OPERATOR := NGE_UQ;
/// 26: CMP_OPERATOR := NGT_UQ;
/// 27: CMP_OPERATOR := FALSE_OS;
/// 28: CMP_OPERATOR := NEQ_OS;
/// 29: CMP_OPERATOR := GE_OQ;
/// 30: CMP_OPERATOR := GT_OQ;
/// 31: CMP_OPERATOR := TRUE_US;
/// ESAC
/// VCMPPH (EVEX Encoded Versions)
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k2[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             tsrc2 := SRC2.fp16[0]
///         ELSE:
///             tsrc2 := SRC2.fp16[j]
///         DEST.bit[j] := SRC1.fp16[j] CMP_OPERATOR tsrc2
///     ELSE
///         DEST.bit[j] := 0
/// DEST[MAXKL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcmpph() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (imm8 & 0x1F) OF
/// 0: CMP_OPERATOR := EQ_OQ;
/// 1: CMP_OPERATOR := LT_OS;
/// 2: CMP_OPERATOR := LE_OS;
/// 3: CMP_OPERATOR := UNORD_Q;
/// 4: CMP_OPERATOR := NEQ_UQ;
/// 5: CMP_OPERATOR := NLT_US;
/// 6: CMP_OPERATOR := NLE_US;
/// 7: CMP_OPERATOR := ORD_Q;
/// 8: CMP_OPERATOR := EQ_UQ;
/// 9: CMP_OPERATOR := NGE_US;
/// 10: CMP_OPERATOR := NGT_US;
/// 11: CMP_OPERATOR := FALSE_OQ;
/// 12: CMP_OPERATOR := NEQ_OQ;
/// 13: CMP_OPERATOR := GE_OS;
/// 14: CMP_OPERATOR := GT_OS;
/// 15: CMP_OPERATOR := TRUE_UQ;
/// 16: CMP_OPERATOR := EQ_OS;
/// 17: CMP_OPERATOR := LT_OQ;
/// 18: CMP_OPERATOR := LE_OQ;
/// 19: CMP_OPERATOR := UNORD_S;
/// 20: CMP_OPERATOR := NEQ_US;
/// 21: CMP_OPERATOR := NLT_UQ;
/// 22: CMP_OPERATOR := NLE_UQ;
/// 23: CMP_OPERATOR := ORD_S;
/// 24: CMP_OPERATOR := EQ_US;
/// 25: CMP_OPERATOR := NGE_UQ;
/// 26: CMP_OPERATOR := NGT_UQ;
/// 27: CMP_OPERATOR := FALSE_OS;
/// 28: CMP_OPERATOR := NEQ_OS;
/// 29: CMP_OPERATOR := GE_OQ;
/// 31: CMP_OPERATOR := TRUE_US;
/// ESAC
/// VCMPSH (EVEX Encoded Versions)
/// IF k2[0] OR *no writemask*:
///     DEST.bit[0] := SRC1.fp16[0] CMP_OPERATOR SRC2.fp16[0]
/// ELSE
///     DEST.bit[0] := 0
/// DEST[MAXKL-1:1] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcmpsh() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VCOMISH SRC1, SRC2
/// RESULT := OrderedCompare(SRC1.fp16[0],SRC2.fp16[0])
/// IF RESULT is UNORDERED:
///     ZF, PF, CF := 1, 1, 1
/// ELSE IF RESULT is GREATER_THAN:
///     ZF, PF, CF := 0, 0, 0
/// ELSE IF RESULT is LESS_THAN:
///     ZF, PF, CF := 0, 0, 1
/// ELSE: // RESULT is EQUALS
///     ZF, PF, CF := 1, 0, 0
/// OF, AF, SF := 0, 0, 0
/// ```
#[box_to_static_reference]
pub(super) fn vcomish() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VCOMPRESSPD (EVEX Encoded Versions) Store Form
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// SIZE := 64
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[k+SIZE-1:k] := SRC[i+63:i]
///                 k := k + SIZE
///     FI;
/// ENDFOR
/// VCOMPRESSPD (EVEX Encoded Versions) Reg-Reg Form
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// SIZE := 64
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[k+SIZE-1:k] := SRC[i+63:i]
///                 k := k + SIZE
///     FI;
/// ENDFOR
/// IF *merging-masking*
///             THEN *DEST[VL-1:k] remains unchanged*
///             ELSE DEST[VL-1:k] := 0
/// FI
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcompresspd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCOMPRESSPS (EVEX Encoded Versions) Store Form
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// SIZE := 32
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             DEST[k+SIZE-1:k] := SRC[i+31:i]
///             k := k + SIZE
///     FI;
/// ENDFOR;
/// VCOMPRESSPS (EVEX Encoded Versions) Reg-Reg Form
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// SIZE := 32
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             DEST[k+SIZE-1:k] := SRC[i+31:i]
///             k := k + SIZE
///     FI;
/// ENDFOR
/// IF *merging-masking*
///     THEN *DEST[VL-1:k] remains unchanged*
///     ELSE DEST[VL-1:k] := 0
/// FI
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcompressps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPCOMPRESSB store form
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.byte[k] := SRC.byte[j]
///         k := k +1
/// VPCOMPRESSB reg-reg form
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.byte[k] := SRC.byte[j]
///         k := k + 1
/// IF *merging-masking*:
///     *DEST[VL-1:k*8] remains unchanged*
///     ELSE DEST[VL-1:k*8] := 0
/// DEST[MAX_VL-1:VL] := 0
/// VPCOMPRESSW store form
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.word[k] := SRC.word[j]
///         k := k + 1
/// VPCOMPRESSW reg-reg form
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.word[k] := SRC.word[j]
///         k := k + 1
/// IF *merging-masking*:
///     *DEST[VL-1:k*16] remains unchanged*
///     ELSE DEST[VL-1:k*16] := 0
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcompressw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTDQ2PH DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 32
/// IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.dword[0]
///         ELSE
///             tsrc := SRC.dword[j]
///         DEST.fp16[j] := Convert_integer32_to_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL/2] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtdq2ph() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTNE2PS2BF16 dest, src1, src2
/// VL = (128, 256, 512)
/// KL = VL/16
/// origdest := dest
/// FOR i := 0 to KL-1:
///     IF k1[ i ] or *no writemask*:
///         IF i < KL/2:
///             IF src2 is memory and evex.b == 1:
///                 t := src2.fp32[0]
///             ELSE:
///                 t := src2.fp32[ i ]
///         ELSE:
///             t := src1.fp32[ i-KL/2]
///         // See VCVTNEPS2BF16 for definition of convert helper function
///         dest.word[i] := convert_fp32_to_bfloat16(t)
///     ELSE IF *zeroing*:
///         dest.word[ i ] := 0
///     ELSE:  // Merge masking, dest element unchanged
///         dest.word[ i ] := origdest.word[ i ]
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtne2ps2bf16() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Define convert_fp32_to_bfloat16(x):
///     IF x is zero or denormal:
///         dest[15] := x[31] // sign preserving zero (denormal go to zero)
///         dest[14:0] := 0
///     ELSE IF x is infinity:
///         dest[15:0] := x[31:16]
///     ELSE IF x is NAN:
///         dest[15:0] := x[31:16] // truncate and set MSB of the mantissa to force QNAN
///         dest[6] := 1
///     ELSE // normal number
///         LSB := x[16]
///         rounding_bias := 0x00007FFF + LSB
///         temp[31:0] := x[31:0] + rounding_bias // integer add
///         dest[15:0] := temp[31:16]
///     RETURN dest
/// VCVTNEPS2BF16 dest, src
/// VL = (128, 256, 512)
/// KL = VL/16
/// origdest := dest
/// FOR i := 0 to KL/2-1:
///     IF k1[ i ] or *no writemask*:
///         IF src is memory and evex.b == 1:
///             t := src.fp32[0]
///         ELSE:
///             t := src.fp32[ i ]
///         dest.word[i] := convert_fp32_to_bfloat16(t)
///     ELSE IF *zeroing*:
///         dest.word[ i ] := 0
///     ELSE:  // Merge masking, dest element unchanged
///         dest.word[ i ] := origdest.word[ i ]
/// DEST[MAXVL-1:VL/2] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtneps2bf16() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPD2PH DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 64
/// IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.double[0]
///         ELSE
///             tsrc := SRC.double[j]
///         DEST.fp16[j] := Convert_fp64_to_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL/4] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtpd2ph() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPD2QQ (EVEX Encoded Version) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL == 512) AND (EVEX.b == 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Double_Precision_Floating_Point_To_QuadInteger(SRC[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPD2QQ (EVEX Encoded Version) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                             Convert_Double_Precision_Floating_Point_To_QuadInteger(SRC[63:0])
///                     ELSE
///                         DEST[i+63:i] := Convert_Double_Precision_Floating_Point_To_QuadInteger(SRC[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtpd2qq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPD2UDQ (EVEX Encoded Versions) When SRC2 Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UInteger(SRC[k+63:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTPD2UDQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UInteger(SRC[63:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UInteger(SRC[k+63:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtpd2udq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPD2UQQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL == 512) AND (EVEX.b == 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UQuadInteger(SRC[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPD2UQQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UQuadInteger(SRC[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UQuadInteger(SRC[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtpd2uqq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPH2DQ DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 32
/// IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.dword[j] := Convert_fp16_to_integer32(tsrc)
///     ELSE IF *zeroing*:
///         DEST.dword[j] := 0
///     // else dest.dword[j] remains unchanged
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2dq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPH2PD DEST, SRC
/// VL = 128, 256, or 512
/// KL := VL/64
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.fp64[j] := Convert_fp16_to_fp64(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp64[j] := 0
///     // else dest.fp64[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2pd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// vCvt_h2s(SRC1[15:0])
/// {
/// RETURN Cvt_Half_Precision_To_Single_Precision(SRC1[15:0]);
/// }
/// VCVTPH2PS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 vCvt_h2s(SRC[k+15:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPH2PS (VEX.256 Encoded Version)
/// DEST[31:0] := vCvt_h2s(SRC1[15:0]);
/// DEST[63:32] := vCvt_h2s(SRC1[31:16]);
/// DEST[95:64] := vCvt_h2s(SRC1[47:32]);
/// DEST[127:96] := vCvt_h2s(SRC1[63:48]);
/// DEST[159:128] := vCvt_h2s(SRC1[79:64]);
/// DEST[191:160] := vCvt_h2s(SRC1[95:80]);
/// DEST[223:192] := vCvt_h2s(SRC1[111:96]);
/// DEST[255:224] := vCvt_h2s(SRC1[127:112]);
/// DEST[MAXVL-1:256] := 0
/// VCVTPH2PS (VEX.128 Encoded Version)
/// DEST[31:0] := vCvt_h2s(SRC1[15:0]);
/// DEST[63:32] := vCvt_h2s(SRC1[31:16]);
/// DEST[95:64] := vCvt_h2s(SRC1[47:32]);
/// DEST[127:96] := vCvt_h2s(SRC1[63:48]);
/// DEST[MAXVL-1:128] := 0
/// VCVTPH2PSX DEST, SRC
/// VL = 128, 256, or 512
/// KL := VL/32
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///                 tsrc := SRC.fp16[0]
///         ELSE
///                 tsrc := SRC.fp16[j]
///         DEST.fp32[j] := Convert_fp16_to_fp32(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp32[j] := 0
///     // else dest.fp32[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2ps() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// vCvt_h2s(SRC1[15:0])
/// {
/// RETURN Cvt_Half_Precision_To_Single_Precision(SRC1[15:0]);
/// }
/// VCVTPH2PS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 vCvt_h2s(SRC[k+15:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPH2PS (VEX.256 Encoded Version)
/// DEST[31:0] := vCvt_h2s(SRC1[15:0]);
/// DEST[63:32] := vCvt_h2s(SRC1[31:16]);
/// DEST[95:64] := vCvt_h2s(SRC1[47:32]);
/// DEST[127:96] := vCvt_h2s(SRC1[63:48]);
/// DEST[159:128] := vCvt_h2s(SRC1[79:64]);
/// DEST[191:160] := vCvt_h2s(SRC1[95:80]);
/// DEST[223:192] := vCvt_h2s(SRC1[111:96]);
/// DEST[255:224] := vCvt_h2s(SRC1[127:112]);
/// DEST[MAXVL-1:256] := 0
/// VCVTPH2PS (VEX.128 Encoded Version)
/// DEST[31:0] := vCvt_h2s(SRC1[15:0]);
/// DEST[63:32] := vCvt_h2s(SRC1[31:16]);
/// DEST[95:64] := vCvt_h2s(SRC1[47:32]);
/// DEST[127:96] := vCvt_h2s(SRC1[63:48]);
/// DEST[MAXVL-1:128] := 0
/// VCVTPH2PSX DEST, SRC
/// VL = 128, 256, or 512
/// KL := VL/32
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///                 tsrc := SRC.fp16[0]
///         ELSE
///                 tsrc := SRC.fp16[j]
///         DEST.fp32[j] := Convert_fp16_to_fp32(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp32[j] := 0
///     // else dest.fp32[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2psx() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPH2QQ DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 64
/// IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.qword[j] := Convert_fp16_to_integer64(tsrc)
///     ELSE IF *zeroing*:
///         DEST.qword[j] := 0
///     // else dest.qword[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2qq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPH2UDQ DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 32
/// IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///             DEST.dword[j] := Convert_fp16_to_unsigned_integer32(tsrc)
///     ELSE IF *zeroing*:
///         DEST.dword[j] := 0
///     // else dest.dword[j] remains unchanged
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2udq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPH2UQQ DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 64
/// IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.qword[j] := Convert_fp16_to_unsigned_integer64(tsrc)
///     ELSE IF *zeroing*:
///         DEST.qword[j] := 0
///     // else dest.qword[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2uqq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPH2UW DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 16
/// IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.word[j] := Convert_fp16_to_unsigned_integer16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.word[j] := 0
///     // else dest.word[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2uw() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPH2W DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 16
/// IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.word[j] := Convert_fp16_to_integer16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.word[j] := 0
///     // else dest.word[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtph2w() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// vCvt_s2h(SRC1[31:0])
/// {
/// IF Imm[2] = 0
/// THEN ; using Imm[1:0] for rounding control, see Table 5-3
///     RETURN Cvt_Single_Precision_To_Half_Precision_FP_Imm(SRC1[31:0]);
/// ELSE ; using MXCSR.RC for rounding control
///     RETURN Cvt_Single_Precision_To_Half_Precision_FP_Mxcsr(SRC1[31:0]);
/// FI;
/// }
/// VCVTPS2PH (EVEX Encoded Versions) When DEST is a Register
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] :=
///                 vCvt_s2h(SRC[k+31:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTPS2PH (EVEX Encoded Versions) When DEST is Memory
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] :=
///                 vCvt_s2h(SRC[k+31:k])
///         ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///     FI;
/// ENDFOR
/// VCVTPS2PH (VEX.256 Encoded Version)
/// DEST[15:0] := vCvt_s2h(SRC1[31:0]);
/// DEST[31:16] := vCvt_s2h(SRC1[63:32]);
/// DEST[47:32] := vCvt_s2h(SRC1[95:64]);
/// DEST[63:48] := vCvt_s2h(SRC1[127:96]);
/// DEST[79:64] := vCvt_s2h(SRC1[159:128]);
/// DEST[95:80] := vCvt_s2h(SRC1[191:160]);
/// DEST[111:96] := vCvt_s2h(SRC1[223:192]);
/// DEST[127:112] := vCvt_s2h(SRC1[255:224]);
/// DEST[MAXVL-1:128] := 0
/// VCVTPS2PH (VEX.128 Encoded Version)
/// DEST[15:0] := vCvt_s2h(SRC1[31:0]);
/// DEST[31:16] := vCvt_s2h(SRC1[63:32]);
/// DEST[47:32] := vCvt_s2h(SRC1[95:64]);
/// DEST[63:48] := vCvt_s2h(SRC1[127:96]);
/// DEST[MAXVL-1:64] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtps2ph() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPS2PHX DEST, SRC (AVX512_FP16 Load Version With Broadcast Support)
/// VL = 128, 256, or 512
/// KL := VL / 32
/// IF *SRC is a register* and (VL == 512) and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp32[0]
///         ELSE
///             tsrc := SRC.fp32[j]
///         DEST.fp16[j] := Convert_fp32_to_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL/2] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtps2phx() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPS2QQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL == 512) AND (EVEX.b == 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Single_Precision_To_QuadInteger(SRC[k+31:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPS2QQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_QuadInteger(SRC[31:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_QuadInteger(SRC[k+31:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtps2qq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPS2UDQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_UInteger(SRC[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPS2UDQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no *
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_UInteger(SRC[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_UInteger(SRC[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtps2udq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTPS2UQQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL == 512) AND (EVEX.b == 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Single_Precision_To_UQuadInteger(SRC[k+31:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTPS2UQQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_UQuadInteger(SRC[31:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_UQuadInteger(SRC[k+31:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtps2uqq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTQQ2PD (EVEX2 Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL == 512) AND (EVEX.b == 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_QuadInteger_To_Double_Precision_Floating_Point(SRC[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTQQ2PD (EVEX Encoded Versions) when SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_QuadInteger_To_Double_Precision_Floating_Point(SRC[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_QuadInteger_To_Double_Precision_Floating_Point(SRC[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtqq2pd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTQQ2PH DEST, SRC
/// VL = 128, 256 or 512
/// KL := VL / 64
/// IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.qword[0]
///         ELSE
///             tsrc := SRC.qword[j]
///         DEST.fp16[j] := Convert_integer64_to_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL/4] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtqq2ph() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTQQ2PS (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[k+31:k] :=
///                 Convert_QuadInteger_To_Single_Precision_Floating_Point(SRC[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[k+31:k] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[k+31:k] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTQQ2PS (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[k+31:k] :=
///                 Convert_QuadInteger_To_Single_Precision_Floating_Point(SRC[63:0])
///                     ELSE
///                         DEST[k+31:k] :=
///                 Convert_QuadInteger_To_Single_Precision_Floating_Point(SRC[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[k+31:k] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[k+31:k] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtqq2ps() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSD2SH dest, src1, src2
/// IF *SRC2 is a register* and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     DEST.fp16[0] := Convert_fp64_to_fp16(SRC2.fp64[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else dest.fp16[0] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtsd2sh() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSD2USI (EVEX Encoded Version)
/// IF (SRC *is register*) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF 64-Bit Mode and OperandSize = 64
///     THEN DEST[63:0] := Convert_Double_Precision_Floating_Point_To_UInteger(SRC[63:0]);
///     ELSEDEST[31:0] := Convert_Double_Precision_Floating_Point_To_UInteger(SRC[63:0]);
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn vcvtsd2usi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSH2SD dest, src1, src2
/// IF k1[0] OR *no writemask*:
///     DEST.fp64[0] := Convert_fp16_to_fp64(SRC2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp64[0] := 0
/// // else dest.fp64[0] remains unchanged
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtsh2sd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSH2SI dest, src
/// IF *SRC is a register* and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// IF 64-mode and OperandSize == 64:
///     DEST.qword := Convert_fp16_to_integer64(SRC.fp16[0])
/// ELSE:
///     DEST.dword := Convert_fp16_to_integer32(SRC.fp16[0])
/// ```
#[box_to_static_reference]
pub(super) fn vcvtsh2si() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSH2SS dest, src1, src2
/// IF k1[0] OR *no writemask*:
///     DEST.fp32[0] := Convert_fp16_to_fp32(SRC2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp32[0] := 0
/// // else dest.fp32[0] remains unchanged
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtsh2ss() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSH2USI dest, src
/// // SET_RM() sets the rounding mode used for this instruction.
/// IF *SRC is a register* and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// IF 64-mode and OperandSize == 64:
///     DEST.qword := Convert_fp16_to_unsigned_integer64(SRC.fp16[0])
/// ELSE:
///     DEST.dword := Convert_fp16_to_unsigned_integer32(SRC.fp16[0])
/// ```
#[box_to_static_reference]
pub(super) fn vcvtsh2usi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSI2SH dest, src1, src2
/// IF *SRC2 is a register* and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// IF 64-mode and OperandSize == 64:
///     DEST.fp16[0] := Convert_integer64_to_fp16(SRC2.qword)
/// ELSE:
///     DEST.fp16[0] := Convert_integer32_to_fp16(SRC2.dword)
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtsi2sh() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSS2SH dest, src1, src2
/// IF *SRC2 is a register* and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     DEST.fp16[0] := Convert_fp32_to_fp16(SRC2.fp32[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else dest.fp16[0] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtss2sh() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTSS2USI (EVEX Encoded Version)
/// IF (SRC *is register*) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF 64-bit Mode and OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Single_Precision_Floating_Point_To_UInteger(SRC[31:0]);
/// ELSE
///     DEST[31:0] := Convert_Single_Precision_Floating_Point_To_UInteger(SRC[31:0]);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn vcvtss2usi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPD2QQ (EVEX Encoded Version) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Double_Precision_Floating_Point_To_QuadInteger_Truncate(SRC[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTTPD2QQ (EVEX Encoded Version) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                             Convert_Double_Precision_Floating_Point_To_QuadInteger_Truncate(SRC[63:0])
///                     ELSE
///                         DEST[i+63:i] := Convert_Double_Precision_Floating_Point_To_QuadInteger_Truncate(SRC[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttpd2qq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPD2UDQ (EVEX Encoded Versions) When SRC2 Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UInteger_Truncate(SRC[k+63:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// VCVTTPD2UDQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256),(8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UInteger_Truncate(SRC[63:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UInteger_Truncate(SRC[k+63:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttpd2udq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPD2UQQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UQuadInteger_Truncate(SRC[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTTPD2UQQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UQuadInteger_Truncate(SRC[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_Double_Precision_Floating_Point_To_UQuadInteger_Truncate(SRC[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttpd2uqq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPH2DQ dest, src
/// VL = 128, 256 or 512
/// KL := VL / 32
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.fp32[j] := Convert_fp16_to_integer32_truncate(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp32[j] := 0
///     // else dest.fp32[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttph2dq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPH2QQ dest, src
/// VL = 128, 256 or 512
/// KL := VL / 64
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.qword[j] := Convert_fp16_to_integer64_truncate(tsrc)
///     ELSE IF *zeroing*:
///         DEST.qword[j] := 0
///     // else dest.qword[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttph2qq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPH2UDQ dest, src
/// VL = 128, 256 or 512
/// KL := VL / 32
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.dword[j] := Convert_fp16_to_unsigned_integer32_truncate(tsrc)
///     ELSE IF *zeroing*:
///         DEST.dword[j] := 0
///     // else dest.dword[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttph2udq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPH2UQQ dest, src
/// VL = 128, 256 or 512
/// KL := VL / 64
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.qword[j] := Convert_fp16_to_unsigned_integer64_truncate(tsrc)
///     ELSE IF *zeroing*:
///         DEST.qword[j] := 0
///     // else dest.qword[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttph2uqq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPH2UW dest, src
/// VL = 128, 256 or 512
/// KL := VL / 16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.word[j] := Convert_fp16_to_unsigned_integer16_truncate(tsrc)
///     ELSE IF *zeroing*:
///         DEST.word[j] := 0
///     // else dest.word[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttph2uw() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPH2W dest, src
/// VL = 128, 256 or 512
/// KL := VL / 16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.fp16[0]
///         ELSE
///             tsrc := SRC.fp16[j]
///         DEST.word[j] := Convert_fp16_to_integer16_truncate(tsrc)
///     ELSE IF *zeroing*:
///         DEST.word[j] := 0
///     // else dest.word[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttph2w() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPS2QQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Single_Precision_To_QuadInteger_Truncate(SRC[k+31:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTTPS2QQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_QuadInteger_Truncate(SRC[31:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_QuadInteger_Truncate(SRC[k+31:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttps2qq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPS2UDQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_UInteger_Truncate(SRC[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTTPS2UDQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_UInteger_Truncate(SRC[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_Single_Precision_Floating_Point_To_UInteger_Truncate(SRC[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttps2udq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTPS2UQQ (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_Single_Precision_To_UQuadInteger_Truncate(SRC[k+31:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTTPS2UQQ (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_UQuadInteger_Truncate(SRC[31:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_Single_Precision_To_UQuadInteger_Truncate(SRC[k+31:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvttps2uqq() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTSD2USI (EVEX Encoded Version)
/// IF 64-Bit Mode and OperandSize = 64
///     THEN DEST[63:0] := Convert_Double_Precision_Floating_Point_To_UInteger_Truncate(SRC[63:0]);
///     ELSEDEST[31:0] := Convert_Double_Precision_Floating_Point_To_UInteger_Truncate(SRC[63:0]);
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn vcvttsd2usi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTSH2SI dest, src
/// IF 64-mode and OperandSize == 64:
///     DEST.qword := Convert_fp16_to_integer64_truncate(SRC.fp16[0])
/// ELSE:
///     DEST.dword := Convert_fp16_to_integer32_truncate(SRC.fp16[0])
/// ```
#[box_to_static_reference]
pub(super) fn vcvttsh2si() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTSH2USI dest, src
/// IF 64-mode and OperandSize == 64:
///     DEST.qword := Convert_fp16_to_unsigned_integer64_truncate(SRC.fp16[0])
/// ELSE:
///     DEST.dword := Convert_fp16_to_unsigned_integer32_truncate(SRC.fp16[0])
/// ```
#[box_to_static_reference]
pub(super) fn vcvttsh2usi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTTSS2USI (EVEX Encoded Version)
/// IF 64-bit Mode and OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_Single_Precision_Floating_Point_To_UInteger_Truncate(SRC[31:0]);
/// ELSE
///     DEST[31:0] := Convert_Single_Precision_Floating_Point_To_UInteger_Truncate(SRC[31:0]);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn vcvttss2usi() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUDQ2PD (EVEX Encoded Versions) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_UInteger_To_Double_Precision_Floating_Point(SRC[k+31:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTUDQ2PD (EVEX Encoded Versions) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_UInteger_To_Double_Precision_Floating_Point(SRC[31:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_UInteger_To_Double_Precision_Floating_Point(SRC[k+31:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtudq2pd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUDQ2PH dest, src
/// VL = 128, 256 or 512
/// KL := VL / 32
/// IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.dword[0]
///         ELSE
///             tsrc := SRC.dword[j]
///         DEST.fp16[j] := Convert_unsigned_integer32_to_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL/2] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtudq2ph() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUDQ2PS (EVEX Encoded Version) When SRC Operand is a Register
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 Convert_UInteger_To_Single_Precision_Floating_Point(SRC[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTUDQ2PS (EVEX Encoded Version) When SRC Operand is a Memory Source
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_UInteger_To_Single_Precision_Floating_Point(SRC[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_UInteger_To_Single_Precision_Floating_Point(SRC[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtudq2ps() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUQQ2PD (EVEX Encoded Version) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL == 512) AND (EVEX.b == 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 Convert_UQuadInteger_To_Double_Precision_Floating_Point(SRC[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VCVTUQQ2PD (EVEX Encoded Version) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 Convert_UQuadInteger_To_Double_Precision_Floating_Point(SRC[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 Convert_UQuadInteger_To_Double_Precision_Floating_Point(SRC[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtuqq2pd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUQQ2PH dest, src
/// VL = 128, 256 or 512
/// KL := VL / 64
/// IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.qword[0]
///         ELSE
///             tsrc := SRC.qword[j]
///         DEST.fp16[j] := Convert_unsigned_integer64_to_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL/4] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtuqq2ph() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUQQ2PS (EVEX Encoded Version) When SRC Operand is a Register
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 Convert_UQuadInteger_To_Single_Precision_Floating_Point(SRC[k+63:k])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VCVTUQQ2PS (EVEX Encoded Version) When SRC Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 Convert_UQuadInteger_To_Single_Precision_Floating_Point(SRC[63:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 Convert_UQuadInteger_To_Single_Precision_Floating_Point(SRC[k+63:k])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL/2] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtuqq2ps() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUSI2SD (EVEX Encoded Version)
/// IF (SRC2 *is register*) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF 64-Bit Mode And OperandSize = 64
/// THEN
///     DEST[63:0] := Convert_UInteger_To_Double_Precision_Floating_Point(SRC2[63:0]);
/// ELSE
///     DEST[63:0] := Convert_UInteger_To_Double_Precision_Floating_Point(SRC2[31:0]);
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtusi2sd() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUSI2SH dest, src1, src2
/// IF *SRC2 is a register* and (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// IF 64-mode and OperandSize == 64:
///     DEST.fp16[0] := Convert_unsigned_integer64_to_fp16(SRC2.qword)
/// ELSE:
///     DEST.fp16[0] := Convert_unsigned_integer32_to_fp16(SRC2.dword)
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtusi2sh() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUSI2SS (EVEX Encoded Version)
/// IF (SRC2 *is register*) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF 64-Bit Mode And OperandSize = 64
/// THEN
///     DEST[31:0] := Convert_UInteger_To_Single_Precision_Floating_Point(SRC[63:0]);
/// ELSE
///     DEST[31:0] := Convert_UInteger_To_Single_Precision_Floating_Point(SRC[31:0]);
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtusi2ss() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTUW2PH dest, src
/// VL = 128, 256 or 512
/// KL := VL / 16
/// IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.word[0]
///         ELSE
///             tsrc := SRC.word[j]
///         DEST.fp16[j] := Convert_unsignd_integer16_to_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtuw2ph() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VCVTW2PH dest, src
/// VL = 128, 256 or 512
/// KL := VL / 16
/// IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE:
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *SRC is memory* and EVEX.b = 1:
///             tsrc := SRC.word[0]
///         ELSE
///             tsrc := SRC.word[j]
///         DEST.fp16[j] := Convert_integer16_to_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vcvtw2ph() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VDBPSADBW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// Selection of quadruplets:
/// FOR I = 0 to VL step 128
///     TMP1[I+31:I] := select (SRC2[I+127: I], imm8[1:0])
///     TMP1[I+63: I+32] := select (SRC2[I+127: I], imm8[3:2])
///     TMP1[I+95: I+64] := select (SRC2[I+127: I], imm8[5:4])
///     TMP1[I+127: I+96]  := select (SRC2[I+127: I], imm8[7:6])
/// END FOR
/// SAD of quadruplets:
/// FOR I =0 to VL step 64
///     TMP_DEST[I+15:I] := ABS(SRC1[I+7: I] - TMP1[I+7: I]) +
///         ABS(SRC1[I+15: I+8]- TMP1[I+15: I+8]) +
///         ABS(SRC1[I+23: I+16]- TMP1[I+23: I+16]) +
///         ABS(SRC1[I+31: I+24]- TMP1[I+31: I+24])
///     TMP_DEST[I+31: I+16] := ABS(SRC1[I+7: I] - TMP1[I+15: I+8]) +
///         ABS(SRC1[I+15: I+8]- TMP1[I+23: I+16]) +
///         ABS(SRC1[I+23: I+16]- TMP1[I+31: I+24]) +
///         ABS(SRC1[I+31: I+24]- TMP1[I+39: I+32])
///     TMP_DEST[I+47: I+32] := ABS(SRC1[I+39: I+32] - TMP1[I+23: I+16]) +
///         ABS(SRC1[I+47: I+40]- TMP1[I+31: I+24]) +
///         ABS(SRC1[I+55: I+48]- TMP1[I+39: I+32]) +
///         ABS(SRC1[I+63: I+56]- TMP1[I+47: I+40])
///     TMP_DEST[I+63: I+48] := ABS(SRC1[I+39: I+32] - TMP1[I+31: I+24]) +
///         ABS(SRC1[I+47: I+40] - TMP1[I+39: I+32]) +
///         ABS(SRC1[I+55: I+48] - TMP1[I+47: I+40]) +
///         ABS(SRC1[I+63: I+56] - TMP1[I+55: I+48])
/// ENDFOR
/// FOR j :=  0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] :=  TMP_DEST[i+15:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] :=  0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] :=  0
/// ```
#[box_to_static_reference]
pub(super) fn vdbpsadbw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VDIVPH (EVEX Encoded Versions) When SRC2 Operand is a Register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.fp16[j] := SRC1.fp16[j] / SRC2.fp16[j]
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VDIVPH (EVEX Encoded Versions) When SRC2 Operand is a Memory Source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             DEST.fp16[j] := SRC1.fp16[j] / SRC2.fp16[0]
///         ELSE:
///             DEST.fp16[j] := SRC1.fp16[j] / SRC2.fp16[j]
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vdivph() -> &'static [IrStatement] {
    let assignment = assign(b::unsigned_div(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VDIVSH (EVEX Encoded Versions)
/// IF EVEX.b = 1 and SRC2 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     DEST.fp16[0] := SRC1.fp16[0] / SRC2.fp16[0]
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else dest.fp16[0] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vdivsh() -> &'static [IrStatement] {
    let assignment = assign(b::unsigned_div(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Define make_fp32(x):
///     // The x parameter is bfloat16. Pack it in to upper 16b of a dword. The bit pattern is a legal fp32 value. Return that bit pattern.
///     dword := 0
///     dword[31:16] := x
///     RETURN dword
/// VDPBF16PS srcdest, src1, src2
/// VL = (128, 256, 512)
/// KL = VL/32
/// origdest := srcdest
/// FOR i := 0 to KL-1:
///     IF k1[ i ] or *no writemask*:
///         IF src2 is memory and evex.b == 1:
///             t := src2.dword[0]
///         ELSE:
///             t := src2.dword[ i ]
///         // FP32 FMA with daz in, ftz out and RNE rounding. MXCSR neither consulted nor updated.
///         srcdest.fp32[ i ] += make_fp32(src1.bfloat16[2*i+1]) * make_fp32(t.bfloat[1])
///         srcdest.fp32[ i ] += make_fp32(src1.bfloat16[2*i+0]) * make_fp32(t.bfloat[0])
///     ELSE IF *zeroing*:
///         srcdest.dword[ i ]
///                 := 0
///     ELSE: // merge masking, dest element unchanged
///         srcdest.dword[ i ] := origdest.dword[ i ]
/// srcdest[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vdpbf16ps() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VEXTRACTI32x4 (EVEX encoded versions) when destination is a register
/// VL = 256, 512
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0] := SRC1[127:0]
///         1: TMP_DEST[127:0] := SRC1[255:128]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0] := SRC1[127:0]
///         01: TMP_DEST[127:0] := SRC1[255:128]
///         10: TMP_DEST[127:0] := SRC1[383:256]
///         11: TMP_DEST[127:0] := SRC1[511:384]
///     ESAC.
/// FI;
/// FOR j := 0 TO 3
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:128] := 0
/// VEXTRACTI32x4 (EVEX encoded versions) when destination is memory
/// VL = 256, 512
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0] := SRC1[127:0]
///         1: TMP_DEST[127:0] := SRC1[255:128]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0] := SRC1[127:0]
///         01: TMP_DEST[127:0] := SRC1[255:128]
///         10: TMP_DEST[127:0] := SRC1[383:256]
///         11: TMP_DEST[127:0] := SRC1[511:384]
///     ESAC.
/// FI;
/// FOR j := 0 TO 3
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE *DEST[i+31:i] remains unchanged*
///                             ; merging-masking
///     FI;
/// ENDFOR
/// VEXTRACTI64x2 (EVEX encoded versions) when destination is a register
/// VL = 256, 512
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0] := SRC1[127:0]
///         1: TMP_DEST[127:0] := SRC1[255:128]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0] := SRC1[127:0]
///         01: TMP_DEST[127:0] := SRC1[255:128]
///         10: TMP_DEST[127:0] := SRC1[383:256]
///         11: TMP_DEST[127:0] := SRC1[511:384]
///     ESAC.
/// FI;
/// FOR j := 0 TO 1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:128] := 0
/// VEXTRACTI64x2 (EVEX encoded versions) when destination is memory
/// VL = 256, 512
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0] := SRC1[127:0]
///         1: TMP_DEST[127:0] := SRC1[255:128]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0] := SRC1[127:0]
///         01: TMP_DEST[127:0] := SRC1[255:128]
///         11: TMP_DEST[127:0] := SRC1[511:384]
///     ESAC.
/// FI;
/// FOR j := 0 TO 1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE *DEST[i+63:i] remains unchanged*
///                             ; merging-masking
///     FI;
/// ENDFOR
/// VEXTRACTI32x8 (EVEX.U1.512 encoded version) when destination is a register
/// VL = 512
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC1[255:0]
///     1: TMP_DEST[255:0] := SRC1[511:256]
/// ESAC.
/// FOR j := 0 TO 7
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:256] := 0
/// VEXTRACTI32x8 (EVEX.U1.512 encoded version) when destination is memory
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC1[255:0]
///     1: TMP_DEST[255:0] := SRC1[511:256]
/// ESAC.
/// FOR j := 0 TO 7
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE *DEST[i+31:i] remains unchanged*
///                             ; merging-masking
///     FI;
/// ENDFOR
/// VEXTRACTI64x4 (EVEX.512 encoded version) when destination is a register
/// VL = 512
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC1[255:0]
///     1: TMP_DEST[255:0] := SRC1[511:256]
/// ESAC.
/// FOR j := 0 TO 3
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:256] := 0
/// VEXTRACTI64x4 (EVEX.512 encoded version) when destination is memory
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC1[255:0]
///     1: TMP_DEST[255:0] := SRC1[511:256]
/// ESAC.
/// FOR j := 0 TO 3
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE *DEST[i+63:i] remains unchanged*
///                             ; merging-masking
///     FI;
/// ENDFOR
/// VEXTRACTI128 (memory destination form)
/// CASE (imm8[0]) OF
///     0: DEST[127:0] := SRC1[127:0]
///     1: DEST[127:0] := SRC1[255:128]
/// ESAC.
/// VEXTRACTI128 (register destination form)
/// CASE (imm8[0]) OF
///     0: DEST[127:0] := SRC1[127:0]
///     1: DEST[127:0] := SRC1[255:128]
/// ESAC.
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn verr() -> &'static [IrStatement] {
    [exception("verr")].into()
}

/// # Pseudocode
/// ```text
/// VEXTRACTI32x4 (EVEX encoded versions) when destination is a register
/// VL = 256, 512
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0] := SRC1[127:0]
///         1: TMP_DEST[127:0] := SRC1[255:128]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0] := SRC1[127:0]
///         01: TMP_DEST[127:0] := SRC1[255:128]
///         10: TMP_DEST[127:0] := SRC1[383:256]
///         11: TMP_DEST[127:0] := SRC1[511:384]
///     ESAC.
/// FI;
/// FOR j := 0 TO 3
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:128] := 0
/// VEXTRACTI32x4 (EVEX encoded versions) when destination is memory
/// VL = 256, 512
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0] := SRC1[127:0]
///         1: TMP_DEST[127:0] := SRC1[255:128]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0] := SRC1[127:0]
///         01: TMP_DEST[127:0] := SRC1[255:128]
///         10: TMP_DEST[127:0] := SRC1[383:256]
///         11: TMP_DEST[127:0] := SRC1[511:384]
///     ESAC.
/// FI;
/// FOR j := 0 TO 3
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE *DEST[i+31:i] remains unchanged*
///                             ; merging-masking
///     FI;
/// ENDFOR
/// VEXTRACTI64x2 (EVEX encoded versions) when destination is a register
/// VL = 256, 512
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0] := SRC1[127:0]
///         1: TMP_DEST[127:0] := SRC1[255:128]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0] := SRC1[127:0]
///         01: TMP_DEST[127:0] := SRC1[255:128]
///         10: TMP_DEST[127:0] := SRC1[383:256]
///         11: TMP_DEST[127:0] := SRC1[511:384]
///     ESAC.
/// FI;
/// FOR j := 0 TO 1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:128] := 0
/// VEXTRACTI64x2 (EVEX encoded versions) when destination is memory
/// VL = 256, 512
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0] := SRC1[127:0]
///         1: TMP_DEST[127:0] := SRC1[255:128]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0] := SRC1[127:0]
///         01: TMP_DEST[127:0] := SRC1[255:128]
///         11: TMP_DEST[127:0] := SRC1[511:384]
///     ESAC.
/// FI;
/// FOR j := 0 TO 1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE *DEST[i+63:i] remains unchanged*
///                             ; merging-masking
///     FI;
/// ENDFOR
/// VEXTRACTI32x8 (EVEX.U1.512 encoded version) when destination is a register
/// VL = 512
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC1[255:0]
///     1: TMP_DEST[255:0] := SRC1[511:256]
/// ESAC.
/// FOR j := 0 TO 7
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:256] := 0
/// VEXTRACTI32x8 (EVEX.U1.512 encoded version) when destination is memory
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC1[255:0]
///     1: TMP_DEST[255:0] := SRC1[511:256]
/// ESAC.
/// FOR j := 0 TO 7
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE *DEST[i+31:i] remains unchanged*
///                             ; merging-masking
///     FI;
/// ENDFOR
/// VEXTRACTI64x4 (EVEX.512 encoded version) when destination is a register
/// VL = 512
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC1[255:0]
///     1: TMP_DEST[255:0] := SRC1[511:256]
/// ESAC.
/// FOR j := 0 TO 3
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:256] := 0
/// VEXTRACTI64x4 (EVEX.512 encoded version) when destination is memory
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC1[255:0]
///     1: TMP_DEST[255:0] := SRC1[511:256]
/// ESAC.
/// FOR j := 0 TO 3
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE *DEST[i+63:i] remains unchanged*
///                             ; merging-masking
///     FI;
/// ENDFOR
/// VEXTRACTI128 (memory destination form)
/// CASE (imm8[0]) OF
///     0: DEST[127:0] := SRC1[127:0]
///     1: DEST[127:0] := SRC1[255:128]
/// ESAC.
/// VEXTRACTI128 (register destination form)
/// CASE (imm8[0]) OF
///     0: DEST[127:0] := SRC1[127:0]
///     1: DEST[127:0] := SRC1[255:128]
/// ESAC.
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn verw() -> &'static [IrStatement] {
    [exception("verw")].into()
}

/// # Pseudocode
/// ```text
/// VEXPANDPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+63:i] := SRC[k+63:k];
///                 k := k + 64
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vexpandpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VEXPANDPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+31:i] := SRC[k+31:k];
///                 k := k + 32
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vexpandps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMADDCPH dest{k1}, src1, src2 (AVX512)
/// VL = 128, 256, 512
/// KL := VL / 32
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF broadcasting and src2 is memory:
///             tsrc2.fp16[2*i+0] := src2.fp16[0]
///             tsrc2.fp16[2*i+1] := src2.fp16[1]
///         ELSE:
///             tsrc2.fp16[2*i+0] := src2.fp16[2*i+0]
///             tsrc2.fp16[2*i+1] := src2.fp16[2*i+1]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         tmp[2*i+0] := dest.fp16[2*i+0] + src1.fp16[2*i+0] * tsrc2.fp16[2*i+0]
///         tmp[2*i+1] := dest.fp16[2*i+1] + src1.fp16[2*i+1] * tsrc2.fp16[2*i+0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         // non-conjugate version subtracts even term
///         dest.fp16[2*i+0] := tmp[2*i+0] - src1.fp16[2*i+1] * tsrc2.fp16[2*i+1]
///         dest.fp16[2*i+1] := tmp[2*i+1] + src1.fp16[2*i+0] * tsrc2.fp16[2*i+1]
///     ELSE IF *zeroing*:
///         dest.fp16[2*i+0] := 0
///         dest.fp16[2*i+1] := 0
/// DEST[MAXVL-1:VL] := 0
/// VFCMADDCPH dest{k1}, src1, src2 (AVX512)
/// VL = 128, 256, 512
/// KL := VL / 32
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF broadcasting and src2 is memory:
///             tsrc2.fp16[2*i+0] := src2.fp16[0]
///             tsrc2.fp16[2*i+1] := src2.fp16[1]
///         ELSE:
///             tsrc2.fp16[2*i+0] := src2.fp16[2*i+0]
///             tsrc2.fp16[2*i+1] := src2.fp16[2*i+1]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         tmp[2*i+0] := dest.fp16[2*i+0] + src1.fp16[2*i+0] * tsrc2.fp16[2*i+0]
///         tmp[2*i+1] := dest.fp16[2*i+1] + src1.fp16[2*i+1] * tsrc2.fp16[2*i+0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         // conjugate version subtracts odd final term
///         dest.fp16[2*i+0] := tmp[2*i+0] + src1.fp16[2*i+1] * tsrc2.fp16[2*i+1]
///         dest.fp16[2*i+1] := tmp[2*i+1] - src1.fp16[2*i+0] * tsrc2.fp16[2*i+1]
///     ELSE IF *zeroing*:
///         dest.fp16[2*i+0] := 0
///         dest.fp16[2*i+1] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfcmaddcph() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMADDCSH dest{k1}, src1, src2 (AVX512)
/// IF k1[0] or *no writemask*:
///     tmp[0] := dest.fp16[0] + src1.fp16[0] * src2.fp16[0]
///     tmp[1] := dest.fp16[1] + src1.fp16[1] * src2.fp16[0]
///     // non-conjugate version subtracts last even term
///     dest.fp16[0] := tmp[0] - src1.fp16[1] * src2.fp16[1]
///     dest.fp16[1] := tmp[1] + src1.fp16[0] * src2.fp16[1]
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
///     dest.fp16[1] := 0
/// DEST[127:32] := src1[127:32] // copy upper part of src1
/// DEST[MAXVL-1:128] := 0
/// VFCMADDCSH dest{k1}, src1, src2 (AVX512)
/// IF k1[0] or *no writemask*:
///     tmp[0] := dest.fp16[0] + src1.fp16[0] * src2.fp16[0]
///     tmp[1] := dest.fp16[1] + src1.fp16[1] * src2.fp16[0]
///     // conjugate version subtracts odd final term
///     dest.fp16[0] := tmp[0] + src1.fp16[1] * src2.fp16[1]
///     dest.fp16[1] := tmp[1] - src1.fp16[0] * src2.fp16[1]
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
///     dest.fp16[1] := 0
/// DEST[127:32] := src1[127:32] // copy upper part of src1
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfcmaddcsh() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMULCPH dest{k1}, src1, src2 (AVX512)
/// VL = 128, 256 or 512
/// KL := VL/32
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF broadcasting and src2 is memory:
///             tsrc2.fp16[2*i+0] := src2.fp16[0]
///             tsrc2.fp16[2*i+1] := src2.fp16[1]
///         ELSE:
///             tsrc2.fp16[2*i+0] := src2.fp16[2*i+0]
///             tsrc2.fp16[2*i+1] := src2.fp16[2*i+1]
///     IF k1[i] or *no writemask*:
///         tmp.fp16[2*i+0] := src1.fp16[2*i+0] * tsrc2.fp16[2*i+0]
///         tmp.fp16[2*i+1] := src1.fp16[2*i+1] * tsrc2.fp16[2*i+0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         // non-conjugate version subtracts last even term
///         dest.fp16[2*i+0] := tmp.fp16[2*i+0] - src1.fp16[2*i+1] * tsrc2.fp16[2*i+1]
///         dest.fp16[2*i+1] := tmp.fp16[2*i+1] + src1.fp16[2*i+0] * tsrc2.fp16[2*i+1]
///     ELSE IF *zeroing*:
///         dest.fp16[2*i+0] := 0
///         dest.fp16[2*i+1] := 0
/// DEST[MAXVL-1:VL] := 0
/// VFCMULCPH dest{k1}, src1, src2 (AVX512)
/// VL = 128, 256 or 512
/// KL := VL/32
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF broadcasting and src2 is memory:
///             tsrc2.fp16[2*i+0] := src2.fp16[0]
///             tsrc2.fp16[2*i+1] := src2.fp16[1]
///         ELSE:
///             tsrc2.fp16[2*i+0] := src2.fp16[2*i+0]
///             tsrc2.fp16[2*i+1] := src2.fp16[2*i+1]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         tmp.fp16[2*i+0] := src1.fp16[2*i+0] * tsrc2.fp16[2*i+0]
///         tmp.fp16[2*i+1] := src1.fp16[2*i+1] * tsrc2.fp16[2*i+0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         // conjugate version subtracts odd final term
///         dest.fp16[2*i] := tmp.fp16[2*i+0] +src1.fp16[2*i+1] * tsrc2.fp16[2*i+1]
///         dest.fp16[2*i+1] := tmp.fp16[2*i+1] - src1.fp16[2*i+0] * tsrc2.fp16[2*i+1]
///     ELSE IF *zeroing*:
///         dest.fp16[2*i+0] := 0
///         dest.fp16[2*i+1] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfcmulcph() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMULCSH dest{k1}, src1, src2 (AVX512)
/// KL := VL / 32
/// IF k1[0] or *no writemask*:
///     // non-conjugate version subtracts last even term
///     tmp.fp16[0] := src1.fp16[0] * src2.fp16[0]
///     tmp.fp16[1] := src1.fp16[1] * src2.fp16[0]
///     dest.fp16[0] := tmp.fp16[0] - src1.fp16[1] * src2.fp16[1]
///     dest.fp16[1] := tmp.fp16[1] + src1.fp16[0] * src2.fp16[1]
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
///     dest.fp16[1] := 0
/// DEST[127:32] := src1[127:32] // copy upper part of src1
/// DEST[MAXVL-1:128] := 0
/// VFCMULCSH dest{k1}, src1, src2 (AVX512)
/// KL := VL / 32
/// IF k1[0] or *no writemask*:
///     tmp.fp16[0] := src1.fp16[0] * src2.fp16[0]
///     tmp.fp16[1] := src1.fp16[1] * src2.fp16[0]
///     // conjugate version subtracts odd final term
///     dest.fp16[0] := tmp.fp16[0] + src1.fp16[1] * src2.fp16[1]
///     dest.fp16[1] := tmp.fp16[1] - src1.fp16[0] * src2.fp16[1]
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
///     dest.fp16[1] := 0
/// DEST[127:32] := src1[127:32] // copy upper part of src1
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfcmulcsh() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// enum TOKEN_TYPE
/// {
///     QNAN_TOKEN := 0,
///     SNAN_TOKEN := 1,
///     ZERO_VALUE_TOKEN := 2,
///     POS_ONE_VALUE_TOKEN := 3,
///     NEG_INF_TOKEN := 4,
///     POS_INF_TOKEN := 5,
///     NEG_VALUE_TOKEN := 6,
///     POS_VALUE_TOKEN := 7
/// }
/// FIXUPIMM_DP (dest[63:0], src1[63:0],tbl3[63:0], imm8 [7:0]){
///     tsrc[63:0] := ((src1[62:52] = 0) AND (MXCSR.DAZ =1)) ? 0.0 : src1[63:0]
///     CASE(tsrc[63:0] of TOKEN_TYPE) {
///         QNAN_TOKEN: j := 0;
///         SNAN_TOKEN: j := 1;
///         ZERO_VALUE_TOKEN: j := 2;
///         POS_ONE_VALUE_TOKEN: j := 3;
///         NEG_INF_TOKEN: j := 4;
///         POS_INF_TOKEN: j := 5;
///         NEG_VALUE_TOKEN: j := 6;
///         POS_VALUE_TOKEN: j := 7;
///     }; end source special CASE(tsrc…)
///     ; The required response from src3 table is extracted
///     token_response[3:0] = tbl3[3+4*j:4*j];
///     CASE(token_response[3:0]) {
///         0000: dest[63:0] := dest[63:0];
///                             ; preserve content of DEST
///         0001: dest[63:0] := tsrc[63:0];
///                             ; pass through src1 normal input value, denormal as zero
///         0010: dest[63:0] := QNaN(tsrc[63:0]);
///         0011: dest[63:0] := QNAN_Indefinite;
///         0100: dest[63:0] := -INF;
///         0101: dest[63:0] := +INF;
///         0110: dest[63:0] := tsrc.sign? -INF : +INF;
///         0111: dest[63:0] := -0;
///         1000: dest[63:0] := +0;
///         1001: dest[63:0] := -1;
///         1010: dest[63:0] := +1;
///         1011: dest[63:0] := ½;
///         1100: dest[63:0] := 90.0;
///         1101: dest[63:0] := PI/2;
///         1110: dest[63:0] := MAX_FLOAT;
///         1111: dest[63:0] := -MAX_FLOAT;
///     }
///             ; end of token_response CASE
///     ; The required fault reporting from imm8 is extracted
///     ; TOKENs are mutually exclusive and TOKENs priority defines the order.
///                                 .
///     ; Multiple faults related to a single token can occur simultaneously
///     IF (tsrc[63:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[0] then set #ZE;
///     IF (tsrc[63:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[1] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: ONE_VALUE_TOKEN) AND imm8[3] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: SNAN_TOKEN) AND imm8[4] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: NEG_INF_TOKEN) AND imm8[5] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: NEG_VALUE_TOKEN) AND imm8[6] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: POS_INF_TOKEN) AND imm8[7] then set #IE;
///         ; end fault reporting
///     return dest[63:0];
/// }
///         ; end of FIXUPIMM_DP()
/// VFIXUPIMMPD
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     DEST[i+63:i] := FIXUPIMM_DP(DEST[i+63:i], SRC1[i+63:i], SRC2[63:0], imm8 [7:0])
///                 ELSE
///                     DEST[i+63:i] := FIXUPIMM_DP(DEST[i+63:i], SRC1[i+63:i], SRC2[i+63:i], imm8 [7:0])
///             FI;
///         ELSE
///             IF *merging-masking*
///                             ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE  DEST[i+63:i] := 0
///                             ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// Immediate Control Description:
///                         F01234567+igINuF re 5-9.  VFIX-#UIINPFEI MMPD ImmeS#dNIiaaEteN  Control D-#eIsEVcrEip ti onO#NIEE  O#NIEE  Z#EZREO Z#EIREO #ZE
/// ```
#[box_to_static_reference]
pub(super) fn vfixupimmpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// enum TOKEN_TYPE
/// {
///     QNAN_TOKEN := 0,
///     SNAN_TOKEN := 1,
///     ZERO_VALUE_TOKEN := 2,
///     POS_ONE_VALUE_TOKEN := 3,
///     NEG_INF_TOKEN := 4,
///     POS_INF_TOKEN := 5,
///     NEG_VALUE_TOKEN := 6,
///     POS_VALUE_TOKEN := 7
/// }
/// FIXUPIMM_SP ( dest[31:0], src1[31:0],tbl3[31:0], imm8 [7:0]){
///     tsrc[31:0] := ((src1[30:23] = 0) AND (MXCSR.DAZ =1)) ? 0.0 : src1[31:0]
///     CASE(tsrc[31:0] of TOKEN_TYPE) {
///         QNAN_TOKEN: j := 0;
///         SNAN_TOKEN: j := 1;
///         ZERO_VALUE_TOKEN: j := 2;
///         POS_ONE_VALUE_TOKEN: j := 3;
///         NEG_INF_TOKEN: j := 4;
///         POS_INF_TOKEN: j := 5;
///         NEG_VALUE_TOKEN: j := 6;
///         POS_VALUE_TOKEN: j := 7;
///     }
///             ; end source special CASE(tsrc…)
///     ; The required response from src3 table is extracted
///     token_response[3:0] = tbl3[3+4*j:4*j];
///     CASE(token_response[3:0]) {
///         0000: dest[31:0] := dest[31:0];
///                             ; preserve content of DEST
///         0001: dest[31:0] := tsrc[31:0];
///                             ; pass through src1 normal input value, denormal as zero
///         0010: dest[31:0] := QNaN(tsrc[31:0]);
///         0011: dest[31:0] := QNAN_Indefinite;
///         0100: dest[31:0] := -INF;
///         0101: dest[31:0] := +INF;
///         0110: dest[31:0] := tsrc.sign? -INF : +INF;
///         0111: dest[31:0] := -0;
///         1000: dest[31:0] := +0;
///         1001: dest[31:0] := -1;
///         1010: dest[31:0] := +1;
///         1011:  dest[31:0] := ½;
///         1100: dest[31:0] := 90.0;
///         1101: dest[31:0] := PI/2;
///         1110: dest[31:0] := MAX_FLOAT;
///         1111: dest[31:0] := -MAX_FLOAT;
///     }
///             ; end of token_response CASE
///     ; The required fault reporting from imm8 is extracted
///     ; TOKENs are mutually exclusive and TOKENs priority defines the order.
///                                     .
///     ; Multiple faults related to a single token can occur simultaneously
///     IF (tsrc[31:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[0] then set #ZE;
///     IF (tsrc[31:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[1] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: ONE_VALUE_TOKEN) AND imm8[3] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: SNAN_TOKEN) AND imm8[4] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: NEG_INF_TOKEN) AND imm8[5] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: NEG_VALUE_TOKEN) AND imm8[6] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: POS_INF_TOKEN) AND imm8[7] then set #IE;
///         ; end fault reporting
///     return dest[31:0];
/// }
///         ; end of FIXUPIMM_SP()
/// VFIXUPIMMPS (EVEX)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     DEST[i+31:i] := FIXUPIMM_SP(DEST[i+31:i], SRC1[i+31:i], SRC2[31:0], imm8 [7:0])
///                 ELSE
///                     DEST[i+31:i] := FIXUPIMM_SP(DEST[i+31:i], SRC1[i+31:i], SRC2[i+31:i], imm8 [7:0])
///             FI;
///         ELSE
///             IF *merging-masking*
///                                 ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE  DEST[i+31:i] := 0
///                                 ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// Immediate Control Description:
///                         F01234567+igINuF re 5-10.  VFI-#XIIUNFE PIMMPS ImmS#eNIdiaEaNte  Control-# DIeEVscEri pt ionO#NIEE  O#NIEE  Z#EZREO Z#EIREO #ZE
/// ```
#[box_to_static_reference]
pub(super) fn vfixupimmps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// enum TOKEN_TYPE
/// {
///     QNAN_TOKEN := 0,
///     SNAN_TOKEN := 1,
///     ZERO_VALUE_TOKEN := 2,
///     POS_ONE_VALUE_TOKEN := 3,
///     NEG_INF_TOKEN := 4,
///     POS_INF_TOKEN := 5,
///     NEG_VALUE_TOKEN := 6,
///     POS_VALUE_TOKEN := 7
/// }
/// FIXUPIMM_DP (dest[63:0], src1[63:0],tbl3[63:0], imm8 [7:0]){
///     tsrc[63:0] := ((src1[62:52] = 0) AND (MXCSR.DAZ =1)) ? 0.0 : src1[63:0]
///     CASE(tsrc[63:0] of TOKEN_TYPE) {
///         QNAN_TOKEN: j := 0;
///         SNAN_TOKEN: j := 1;
///         ZERO_VALUE_TOKEN: j := 2;
///         POS_ONE_VALUE_TOKEN: j := 3;
///         NEG_INF_TOKEN: j := 4;
///         POS_INF_TOKEN: j := 5;
///         NEG_VALUE_TOKEN: j := 6;
///         POS_VALUE_TOKEN: j := 7;
///     }
///             ; end source special CASE(tsrc…)
///     ; The required response from src3 table is extracted
///     token_response[3:0] = tbl3[3+4*j:4*j];
///     CASE(token_response[3:0]) {
///         0000: dest[63:0] := dest[63:0]
///                     ; preserve content of DEST
///         0001: dest[63:0] := tsrc[63:0];
///                     ; pass through src1 normal input value, denormal as zero
///         0010: dest[63:0] := QNaN(tsrc[63:0]);
///         0011: dest[63:0] := QNAN_Indefinite;
///         0100:dest[63:0] := -INF;
///         0101: dest[63:0] := +INF;
///         0110: dest[63:0] := tsrc.sign? -INF : +INF;
///         0111: dest[63:0] := -0;
///         1000: dest[63:0] := +0;
///         1001: dest[63:0] := -1;
///         1010: dest[63:0] := +1;
///         1011: dest[63:0] := ½;
///         1100: dest[63:0] := 90.0;
///         1101: dest[63:0] := PI/2;
///         1110: dest[63:0] := MAX_FLOAT;
///         1111: dest[63:0] := -MAX_FLOAT;
///     }
///             ; end of token_response CASE
///     ; The required fault reporting from imm8 is extracted
///     ; TOKENs are mutually exclusive and TOKENs priority defines the order.
///                         .
///     ; Multiple faults related to a single token can occur simultaneously
///     IF (tsrc[63:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[0] then set #ZE;
///     IF (tsrc[63:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[1] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: ONE_VALUE_TOKEN) AND imm8[2] then set #ZE;
///     IF (tsrc[63:0] of TOKEN_TYPE: ONE_VALUE_TOKEN) AND imm8[3] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: SNAN_TOKEN) AND imm8[4] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: NEG_INF_TOKEN) AND imm8[5] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: NEG_VALUE_TOKEN) AND imm8[6] then set #IE;
///     IF (tsrc[63:0] of TOKEN_TYPE: POS_INF_TOKEN) AND imm8[7] then set #IE;
///         ; end fault reporting
///     return dest[63:0];
/// }
///         ; end of FIXUPIMM_DP()
/// VFIXUPIMMSD (EVEX encoded version)
/// IF k1[0] OR *no writemask*
///     THEN DEST[63:0] := FIXUPIMM_DP(DEST[63:0], SRC1[63:0], SRC2[63:0], imm8 [7:0])
///     ELSE
///         IF *merging-masking*
///                     ; merging-masking
///             THEN *DEST[63:0] remains unchanged*
///             ELSE  DEST[63:0] := 0
///                     ; zeroing-masking
///         FI
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// Immediate Control Description:
///                 Figure 5-11.  VFIXUPIMMSD Im01234567+meINdF iate Control -#DIIeNFEs criptionS#NIaEN -#IEVE  O#NIEE  O#NIEE  Z#EZREO Z#EIREO #ZE
/// ```
#[box_to_static_reference]
pub(super) fn vfixupimmsd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// enum TOKEN_TYPE
/// {
///     QNAN_TOKEN := 0,
///     SNAN_TOKEN := 1,
///     ZERO_VALUE_TOKEN := 2,
///     POS_ONE_VALUE_TOKEN := 3,
///     NEG_INF_TOKEN := 4,
///     POS_INF_TOKEN := 5,
///     NEG_VALUE_TOKEN := 6,
///     POS_VALUE_TOKEN := 7
/// }
/// FIXUPIMM_SP (dest[31:0], src1[31:0],tbl3[31:0], imm8 [7:0]){
///     tsrc[31:0] := ((src1[30:23] = 0) AND (MXCSR.DAZ =1)) ? 0.0 : src1[31:0]
///     CASE(tsrc[63:0] of TOKEN_TYPE) {
///         QNAN_TOKEN: j := 0;
///         SNAN_TOKEN: j := 1;
///         ZERO_VALUE_TOKEN: j := 2;
///         POS_ONE_VALUE_TOKEN: j := 3;
///         NEG_INF_TOKEN: j := 4;
///         POS_INF_TOKEN: j := 5;
///         NEG_VALUE_TOKEN: j := 6;
///         POS_VALUE_TOKEN: j := 7;
///     }
///             ; end source special CASE(tsrc…)
///     ; The required response from src3 table is extracted
///     token_response[3:0] = tbl3[3+4*j:4*j];
///     CASE(token_response[3:0]) {
///         0000: dest[31:0] := dest[31:0];
///                     ; preserve content of DEST
///         0001: dest[31:0] := tsrc[31:0];
///                     ; pass through src1 normal input value, denormal as zero
///         0010: dest[31:0] := QNaN(tsrc[31:0]);
///         0011: dest[31:0] := QNAN_Indefinite;
///         0100: dest[31:0] := -INF;
///         0101: dest[31:0] := +INF;
///         0110: dest[31:0] := tsrc.sign? -INF : +INF;
///         0111: dest[31:0] := -0;
///         1000: dest[31:0] := +0;
///         1001: dest[31:0] := -1;
///         1010: dest[31:0] := +1;
///         1011: dest[31:0] := ½;
///         1100: dest[31:0] := 90.0;
///         1101: dest[31:0] := PI/2;
///         1110: dest[31:0] := MAX_FLOAT;
///         1111: dest[31:0] := -MAX_FLOAT;
///     }
///             ; end of token_response CASE
///     ; The required fault reporting from imm8 is extracted
///     ; TOKENs are mutually exclusive and TOKENs priority defines the order.
///                         .
///     ; Multiple faults related to a single token can occur simultaneously
///     IF (tsrc[31:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[0] then set #ZE;
///     IF (tsrc[31:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[1] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: ONE_VALUE_TOKEN) AND imm8[2] then set #ZE;
///     IF (tsrc[31:0] of TOKEN_TYPE: ONE_VALUE_TOKEN) AND imm8[3] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: SNAN_TOKEN) AND imm8[4] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: NEG_INF_TOKEN) AND imm8[5] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: NEG_VALUE_TOKEN) AND imm8[6] then set #IE;
///     IF (tsrc[31:0] of TOKEN_TYPE: POS_INF_TOKEN) AND imm8[7] then set #IE;
///         ; end fault reporting
///     return dest[31:0];
/// }
///         ; end of FIXUPIMM_SP()
/// VFIXUPIMMSS (EVEX encoded version)
/// IF k1[0] OR *no writemask*
///     THEN DEST[31:0] := FIXUPIMM_SP(DEST[31:0], SRC1[31:0], SRC2[31:0], imm8 [7:0])
///     ELSE
///         IF *merging-masking*
///                     ; merging-masking
///             THEN *DEST[31:0] remains unchanged*
///             ELSE  DEST[31:0] := 0
///                     ; zeroing-masking
///         FI
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// Immediate Control Description:
///                 F01234567+igIurNeF 5 -12.  VFI-#XIUIEPNIFM MSS ImmS#eNIdiaEatNe  Control-# DIeEVscEri pt ionO#NIEE  O#NIEE  Z#EZREO Z#EIREO #ZE
/// ```
#[box_to_static_reference]
pub(super) fn vfixupimmss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VF[,N]MADD132SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form*:
///         DEST.fp16[0] := RoundFPControl(-DEST.fp16[0]*SRC3.fp16[0] + SRC2.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(DEST.fp16[0]*SRC3.fp16[0] + SRC2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// DEST[MAXVL-1:128] := 0
/// VF[,N]MADD213SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form:
///         DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*DEST.fp16[0] + SRC3.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*DEST.fp16[0] + SRC3.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// DEST[MAXVL-1:128] := 0
/// VF[,N]MADD231SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form*:
///         DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*SRC3.fp16[0] + DEST.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*SRC3.fp16[0] + DEST.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o1(), o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(DEST[n+63:n]*SRC3[n+63:n] + SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*DEST[n+63:n] + SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*SRC3[n+63:n] + DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd132pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] + SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] + SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] + DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd132ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := MAXVL-1:128RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd132sd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(DEST[31:0]*SRC3[31:0] + SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*DEST[31:0] + SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*SRC3[31:0] + DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0]] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(DEST[31:0]*SRC3[31:0] + SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*DEST[31:0] + SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*SRC3[31:0] + DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd132ss() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(DEST[n+63:n]*SRC3[n+63:n] + SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*DEST[n+63:n] + SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*SRC3[n+63:n] + DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd213pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] + SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] + SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] + DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd213ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := MAXVL-1:128RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd213sd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(DEST[31:0]*SRC3[31:0] + SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*DEST[31:0] + SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*SRC3[31:0] + DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0]] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(DEST[31:0]*SRC3[31:0] + SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*DEST[31:0] + SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*SRC3[31:0] + DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd213ss() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(DEST[n+63:n]*SRC3[n+63:n] + SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*DEST[n+63:n] + SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*SRC3[n+63:n] + DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd231pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] + SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] + SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 4
/// ELSEIF (VEX.256)
///     MAXNUM := 8
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] + DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd231ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMADD132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := MAXVL-1:128RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
/// DEST[127:63] := DEST[127:63]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd231sd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADD132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(DEST[31:0]*SRC3[31:0] + SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*DEST[31:0] + SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*SRC3[31:0] + DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0]] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(DEST[31:0]*SRC3[31:0] + SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*DEST[31:0] + SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMADD231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*SRC3[31:0] + DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmadd231ss() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMADDCPH dest{k1}, src1, src2 (AVX512)
/// VL = 128, 256, 512
/// KL := VL / 32
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF broadcasting and src2 is memory:
///             tsrc2.fp16[2*i+0] := src2.fp16[0]
///             tsrc2.fp16[2*i+1] := src2.fp16[1]
///         ELSE:
///             tsrc2.fp16[2*i+0] := src2.fp16[2*i+0]
///             tsrc2.fp16[2*i+1] := src2.fp16[2*i+1]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         tmp[2*i+0] := dest.fp16[2*i+0] + src1.fp16[2*i+0] * tsrc2.fp16[2*i+0]
///         tmp[2*i+1] := dest.fp16[2*i+1] + src1.fp16[2*i+1] * tsrc2.fp16[2*i+0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         // non-conjugate version subtracts even term
///         dest.fp16[2*i+0] := tmp[2*i+0] - src1.fp16[2*i+1] * tsrc2.fp16[2*i+1]
///         dest.fp16[2*i+1] := tmp[2*i+1] + src1.fp16[2*i+0] * tsrc2.fp16[2*i+1]
///     ELSE IF *zeroing*:
///         dest.fp16[2*i+0] := 0
///         dest.fp16[2*i+1] := 0
/// DEST[MAXVL-1:VL] := 0
/// VFCMADDCPH dest{k1}, src1, src2 (AVX512)
/// VL = 128, 256, 512
/// KL := VL / 32
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF broadcasting and src2 is memory:
///             tsrc2.fp16[2*i+0] := src2.fp16[0]
///             tsrc2.fp16[2*i+1] := src2.fp16[1]
///         ELSE:
///             tsrc2.fp16[2*i+0] := src2.fp16[2*i+0]
///             tsrc2.fp16[2*i+1] := src2.fp16[2*i+1]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         tmp[2*i+0] := dest.fp16[2*i+0] + src1.fp16[2*i+0] * tsrc2.fp16[2*i+0]
///         tmp[2*i+1] := dest.fp16[2*i+1] + src1.fp16[2*i+1] * tsrc2.fp16[2*i+0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         // conjugate version subtracts odd final term
///         dest.fp16[2*i+0] := tmp[2*i+0] + src1.fp16[2*i+1] * tsrc2.fp16[2*i+1]
///         dest.fp16[2*i+1] := tmp[2*i+1] - src1.fp16[2*i+0] * tsrc2.fp16[2*i+1]
///     ELSE IF *zeroing*:
///         dest.fp16[2*i+0] := 0
///         dest.fp16[2*i+1] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddcph() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMADDCSH dest{k1}, src1, src2 (AVX512)
/// IF k1[0] or *no writemask*:
///     tmp[0] := dest.fp16[0] + src1.fp16[0] * src2.fp16[0]
///     tmp[1] := dest.fp16[1] + src1.fp16[1] * src2.fp16[0]
///     // non-conjugate version subtracts last even term
///     dest.fp16[0] := tmp[0] - src1.fp16[1] * src2.fp16[1]
///     dest.fp16[1] := tmp[1] + src1.fp16[0] * src2.fp16[1]
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
///     dest.fp16[1] := 0
/// DEST[127:32] := src1[127:32] // copy upper part of src1
/// DEST[MAXVL-1:128] := 0
/// VFCMADDCSH dest{k1}, src1, src2 (AVX512)
/// IF k1[0] or *no writemask*:
///     tmp[0] := dest.fp16[0] + src1.fp16[0] * src2.fp16[0]
///     tmp[1] := dest.fp16[1] + src1.fp16[1] * src2.fp16[0]
///     // conjugate version subtracts odd final term
///     dest.fp16[0] := tmp[0] + src1.fp16[1] * src2.fp16[1]
///     dest.fp16[1] := tmp[1] - src1.fp16[0] * src2.fp16[1]
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
///     dest.fp16[1] := 0
/// DEST[127:32] := src1[127:32] // copy upper part of src1
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddcsh() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMADDSUB132PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] + SRC2[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] + SRC2[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(DEST[191:128]*SRC3[191:128] - SRC2[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(DEST[255:192]*SRC3[255:192] + SRC2[255:192]
/// FI
/// VFMADDSUB213PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] + SRC3[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] + SRC3[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*DEST[191:128] - SRC3[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*DEST[255:192] + SRC3[255:192]
/// FI
/// VFMADDSUB231PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] + DEST[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] + DEST[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*SRC3[191:128] - DEST[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*SRC3[255:192] + DEST[255:192]
/// FI
/// VFMADDSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                     FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                     FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                         FI;
///                     ELSE
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub132pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMADDSUB132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * SRC3.fp16[j] - SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * SRC3.fp16[j] + SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
/// // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 - SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 + SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] - SRC3.fp16[j])
///         ELSE
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] + SRC3.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] - t3)
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] + t3)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * SRC3.fp16[j] - DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * SRC3.fp16[j] + DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 - DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 + DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub132ph() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADDSUB132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM :=2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] - SRC2[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(DEST[n+63:n+32]*SRC3[n+63:n+32] + SRC2[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] - SRC3[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*DEST[n+63:n+32] + SRC3[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] - DEST[n+31:n])
///     DEST[n+63:n+32] :=RoundFPControl_MXCSR(SRC2[n+63:n+32]*SRC3[n+63:n+32] + DEST[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) (4, 128), (8, 256),= (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                             ELSE
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub132ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMADDSUB132PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] + SRC2[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] + SRC2[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(DEST[191:128]*SRC3[191:128] - SRC2[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(DEST[255:192]*SRC3[255:192] + SRC2[255:192]
/// FI
/// VFMADDSUB213PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] + SRC3[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] + SRC3[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*DEST[191:128] - SRC3[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*DEST[255:192] + SRC3[255:192]
/// FI
/// VFMADDSUB231PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] + DEST[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] + DEST[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*SRC3[191:128] - DEST[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*SRC3[255:192] + DEST[255:192]
/// FI
/// VFMADDSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                     FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                     FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                         FI;
///                     ELSE
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub213pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMADDSUB132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * SRC3.fp16[j] - SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * SRC3.fp16[j] + SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
/// // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 - SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 + SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] - SRC3.fp16[j])
///         ELSE
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] + SRC3.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] - t3)
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] + t3)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * SRC3.fp16[j] - DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * SRC3.fp16[j] + DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 - DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 + DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub213ph() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADDSUB132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM :=2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] - SRC2[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(DEST[n+63:n+32]*SRC3[n+63:n+32] + SRC2[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] - SRC3[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*DEST[n+63:n+32] + SRC3[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] - DEST[n+31:n])
///     DEST[n+63:n+32] :=RoundFPControl_MXCSR(SRC2[n+63:n+32]*SRC3[n+63:n+32] + DEST[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) (4, 128), (8, 256),= (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                             ELSE
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub213ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMADDSUB132PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] + SRC2[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] + SRC2[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(DEST[191:128]*SRC3[191:128] - SRC2[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(DEST[255:192]*SRC3[255:192] + SRC2[255:192]
/// FI
/// VFMADDSUB213PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] + SRC3[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] + SRC3[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*DEST[191:128] - SRC3[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*DEST[255:192] + SRC3[255:192]
/// FI
/// VFMADDSUB231PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] + DEST[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] + DEST[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*SRC3[191:128] - DEST[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*SRC3[255:192] + DEST[255:192]
/// FI
/// VFMADDSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                     FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                     FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                         FI;
///                     ELSE
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub231pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMADDSUB132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * SRC3.fp16[j] - SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * SRC3.fp16[j] + SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
/// // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 - SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 + SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] - SRC3.fp16[j])
///         ELSE
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] + SRC3.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] - t3)
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] + t3)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * SRC3.fp16[j] - DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * SRC3.fp16[j] + DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 - DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 + DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub231ph() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMADDSUB132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM :=2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] - SRC2[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(DEST[n+63:n+32]*SRC3[n+63:n+32] + SRC2[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] - SRC3[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*DEST[n+63:n+32] + SRC3[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] - DEST[n+31:n])
///     DEST[n+63:n+32] :=RoundFPControl_MXCSR(SRC2[n+63:n+32]*SRC3[n+63:n+32] + DEST[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMADDSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) (4, 128), (8, 256),= (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMADDSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                             ELSE
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmaddsub231ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VF[,N]MSUB132SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form*:
///         DEST.fp16[0] := RoundFPControl(-DEST.fp16[0]*SRC3.fp16[0] - SRC2.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(DEST.fp16[0]*SRC3.fp16[0] - SRC2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// DEST[MAXVL-1:128] := 0
/// VF[,N]MSUB213SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form:
///         DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*DEST.fp16[0] - SRC3.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*DEST.fp16[0] - SRC3.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// DEST[MAXVL-1:128] := 0
/// VF[,N]MSUB231SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form*:
///         DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*SRC3.fp16[0] - DEST.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*SRC3.fp16[0] - DEST.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o1(), o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(DEST[n+63:n]*SRC3[n+63:n] - SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB213PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*DEST[n+63:n] - SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB231PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*SRC3[n+63:n] - DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB132PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB132PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                 FI;
///                 ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
/// +31:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub132pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] - SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] - SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] - DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub132ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub132sd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(DEST[31:0]*SRC3[31:0] - SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*DEST[31:0] - SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*SRC3[63:0] - DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(DEST[31:0]*SRC3[31:0] - SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*DEST[31:0] - SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*SRC3[31:0] - DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub132ss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(DEST[n+63:n]*SRC3[n+63:n] - SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB213PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*DEST[n+63:n] - SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB231PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*SRC3[n+63:n] - DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB132PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB132PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                 FI;
///                 ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
/// +31:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub213pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] - SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] - SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] - DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub213ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub213sd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(DEST[31:0]*SRC3[31:0] - SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*DEST[31:0] - SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*SRC3[63:0] - DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(DEST[31:0]*SRC3[31:0] - SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*DEST[31:0] - SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*SRC3[31:0] - DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub213ss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(DEST[n+63:n]*SRC3[n+63:n] - SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB213PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*DEST[n+63:n] - SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB231PD DEST, SRC2, SRC3 (VEX encoded versions)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(SRC2[n+63:n]*SRC3[n+63:n] - DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB132PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB132PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                 FI;
///                 ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
/// +31:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PD DEST, SRC2, SRC3 (EVEX encoded versions, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub231pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] - SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] - SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] - DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub231ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub231sd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUB132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(DEST[31:0]*SRC3[31:0] - SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*DEST[31:0] - SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(SRC2[31:0]*SRC3[63:0] - DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(DEST[31:0]*SRC3[31:0] - SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*DEST[31:0] - SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFMSUB231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*SRC3[31:0] - DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsub231ss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUBADD132PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] - SRC2[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] - SRC2[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(DEST[191:128]*SRC3[191:128] + SRC2[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(DEST[255:192]*SRC3[255:192] - SRC2[255:192]
/// FI
/// VFMSUBADD213PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] - SRC3[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] - SRC3[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*DEST[191:128] + SRC3[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*DEST[255:192] - SRC3[255:192]
/// FI
/// VFMSUBADD231PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] - DEST[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] - DEST[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*SRC3[191:128] + DEST[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*SRC3[255:192] - DEST[255:192]
/// FI
/// VFMSUBADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFMSUBADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                             ELSE
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd132pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMSUBADD132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j]*SRC3.fp16[j] + SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j]*SRC3.fp16[j] - SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 + SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 - SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0:
/// VFMSUBADD213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] + SRC3.fp16[j])
///         ELSE
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] - SRC3.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] + t3 )
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] - t3 )
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0:
/// VFMSUBADD231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*SRC3.fp16[j] + DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*SRC3.fp16[j] - DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 + DEST.fp16[j] )
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 - DEST.fp16[j] )
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd132ph() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUBADD132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] + SRC2[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(DEST[n+63:n+32]*SRC3[n+63:n+32] -SRC2[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] +SRC3[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*DEST[n+63:n+32] -SRC3[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] + DEST[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*SRC3[n+63:n+32] -DEST[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                         ELSE
///                             DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                     FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                             ELSE
///                             RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd132ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUBADD132PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] - SRC2[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] - SRC2[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(DEST[191:128]*SRC3[191:128] + SRC2[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(DEST[255:192]*SRC3[255:192] - SRC2[255:192]
/// FI
/// VFMSUBADD213PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] - SRC3[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] - SRC3[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*DEST[191:128] + SRC3[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*DEST[255:192] - SRC3[255:192]
/// FI
/// VFMSUBADD231PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] - DEST[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] - DEST[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*SRC3[191:128] + DEST[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*SRC3[255:192] - DEST[255:192]
/// FI
/// VFMSUBADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFMSUBADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                             ELSE
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd213pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMSUBADD132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j]*SRC3.fp16[j] + SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j]*SRC3.fp16[j] - SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 + SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 - SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0:
/// VFMSUBADD213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] + SRC3.fp16[j])
///         ELSE
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] - SRC3.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] + t3 )
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] - t3 )
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0:
/// VFMSUBADD231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*SRC3.fp16[j] + DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*SRC3.fp16[j] - DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 + DEST.fp16[j] )
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 - DEST.fp16[j] )
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd213ph() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUBADD132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] + SRC2[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(DEST[n+63:n+32]*SRC3[n+63:n+32] -SRC2[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] +SRC3[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*DEST[n+63:n+32] -SRC3[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] + DEST[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*SRC3[n+63:n+32] -DEST[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                         ELSE
///                             DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                     FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                             ELSE
///                             RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd213ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUBADD132PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] - SRC2[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] + SRC2[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(DEST[127:64]*SRC3[127:64] - SRC2[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(DEST[191:128]*SRC3[191:128] + SRC2[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(DEST[255:192]*SRC3[255:192] - SRC2[255:192]
/// FI
/// VFMSUBADD213PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] - SRC3[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] + SRC3[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*DEST[127:64] - SRC3[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*DEST[191:128] + SRC3[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*DEST[255:192] - SRC3[255:192]
/// FI
/// VFMSUBADD231PD DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] - DEST[127:64])
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] + DEST[63:0])
///     DEST[127:64] := RoundFPControl_MXCSR(SRC2[127:64]*SRC3[127:64] - DEST[127:64])
///     DEST[191:128] := RoundFPControl_MXCSR(SRC2[191:128]*SRC3[191:128] + DEST[191:128])
///     DEST[255:192] := RoundFPControl_MXCSR(SRC2[255:192]*SRC3[255:192] - DEST[255:192]
/// FI
/// VFMSUBADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] + SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] + SRC2[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[63:0] - SRC2[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(DEST[i+63:i]*SRC3[i+63:i] - SRC2[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFMSUBADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] + SRC3[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[63:0])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*DEST[i+63:i] - SRC3[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                     ELSE DEST[i+63:i] :=
///                         RoundFPControl(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] + DEST[i+63:i])
///                             ELSE
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] + DEST[i+63:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[63:0] - DEST[i+63:i])
///                             ELSE
///                                 DEST[i+63:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+63:i]*SRC3[i+63:i] - DEST[i+63:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd231pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMSUBADD132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j]*SRC3.fp16[j] + SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j]*SRC3.fp16[j] - SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 + SRC2.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(DEST.fp16[j] * t3 - SRC2.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0:
/// VFMSUBADD213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] + SRC3.fp16[j])
///         ELSE
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*DEST.fp16[j] - SRC3.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] + t3 )
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * DEST.fp16[j] - t3 )
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0:
/// VFMSUBADD231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF *j is even:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*SRC3.fp16[j] + DEST.fp16[j])
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j]*SRC3.fp16[j] - DEST.fp16[j])
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PH DEST, SRC2, SRC3 (EVEX encoded versions) when src3 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             t3 := SRC3.fp16[0]
///         ELSE:
///             t3 := SRC3.fp16[j]
///         IF *j is even*:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 + DEST.fp16[j] )
///         ELSE:
///             DEST.fp16[j] := RoundFPControl(SRC2.fp16[j] * t3 - DEST.fp16[j] )
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd231ph() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFMSUBADD132PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(DEST[n+31:n]*SRC3[n+31:n] + SRC2[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(DEST[n+63:n+32]*SRC3[n+63:n+32] -SRC2[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD213PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*DEST[n+31:n] +SRC3[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*DEST[n+63:n+32] -SRC3[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD231PS DEST, SRC2, SRC3
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM -1{
///     n := 64*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(SRC2[n+31:n]*SRC3[n+31:n] + DEST[n+31:n])
///     DEST[n+63:n+32] := RoundFPControl_MXCSR(SRC2[n+63:n+32]*SRC3[n+63:n+32] -DEST[n+63:n+32])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFMSUBADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] + SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] + SRC2[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[31:0] - SRC2[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(DEST[i+31:i]*SRC3[i+31:i] - SRC2[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[31:0])
///                         ELSE
///                             DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] + SRC3[i+31:i])
///                     FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*DEST[i+31:i] - SRC3[31:0])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                     ELSE DEST[i+31:i] :=
///                         RoundFPControl(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFMSUBADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF j *is even*
///                     THEN
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                             RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] + DEST[i+31:i])
///                             ELSE
///                             RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] + DEST[i+31:i])
///                         FI;
///                     ELSE
///                         IF (EVEX.b = 1)
///                             THEN
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[31:0] - DEST[i+31:i])
///                             ELSE
///                                 DEST[i+31:i] :=
///                         RoundFPControl_MXCSR(SRC2[i+31:i]*SRC3[i+31:i] - DEST[i+31:i])
///                         FI;
///                 FI
///         ELSE
///                 IF *merging-masking*
///                                     ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmsubadd231ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(b::mul(o2(), o3()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMULCPH dest{k1}, src1, src2 (AVX512)
/// VL = 128, 256 or 512
/// KL := VL/32
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF broadcasting and src2 is memory:
///             tsrc2.fp16[2*i+0] := src2.fp16[0]
///             tsrc2.fp16[2*i+1] := src2.fp16[1]
///         ELSE:
///             tsrc2.fp16[2*i+0] := src2.fp16[2*i+0]
///             tsrc2.fp16[2*i+1] := src2.fp16[2*i+1]
///     IF k1[i] or *no writemask*:
///         tmp.fp16[2*i+0] := src1.fp16[2*i+0] * tsrc2.fp16[2*i+0]
///         tmp.fp16[2*i+1] := src1.fp16[2*i+1] * tsrc2.fp16[2*i+0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         // non-conjugate version subtracts last even term
///         dest.fp16[2*i+0] := tmp.fp16[2*i+0] - src1.fp16[2*i+1] * tsrc2.fp16[2*i+1]
///         dest.fp16[2*i+1] := tmp.fp16[2*i+1] + src1.fp16[2*i+0] * tsrc2.fp16[2*i+1]
///     ELSE IF *zeroing*:
///         dest.fp16[2*i+0] := 0
///         dest.fp16[2*i+1] := 0
/// DEST[MAXVL-1:VL] := 0
/// VFCMULCPH dest{k1}, src1, src2 (AVX512)
/// VL = 128, 256 or 512
/// KL := VL/32
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF broadcasting and src2 is memory:
///             tsrc2.fp16[2*i+0] := src2.fp16[0]
///             tsrc2.fp16[2*i+1] := src2.fp16[1]
///         ELSE:
///             tsrc2.fp16[2*i+0] := src2.fp16[2*i+0]
///             tsrc2.fp16[2*i+1] := src2.fp16[2*i+1]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         tmp.fp16[2*i+0] := src1.fp16[2*i+0] * tsrc2.fp16[2*i+0]
///         tmp.fp16[2*i+1] := src1.fp16[2*i+1] * tsrc2.fp16[2*i+0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         // conjugate version subtracts odd final term
///         dest.fp16[2*i] := tmp.fp16[2*i+0] +src1.fp16[2*i+1] * tsrc2.fp16[2*i+1]
///         dest.fp16[2*i+1] := tmp.fp16[2*i+1] - src1.fp16[2*i+0] * tsrc2.fp16[2*i+1]
///     ELSE IF *zeroing*:
///         dest.fp16[2*i+0] := 0
///         dest.fp16[2*i+1] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmulcph() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFMULCSH dest{k1}, src1, src2 (AVX512)
/// KL := VL / 32
/// IF k1[0] or *no writemask*:
///     // non-conjugate version subtracts last even term
///     tmp.fp16[0] := src1.fp16[0] * src2.fp16[0]
///     tmp.fp16[1] := src1.fp16[1] * src2.fp16[0]
///     dest.fp16[0] := tmp.fp16[0] - src1.fp16[1] * src2.fp16[1]
///     dest.fp16[1] := tmp.fp16[1] + src1.fp16[0] * src2.fp16[1]
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
///     dest.fp16[1] := 0
/// DEST[127:32] := src1[127:32] // copy upper part of src1
/// DEST[MAXVL-1:128] := 0
/// VFCMULCSH dest{k1}, src1, src2 (AVX512)
/// KL := VL / 32
/// IF k1[0] or *no writemask*:
///     tmp.fp16[0] := src1.fp16[0] * src2.fp16[0]
///     tmp.fp16[1] := src1.fp16[1] * src2.fp16[0]
///     // conjugate version subtracts odd final term
///     dest.fp16[0] := tmp.fp16[0] + src1.fp16[1] * src2.fp16[1]
///     dest.fp16[1] := tmp.fp16[1] - src1.fp16[0] * src2.fp16[1]
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
///     dest.fp16[1] := 0
/// DEST[127:32] := src1[127:32] // copy upper part of src1
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfmulcsh() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VF[,N]MADD132SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form*:
///         DEST.fp16[0] := RoundFPControl(-DEST.fp16[0]*SRC3.fp16[0] + SRC2.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(DEST.fp16[0]*SRC3.fp16[0] + SRC2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// DEST[MAXVL-1:128] := 0
/// VF[,N]MADD213SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form:
///         DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*DEST.fp16[0] + SRC3.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*DEST.fp16[0] + SRC3.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// DEST[MAXVL-1:128] := 0
/// VF[,N]MADD231SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form*:
///         DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*SRC3.fp16[0] + DEST.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*SRC3.fp16[0] + DEST.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), b::mul(o1(), o2())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(DEST[n+63:n]*SRC3[n+63:n]) + SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(SRC2[n+63:n]*DEST[n+63:n]) + SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(SRC2[n+63:n]*SRC3[n+63:n]) + DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(DEST[i+63:i]*SRC3[i+63:i]) + SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[63:0]) + SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[i+63:i]) + SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*SRC3[i+63:i]) + DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[63:0]) + DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[i+63:i]) + DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd132pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (DEST[n+31:n]*SRC3[n+31:n]) + SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (SRC2[n+31:n]*DEST[n+31:n]) + SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (SRC2[n+31:n]*SRC3[n+31:n]) + DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(DEST[i+31:i]*SRC3[i+31:i]) + SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFNMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[31:0]) + SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[i+31:i]) + SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(SRC2[i+31:i]*SRC3[i+31:i]) + DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[31:0]) + DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) + DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd132ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(DEST[63:0]*SRC3[63:0]) + SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*DEST[63:0]) + SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*SRC3[63:0]) + DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (DEST[63:0]*SRC3[63:0]) + SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*DEST[63:0]) + SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*SRC3[63:0]) + DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd132sd() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(DEST[31:0]*SRC3[31:0]) + SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*DEST[31:0]) + SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*SRC3[63:0]) + DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (DEST[31:0]*SRC3[31:0]) + SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*DEST[31:0]) + SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*SRC3[31:0]) + DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd132ss() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(DEST[n+63:n]*SRC3[n+63:n]) + SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(SRC2[n+63:n]*DEST[n+63:n]) + SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(SRC2[n+63:n]*SRC3[n+63:n]) + DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(DEST[i+63:i]*SRC3[i+63:i]) + SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[63:0]) + SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[i+63:i]) + SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*SRC3[i+63:i]) + DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[63:0]) + DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[i+63:i]) + DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd213pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (DEST[n+31:n]*SRC3[n+31:n]) + SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (SRC2[n+31:n]*DEST[n+31:n]) + SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (SRC2[n+31:n]*SRC3[n+31:n]) + DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(DEST[i+31:i]*SRC3[i+31:i]) + SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFNMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[31:0]) + SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[i+31:i]) + SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(SRC2[i+31:i]*SRC3[i+31:i]) + DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[31:0]) + DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) + DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd213ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(DEST[63:0]*SRC3[63:0]) + SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*DEST[63:0]) + SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*SRC3[63:0]) + DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (DEST[63:0]*SRC3[63:0]) + SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*DEST[63:0]) + SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*SRC3[63:0]) + DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd213sd() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(DEST[31:0]*SRC3[31:0]) + SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*DEST[31:0]) + SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*SRC3[63:0]) + DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (DEST[31:0]*SRC3[31:0]) + SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*DEST[31:0]) + SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*SRC3[31:0]) + DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd213ss() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(DEST[n+63:n]*SRC3[n+63:n]) + SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(SRC2[n+63:n]*DEST[n+63:n]) + SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR(-(SRC2[n+63:n]*SRC3[n+63:n]) + DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(DEST[i+63:i]*SRC3[i+63:i]) + SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[63:0]) + SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[i+63:i]) + SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) + SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*SRC3[i+63:i]) + DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[63:0]) + DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[i+63:i]) + DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd231pd() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (DEST[n+31:n]*SRC3[n+31:n]) + SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (SRC2[n+31:n]*DEST[n+31:n]) + SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR(- (SRC2[n+31:n]*SRC3[n+31:n]) + DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(DEST[i+31:i]*SRC3[i+31:i]) + SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFNMADD132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[31:0]) + SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[i+31:i]) + SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) + SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(SRC2[i+31:i]*SRC3[i+31:i]) + DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMADD231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[31:0]) + DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) + DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd231ps() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(DEST[63:0]*SRC3[63:0]) + SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*DEST[63:0]) + SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*SRC3[63:0]) + DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (DEST[63:0]*SRC3[63:0]) + SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*DEST[63:0]) + SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*SRC3[63:0]) + DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd231sd() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "+" symbols represent multiplication and addition with infinite precision inputs and outputs (no
/// rounding).
/// VFNMADD132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(DEST[31:0]*SRC3[31:0]) + SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*DEST[31:0]) + SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*SRC3[63:0]) + DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (DEST[31:0]*SRC3[31:0]) + SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*DEST[31:0]) + SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMADD231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*SRC3[31:0]) + DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmadd231ss() -> &'static [IrStatement] {
    let assignment = assign(b::add(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VF[,N]MSUB132SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form*:
///         DEST.fp16[0] := RoundFPControl(-DEST.fp16[0]*SRC3.fp16[0] - SRC2.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(DEST.fp16[0]*SRC3.fp16[0] - SRC2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// DEST[MAXVL-1:128] := 0
/// VF[,N]MSUB213SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form:
///         DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*DEST.fp16[0] - SRC3.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*DEST.fp16[0] - SRC3.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// DEST[MAXVL-1:128] := 0
/// VF[,N]MSUB231SH DEST, SRC2, SRC3 (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC3 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     IF *negative form*:
///         DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*SRC3.fp16[0] - DEST.fp16[0])
///     ELSE:
///         DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*SRC3.fp16[0] - DEST.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else DEST.fp16[0] remains unchanged
/// //DEST[127:16] remains unchanged
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub() -> &'static [IrStatement] {
    let assignment = assign(u::neg(b::add(b::mul(o1(), o2()), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (DEST[n+63:n]*SRC3[n+63:n]) - SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (SRC2[n+63:n]*DEST[n+63:n]) - SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (SRC2[n+63:n]*SRC3[n+63:n]) - DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(DEST[i+63:i]*SRC3[i+63:i]) - SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[63:0]) - SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[i+63:i]) - SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*SRC3[i+63:i]) - DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[63:0]) - DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[i+63:i]) - DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub132pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (DEST[n+31:n]*SRC3[n+31:n]) - SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (SRC2[n+31:n]*DEST[n+31:n]) - SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (SRC2[n+31:n]*SRC3[n+31:n]) - DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(DEST[i+31:i]*SRC3[i+31:i]) - SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFNMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[31:0]) - SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[i+31:i]) - SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) - DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[31:0]) - DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) - DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub132ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(DEST[63:0]*SRC3[63:0]) - SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*DEST[63:0]) - SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*SRC3[63:0]) - DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (DEST[63:0]*SRC3[63:0]) - SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*DEST[63:0]) - SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*SRC3[63:0]) - DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub132sd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(DEST[31:0]*SRC3[31:0]) - SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*DEST[31:0]) - SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*SRC3[63:0]) - DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (DEST[31:0]*SRC3[31:0]) - SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*DEST[31:0]) - SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*SRC3[31:0]) - DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub132ss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (DEST[n+63:n]*SRC3[n+63:n]) - SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (SRC2[n+63:n]*DEST[n+63:n]) - SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (SRC2[n+63:n]*SRC3[n+63:n]) - DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(DEST[i+63:i]*SRC3[i+63:i]) - SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[63:0]) - SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[i+63:i]) - SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*SRC3[i+63:i]) - DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[63:0]) - DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[i+63:i]) - DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub213pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (DEST[n+31:n]*SRC3[n+31:n]) - SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (SRC2[n+31:n]*DEST[n+31:n]) - SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (SRC2[n+31:n]*SRC3[n+31:n]) - DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(DEST[i+31:i]*SRC3[i+31:i]) - SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFNMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[31:0]) - SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[i+31:i]) - SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) - DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[31:0]) - DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) - DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub213ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(DEST[63:0]*SRC3[63:0]) - SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*DEST[63:0]) - SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*SRC3[63:0]) - DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (DEST[63:0]*SRC3[63:0]) - SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*DEST[63:0]) - SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*SRC3[63:0]) - DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub213sd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(DEST[31:0]*SRC3[31:0]) - SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*DEST[31:0]) - SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*SRC3[63:0]) - DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (DEST[31:0]*SRC3[31:0]) - SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*DEST[31:0]) - SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*SRC3[31:0]) - DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub213ss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (DEST[n+63:n]*SRC3[n+63:n]) - SRC2[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB213PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (SRC2[n+63:n]*DEST[n+63:n]) - SRC3[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB231PD DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 64*i;
///     DEST[n+63:n] := RoundFPControl_MXCSR( - (SRC2[n+63:n]*SRC3[n+63:n]) - DEST[n+63:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(DEST[i+63:i]*SRC3[i+63:i]) - SRC2[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB132PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[63:0]) - SRC2[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+63:i]*SRC3[i+63:i]) - SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*DEST[i+63:i]) - SRC3[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] :=
///                 RoundFPControl(-(SRC2[i+63:i]*SRC3[i+63:i]) - DEST[i+63:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PD DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[63:0]) - DEST[i+63:i])
///                     ELSE
///                         DEST[i+63:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+63:i]*SRC3[i+63:i]) - DEST[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub231pd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (DEST[n+31:n]*SRC3[n+31:n]) - SRC2[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB213PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (SRC2[n+31:n]*DEST[n+31:n]) - SRC3[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB231PS DEST, SRC2, SRC3 (VEX encoded version)
/// IF (VEX.128) THEN
///     MAXNUM := 2
/// ELSEIF (VEX.256)
///     MAXNUM := 4
/// FI
/// For i = 0 to MAXNUM-1 {
///     n := 32*i;
///     DEST[n+31:n] := RoundFPControl_MXCSR( - (SRC2[n+31:n]*SRC3[n+31:n]) - DEST[n+31:n])
/// }
/// IF (VEX.128) THEN
///     DEST[MAXVL-1:128] := 0
/// ELSEIF (VEX.256)
///     DEST[MAXVL-1:256] := 0
/// FI
/// VFNMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl(-(DEST[i+31:i]*SRC3[i+31:i]) - SRC2[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VFNMSUB132PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[31:0]) - SRC2[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(DEST[i+31:i]*SRC3[i+31:i]) - SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB213PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*DEST[i+31:i]) - SRC3[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) - DEST[i+31:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VFNMSUB231PS DEST, SRC2, SRC3 (EVEX encoded version, when src3 operand is a memory source)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[31:0]) - DEST[i+31:i])
///                     ELSE
///                         DEST[i+31:i] :=
///                 RoundFPControl_MXCSR(-(SRC2[i+31:i]*SRC3[i+31:i]) - DEST[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub231ps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(DEST[63:0]*SRC3[63:0]) - SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*DEST[63:0]) - SRC3[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SD DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundFPControl(-(SRC2[63:0]*SRC3[63:0]) - DEST[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB132SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (DEST[63:0]*SRC3[63:0]) - SRC2[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*DEST[63:0]) - SRC3[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SD DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[63:0] := RoundFPControl_MXCSR(- (SRC2[63:0]*SRC3[63:0]) - DEST[63:0])
/// DEST[127:64] := DEST[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub231sd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no
/// rounding).
/// VFNMSUB132SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(DEST[31:0]*SRC3[31:0]) - SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*DEST[31:0]) - SRC3[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SS DEST, SRC2, SRC3 (EVEX encoded version)
/// IF (EVEX.b = 1) and SRC3 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundFPControl(-(SRC2[31:0]*SRC3[63:0]) - DEST[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB132SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (DEST[31:0]*SRC3[31:0]) - SRC2[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB213SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*DEST[31:0]) - SRC3[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// VFNMSUB231SS DEST, SRC2, SRC3 (VEX encoded version)
/// DEST[31:0] := RoundFPControl_MXCSR(- (SRC2[31:0]*SRC3[31:0]) - DEST[31:0])
/// DEST[127:32] := DEST[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfnmsub231ss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(u::neg(b::mul(o2(), o3())), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPClassDP (tsrc[63:0], imm8[7:0]){
///     //* Start checking the source operand for special type *//
///     NegNum := tsrc[63];
///     IF (tsrc[62:52]=07FFh) Then ExpAllOnes := 1; FI;
///     IF (tsrc[62:52]=0h) Then ExpAllZeros := 1;
///     IF (ExpAllZeros AND MXCSR.DAZ) Then
///         MantAllZeros := 1;
///     ELSIF (tsrc[51:0]=0h) Then
///         MantAllZeros := 1;
///     FI;
///     ZeroNumber := ExpAllZeros AND MantAllZeros
///     SignalingBit := tsrc[51];
///     sNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND NOT(SignalingBit); // sNaN
///     qNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND SignalingBit; // qNaN
///     Pzero_res := NOT(NegNum) AND ExpAllZeros AND MantAllZeros; // +0
///     Nzero_res := NegNum AND ExpAllZeros AND MantAllZeros; // -0
///     PInf_res := NOT(NegNum) AND ExpAllOnes AND MantAllZeros; // +Inf
///     NInf_res := NegNum AND ExpAllOnes AND MantAllZeros; // -Inf
///     Denorm_res := ExpAllZeros AND NOT(MantAllZeros); // denorm
///     FinNeg_res := NegNum AND NOT(ExpAllOnes) AND NOT(ZeroNumber); // -finite
///     bResult = ( imm8[0] AND qNaN_res ) OR (imm8[1] AND Pzero_res ) OR
///             ( imm8[2] AND Nzero_res ) OR ( imm8[3] AND PInf_res ) OR
///             ( imm8[4] AND NInf_res ) OR ( imm8[5] AND Denorm_res ) OR
///             ( imm8[6] AND FinNeg_res ) OR ( imm8[7] AND sNaN_res );
///     Return bResult;
/// } //* end of CheckFPClassDP() *//
/// VFPCLASSPD (EVEX Encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b == 1) AND (SRC *is memory*)
///                 THEN
///                     DEST[j] := CheckFPClassDP(SRC1[63:0], imm8[7:0]);
///                 ELSE
///                     DEST[j] := CheckFPClassDP(SRC1[i+63:i], imm8[7:0]);
///             FI;
///         ELSE DEST[j] := 0
///                         ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfpclasspd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def check_fp_class_fp16(tsrc, imm8):
///     negative := tsrc[15]
///     exponent_all_ones := (tsrc[14:10] == 0x1F)
///     exponent_all_zeros := (tsrc[14:10] == 0)
///     mantissa_all_zeros := (tsrc[9:0] == 0)
///     zero := exponent_all_zeros and mantissa_all_zeros
///     signaling_bit := tsrc[9]
///     snan := exponent_all_ones and not(mantissa_all_zeros) and not(signaling_bit)
///     qnan := exponent_all_ones and not(mantissa_all_zeros) and signaling_bit
///     positive_zero := not(negative) and zero
///     negative_zero := negative and zero
///     positive_infinity := not(negative) and exponent_all_ones and mantissa_all_zeros
///     negative_infinity := negative and exponent_all_ones and mantissa_all_zeros
///     denormal := exponent_all_zeros and not(mantissa_all_zeros)
///     finite_negative := negative and not(exponent_all_ones) and not(zero)
///     return (imm8[0] and qnan) OR
///         (imm8[1] and positive_zero) OR
///         (imm8[2] and negative_zero) OR
///         (imm8[3] and positive_infinity) OR
///         (imm8[4] and negative_infinity) OR
///         (imm8[5] and denormal) OR
///         (imm8[6] and finite_negative) OR
///         (imm8[7] and snan)
/// VFPCLASSPH dest{k2}, src, imm8
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR i := 0 to KL-1:
///     IF k2[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := SRC.fp16[0]
///         ELSE:
///             tsrc := SRC.fp16[i]
///         DEST.bit[i] := check_fp_class_fp16(tsrc, imm8)
///     ELSE:
///         DEST.bit[i] := 0
/// DEST[MAXKL-1:kl] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfpclassph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPClassSP (tsrc[31:0], imm8[7:0]){
///     //* Start checking the source operand for special type *//
///     NegNum := tsrc[31];
///     IF (tsrc[30:23]=0FFh) Then ExpAllOnes := 1; FI;
///     IF (tsrc[30:23]=0h) Then ExpAllZeros := 1;
///     IF (ExpAllZeros AND MXCSR.DAZ) Then
///         MantAllZeros := 1;
///     ELSIF (tsrc[22:0]=0h) Then
///         MantAllZeros := 1;
///     FI;
///     ZeroNumber= ExpAllZeros AND MantAllZeros
///     SignalingBit= tsrc[22];
///     sNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND NOT(SignalingBit); // sNaN
///     qNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND SignalingBit; // qNaN
///     Nzero_res := NegNum AND ExpAllZeros AND MantAllZeros; // -0
///     PInf_res := NOT(NegNum) AND ExpAllOnes AND MantAllZeros; // +Inf
///     NInf_res := NegNum AND ExpAllOnes AND MantAllZeros; // -Inf
///     Denorm_res := ExpAllZeros AND NOT(MantAllZeros); // denorm
///     FinNeg_res := NegNum AND NOT(ExpAllOnes) AND NOT(ZeroNumber); // -finite
///     bResult = ( imm8[0] AND qNaN_res ) OR (imm8[1] AND Pzero_res ) OR
///             ( imm8[2] AND Nzero_res ) OR ( imm8[3] AND PInf_res ) OR
///             ( imm8[4] AND NInf_res ) OR ( imm8[5] AND Denorm_res ) OR
///             ( imm8[6] AND FinNeg_res ) OR ( imm8[7] AND sNaN_res );
///     Return bResult;
/// } //* end of CheckSPClassSP() *//
/// VFPCLASSPS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b == 1) AND (SRC *is memory*)
///                 THEN
///                     DEST[j] := CheckFPClassDP(SRC1[31:0], imm8[7:0]);
///                 ELSE
///                     DEST[j] := CheckFPClassDP(SRC1[i+31:i], imm8[7:0]);
///             FI;
///         ELSE  DEST[j] := 0
///                         ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfpclassps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPClassDP (tsrc[63:0], imm8[7:0]){
///     NegNum := tsrc[63];
///     IF (tsrc[62:52]=07FFh) Then ExpAllOnes := 1; FI;
///     IF (tsrc[62:52]=0h) Then ExpAllZeros := 1;
///     IF (ExpAllZeros AND MXCSR.DAZ) Then
///         MantAllZeros := 1;
///     ELSIF (tsrc[51:0]=0h) Then
///         MantAllZeros := 1;
///     FI;
///     ZeroNumber := ExpAllZeros AND MantAllZeros
///     SignalingBit := tsrc[51];
///     sNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND NOT(SignalingBit); // sNaN
///     qNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND SignalingBit; // qNaN
///     Pzero_res := NOT(NegNum) AND ExpAllZeros AND MantAllZeros; // +0
///     Nzero_res := NegNum AND ExpAllZeros AND MantAllZeros; // -0
///     PInf_res := NOT(NegNum) AND ExpAllOnes AND MantAllZeros; // +Inf
///     NInf_res := NegNum AND ExpAllOnes AND MantAllZeros; // -Inf
///     Denorm_res := ExpAllZeros AND NOT(MantAllZeros); // denorm
///     FinNeg_res := NegNum AND NOT(ExpAllOnes) AND NOT(ZeroNumber); // -finite
///     bResult = ( imm8[0] AND qNaN_res ) OR (imm8[1] AND Pzero_res ) OR
///             ( imm8[2] AND Nzero_res ) OR ( imm8[3] AND PInf_res ) OR
///             ( imm8[4] AND NInf_res ) OR ( imm8[5] AND Denorm_res ) OR
///             ( imm8[6] AND FinNeg_res ) OR ( imm8[7] AND sNaN_res );
///     Return bResult;
/// VFPCLASSSD (EVEX encoded version)
/// IF k1[0] OR *no writemask*
///     THEN DEST[0] :=
///             CheckFPClassDP(SRC1[63:0], imm8[7:0])
///     ELSE  DEST[0] := 0
///                 ; zeroing-masking only
/// FI;
/// DEST[MAX_KL-1:1] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfpclasssd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VFPCLASSSH dest{k2}, src, imm8
/// IF k2[0] or *no writemask*:
///     DEST.bit[0] := check_fp_class_fp16(src.fp16[0], imm8) // see VFPCLASSPH
/// ELSE:
///     DEST.bit[0] := 0
/// DEST[MAXKL-1:1] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfpclasssh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CheckFPClassSP (tsrc[31:0], imm8[7:0]){
///     //* Start checking the source operand for special type *//
///     NegNum := tsrc[31];
///     IF (tsrc[30:23]=0FFh) Then ExpAllOnes := 1; FI;
///     IF (tsrc[30:23]=0h) Then ExpAllZeros := 1;
///     IF (ExpAllZeros AND MXCSR.DAZ) Then
///         MantAllZeros := 1;
///     ELSIF (tsrc[22:0]=0h) Then
///         MantAllZeros := 1;
///     FI;
///     ZeroNumber= ExpAllZeros AND MantAllZeros
///     SignalingBit= tsrc[22];
///     sNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND NOT(SignalingBit); // sNaN
///     qNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND SignalingBit; // qNaN
///     Pzero_res := NOT(NegNum) AND ExpAllZeros AND MantAllZeros; // +0
///     Nzero_res := NegNum AND ExpAllZeros AND MantAllZeros; // -0
///     PInf_res := NOT(NegNum) AND ExpAllOnes AND MantAllZeros; // +Inf
///     NInf_res := NegNum AND ExpAllOnes AND MantAllZeros; // -Inf
///     Denorm_res := ExpAllZeros AND NOT(MantAllZeros); // denorm
///     FinNeg_res := NegNum AND NOT(ExpAllOnes) AND NOT(ZeroNumber); // -finite
///     bResult = ( imm8[0] AND qNaN_res ) OR (imm8[1] AND Pzero_res ) OR
///             ( imm8[2] AND Nzero_res ) OR ( imm8[3] AND PInf_res ) OR
///             ( imm8[4] AND NInf_res ) OR ( imm8[5] AND Denorm_res ) OR
///             ( imm8[6] AND FinNeg_res ) OR ( imm8[7] AND sNaN_res );
/// } //* end of CheckSPClassSP() *//
/// VFPCLASSSS (EVEX encoded version)
/// IF k1[0] OR *no writemask*
///     THEN DEST[0] :=
///             CheckFPClassSP(SRC1[31:0], imm8[7:0])
///     ELSE  DEST[0] := 0
///                 ; zeroing-masking only
/// FI;
/// DEST[MAX_KL-1:1] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vfpclassss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a vector register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VGATHERDPS (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j]
///         THEN DEST[i+31:i] :=
///             MEM[BASE_ADDR +
///                 SignExtend(VINDEX[i+31:i]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+31:i] := remains unchanged*
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// VGATHERDPD (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j]
///         THEN DEST[i+63:i] := MEM[BASE_ADDR +
///                 SignExtend(VINDEX[k+31:k]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+63:i] := remains unchanged*
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgatherdpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a vector register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VGATHERDPS (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j]
///         THEN DEST[i+31:i] :=
///             MEM[BASE_ADDR +
///                 SignExtend(VINDEX[i+31:i]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+31:i] := remains unchanged*
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// VGATHERDPD (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j]
///         THEN DEST[i+63:i] := MEM[BASE_ADDR +
///                 SignExtend(VINDEX[k+31:k]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+63:i] := remains unchanged*
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgatherdps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VGATHERQPS (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///             MEM[BASE_ADDR + (VINDEX[k+63:k]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+31:i] := remains unchanged*
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL/2] := 0
/// VGATHERQPD (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := MEM[BASE_ADDR + (VINDEX[i+63:i]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+63:i] := remains unchanged*
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgatherqpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VGATHERQPS (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///             MEM[BASE_ADDR + (VINDEX[k+63:k]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+31:i] := remains unchanged*
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL/2] := 0
/// VGATHERQPD (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := MEM[BASE_ADDR + (VINDEX[i+63:i]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+63:i] := remains unchanged*
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgatherqps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// NormalizeExpTinyDPFP(SRC[63:0])
/// {
///     // Jbit is the hidden integral bit of a floating-point number. In case of denormal number it has the value of ZERO.
///     Src.Jbit := 0;
///     Dst.exp := 1;
///     Dst.fraction := SRC[51:0];
///     WHILE(Src.Jbit = 0)
///     {
///         Src.Jbit := Dst.fraction[51];
///                                     // Get the fraction MSB
///         Dst.fraction := Dst.fraction << 1;
///                                         // One bit shift left
///         Dst.exp--;
///                             // Decrement the exponent
///     }
///     Dst.fraction := 0;
///                             // zero out fraction bits
///     Dst.sign := 1;
///                             // Return negative sign
///     TMP[63:0] := MXCSR.DAZ? 0 : (Dst.sign << 63) OR (Dst.exp << 52) OR (Dst.fraction);
///     Return (TMP[63:0]);
/// }
/// ConvertExpDPFP(SRC[63:0])
/// {
///     Src.sign := 0;
///                             // Zero out sign bit
///     Src.exp := SRC[62:52];
///     Src.fraction := SRC[51:0];
///     // Check for NaN
///     IF (SRC = NaN)
///     {
///         IF ( SRC = SNAN ) SET IE;
///         Return QNAN(SRC);
///     }
///     // Check for +INF
///     IF (Src = +INF) RETURN (Src);
///     // check if zero operand
///     IF ((Src.exp = 0) AND ((Src.fraction = 0) OR (MXCSR.DAZ = 1))) Return (-INF);
///     }
///     ELSE
///                     // check if denormal operand (notice that MXCSR.DAZ = 0)
///     {
///         IF ((Src.exp = 0) AND (Src.fraction != 0))
///         {
///                 TMP[63:0] := NormalizeExpTinyDPFP(SRC[63:0]);
///                                         // Get Normalized Exponent
///                 Set #DE
///         }
///         ELSE
///                         // exponent value is correct
///         {
///                 TMP[63:0] := (Src.sign << 63) OR (Src.exp << 52) OR (Src.fraction);
///         }
///         TMP := SAR(TMP, 52);
///                                 // Shift Arithmetic Right
///         TMP := TMP - 1023;
///                                 // Subtract Bias
///         Return CvtI2D(TMP);
///                                 // Convert INT to double precision floating-point number
///     }
/// }
/// VGETEXPPD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC *is memory*)
///                     THEN
///                         DEST[i+63:i] :=
///                 ConvertExpDPFP(SRC[63:0])
///                     ELSE
///                         DEST[i+63:i] :=
///                 ConvertExpDPFP(SRC[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                                         ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetexppd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def normalize_exponent_tiny_fp16(src):
///     jbit := 0
///     // src & dst are FP16 numbers with sign(1b), exp(5b) and fraction (10b) fields
///     dst.exp := 1
///                 // write bits 14:10
///     dst.fraction := src.fraction // copy bits 9:0
///     while jbit == 0:
///         jbit := dst.fraction[9]
///                 // msb of the fraction
///         dst.fraction := dst.fraction << 1
///         dst.exp := dst.exp - 1
///     dst.fraction := 0
///     return dst
/// def getexp_fp16(src):
///     src.sign := 0
///                 // make positive
///     exponent_all_ones := (src[14:10] == 0x1F)
///     exponent_all_zeros := (src[14:10] == 0)
///     mantissa_all_zeros := (src[9:0] == 0)
///     zero := exponent_all_zeros and mantissa_all_zeros
///     signaling_bit := src[9]
///     nan := exponent_all_ones and not(mantissa_all_zeros)
///     snan := nan and not(signaling_bit)
///     qnan := nan and signaling_bit
///     positive_infinity := not(negative) and exponent_all_ones and mantissa_all_zeros
///     denormal := exponent_all_zeros and not(mantissa_all_zeros)
///     if nan:
///         if snan:
///             MXCSR.IE := 1
///         return qnan(src)
///                 // convert snan to a qnan
///     if positive_infinity:
///         return src
///     if zero:
///         return -INF
///     if denormal:
///         tmp := normalize_exponent_tiny_fp16(src)
///         MXCSR.DE := 1
///     else:
///         tmp := src
///     tmp := SAR(tmp, 10)
///                 // shift arithmetic right
///     tmp := tmp - 15
///                 // subtract bias
///     return convert_integer_to_fp16(tmp)
/// VGETEXPPH dest{k1}, src
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.fp16[0]
///         ELSE:
///             tsrc := src.fp16[i]
///         DEST.fp16[i] := getexp_fp16(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[i] := 0
///     //else DEST.fp16[i] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetexpph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// NormalizeExpTinySPFP(SRC[31:0])
/// {
///     // Jbit is the hidden integral bit of a floating-point number. In case of denormal number it has the value of ZERO.
///     Src.Jbit := 0;
///     Dst.exp := 1;
///     Dst.fraction := SRC[22:0];
///     WHILE(Src.Jbit = 0)
///     {
///         Src.Jbit := Dst.fraction[22];
///                                     // Get the fraction MSB
///         Dst.fraction := Dst.fraction << 1;
///                                     // One bit shift left
///         Dst.exp--;
///                             // Decrement the exponent
///     }
///     Dst.fraction := 0;
///                             // zero out fraction bits
///     Dst.sign := 1;
///                             // Return negative sign
///     TMP[31:0] := MXCSR.DAZ? 0 : (Dst.sign << 31) OR (Dst.exp << 23) OR (Dst.fraction);
///     Return (TMP[31:0]);
/// }
/// ConvertExpSPFP(SRC[31:0])
/// {
///     Src.sign := 0;
///                             // Zero out sign bit
///     Src.exp := SRC[30:23];
///     Src.fraction := SRC[22:0];
///     // Check for NaN
///     IF (SRC = NaN)
///     {
///         IF ( SRC = SNAN ) SET IE;
///         Return QNAN(SRC);
///     }
///     // Check for +INF
///     IF (Src = +INF) RETURN (Src);
///     // check if zero operand
///     IF ((Src.exp = 0) AND ((Src.fraction = 0) OR (MXCSR.DAZ = 1))) Return (-INF);
///     }
///     ELSE
///                     // check if denormal operand (notice that MXCSR.DAZ = 0)
///         IF ((Src.exp = 0) AND (Src.fraction != 0))
///         {
///                 TMP[31:0] := NormalizeExpTinySPFP(SRC[31:0]);
///                                         // Get Normalized Exponent
///                 Set #DE
///         }
///         ELSE
///                         // exponent value is correct
///         {
///                 TMP[31:0] := (Src.sign << 31) OR (Src.exp << 23) OR (Src.fraction);
///         }
///         TMP := SAR(TMP, 23);
///                                 // Shift Arithmetic Right
///         TMP := TMP - 127;
///                                 // Subtract Bias
///         Return CvtI2S(TMP);
///                                 // Convert INT to single precision floating-point number
///     }
/// }
/// VGETEXPPS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC *is memory*)
///                     THEN
///                         DEST[i+31:i] :=
///                 ConvertExpSPFP(SRC[31:0])
///                     ELSE
///                         DEST[i+31:i] :=
///                 ConvertExpSPFP(SRC[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                                         ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetexpps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// // NormalizeExpTinyDPFP(SRC[63:0]) is defined in the Operation section of VGETEXPPD
/// // ConvertExpDPFP(SRC[63:0]) is defined in the Operation section of VGETEXPPD
/// VGETEXPSD (EVEX encoded version)
/// IF k1[0] OR *no writemask*
///     THEN DEST[63:0] :=
///                 ConvertExpDPFP(SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[63:0] := 0
///             FI
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetexpsd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VGETEXPSH dest{k1}, src1, src2
/// IF k1[0] or *no writemask*:
///     DEST.fp16[0] := getexp_fp16(src2.fp16[0]) // see VGETEXPPH
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// //else DEST.fp16[0] remains unchanged
/// DEST[127:16] := src1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetexpsh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// // NormalizeExpTinySPFP(SRC[31:0]) is defined in the Operation section of VGETEXPPS
/// // ConvertExpSPFP(SRC[31:0]) is defined in the Operation section of VGETEXPPS
/// VGETEXPSS (EVEX encoded version)
/// IF k1[0] OR *no writemask*
///     THEN DEST[31:0] :=
///                 ConvertExpDPFP(SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[31:0]:= 0
///                 FI
///     FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetexpss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def getmant_fp64(src, sign_control, normalization_interval):
///     bias := 1023
///     dst.sign := sign_control[0] ? 0 : src.sign
///     signed_one := sign_control[0] ? +1.0 : -1.0
///     dst.exp := src.exp
///     dst.fraction := src.fraction
///     zero := (dst.exp = 0) and ((dst.fraction = 0) or (MXCSR.DAZ=1))
///     denormal := (dst.exp = 0) and (dst.fraction != 0) and (MXCSR.DAZ=0)
///     infinity := (dst.exp = 0x7FF) and (dst.fraction = 0)
///     nan := (dst.exp = 0x7FF) and (dst.fraction != 0)
///     src_signaling := src.fraction[51]
///     snan := nan and (src_signaling = 0)
///     positive := (src.sign = 0)
///     negative := (src.sign = 1)
///     if nan:
///         if snan:
///             MXCSR.IE := 1
///         return qnan(src)
///     if positive and (zero or infinity):
///         return 1.0
///     if negative:
///         if zero:
///             return signed_one
///         if infinity:
///             if sign_control[1]:
///                 MXCSR.IE := 1
///                 return QNaN_Indefinite
///             return signed_one
///         if sign_control[1]:
///             MXCSR.IE := 1
///             return QNaN_Indefinite
///     if denormal:
///         jbit := 0
///         dst.exp := bias
///         while jbit = 0:
///             jbit := dst.fraction[51]
///             dst.fraction := dst.fraction << 1
///             dst.exp : = dst.exp - 1
///         MXCSR.DE := 1
///     unbiased_exp := dst.exp - bias
///     odd_exp := unbiased_exp[0]
///     signaling_bit := dst.fraction[51]
///     if normalization_interval = 0b00:
///         dst.exp := bias
///     else if normalization_interval = 0b01:
///         dst.exp := odd_exp ? bias-1 : bias
///     else if normalization_interval = 0b10:
///         dst.exp := bias-1
///     else if normalization_interval = 0b11:
///         dst.exp := signaling_bit ? bias-1 : bias
///     return dst
/// VGETMANTPD (EVEX Encoded Versions)
/// VGETMANTPD dest{k1}, src, imm8
/// VL = 128, 256, or 512
/// KL := VL / 64
/// sign_control := imm8[3:2]
/// normalization_interval := imm8[1:0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.double[0]
///         ELSE:
///             tsrc := src.double[i]
///         DEST.double[i] := getmant_fp64(tsrc, sign_control, normalization_interval)
///     ELSE IF *zeroing*:
///         DEST.double[i] := 0
///     //else DEST.double[i] remains unchanged
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetmantpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def getmant_fp16(src, sign_control, normalization_interval):
///     bias := 15
///     dst.sign := sign_control[0] ? 0 : src.sign
///     signed_one := sign_control[0] ? +1.0 : -1.0
///     dst.exp := src.exp
///     dst.fraction := src.fraction
///     zero := (dst.exp = 0) and (dst.fraction = 0)
///     denormal := (dst.exp = 0) and (dst.fraction != 0)
///     infinity := (dst.exp = 0x1F) and (dst.fraction = 0)
///     nan := (dst.exp = 0x1F) and (dst.fraction != 0)
///     src_signaling := src.fraction[9]
///     snan := nan and (src_signaling = 0)
///     positive := (src.sign = 0)
///     negative := (src.sign = 1)
///     if nan:
///         if snan:
///             MXCSR.IE := 1
///         return qnan(src)
///     if positive and (zero or infinity):
///         return 1.0
///     if negative:
///         if zero:
///             return signed_one
///         if infinity:
///             if sign_control[1]:
///                 MXCSR.IE := 1
///                 return QNaN_Indefinite
///             return signed_one
///         if sign_control[1]:
///             MXCSR.IE := 1
///             return QNaN_Indefinite
///     if denormal:
///         jbit := 0
///         dst.exp := bias
///                     // set exponent to bias value
///         while jbit = 0:
///             jbit := dst.fraction[9]
///             dst.fraction := dst.fraction << 1
///             dst.exp : = dst.exp - 1
///         MXCSR.DE := 1
///     unbaiased_exp := dst.exp - bias
///     odd_exp := unbaiased_exp[0]
///     signaling_bit := dst.fraction[9]
///     if normalization_interval = 0b00:
///         dst.exp := bias
///     else if normalization_interval = 0b01:
///         dst.exp := odd_exp ? bias-1 : bias
///     else if normalization_interval = 0b10:
///         dst.exp := bias-1
///     else if normalization_interval = 0b11:
///         dst.exp := signaling_bit ? bias-1 : bias
///     return dst
/// VGETMANTPH dest{k1}, src, imm8
/// VL = 128, 256 or 512
/// KL := VL/16
/// sign_control := imm8[3:2]
/// normalization_interval := imm8[1:0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.fp16[0]
///         ELSE:
///             tsrc := src.fp16[i]
///         DEST.fp16[i] := getmant_fp16(tsrc, sign_control, normalization_interval)
///     ELSE IF *zeroing*:
///         DEST.fp16[i] := 0
///     //else DEST.fp16[i] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetmantph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def getmant_fp32(src, sign_control, normalization_interval):
///     bias := 127
///     dst.sign := sign_control[0] ? 0 : src.sign
///     signed_one := sign_control[0] ? +1.0 : -1.0
///     dst.exp := src.exp
///     dst.fraction := src.fraction
///     zero := (dst.exp = 0) and ((dst.fraction = 0) or (MXCSR.DAZ=1))
///     denormal := (dst.exp = 0) and (dst.fraction != 0) and (MXCSR.DAZ=0)
///     infinity := (dst.exp = 0xFF) and (dst.fraction = 0)
///     nan := (dst.exp = 0xFF) and (dst.fraction != 0)
///     src_signaling := src.fraction[22]
///     snan := nan and (src_signaling = 0)
///     positive := (src.sign = 0)
///     negative := (src.sign = 1)
///     if nan:
///         if snan:
///             MXCSR.IE := 1
///         return qnan(src)
///     if positive and (zero or infinity):
///         return 1.0
///     if negative:
///         if zero:
///             return signed_one
///         if infinity:
///             if sign_control[1]:
///                 MXCSR.IE := 1
///                 return QNaN_Indefinite
///             return signed_one
///         if sign_control[1]:
///             MXCSR.IE := 1
///             return QNaN_Indefinite
///     if denormal:
///         jbit := 0
///         dst.exp := bias
///         while jbit = 0:
///             jbit := dst.fraction[22]
///             dst.fraction := dst.fraction << 1
///             dst.exp : = dst.exp - 1
///         MXCSR.DE := 1
///     unbiased_exp := dst.exp - bias
///     odd_exp  := unbiased_exp[0]
///     signaling_bit := dst.fraction[22]
///     if normalization_interval = 0b00:
///         dst.exp := bias
///     else if normalization_interval = 0b01:
///         dst.exp := odd_exp ? bias-1 : bias
///     else if normalization_interval = 0b10:
///         dst.exp := bias-1
///     else if normalization_interval = 0b11:
///     return dst
/// VGETMANTPS (EVEX encoded versions)
/// VGETMANTPS dest{k1}, src, imm8
/// VL = 128, 256, or 512
/// KL := VL / 32
/// sign_control := imm8[3:2]
/// normalization_interval := imm8[1:0]
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.float[0]
///         ELSE:
///             tsrc := src.float[i]
///         DEST.float[i] := getmant_fp32(tsrc, sign_control, normalization_interval)
///     ELSE IF *zeroing*:
///         DEST.float[i] := 0
///     //else DEST.float[i] remains unchanged
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetmantps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// // getmant_fp64(src, sign_control, normalization_interval) is defined in the operation section of VGETMANTPD
/// VGETMANTSD (EVEX encoded version)
/// SignCtrl[1:0] := IMM8[3:2];
/// Interv[1:0] := IMM8[1:0];
/// IF k1[0] OR *no writemask*
///     THEN DEST[63:0] :=
///                 getmant_fp64(src, sign_control, normalization_interval)
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[63:0] := 0
///             FI
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetmantsd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VGETMANTSH dest{k1}, src1, src2, imm8
/// sign_control := imm8[3:2]
/// normalization_interval := imm8[1:0]
/// IF k1[0] or *no writemask*:
///     dest.fp16[0] := getmant_fp16(src2.fp16[0], // see VGETMANTPH
///         sign_control,
///         normalization_interval)
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
/// //else dest.fp16[0] remains unchanged
/// DEST[127:16] := src1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vgetmantsh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VINSERTI32x4 (EVEX encoded versions)
/// (KL, VL) = (8, 256), (16, 512)
/// TEMP_DEST[VL-1:0] := SRC1[VL-1:0]
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0:S]=  RC2[127:0]
///         1: TMP_DEST[255:128:S]=  RC2[127:0]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0]: = SRC2[127:0]
///         01: TMP_DEST[255:128]: = SRC2[127:0]
///         10: TMP_DEST[383:256]: = SRC2[127:0]
///         11: TMP_DEST[511:384]: = SRC2[127:0]
///     ESAC.
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VINSERTI64x2 (EVEX encoded versions)
/// (KL, VL) = (4, 256), (8, 512)
/// TEMP_DEST[VL-1:0] := SRC1[VL-1:0]
/// IF VL = 256
///     CASE (imm8[0]) OF
///         0: TMP_DEST[127:0:S]=  RC2[127:0]
///         1: TMP_DEST[255:128:S]=  RC2[127:0]
///     ESAC.
/// FI;
/// IF VL = 512
///     CASE (imm8[1:0]) OF
///         00: TMP_DEST[127:0]: = SRC2[127:0]
///         01: TMP_DEST[255:128]: = SRC2[127:0]
///         10: TMP_DEST[383:256]: = SRC2[127:0]
///         11: TMP_DEST[511:384]: = SRC2[127:0]
///     ESAC.
/// FI;
/// FOR j := 0 TO KL-1
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VINSERTI32x8 (EVEX.U1.512 encoded version)
/// TEMP_DEST[VL-1:0] := SRC1[VL-1:0]
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC2[255:0]
///     1: TMP_DEST[511:256] := SRC2[255:0]
/// ESAC.
/// FOR j := 0 TO 15
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VINSERTI64x4 (EVEX.512 encoded version)
/// VL = 512
/// TEMP_DEST[VL-1:0] := SRC1[VL-1:0]
/// CASE (imm8[0]) OF
///     0: TMP_DEST[255:0] := SRC2[255:0]
///     1: TMP_DEST[511:256] := SRC2[255:0]
/// ESAC.
/// FOR j := 0 TO 7
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// VINSERTI128
/// TEMP[255:0] := SRC1[255:0]
/// CASE (imm8[0]) OF
///     0: TEMP[127:0] := SRC2[127:0]
///     1: TEMP[255:128] := SRC2[127:0]
/// ESAC
/// DEST := TEMP
/// ```
#[box_to_static_reference]
pub(super) fn vgetmantss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMASKMOVPS -128-bit load
/// DEST[31:0] := IF (SRC1[31]) Load_32(mem) ELSE 0
/// DEST[63:32] := IF (SRC1[63]) Load_32(mem + 4) ELSE 0
/// DEST[95:64] := IF (SRC1[95]) Load_32(mem + 8) ELSE 0
/// DEST[127:97] := IF (SRC1[127]) Load_32(mem + 12) ELSE 0
/// DEST[MAXVL-1:128] := 0
/// VMASKMOVPS - 256-bit load
/// DEST[31:0] := IF (SRC1[31]) Load_32(mem) ELSE 0
/// DEST[63:32] := IF (SRC1[63]) Load_32(mem + 4) ELSE 0
/// DEST[95:64] := IF (SRC1[95]) Load_32(mem + 8) ELSE 0
/// DEST[127:96] := IF (SRC1[127]) Load_32(mem + 12) ELSE 0
/// DEST[159:128] := IF (SRC1[159]) Load_32(mem + 16) ELSE 0
/// DEST[191:160] := IF (SRC1[191]) Load_32(mem + 20) ELSE 0
/// DEST[223:192] := IF (SRC1[223]) Load_32(mem + 24) ELSE 0
/// DEST[255:224] := IF (SRC1[255]) Load_32(mem + 28) ELSE 0
/// VMASKMOVPD - 128-bit load
/// DEST[63:0] := IF (SRC1[63]) Load_64(mem) ELSE 0
/// DEST[127:64] := IF (SRC1[127]) Load_64(mem + 16) ELSE 0
/// DEST[MAXVL-1:128] := 0
/// VMASKMOVPD - 256-bit load
/// DEST[63:0] := IF (SRC1[63]) Load_64(mem) ELSE 0
/// DEST[127:64] := IF (SRC1[127]) Load_64(mem + 8) ELSE 0
/// DEST[195:128] := IF (SRC1[191]) Load_64(mem + 16) ELSE 0
/// DEST[255:196] := IF (SRC1[255]) Load_64(mem + 24) ELSE 0
/// VMASKMOVPS - 128-bit store
/// IF (SRC1[31]) DEST[31:0] := SRC2[31:0]
/// IF (SRC1[63]) DEST[63:32] := SRC2[63:32]
/// IF (SRC1[95]) DEST[95:64] := SRC2[95:64]
/// IF (SRC1[127]) DEST[127:96] := SRC2[127:96]
/// VMASKMOVPS - 256-bit store
/// IF (SRC1[31]) DEST[31:0] := SRC2[31:0]
/// IF (SRC1[63]) DEST[63:32] := SRC2[63:32]
/// IF (SRC1[95]) DEST[95:64] := SRC2[95:64]
/// IF (SRC1[127]) DEST[127:96] := SRC2[127:96]
/// IF (SRC1[159]) DEST[159:128] :=SRC2[159:128]
/// IF (SRC1[191]) DEST[191:160] := SRC2[191:160]
/// IF (SRC1[223]) DEST[223:192] := SRC2[223:192]
/// IF (SRC1[255]) DEST[255:224] := SRC2[255:224]
/// VMASKMOVPD - 128-bit store
/// IF (SRC1[63]) DEST[63:0] := SRC2[63:0]
/// IF (SRC1[127]) DEST[127:64] := SRC2[127:64]
/// VMASKMOVPD - 256-bit store
/// IF (SRC1[63]) DEST[63:0] := SRC2[63:0]
/// IF (SRC1[127]) DEST[127:64] := SRC2[127:64]
/// IF (SRC1[191]) DEST[191:128] := SRC2[191:128]
/// IF (SRC1[255]) DEST[255:192] := SRC2[255:192]
/// ```
#[box_to_static_reference]
pub(super) fn vmaskmov() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def MAX(SRC1, SRC2):
///     IF (SRC1 = 0.0) and (SRC2 = 0.0):
///         DEST := SRC2
///     ELSE IF (SRC1 = NaN):
///         DEST := SRC2
///     ELSE IF (SRC2 = NaN):
///         DEST := SRC2
///     ELSE IF (SRC1 > SRC2):
///         DEST := SRC1
///     ELSE:
///         DEST := SRC2
/// VMAXPH dest, src1, src2
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             tsrc2 := SRC2.fp16[0]
///         ELSE:
///             tsrc2 := SRC2.fp16[j]
///         DEST.fp16[j] := MAX(SRC1.fp16[j], tsrc2)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vmaxph() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def MAX(SRC1, SRC2):
///     IF (SRC1 = 0.0) and (SRC2 = 0.0):
///         DEST := SRC2
///     ELSE IF (SRC1 = NaN):
///         DEST := SRC2
///     ELSE IF (SRC2 = NaN):
///         DEST := SRC2
///     ELSE IF (SRC1 > SRC2):
///         DEST := SRC1
///     ELSE:
///         DEST := SRC2
/// VMAXSH dest, src1, src2
/// IF k1[0] OR *no writemask*:
///     DEST.fp16[0] := MAX(SRC1.fp16[0], SRC2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else dest.fp16[j] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vmaxsh() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def MIN(SRC1, SRC2):
///     IF (SRC1 = 0.0) and (SRC2 = 0.0):
///         DEST := SRC2
///     ELSE IF (SRC1 = NaN):
///         DEST := SRC2
///     ELSE IF (SRC2 = NaN):
///         DEST := SRC2
///     ELSE IF (SRC1 < SRC2):
///         DEST := SRC1
///     ELSE:
///         DEST := SRC2
/// VMINPH dest, src1, src2
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             tsrc2 := SRC2.fp16[0]
///         ELSE:
///             tsrc2 := SRC2.fp16[j]
///         DEST.fp16[j] := MIN(SRC1.fp16[j], tsrc2)
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vminph() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def MIN(SRC1, SRC2):
///     IF (SRC1 = 0.0) and (SRC2 = 0.0):
///         DEST := SRC2
///     ELSE IF (SRC1 = NaN):
///         DEST := SRC2
///     ELSE IF (SRC2 = NaN):
///         DEST := SRC2
///     ELSE IF (SRC1 < SRC2):
///         DEST := SRC1
///     ELSE:
///         DEST := SRC2
/// VMINSH dest, src1, src2
/// IF k1[0] OR *no writemask*:
///     DEST.fp16[0] := MIN(SRC1.fp16[0], SRC2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else dest.fp16[j] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vminsh() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVDQA32 (EVEX Encoded Versions, Register-Copy Form)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[i+31:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE  DEST[i+31:i] := 0
///                     ; zeroing-masking
///             FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQA32 (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[i+31:i]
///         ELSE *DEST[i+31:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVDQA32 (EVEX Encoded Versions, Load-Form)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[i+31:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE  DEST[i+31:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQA64 (EVEX Encoded Versions, Register-Copy Form)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[i+63:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE  DEST[i+63:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQA64 (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[i+63:i]
///         ELSE *DEST[i+63:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVDQA64 (EVEX Encoded Versions, Load-Form)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[i+63:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE  DEST[i+63:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQA (VEX.256 Encoded Version, Load - and Register Copy)
/// DEST[255:0] := SRC[255:0]
/// DEST[MAXVL-1:256] := 0
/// VMOVDQA (VEX.256 Encoded Version, Store-Form)
/// DEST[255:0] := SRC[255:0]
/// VMOVDQA (VEX.128 Encoded Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] := 0
/// VMOVDQA (128-bit Load- and Register-Copy- Form Legacy SSE Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// (V)MOVDQA (128-bit Store-Form Version)
/// DEST[127:0] := SRC[127:0]
/// ```
#[box_to_static_reference]
pub(super) fn vmovdqa32() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVDQU8 (EVEX Encoded Versions, Register-Copy Form)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC[i+7:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE  DEST[i+7:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQU8 (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] :=
///             SRC[i+7:i]
///         ELSE *DEST[i+7:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVDQU8 (EVEX Encoded Versions, Load-Form)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC[i+7:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE  DEST[i+7:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQU16 (EVEX Encoded Versions, Register-Copy Form)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC[i+15:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE  DEST[i+15:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQU16 (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] :=
///             SRC[i+15:i]
///         ELSE *DEST[i+15:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVDQU16 (EVEX Encoded Versions, Load-Form)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC[i+15:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE  DEST[i+15:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQU32 (EVEX Encoded Versions, Register-Copy Form)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[i+31:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE  DEST[i+31:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQU32 (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] :=
///             SRC[i+31:i]
///         ELSE *DEST[i+31:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVDQU32 (EVEX Encoded Versions, Load-Form)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[i+31:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE  DEST[i+31:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQU64 (EVEX Encoded Versions, Register-Copy Form)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[i+63:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE  DEST[i+63:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQU64 (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[i+63:i]
///         ELSE *DEST[i+63:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVDQU64 (EVEX Encoded Versions, Load-Form)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[i+63:i]
///         ELSE
///             IF *merging-masking*
///                     ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE  DEST[i+63:i] := 0
///                     ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDQU (VEX.256 Encoded Version, Load - and Register Copy)
/// DEST[255:0] := SRC[255:0]
/// DEST[MAXVL-1:256] := 0
/// VMOVDQU (VEX.256 Encoded Version, Store-Form)
/// DEST[255:0] := SRC[255:0]
/// VMOVDQU (VEX.128 encoded version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] := 0
/// VMOVDQU (128-bit Load- and Register-Copy- Form Legacy SSE Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// (V)MOVDQU (128-bit Store-Form Version)
/// DEST[127:0] := SRC[127:0]
/// ```
#[box_to_static_reference]
pub(super) fn vmovdqu8() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVSH dest, src (two operand load)
/// IF k1[0] or no writemask:
///     DEST.fp16[0] := SRC.fp16[0]
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // ELSE DEST.fp16[0] remains unchanged
/// DEST[MAXVL:16] := 0
/// VMOVSH dest, src (two operand store)
/// IF k1[0] or no writemask:
///     DEST.fp16[0] := SRC.fp16[0]
/// // ELSE DEST.fp16[0] remains unchanged
/// VMOVSH dest, src1, src2 (three operand copy)
/// IF k1[0] or no writemask:
///     DEST.fp16[0] := SRC2.fp16[0]
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // ELSE DEST.fp16[0] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vmovsh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVW dest, src (two operand load)
/// DEST.word[0] := SRC.word[0]
/// DEST[MAXVL:16] := 0
/// VMOVW dest, src (two operand store)
/// DEST.word[0] := SRC.word[0]
/// // upper bits of GPR DEST are zeroed
/// ```
#[box_to_static_reference]
pub(super) fn vmovw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMULPH (EVEX encoded versions) when src2 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.fp16[j] := SRC1.fp16[j] * SRC2.fp16[j]
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VMULPH (EVEX encoded versions) when src2 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             DEST.fp16[j] := SRC1.fp16[j] * SRC2.fp16[0]
///         ELSE:
///             DEST.fp16[j] := SRC1.fp16[j] * SRC2.fp16[j]
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vmulph() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMULSH (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC2 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     DEST.fp16[0] := SRC1.fp16[0] * SRC2.fp16[0]
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else dest.fp16[0] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vmulsh() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VP2INTERSECTD destmask, src1, src2
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// // dest_mask_reg_id is the register id specified in the instruction for destmask
/// dest_base := dest_mask_reg_id & ~1
/// // maskregs[ ] is an array representing the mask registers
/// maskregs[dest_base+0][MAX_KL-1:0] := 0
/// maskregs[dest_base+1][MAX_KL-1:0] := 0
/// FOR i := 0 to KL-1:
///     FOR j := 0 to KL-1:
///         match := (src1.dword[i] == src2.dword[j])
///         maskregs[dest_base+0].bit[i] |= match
///         maskregs[dest_base+1].bit[j] |= match
/// VP2INTERSECTQ destmask, src1, src2
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// // dest_mask_reg_id is the register id specified in the instruction for destmask
/// dest_base := dest_mask_reg_id & ~1
/// // maskregs[ ] is an array representing the mask registers
/// maskregs[dest_base+0][MAX_KL-1:0] := 0
/// maskregs[dest_base+1][MAX_KL-1:0] := 0
/// FOR i = 0 to KL-1:
///     FOR j = 0 to KL-1:
///         match := (src1.qword[i] == src2.qword[j])
///         maskregs[dest_base+0].bit[i] |=  match
///         maskregs[dest_base+1].bit[j] |=  match
/// ```
#[box_to_static_reference]
pub(super) fn vp2intersectd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VP2INTERSECTD destmask, src1, src2
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// // dest_mask_reg_id is the register id specified in the instruction for destmask
/// dest_base := dest_mask_reg_id & ~1
/// // maskregs[ ] is an array representing the mask registers
/// maskregs[dest_base+0][MAX_KL-1:0] := 0
/// maskregs[dest_base+1][MAX_KL-1:0] := 0
/// FOR i := 0 to KL-1:
///     FOR j := 0 to KL-1:
///         match := (src1.dword[i] == src2.dword[j])
///         maskregs[dest_base+0].bit[i] |= match
///         maskregs[dest_base+1].bit[j] |= match
/// VP2INTERSECTQ destmask, src1, src2
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// // dest_mask_reg_id is the register id specified in the instruction for destmask
/// dest_base := dest_mask_reg_id & ~1
/// // maskregs[ ] is an array representing the mask registers
/// maskregs[dest_base+0][MAX_KL-1:0] := 0
/// maskregs[dest_base+1][MAX_KL-1:0] := 0
/// FOR i = 0 to KL-1:
///     FOR j = 0 to KL-1:
///         match := (src1.qword[i] == src2.qword[j])
///         maskregs[dest_base+0].bit[i] |=  match
///         maskregs[dest_base+1].bit[j] |=  match
/// ```
#[box_to_static_reference]
pub(super) fn vp2intersectq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBLENDD (VEX.256 encoded version)
/// IF (imm8[0] == 1) THEN DEST[31:0] := SRC2[31:0]
/// ELSE DEST[31:0] := SRC1[31:0]
/// IF (imm8[1] == 1) THEN DEST[63:32] := SRC2[63:32]
/// ELSE DEST[63:32] := SRC1[63:32]
/// IF (imm8[2] == 1) THEN DEST[95:64] := SRC2[95:64]
/// ELSE DEST[95:64] := SRC1[95:64]
/// IF (imm8[3] == 1) THEN DEST[127:96] := SRC2[127:96]
/// ELSE DEST[127:96] := SRC1[127:96]
/// IF (imm8[4] == 1) THEN DEST[159:128] := SRC2[159:128]
/// ELSE DEST[159:128] := SRC1[159:128]
/// IF (imm8[5] == 1) THEN DEST[191:160] := SRC2[191:160]
/// ELSE DEST[191:160] := SRC1[191:160]
/// IF (imm8[6] == 1) THEN DEST[223:192] := SRC2[223:192]
/// ELSE DEST[223:192] := SRC1[223:192]
/// IF (imm8[7] == 1) THEN DEST[255:224] := SRC2[255:224]
/// ELSE DEST[255:224] := SRC1[255:224]
/// VPBLENDD (VEX.128 encoded version)
/// IF (imm8[0] == 1) THEN DEST[31:0] := SRC2[31:0]
/// ELSE DEST[31:0] := SRC1[31:0]
/// IF (imm8[1] == 1) THEN DEST[63:32] := SRC2[63:32]
/// ELSE DEST[63:32] := SRC1[63:32]
/// IF (imm8[2] == 1) THEN DEST[95:64] := SRC2[95:64]
/// ELSE DEST[95:64] := SRC1[95:64]
/// IF (imm8[3] == 1) THEN DEST[127:96] := SRC2[127:96]
/// ELSE DEST[127:96] := SRC1[127:96]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpblendd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBLENDMB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC2[i+7:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+7:i] := SRC1[i+7:i]
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0;
/// VPBLENDMW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC2[i+15:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+15:i] := SRC1[i+15:i]
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpblendmb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBLENDMD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no controlmask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC2[i+31:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+31:i] := SRC1[i+31:i]
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0;
/// VPBLENDMD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no controlmask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC2[i+31:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+31:i] := SRC1[i+31:i]
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpblendmd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBLENDMD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no controlmask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC2[i+31:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+31:i] := SRC1[i+31:i]
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0;
/// VPBLENDMD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no controlmask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC2[i+31:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+31:i] := SRC1[i+31:i]
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpblendmq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBLENDMB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC2[i+7:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+7:i] := SRC1[i+7:i]
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0;
/// VPBLENDMW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC2[i+15:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN DEST[i+15:i] := SRC1[i+15:i]
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpblendmw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBROADCASTB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC[7:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC[15:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTD (128 bit version)
/// temp := SRC[31:0]
/// DEST[31:0] := temp
/// DEST[63:32] := temp
/// DEST[95:64] := temp
/// DEST[127:96] := temp
/// DEST[MAXVL-1:128] := 0
/// VPBROADCASTD (VEX.256 encoded version)
/// temp := SRC[31:0]
/// DEST[31:0] := temp
/// DEST[63:32] := temp
/// DEST[95:64] := temp
/// DEST[127:96] := temp
/// DEST[159:128] := temp
/// DEST[191:160] := temp
/// DEST[223:192] := temp
/// DEST[255:224] := temp
/// DEST[MAXVL-1:256] := 0
/// VPBROADCASTD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[31:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTQ (VEX.256 encoded version)
/// temp := SRC[63:0]
/// DEST[63:0] := temp
/// DEST[127:64] := temp
/// DEST[191:128] := temp
/// DEST[255:192] := temp
/// DEST[MAXVL-1:256] := 0
/// VPBROADCASTQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[63:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTI32x2 (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     n := (j mod 2) * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[n+31:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTI128 (VEX.256 encoded version)
/// temp := SRC[127:0]
/// DEST[127:0] := temp
/// DEST[255:128] := temp
/// DEST[MAXVL-1:256] := 0
/// VBROADCASTI32X4 (EVEX encoded versions)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j* 32
///     n := (j modulo 4) * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[n+31:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTI64X2 (EVEX encoded versions)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     n := (j modulo 2) * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[n+63:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] = 0
///                 FI
///     FI;
/// ENDFOR;
/// VBROADCASTI32X8 (EVEX.U1.512 encoded version)
/// FOR j := 0 TO 15
///     i := j * 32
///     n := (j modulo 8) * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[n+31:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VBROADCASTI64X4 (EVEX.512 encoded version)
/// FOR j := 0 TO 7
///     i := j * 64
///     n := (j modulo 4) * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[n+63:n]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpbroadcast() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBROADCASTB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC[7:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC[15:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[31:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[63:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpbroadcastb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBROADCASTB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC[7:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC[15:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[31:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[63:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpbroadcastd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBROADCASTMB2Q
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j*64
///     DEST[i+63:i] := ZeroExtend(SRC[7:0])
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTMW2D
/// (KL, VL) = (4, 128), (8, 256),(16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     DEST[i+31:i] := ZeroExtend(SRC[15:0])
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpbroadcastm() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBROADCASTB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC[7:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC[15:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[31:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[63:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpbroadcastq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPBROADCASTB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC[7:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC[15:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[31:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPBROADCASTQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[63:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpbroadcastw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP := EQ;
///     1: OP := LT;
///     2: OP := LE;
///     3: OP := FALSE;
///     4: OP := NEQ;
///     5: OP := NLT;
///     6: OP := NLE;
///     7: OP := TRUE;
/// ESAC;
/// VPCMPB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             CMP := SRC1[i+7:i] OP SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPCMPUB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             CMP := SRC1[i+7:i] OP SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcmpb() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP := EQ;
///     1: OP := LT;
///     2: OP := LE;
///     3: OP := FALSE;
///     4: OP := NEQ;
///     5: OP := NLT;
///     6: OP := NLE;
///     7: OP := TRUE;
/// ESAC;
/// VPCMPD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] OP SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] OP SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPCMPUD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] OP SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] OP SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcmpd() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP := EQ;
///     1: OP := LT;
///     2: OP := LE;
///     3: OP := FALSE;
///     4: OP := NEQ;
///     5: OP := NLT;
///     6: OP := NLE;
///     7: OP := TRUE;
/// ESAC;
/// VPCMPQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+63:i] OP SRC2[63:0];
///                 ELSE CMP := SRC1[i+63:i] OP SRC2[i+63:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] ;
///                 0: =
///                     zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPCMPUQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+63:i] OP SRC2[63:0];
///                 ELSE CMP := SRC1[i+63:i] OP SRC2[i+63:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] ;
///                 0: =
///                     zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcmpq() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP := EQ;
///     1: OP := LT;
///     2: OP := LE;
///     3: OP := FALSE;
///     4: OP := NEQ;
///     5: OP := NLT;
///     6: OP := NLE;
///     7: OP := TRUE;
/// ESAC;
/// VPCMPB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             CMP := SRC1[i+7:i] OP SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPCMPUB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             CMP := SRC1[i+7:i] OP SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcmpub() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP := EQ;
///     1: OP := LT;
///     2: OP := LE;
///     3: OP := FALSE;
///     4: OP := NEQ;
///     5: OP := NLT;
///     6: OP := NLE;
///     7: OP := TRUE;
/// ESAC;
/// VPCMPD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] OP SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] OP SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPCMPUD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] OP SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] OP SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcmpud() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP := EQ;
///     1: OP := LT;
///     2: OP := LE;
///     3: OP := FALSE;
///     4: OP := NEQ;
///     5: OP := NLT;
///     6: OP := NLE;
///     7: OP := TRUE;
/// ESAC;
/// VPCMPQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+63:i] OP SRC2[63:0];
///                 ELSE CMP := SRC1[i+63:i] OP SRC2[i+63:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] ;
///                 0: =
///                     zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPCMPUQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+63:i] OP SRC2[63:0];
///                 ELSE CMP := SRC1[i+63:i] OP SRC2[i+63:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] ;
///                 0: =
///                     zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcmpuq() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP := EQ;
///     1: OP := LT;
///     2: OP := LE;
///     3: OP := FALSE;
///     4: OP := NEQ;
///     5: OP := NLT;
///     6: OP := NLE;
///     7: OP := TRUE;
/// ESAC;
/// VPCMPW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             ICMP := SRC1[i+15:i] OP SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPCMPUW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             CMP := SRC1[i+15:i] OP SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcmpuw() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// CASE (COMPARISON PREDICATE) OF
///     0: OP := EQ;
///     1: OP := LT;
///     2: OP := LE;
///     3: OP := FALSE;
///     4: OP := NEQ;
///     5: OP := NLT;
///     6: OP := NLE;
///     7: OP := TRUE;
/// ESAC;
/// VPCMPW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             ICMP := SRC1[i+15:i] OP SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPCMPUW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             CMP := SRC1[i+15:i] OP SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcmpw() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VPCOMPRESSB store form
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.byte[k] := SRC.byte[j]
///         k := k +1
/// VPCOMPRESSB reg-reg form
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.byte[k] := SRC.byte[j]
///         k := k + 1
/// IF *merging-masking*:
///     *DEST[VL-1:k*8] remains unchanged*
///     ELSE DEST[VL-1:k*8] := 0
/// DEST[MAX_VL-1:VL] := 0
/// VPCOMPRESSW store form
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.word[k] := SRC.word[j]
///         k := k + 1
/// VPCOMPRESSW reg-reg form
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.word[k] := SRC.word[j]
///         k := k + 1
/// IF *merging-masking*:
///     *DEST[VL-1:k*16] remains unchanged*
///     ELSE DEST[VL-1:k*16] := 0
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcompressb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPCOMPRESSD (EVEX encoded versions) store form
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// SIZE := 32
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no controlmask*
///         THEN
///                 DEST[k+SIZE-1:k] := SRC[i+31:i]
///                 k := k + SIZE
///     FI;
/// ENDFOR;
/// VPCOMPRESSD (EVEX encoded versions) reg-reg form
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// SIZE := 32
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no controlmask*
///         THEN
///                 DEST[k+SIZE-1:k] := SRC[i+31:i]
///                 k := k + SIZE
///     FI;
/// ENDFOR
/// IF *merging-masking*
///             THEN *DEST[VL-1:k] remains unchanged*
///             ELSE DEST[VL-1:k] := 0
/// FI
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcompressd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPCOMPRESSQ (EVEX encoded versions) store form
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// SIZE := 64
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no controlmask*
///         THEN
///                 DEST[k+SIZE-1:k] := SRC[i+63:i]
///                 k := k + SIZE
///     FI;
/// ENFOR
/// VPCOMPRESSQ (EVEX encoded versions) reg-reg form
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// SIZE := 64
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no controlmask*
///         THEN
///                 DEST[k+SIZE-1:k] := SRC[i+63:i]
///                 k := k + SIZE
///     FI;
/// ENDFOR
/// IF *merging-masking*
///             THEN *DEST[VL-1:k] remains unchanged*
///             ELSE DEST[VL-1:k] := 0
/// FI
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpcompressq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPCONFLICTD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     IF MaskBit(j) OR *no writemask*THEN
///             FOR k := 0 TO j-1
///                     m := k*32
///                     IF ((SRC[i+31:i] = SRC[m+31:m])) THEN
///                             DEST[i+k] := 1
///                     ELSE
///                             DEST[i+k] := 0
///                     FI
///             ENDFOR
///             DEST[i+31:i+j] := 0
///     ELSE
///             IF *merging-masking* THEN
///                     *DEST[i+31:i] remains unchanged*
///             ELSE
///                     DEST[i+31:i] := 0
///             FI
///     FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPCONFLICTQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///         i := j*64
///         IF MaskBit(j) OR *no writemask*THEN
///             FOR k := 0 TO j-1
///                     m := k*64
///                     IF ((SRC[i+63:i] = SRC[m+63:m])) THEN
///                             DEST[i+k] := 1
///                     ELSE
///                             DEST[i+k] := 0
///                     FI
///             ENDFOR
///             DEST[i+63:i+j] := 0
///     ELSE
///             IF *merging-masking* THEN
///                     *DEST[i+63:i] remains unchanged*
///                 ELSE
///                         DEST[i+63:i] := 0
///             FI
///     FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpconflictd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPCONFLICTD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     IF MaskBit(j) OR *no writemask*THEN
///             FOR k := 0 TO j-1
///                     m := k*32
///                     IF ((SRC[i+31:i] = SRC[m+31:m])) THEN
///                             DEST[i+k] := 1
///                     ELSE
///                             DEST[i+k] := 0
///                     FI
///             ENDFOR
///             DEST[i+31:i+j] := 0
///     ELSE
///             IF *merging-masking* THEN
///                     *DEST[i+31:i] remains unchanged*
///             ELSE
///                     DEST[i+31:i] := 0
///             FI
///     FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPCONFLICTQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///         i := j*64
///         IF MaskBit(j) OR *no writemask*THEN
///             FOR k := 0 TO j-1
///                     m := k*64
///                     IF ((SRC[i+63:i] = SRC[m+63:m])) THEN
///                             DEST[i+k] := 1
///                     ELSE
///                             DEST[i+k] := 0
///                     FI
///             ENDFOR
///             DEST[i+63:i+j] := 0
///     ELSE
///             IF *merging-masking* THEN
///                     *DEST[i+63:i] remains unchanged*
///                 ELSE
///                         DEST[i+63:i] := 0
///             FI
///     FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpconflictq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPDPBUSD dest, src1, src2 (VEX encoded versions)
/// VL=(128, 256)
/// KL=VL/32
/// ORIGDEST := DEST
/// FOR i := 0 TO KL-1:
///     // Extending to 16b
///     // src1extend := ZERO_EXTEND
///     // src2extend := SIGN_EXTEND
///     p2word := src1extend(SRC1.byte[4*i+1]) * src2extend(SRC2.byte[4*i+1])
///     p3word := src1extend(SRC1.byte[4*i+2]) * src2extend(SRC2.byte[4*i+2])
///     p4word := src1extend(SRC1.byte[4*i+3]) * src2extend(SRC2.byte[4*i+3])
///     DEST.dword[i] := ORIGDEST.dword[i] + p1word + p2word + p3word + p4word
/// DEST[MAX_VL-1:VL] := 0
/// VPDPBUSD dest, src1, src2 (EVEX encoded versions)
/// (KL,VL)=(4,128), (8,256), (16,512)
/// ORIGDEST := DEST
/// FOR i := 0 TO KL-1:
///     IF k1[i] or *no writemask*:
///         // Byte elements of SRC1 are zero-extended to 16b and
///         // byte elements of SRC2 are sign extended to 16b before multiplication.
///         IF SRC2 is memory and EVEX.b == 1:
///             t := SRC2.dword[0]
///         ELSE:
///             t := SRC2.dword[i]
///         p1word := ZERO_EXTEND(SRC1.byte[4*i]) * SIGN_EXTEND(t.byte[0])
///         p2word := ZERO_EXTEND(SRC1.byte[4*i+1]) * SIGN_EXTEND(t.byte[1])
///         p3word := ZERO_EXTEND(SRC1.byte[4*i+2]) * SIGN_EXTEND(t.byte[2])
///         p4word := ZERO_EXTEND(SRC1.byte[4*i+3]) * SIGN_EXTEND(t.byte[3])
///         DEST.dword[i] := ORIGDEST.dword[i] + p1word + p2word + p3word + p4word
///     ELSE IF *zeroing*:
///         DEST.dword[i] := 0
///     ELSE: // Merge masking, dest element unchanged
///         DEST.dword[i] := ORIGDEST.dword[i]
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpdpbusd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPDPBUSDS dest, src1, src2 (VEX encoded versions)
/// VL=(128, 256)
/// KL=VL/32
/// ORIGDEST := DEST
/// FOR i := 0 TO KL-1:
///     // Extending to 16b
///     // src1extend := ZERO_EXTEND
///     // src2extend := SIGN_EXTEND
///     p1word := src1extend(SRC1.byte[4*i+0]) * src2extend(SRC2.byte[4*i+0])
///     p2word := src1extend(SRC1.byte[4*i+1]) * src2extend(SRC2.byte[4*i+1])
///     p3word := src1extend(SRC1.byte[4*i+2]) * src2extend(SRC2.byte[4*i+2])
///     p4word := src1extend(SRC1.byte[4*i+3]) * src2extend(SRC2.byte[4*i+3])
///     DEST.dword[i] := SIGNED_DWORD_SATURATE(ORIGDEST.dword[i] + p1word + p2word + p3word + p4word)
/// DEST[MAX_VL-1:VL] := 0
/// VPDPBUSDS dest, src1, src2 (EVEX encoded versions)
/// (KL,VL)=(4,128), (8,256), (16,512)
/// ORIGDEST := DEST
/// FOR i := 0 TO KL-1:
///     IF k1[i] or *no writemask*:
///         // Byte elements of SRC1 are zero-extended to 16b and
///         // byte elements of SRC2 are sign extended to 16b before multiplication.
///         IF SRC2 is memory and EVEX.b == 1:
///             t := SRC2.dword[0]
///         ELSE:
///             t := SRC2.dword[i]
///         p1word := ZERO_EXTEND(SRC1.byte[4*i]) * SIGN_EXTEND(t.byte[0])
///         p2word := ZERO_EXTEND(SRC1.byte[4*i+1]) * SIGN_EXTEND(t.byte[1])
///         p3word := ZERO_EXTEND(SRC1.byte[4*i+2]) * SIGN_EXTEND(t.byte[2])
///         p4word := ZERO_EXTEND(SRC1.byte[4*i+3]) *SIGN_EXTEND(t.byte[3])
///         DEST.dword[i] := SIGNED_DWORD_SATURATE(ORIGDEST.dword[i] + p1word + p2word + p3word + p4word)
///     ELSE IF *zeroing*:
///         DEST.dword[i] := 0
///     ELSE: // Merge masking, dest element unchanged
///         DEST.dword[i] := ORIGDEST.dword[i]
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpdpbusds() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPDPWSSD dest, src1, src2 (VEX encoded versions)
/// VL=(128, 256)
/// KL=VL/32
/// ORIGDEST := DEST
/// FOR i := 0 TO KL-1:
///     p1dword := SIGN_EXTEND(SRC1.word[2*i+0]) * SIGN_EXTEND(SRC2.word[2*i+0] )
///     p2dword := SIGN_EXTEND(SRC1.word[2*i+1]) * SIGN_EXTEND(SRC2.word[2*i+1] )
///     DEST.dword[i] := ORIGDEST.dword[i] + p1dword + p2dword
/// DEST[MAX_VL-1:VL] := 0
/// VPDPWSSD dest, src1, src2 (EVEX encoded versions)
/// (KL,VL)=(4,128), (8,256), (16,512)
/// ORIGDEST := DEST
/// FOR i := 0 TO KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC2 is memory and EVEX.b == 1:
///             t := SRC2.dword[0]
///         ELSE:
///             t := SRC2.dword[i]
///         p1dword := SIGN_EXTEND(SRC1.word[2*i]) * SIGN_EXTEND(t.word[0])
///         p2dword := SIGN_EXTEND(SRC1.word[2*i+1]) * SIGN_EXTEND(t.word[1])
///         DEST.dword[i] := ORIGDEST.dword[i] + p1dword + p2dword
///     ELSE IF *zeroing*:
///         DEST.dword[i] := 0
///     ELSE: // Merge masking, dest element unchanged
///         DEST.dword[i] := ORIGDEST.dword[i]
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpdpwssd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPDPWSSDS dest, src1, src2 (VEX encoded versions)
/// VL=(128, 256)
/// KL=VL/32
/// ORIGDEST := DEST
/// FOR i := 0 TO KL-1:
///     p1dword := SIGN_EXTEND(SRC1.word[2*i+0]) * SIGN_EXTEND(SRC2.word[2*i+0])
///     p2dword := SIGN_EXTEND(SRC1.word[2*i+1]) * SIGN_EXTEND(SRC2.word[2*i+1])
///     DEST.dword[i] := SIGNED_DWORD_SATURATE(ORIGDEST.dword[i] + p1dword + p2dword)
/// DEST[MAX_VL-1:VL] := 0
/// VPDPWSSDS dest, src1, src2 (EVEX encoded versions)
/// (KL,VL)=(4,128), (8,256), (16,512)
/// ORIGDEST := DEST
/// FOR i := 0 TO KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC2 is memory and EVEX.b == 1:
///             t := SRC2.dword[0]
///         ELSE:
///             t := SRC2.dword[i]
///         p1dword := SIGN_EXTEND(SRC1.word[2*i]) * SIGN_EXTEND(t.word[0])
///         p2dword := SIGN_EXTEND(SRC1.word[2*i+1]) * SIGN_EXTEND(t.word[1])
///         DEST.dword[i] := SIGNED_DWORD_SATURATE(ORIGDEST.dword[i] + p1dword + p2dword)
///     ELSE IF *zeroing*:
///         DEST.dword[i] := 0
///     ELSE: // Merge masking, dest element unchanged
///         DEST.dword[i] := ORIGDEST.dword[i]
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpdpwssds() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERM2F128
/// CASE IMM8[1:0] of
/// 0: DEST[127:0] := SRC1[127:0]
/// 1: DEST[127:0] := SRC1[255:128]
/// 2: DEST[127:0] := SRC2[127:0]
/// 3: DEST[127:0] := SRC2[255:128]
/// ESAC
/// CASE IMM8[5:4] of
/// 0: DEST[255:128] := SRC1[127:0]
/// 1: DEST[255:128] := SRC1[255:128]
/// 2: DEST[255:128] := SRC2[127:0]
/// 3: DEST[255:128] := SRC2[255:128]
/// ESAC
/// IF (imm8[3])
/// DEST[127:0] := 0
/// FI
/// IF (imm8[7])
/// DEST[MAXVL-1:128] := 0
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn vperm2f128() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERM2I128
/// CASE IMM8[1:0] of
/// 0: DEST[127:0] := SRC1[127:0]
/// 1: DEST[127:0] := SRC1[255:128]
/// 2: DEST[127:0] := SRC2[127:0]
/// 3: DEST[127:0] := SRC2[255:128]
/// ESAC
/// CASE IMM8[5:4] of
/// 0: DEST[255:128] := SRC1[127:0]
/// 1: DEST[255:128] := SRC1[255:128]
/// 2: DEST[255:128] := SRC2[127:0]
/// 3: DEST[255:128] := SRC2[255:128]
/// ESAC
/// IF (imm8[3])
/// DEST[127:0] := 0
/// FI
/// IF (imm8[7])
/// DEST[255:128] := 0
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn vperm2i128() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128:
///     n := 3;
/// ELSE IF VL = 256:
///     n := 4;
/// ELSE IF VL = 512:
///     n := 5;
/// FI;
/// FOR j := 0 TO KL-1:
///     id := SRC1[j*8 + n : j*8] ; // location of the source byte
///     IF k1[j] OR *no writemask* THEN
///         DEST[j*8 + 7: j*8] := SRC2[id*8 +7: id*8];
///     ELSE IF zeroing-masking THEN
///         DEST[j*8 + 7: j*8] := 0;
///     *ELSE
///         DEST[j*8 + 7: j*8] remains unchanged*
///     FI
/// ENDFOR
/// DEST[MAX_VL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpermb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMD (EVEX encoded versions)
/// (KL, VL) = (8, 256), (16, 512)
/// IF VL = 256 THEN n := 2; FI;
/// IF VL = 512 THEN n := 3; FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     id := 32*SRC1[i+n:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := SRC2[31:0];
///                     ELSE DEST[i+31:i] := SRC2[id+31:id];
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMD (VEX.256 encoded version)
/// DEST[31:0] := (SRC2[255:0] >> (SRC1[2:0] * 32))[31:0];
/// DEST[63:32] := (SRC2[255:0] >> (SRC1[34:32] * 32))[31:0];
/// DEST[95:64] := (SRC2[255:0] >> (SRC1[66:64] * 32))[31:0];
/// DEST[127:96] := (SRC2[255:0] >> (SRC1[98:96] * 32))[31:0];
/// DEST[159:128] := (SRC2[255:0] >> (SRC1[130:128] * 32))[31:0];
/// DEST[191:160] := (SRC2[255:0] >> (SRC1[162:160] * 32))[31:0];
/// DEST[223:192] := (SRC2[255:0] >> (SRC1[194:192] * 32))[31:0];
/// DEST[255:224] := (SRC2[255:0] >> (SRC1[226:224] * 32))[31:0];
/// DEST[MAXVL-1:256] := 0
/// VPERMW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128 THEN n := 2; FI;
/// IF VL = 256 THEN n := 3; FI;
/// IF VL = 512 THEN n := 4; FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     id := 16*SRC1[i+n:i]
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC2[id+15:id]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpermd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMI2B (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128:
///     id := 3;
/// ELSE IF VL = 256:
///     id := 4;
/// ELSE IF VL = 512:
///     id := 5;
/// FI;
/// TMP_DEST[VL-1:0] := DEST[VL-1:0];
/// FOR j := 0 TO KL-1
///     off := 8*SRC1[j*8 + id: j*8] ;
///     IF k1[j] OR *no writemask*:
///         DEST[j*8 + 7: j*8] := TMP_DEST[j*8+id+1]? SRC2[off+7:off] : SRC1[off+7:off];
///     ELSE IF *zeroing-masking*
///         DEST[j*8 + 7: j*8] := 0;
///     *ELSE
///         DEST[j*8 + 7: j*8] remains unchanged*
///     FI;
/// ENDFOR
/// DEST[MAX_VL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpermi2b() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMI2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=TMP_DEST[i+id+1] ? SRC2[off+15:off]
///                     : SRC1[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2D/VPERMI2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i]  := TMP_DEST[i+id+1] ? SRC2[31:0]
///                     : SRC1[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := TMP_DEST[i+id+1] ? SRC2[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2Q/VPERMI2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
/// FOR j := 0 TO KL-1
///     i := j * 64
///     off := 64*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[63:0]
///                     : SRC1[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[off+63:off]
///                     : SRC1[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermi2d() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMI2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=TMP_DEST[i+id+1] ? SRC2[off+15:off]
///                     : SRC1[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2D/VPERMI2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i]  := TMP_DEST[i+id+1] ? SRC2[31:0]
///                     : SRC1[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := TMP_DEST[i+id+1] ? SRC2[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2Q/VPERMI2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
/// FOR j := 0 TO KL-1
///     i := j * 64
///     off := 64*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[63:0]
///                     : SRC1[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[off+63:off]
///                     : SRC1[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermi2pd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMI2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=TMP_DEST[i+id+1] ? SRC2[off+15:off]
///                     : SRC1[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2D/VPERMI2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i]  := TMP_DEST[i+id+1] ? SRC2[31:0]
///                     : SRC1[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := TMP_DEST[i+id+1] ? SRC2[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2Q/VPERMI2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
/// FOR j := 0 TO KL-1
///     i := j * 64
///     off := 64*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[63:0]
///                     : SRC1[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[off+63:off]
///                     : SRC1[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermi2ps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMI2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=TMP_DEST[i+id+1] ? SRC2[off+15:off]
///                     : SRC1[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2D/VPERMI2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i]  := TMP_DEST[i+id+1] ? SRC2[31:0]
///                     : SRC1[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := TMP_DEST[i+id+1] ? SRC2[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2Q/VPERMI2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
/// FOR j := 0 TO KL-1
///     i := j * 64
///     off := 64*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[63:0]
///                     : SRC1[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[off+63:off]
///                     : SRC1[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermi2q() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMI2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=TMP_DEST[i+id+1] ? SRC2[off+15:off]
///                     : SRC1[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2D/VPERMI2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i]  := TMP_DEST[i+id+1] ? SRC2[31:0]
///                     : SRC1[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := TMP_DEST[i+id+1] ? SRC2[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMI2Q/VPERMI2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
/// FOR j := 0 TO KL-1
///     i := j * 64
///     off := 64*TMP_DEST[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[63:0]
///                     : SRC1[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := TMP_DEST[i+id+1] ? SRC2[off+63:off]
///                     : SRC1[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermi2w() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMILPD (EVEX immediate versions)
/// (KL, VL) = (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC1 *is memory*)
///         THEN TMP_SRC1[i+63:i] := SRC1[63:0];
///         ELSE TMP_SRC1[i+63:i] := SRC1[i+63:i];
///     FI;
/// ENDFOR;
/// IF (imm8[0] = 0) THEN TMP_DEST[63:0] := SRC1[63:0]; FI;
/// IF (imm8[0] = 1) THEN TMP_DEST[63:0] := TMP_SRC1[127:64]; FI;
/// IF (imm8[1] = 0) THEN TMP_DEST[127:64] := TMP_SRC1[63:0]; FI;
/// IF (imm8[1] = 1) THEN TMP_DEST[127:64] := TMP_SRC1[127:64]; FI;
/// IF VL >= 256
///     IF (imm8[2] = 0) THEN TMP_DEST[191:128] := TMP_SRC1[191:128]; FI;
///     IF (imm8[2] = 1) THEN TMP_DEST[191:128] := TMP_SRC1[255:192]; FI;
///     IF (imm8[3] = 0) THEN TMP_DEST[255:192] := TMP_SRC1[191:128]; FI;
///     IF (imm8[3] = 1) THEN TMP_DEST[255:192] := TMP_SRC1[255:192]; FI;
/// FI;
/// IF VL >= 512
///     IF (imm8[4] = 0) THEN TMP_DEST[319:256] := TMP_SRC1[319:256]; FI;
///     IF (imm8[4] = 1) THEN TMP_DEST[319:256] := TMP_SRC1[383:320]; FI;
///     IF (imm8[5] = 0) THEN TMP_DEST[383:320] := TMP_SRC1[319:256]; FI;
///     IF (imm8[5] = 1) THEN TMP_DEST[383:320] := TMP_SRC1[383:320]; FI;
///     IF (imm8[6] = 0) THEN TMP_DEST[447:384] := TMP_SRC1[447:384]; FI;
///     IF (imm8[6] = 1) THEN TMP_DEST[447:384] := TMP_SRC1[511:448]; FI;
///     IF (imm8[7] = 0) THEN TMP_DEST[511:448] := TMP_SRC1[447:384]; FI;
///     IF (imm8[7] = 1) THEN TMP_DEST[511:448] := TMP_SRC1[511:448]; FI;
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMILPD (256-bit immediate version)
/// IF (imm8[0] = 0) THEN DEST[63:0] := SRC1[63:0]
/// IF (imm8[0] = 1) THEN DEST[63:0] := SRC1[127:64]
/// IF (imm8[1] = 0) THEN DEST[127:64] := SRC1[63:0]
/// IF (imm8[1] = 1) THEN DEST[127:64] := SRC1[127:64]
/// IF (imm8[2] = 0) THEN DEST[191:128] := SRC1[191:128]
/// IF (imm8[2] = 1) THEN DEST[191:128] := SRC1[255:192]
/// IF (imm8[3] = 0) THEN DEST[255:192] := SRC1[191:128]
/// IF (imm8[3] = 1) THEN DEST[255:192] := SRC1[255:192]
/// VPERMILPD (128-bit immediate version)
/// IF (imm8[0] = 0) THEN DEST[63:0] := SRC1[63:0]
/// IF (imm8[0] = 1) THEN DEST[63:0] := SRC1[127:64]
/// IF (imm8[1] = 0) THEN DEST[127:64] := SRC1[63:0]
/// IF (imm8[1] = 1) THEN DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VPERMILPD (EVEX variable versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0];
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i];
///     FI;
/// ENDFOR;
/// IF (TMP_SRC2[1] = 0) THEN TMP_DEST[63:0] := SRC1[63:0]; FI;
/// IF (TMP_SRC2[1] = 1) THEN TMP_DEST[63:0] := SRC1[127:64]; FI;
/// IF (TMP_SRC2[65] = 0) THEN TMP_DEST[127:64] := SRC1[63:0]; FI;
/// IF (TMP_SRC2[65] = 1) THEN TMP_DEST[127:64] := SRC1[127:64]; FI;
/// IF VL >= 256
///     IF (TMP_SRC2[129] = 0) THEN TMP_DEST[191:128] := SRC1[191:128]; FI;
///     IF (TMP_SRC2[129] = 1) THEN TMP_DEST[191:128] := SRC1[255:192]; FI;
///     IF (TMP_SRC2[193] = 0) THEN TMP_DEST[255:192] := SRC1[191:128]; FI;
///     IF (TMP_SRC2[193] = 1) THEN TMP_DEST[255:192] := SRC1[255:192]; FI;
/// FI;
/// IF VL >= 512
///     IF (TMP_SRC2[257] = 0) THEN TMP_DEST[319:256] := SRC1[319:256]; FI;
///     IF (TMP_SRC2[257] = 1) THEN TMP_DEST[319:256] := SRC1[383:320]; FI;
///     IF (TMP_SRC2[321] = 0) THEN TMP_DEST[383:320] := SRC1[319:256]; FI;
///     IF (TMP_SRC2[321] = 1) THEN TMP_DEST[383:320] := SRC1[383:320]; FI;
///     IF (TMP_SRC2[385] = 0) THEN TMP_DEST[447:384] := SRC1[447:384]; FI;
///     IF (TMP_SRC2[385] = 1) THEN TMP_DEST[447:384] := SRC1[511:448]; FI;
///     IF (TMP_SRC2[449] = 0) THEN TMP_DEST[511:448] := SRC1[447:384]; FI;
///     IF (TMP_SRC2[449] = 1) THEN TMP_DEST[511:448] := SRC1[511:448]; FI;
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMILPD (256-bit variable version)
/// IF (SRC2[1] = 0) THEN DEST[63:0] := SRC1[63:0]
/// IF (SRC2[1] = 1) THEN DEST[63:0] := SRC1[127:64]
/// IF (SRC2[65] = 0) THEN DEST[127:64] := SRC1[63:0]
/// IF (SRC2[65] = 1) THEN DEST[127:64] := SRC1[127:64]
/// IF (SRC2[129] = 0) THEN DEST[191:128] := SRC1[191:128]
/// IF (SRC2[129] = 1) THEN DEST[191:128] := SRC1[255:192]
/// IF (SRC2[193] = 0) THEN DEST[255:192] := SRC1[191:128]
/// IF (SRC2[193] = 1) THEN DEST[255:192] := SRC1[255:192]
/// DEST[MAXVL-1:256] := 0
/// VPERMILPD (128-bit variable version)
/// IF (SRC2[1] = 0) THEN DEST[63:0] := SRC1[63:0]
/// IF (SRC2[1] = 1) THEN DEST[63:0] := SRC1[127:64]
/// IF (SRC2[65] = 0) THEN DEST[127:64] := SRC1[63:0]
/// IF (SRC2[65] = 1) THEN DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermilpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Select4(SRC, control) {
/// CASE (control[1:0]) OF
///     0: TMP := SRC[31:0];
///     1: TMP := SRC[63:32];
///     2: TMP := SRC[95:64];
///     3: TMP := SRC[127:96];
/// ESAC;
/// RETURN TMP
/// }
/// VPERMILPS (EVEX immediate versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC1 *is memory*)
///         THEN TMP_SRC1[i+31:i] := SRC1[31:0];
///         ELSE TMP_SRC1[i+31:i] := SRC1[i+31:i];
///     FI;
/// ENDFOR;
/// TMP_DEST[31:0] := Select4(TMP_SRC1[127:0], imm8[1:0]);
/// TMP_DEST[63:32] := Select4(TMP_SRC1[127:0], imm8[3:2]);
/// TMP_DEST[95:64] := Select4(TMP_SRC1[127:0], imm8[5:4]);
/// TMP_DEST[127:96] := Select4(TMP_SRC1[127:0], imm8[7:6]); FI;
/// IF VL >= 256
///     TMP_DEST[159:128] := Select4(TMP_SRC1[255:128], imm8[1:0]); FI;
///     TMP_DEST[191:160] := Select4(TMP_SRC1[255:128], imm8[3:2]); FI;
///     TMP_DEST[223:192] := Select4(TMP_SRC1[255:128], imm8[5:4]); FI;
///     TMP_DEST[255:224] := Select4(TMP_SRC1[255:128], imm8[7:6]); FI;
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := Select4(TMP_SRC1[383:256], imm8[1:0]); FI;
///     TMP_DEST[319:288] := Select4(TMP_SRC1[383:256], imm8[3:2]); FI;
///     TMP_DEST[351:320] := Select4(TMP_SRC1[383:256], imm8[5:4]); FI;
///     TMP_DEST[383:352] := Select4(TMP_SRC1[383:256], imm8[7:6]); FI;
///     TMP_DEST[415:384] := Select4(TMP_SRC1[511:384], imm8[1:0]); FI;
///     TMP_DEST[447:416] := Select4(TMP_SRC1[511:384], imm8[3:2]); FI;
///     TMP_DEST[479:448] := Select4(TMP_SRC1[511:384], imm8[5:4]); FI;
///     TMP_DEST[511:480] := Select4(TMP_SRC1[511:384], imm8[7:6]); FI;
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE DEST[i+31:i] := 0
///                     ;zeroing-masking
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMILPS (256-bit immediate version)
/// DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);
/// DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);
/// DEST[95:64] := Select4(SRC1[127:0], imm8[5:4]);
/// DEST[127:96] := Select4(SRC1[127:0], imm8[7:6]);
/// DEST[159:128] := Select4(SRC1[255:128], imm8[1:0]);
/// DEST[191:160] := Select4(SRC1[255:128], imm8[3:2]);
/// DEST[223:192] := Select4(SRC1[255:128], imm8[5:4]);
/// DEST[255:224] := Select4(SRC1[255:128], imm8[7:6]);
/// VPERMILPS (128-bit immediate version)
/// DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);
/// DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);
/// DEST[95:64] := Select4(SRC1[127:0], imm8[5:4]);
/// DEST[127:96] := Select4(SRC1[127:0], imm8[7:6]);
/// DEST[MAXVL-1:128] := 0
/// VPERMILPS (EVEX variable versions)
/// (KL, VL) = (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0];
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i];
///     FI;
/// ENDFOR;
/// TMP_DEST[31:0] := Select4(SRC1[127:0], TMP_SRC2[1:0]);
/// TMP_DEST[63:32] := Select4(SRC1[127:0], TMP_SRC2[33:32]);
/// TMP_DEST[95:64] := Select4(SRC1[127:0], TMP_SRC2[65:64]);
/// TMP_DEST[127:96] := Select4(SRC1[127:0], TMP_SRC2[97:96]);
/// IF VL >= 256
///     TMP_DEST[159:128] := Select4(SRC1[255:128], TMP_SRC2[129:128]);
///     TMP_DEST[191:160] := Select4(SRC1[255:128], TMP_SRC2[161:160]);
///     TMP_DEST[223:192] := Select4(SRC1[255:128], TMP_SRC2[193:192]);
///     TMP_DEST[255:224] := Select4(SRC1[255:128], TMP_SRC2[225:224]);
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := Select4(SRC1[383:256], TMP_SRC2[257:256]);
///     TMP_DEST[319:288] := Select4(SRC1[383:256], TMP_SRC2[289:288]);
///     TMP_DEST[351:320] := Select4(SRC1[383:256], TMP_SRC2[321:320]);
///     TMP_DEST[383:352] := Select4(SRC1[383:256], TMP_SRC2[353:352]);
///     TMP_DEST[415:384] := Select4(SRC1[511:384], TMP_SRC2[385:384]);
///     TMP_DEST[447:416] := Select4(SRC1[511:384], TMP_SRC2[417:416]);
///     TMP_DEST[479:448] := Select4(SRC1[511:384], TMP_SRC2[449:448]);
///     TMP_DEST[511:480] := Select4(SRC1[511:384], TMP_SRC2[481:480]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                 THEN *DEST[i+31:i] remains unchanged*
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMILPS (256-bit variable version)
/// DEST[31:0] := Select4(SRC1[127:0], SRC2[1:0]);
/// DEST[63:32] := Select4(SRC1[127:0], SRC2[33:32]);
/// DEST[95:64] := Select4(SRC1[127:0], SRC2[65:64]);
/// DEST[127:96] := Select4(SRC1[127:0], SRC2[97:96]);
/// DEST[159:128] := Select4(SRC1[255:128], SRC2[129:128]);
/// DEST[191:160] := Select4(SRC1[255:128], SRC2[161:160]);
/// DEST[223:192] := Select4(SRC1[255:128], SRC2[193:192]);
/// DEST[255:224] := Select4(SRC1[255:128], SRC2[225:224]);
/// DEST[MAXVL-1:256] := 0
/// VPERMILPS (128-bit variable version)
/// DEST[31:0] := Select4(SRC1[127:0], SRC2[1:0]);
/// DEST[63:32] := Select4(SRC1[127:0], SRC2[33:32]);
/// DEST[95:64] :=Select4(SRC1[127:0], SRC2[65:64]);
/// DEST[127:96] := Select4(SRC1[127:0], SRC2[97:96]);
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermilps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMPD (EVEX - imm8 control forms)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC *is memory*)
///         THEN TMP_SRC[i+63:i] := SRC[63:0];
///         ELSE TMP_SRC[i+63:i] := SRC[i+63:i];
///     FI;
/// ENDFOR;
/// TMP_DEST[63:0] := (TMP_SRC[256:0] >> (IMM8[1:0] * 64))[63:0];
/// TMP_DEST[127:64] := (TMP_SRC[256:0] >> (IMM8[3:2] * 64))[63:0];
/// TMP_DEST[191:128] := (TMP_SRC[256:0] >> (IMM8[5:4] * 64))[63:0];
/// TMP_DEST[255:192] := (TMP_SRC[256:0] >> (IMM8[7:6] * 64))[63:0];
/// IF VL >= 512
///     TMP_DEST[319:256] := (TMP_SRC[511:256] >> (IMM8[1:0] * 64))[63:0];
///     TMP_DEST[383:320] := (TMP_SRC[511:256] >> (IMM8[3:2] * 64))[63:0];
///     TMP_DEST[447:384] := (TMP_SRC[511:256] >> (IMM8[5:4] * 64))[63:0];
///     TMP_DEST[511:448] := (TMP_SRC[511:256] >> (IMM8[7:6] * 64))[63:0];
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                                 ;zeroing-masking
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMPD (EVEX - vector control forms)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0];
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i];
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[63:0] := (TMP_SRC2[255:0] >> (SRC1[1:0] * 64))[63:0];
///     TMP_DEST[127:64] := (TMP_SRC2[255:0] >> (SRC1[65:64] * 64))[63:0];
///     TMP_DEST[191:128] := (TMP_SRC2[255:0] >> (SRC1[129:128] * 64))[63:0];
///     TMP_DEST[255:192] := (TMP_SRC2[255:0] >> (SRC1[193:192] * 64))[63:0];
/// FI;
/// IF VL = 512
///     TMP_DEST[63:0] := (TMP_SRC2[511:0] >> (SRC1[2:0] * 64))[63:0];
///     TMP_DEST[127:64] := (TMP_SRC2[511:0] >> (SRC1[66:64] * 64))[63:0];
///     TMP_DEST[191:128] := (TMP_SRC2[511:0] >> (SRC1[130:128] * 64))[63:0];
///     TMP_DEST[255:192] := (TMP_SRC2[511:0] >> (SRC1[194:192] * 64))[63:0];
///     TMP_DEST[319:256] := (TMP_SRC2[511:0] >> (SRC1[258:256] * 64))[63:0];
///     TMP_DEST[383:320] := (TMP_SRC2[511:0] >> (SRC1[322:320] * 64))[63:0];
///     TMP_DEST[447:384] := (TMP_SRC2[511:0] >> (SRC1[386:384] * 64))[63:0];
///     TMP_DEST[511:448] := (TMP_SRC2[511:0] >> (SRC1[450:448] * 64))[63:0];
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                                 ;zeroing-masking
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMPD (VEX.256 encoded version)
/// DEST[63:0] := (SRC[255:0] >> (IMM8[1:0] * 64))[63:0];
/// DEST[127:64] := (SRC[255:0] >> (IMM8[3:2] * 64))[63:0];
/// DEST[191:128] := (SRC[255:0] >> (IMM8[5:4] * 64))[63:0];
/// DEST[255:192] := (SRC[255:0] >> (IMM8[7:6] * 64))[63:0];
/// DEST[MAXVL-1:256] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMPS (EVEX forms)
/// (KL, VL) (8, 256),= (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0];
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i];
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[31:0] := (TMP_SRC2[255:0] >> (SRC1[2:0] * 32))[31:0];
///     TMP_DEST[63:32] := (TMP_SRC2[255:0] >> (SRC1[34:32] * 32))[31:0];
///     TMP_DEST[95:64] := (TMP_SRC2[255:0] >> (SRC1[66:64] * 32))[31:0];
///     TMP_DEST[127:96] := (TMP_SRC2[255:0] >> (SRC1[98:96] * 32))[31:0];
///     TMP_DEST[159:128] := (TMP_SRC2[255:0] >> (SRC1[130:128] * 32))[31:0];
///     TMP_DEST[191:160] := (TMP_SRC2[255:0] >> (SRC1[162:160] * 32))[31:0];
///     TMP_DEST[223:192] := (TMP_SRC2[255:0] >> (SRC1[193:192] * 32))[31:0];
///     TMP_DEST[255:224] := (TMP_SRC2[255:0] >> (SRC1[226:224] * 32))[31:0];
/// FI;
/// IF VL = 512
///     TMP_DEST[31:0] := (TMP_SRC2[511:0] >> (SRC1[3:0] * 32))[31:0];
///     TMP_DEST[63:32] := (TMP_SRC2[511:0] >> (SRC1[35:32] * 32))[31:0];
///     TMP_DEST[95:64] := (TMP_SRC2[511:0] >> (SRC1[67:64] * 32))[31:0];
///     TMP_DEST[127:96] := (TMP_SRC2[511:0] >> (SRC1[99:96] * 32))[31:0];
///     TMP_DEST[159:128] := (TMP_SRC2[511:0] >> (SRC1[131:128] * 32))[31:0];
///     TMP_DEST[191:160] := (TMP_SRC2[511:0] >> (SRC1[163:160] * 32))[31:0];
///     TMP_DEST[223:192] := (TMP_SRC2[511:0] >> (SRC1[195:192] * 32))[31:0];
///     TMP_DEST[255:224] := (TMP_SRC2[511:0] >> (SRC1[227:224] * 32))[31:0];
///     TMP_DEST[287:256] := (TMP_SRC2[511:0] >> (SRC1[259:256] * 32))[31:0];
///     TMP_DEST[319:288] := (TMP_SRC2[511:0] >> (SRC1[291:288] * 32))[31:0];
///     TMP_DEST[351:320] := (TMP_SRC2[511:0] >> (SRC1[323:320] * 32))[31:0];
///     TMP_DEST[383:352] := (TMP_SRC2[511:0] >> (SRC1[355:352] * 32))[31:0];
///     TMP_DEST[415:384] := (TMP_SRC2[511:0] >> (SRC1[387:384] * 32))[31:0];
///     TMP_DEST[447:416] := (TMP_SRC2[511:0] >> (SRC1[419:416] * 32))[31:0];
///     TMP_DEST[479:448] :=(TMP_SRC2[511:0] >> (SRC1[451:448] * 32))[31:0];
///     TMP_DEST[511:480] := (TMP_SRC2[511:0] >> (SRC1[483:480] * 32))[31:0];
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                                 ;zeroing-masking
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMPS (VEX.256 encoded version)
/// DEST[31:0] := (SRC2[255:0] >> (SRC1[2:0] * 32))[31:0];
/// DEST[63:32] := (SRC2[255:0] >> (SRC1[34:32] * 32))[31:0];
/// DEST[95:64] := (SRC2[255:0] >> (SRC1[66:64] * 32))[31:0];
/// DEST[127:96] := (SRC2[255:0] >> (SRC1[98:96] * 32))[31:0];
/// DEST[159:128] := (SRC2[255:0] >> (SRC1[130:128] * 32))[31:0];
/// DEST[191:160] := (SRC2[255:0] >> (SRC1[162:160] * 32))[31:0];
/// DEST[223:192] := (SRC2[255:0] >> (SRC1[194:192] * 32))[31:0];
/// DEST[255:224] := (SRC2[255:0] >> (SRC1[226:224] * 32))[31:0];
/// DEST[MAXVL-1:256] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMQ (EVEX - imm8 control forms)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC *is memory*)
///         THEN TMP_SRC[i+63:i] := SRC[63:0];
///         ELSE TMP_SRC[i+63:i] := SRC[i+63:i];
///     FI;
/// ENDFOR;
///     TMP_DEST[63:0] := (TMP_SRC[255:0] >> (IMM8[1:0] * 64))[63:0];
///     TMP_DEST[127:64] := (TMP_SRC[255:0] >> (IMM8[3:2] * 64))[63:0];
///     TMP_DEST[191:128] := (TMP_SRC[255:0] >> (IMM8[5:4] * 64))[63:0];
///     TMP_DEST[255:192] := (TMP_SRC[255:0] >> (IMM8[7:6] * 64))[63:0];
/// IF VL >= 512
///     TMP_DEST[319:256] := (TMP_SRC[511:256] >> (IMM8[1:0] * 64))[63:0];
///     TMP_DEST[383:320] := (TMP_SRC[511:256] >> (IMM8[3:2] * 64))[63:0];
///     TMP_DEST[447:384] := (TMP_SRC[511:256] >> (IMM8[5:4] * 64))[63:0];
///     TMP_DEST[511:448] := (TMP_SRC[511:256] >> (IMM8[7:6] * 64))[63:0];
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                                 ;zeroing-masking
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMQ (EVEX - vector control forms)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0];
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i];
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[63:0] := (TMP_SRC2[255:0] >> (SRC1[1:0] * 64))[63:0];
///     TMP_DEST[127:64] := (TMP_SRC2[255:0] >> (SRC1[65:64] * 64))[63:0];
///     TMP_DEST[191:128] := (TMP_SRC2[255:0] >> (SRC1[129:128] * 64))[63:0];
///     TMP_DEST[255:192] := (TMP_SRC2[255:0] >> (SRC1[193:192] * 64))[63:0];
/// FI;
/// IF VL = 512
///     TMP_DEST[63:0] := (TMP_SRC2[511:0] >> (SRC1[2:0] * 64))[63:0];
///     TMP_DEST[127:64] := (TMP_SRC2[511:0] >> (SRC1[66:64] * 64))[63:0];
///     TMP_DEST[191:128] := (TMP_SRC2[511:0] >> (SRC1[130:128] * 64))[63:0];
///     TMP_DEST[319:256] := (TMP_SRC2[511:0] >> (SRC1[258:256] * 64))[63:0];
///     TMP_DEST[383:320] := (TMP_SRC2[511:0] >> (SRC1[322:320] * 64))[63:0];
///     TMP_DEST[447:384] := (TMP_SRC2[511:0] >> (SRC1[386:384] * 64))[63:0];
///     TMP_DEST[511:448] := (TMP_SRC2[511:0] >> (SRC1[450:448] * 64))[63:0];
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                                 ;zeroing-masking
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMQ (VEX.256 encoded version)
/// DEST[63:0] := (SRC[255:0] >> (IMM8[1:0] * 64))[63:0];
/// DEST[127:64] := (SRC[255:0] >> (IMM8[3:2] * 64))[63:0];
/// DEST[191:128] := (SRC[255:0] >> (IMM8[5:4] * 64))[63:0];
/// DEST[255:192] := (SRC[255:0] >> (IMM8[7:6] * 64))[63:0];
/// DEST[MAXVL-1:256] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMT2B (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128:
///     id := 3;
/// ELSE IF VL = 256:
///     id := 4;
/// ELSE IF VL = 512:
///     id := 5;
/// FI;
/// TMP_DEST[VL-1:0] := DEST[VL-1:0];
/// FOR j := 0 TO KL-1
///     off := 8*SRC1[j*8 + id: j*8] ;
///     IF k1[j] OR *no writemask*:
///         DEST[j*8 + 7: j*8] := SRC1[j*8+id+1]? SRC2[off+7:off] : TMP_DEST[off+7:off];
///     ELSE IF *zeroing-masking*
///         DEST[j*8 + 7: j*8] := 0;
///     *ELSE
///         DEST[j*8 + 7: j*8] remains unchanged*
///     FI;
/// ENDFOR
/// DEST[MAX_VL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpermt2b() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMT2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=SRC1[i+id+1] ? SRC2[off+15:off]
///                     : TMP_DEST[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2D/VPERMT2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC1[i+id+1] ? SRC2[31:0]
///                     : TMP_DEST[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := SRC1[i+id+1] ? SRC2[off+31:off]
///                     : TMP_DEST[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2Q/VPERMT2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
///     i := j * 64
///     off := 64*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+id+1] ? SRC2[63:0]
///                     : TMP_DEST[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := SRC1[i+id+1] ? SRC2[off+63:off]
///                     : TMP_DEST[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermt2d() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMT2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=SRC1[i+id+1] ? SRC2[off+15:off]
///                     : TMP_DEST[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2D/VPERMT2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC1[i+id+1] ? SRC2[31:0]
///                     : TMP_DEST[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := SRC1[i+id+1] ? SRC2[off+31:off]
///                     : TMP_DEST[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2Q/VPERMT2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
///     i := j * 64
///     off := 64*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+id+1] ? SRC2[63:0]
///                     : TMP_DEST[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := SRC1[i+id+1] ? SRC2[off+63:off]
///                     : TMP_DEST[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermt2pd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMT2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=SRC1[i+id+1] ? SRC2[off+15:off]
///                     : TMP_DEST[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2D/VPERMT2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC1[i+id+1] ? SRC2[31:0]
///                     : TMP_DEST[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := SRC1[i+id+1] ? SRC2[off+31:off]
///                     : TMP_DEST[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2Q/VPERMT2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
///     i := j * 64
///     off := 64*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+id+1] ? SRC2[63:0]
///                     : TMP_DEST[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := SRC1[i+id+1] ? SRC2[off+63:off]
///                     : TMP_DEST[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermt2ps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMT2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=SRC1[i+id+1] ? SRC2[off+15:off]
///                     : TMP_DEST[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2D/VPERMT2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC1[i+id+1] ? SRC2[31:0]
///                     : TMP_DEST[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := SRC1[i+id+1] ? SRC2[off+31:off]
///                     : TMP_DEST[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2Q/VPERMT2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
///     i := j * 64
///     off := 64*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+id+1] ? SRC2[63:0]
///                     : TMP_DEST[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := SRC1[i+id+1] ? SRC2[off+63:off]
///                     : TMP_DEST[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermt2q() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMT2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     id := 2
/// FI;
/// IF VL = 256
///     id := 3
/// FI;
/// IF VL = 512
///     id := 4
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 16
///     off := 16*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+15:i]=SRC1[i+id+1] ? SRC2[off+15:off]
///                     : TMP_DEST[off+15:off]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2D/VPERMT2PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     id := 1
/// FI;
/// IF VL = 256
///     id := 2
/// FI;
/// IF VL = 512
///     id := 3
/// FI;
/// TMP_DEST := DEST
/// FOR j := 0 TO KL-1
///     i := j * 32
///     off := 32*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC1[i+id+1] ? SRC2[31:0]
///                     : TMP_DEST[off+31:off]
///                 ELSE
///                     DEST[i+31:i] := SRC1[i+id+1] ? SRC2[off+31:off]
///                     : TMP_DEST[off+31:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMT2Q/VPERMT2PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8 512)
/// IF VL = 128
///     id := 0
/// FI;
/// IF VL = 256
///     id := 1
/// FI;
/// IF VL = 512
///     id := 2
/// FI;
/// TMP_DEST:= DEST
///     i := j * 64
///     off := 64*SRC1[i+id:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+id+1] ? SRC2[63:0]
///                     : TMP_DEST[off+63:off]
///                 ELSE
///                     DEST[i+63:i] := SRC1[i+id+1] ? SRC2[off+63:off]
///                     : TMP_DEST[off+63:off]
///                 FI
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpermt2w() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPERMD (EVEX encoded versions)
/// (KL, VL) = (8, 256), (16, 512)
/// IF VL = 256 THEN n := 2; FI;
/// IF VL = 512 THEN n := 3; FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     id := 32*SRC1[i+n:i]
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := SRC2[31:0];
///                     ELSE DEST[i+31:i] := SRC2[id+31:id];
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPERMD (VEX.256 encoded version)
/// DEST[31:0] := (SRC2[255:0] >> (SRC1[2:0] * 32))[31:0];
/// DEST[63:32] := (SRC2[255:0] >> (SRC1[34:32] * 32))[31:0];
/// DEST[95:64] := (SRC2[255:0] >> (SRC1[66:64] * 32))[31:0];
/// DEST[127:96] := (SRC2[255:0] >> (SRC1[98:96] * 32))[31:0];
/// DEST[159:128] := (SRC2[255:0] >> (SRC1[130:128] * 32))[31:0];
/// DEST[191:160] := (SRC2[255:0] >> (SRC1[162:160] * 32))[31:0];
/// DEST[223:192] := (SRC2[255:0] >> (SRC1[194:192] * 32))[31:0];
/// DEST[255:224] := (SRC2[255:0] >> (SRC1[226:224] * 32))[31:0];
/// DEST[MAXVL-1:256] := 0
/// VPERMW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128 THEN n := 2; FI;
/// IF VL = 256 THEN n := 3; FI;
/// IF VL = 512 THEN n := 4; FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     id := 16*SRC1[i+n:i]
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC2[id+15:id]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpermw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPEXPANDB
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.byte[j] := SRC.byte[k];
///         k := k + 1
///         ELSE:
///             IF *merging-masking*:
///                 *DEST.byte[j] remains unchanged*
///                 EzeLroSiEn: g;-m asking
///                     DEST.byte[j] := 0
/// DEST[MAX_VL-1:VL] := 0
/// VPEXPANDW
/// (KL, VL) = (8,128), (16,256), (32, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.word[j] := SRC.word[k];
///         k := k + 1
///         ELSE:
///             IF *merging-masking*:
///                 *DEST.word[j] remains unchanged*
///                 ELSE:
///                         ; zeroing-masking
///                     DEST.word[j] := 0
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpexpandb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPEXPANDD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+31:i] := SRC[k+31:k];
///                 k := k + 32
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpexpandd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPEXPANDQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// k := 0
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 DEST[i+63:i] := SRC[k+63:k];
///                 k := k + 64
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpexpandq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPEXPANDB
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.byte[j] := SRC.byte[k];
///         k := k + 1
///         ELSE:
///             IF *merging-masking*:
///                 *DEST.byte[j] remains unchanged*
///                 EzeLroSiEn: g;-m asking
///                     DEST.byte[j] := 0
/// DEST[MAX_VL-1:VL] := 0
/// VPEXPANDW
/// (KL, VL) = (8,128), (16,256), (32, 512)
/// k := 0
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.word[j] := SRC.word[k];
///         k := k + 1
///         ELSE:
///             IF *merging-masking*:
///                 *DEST.word[j] remains unchanged*
///                 ELSE:
///                         ; zeroing-masking
///                     DEST.word[j] := 0
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpexpandw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VPGATHERDD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j]
///         THEN DEST[i+31:i] := MEM[BASE_ADDR +
///                 SignExtend(VINDEX[i+31:i]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+31:i] := remains unchanged*
///                     ; Only merging masking is allowed
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// VPGATHERDQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j]
///         THEN DEST[i+63:i] :=
///             MEM[BASE_ADDR + SignExtend(VINDEX[k+31:k]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+63:i] := remains unchanged*
///                     ; Only merging masking is allowed
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpgatherdd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC1;
/// BASE_ADDR: base register encoded in VSIB addressing;
/// VINDEX: the vector index register encoded by VSIB addressing;
/// SCALE: scale factor encoded by SIB:[7:6];
/// DISP: optional 1, 4 byte displacement;
/// MASK := SRC3;
/// VPGATHERDQ (VEX.128 version)
/// MASK[MAXVL-1:128] := 0;
/// FOR j := 0 to 1
///     i := j * 64;
///     IF MASK[63+i] THEN
///         MASK[i +63:i] := FFFFFFFF_FFFFFFFFH; // extend from most significant bit
///     ELSE
///         MASK[i +63:i] := 0;
///     FI;
/// ENDFOR
/// FOR j := 0 to 1
///     k := j * 32;
///     i := j * 64;
///     DATA_ADDR := BASE_ADDR + (SignExtend(VINDEX[k+31:k])*SCALE + DISP);
///     IF MASK[63+i] THEN
///         DEST[i +63:i] := FETCH_64BITS(DATA_ADDR); // a fault exits the instruction
///     FI;
///     MASK[i +63:i] := 0;
/// ENDFOR
/// VPGATHERQQ (VEX.128 version)
/// MASK[MAXVL-1:128] := 0;
/// FOR j := 0 to 1
///     i := j * 64;
///     IF MASK[63+i] THEN
///         MASK[i +63:i] := FFFFFFFF_FFFFFFFFH; // extend from most significant bit
///     ELSE
///         MASK[i +63:i] := 0;
///     FI;
/// ENDFOR
/// FOR j := 0 to 1
///     i := j * 64;
///     DATA_ADDR := BASE_ADDR + (SignExtend(VINDEX1[i+63:i])*SCALE + DISP);
///     IF MASK[63+i] THEN
///         DEST[i +63:i] := FETCH_64BITS(DATA_ADDR); // a fault exits the instruction
///     FI;
///     MASK[i +63:i] := 0;
/// ENDFOR
/// DEST[MAXVL-1:128] := 0;
/// VPGATHERQQ (VEX.256 version)
/// MASK[MAXVL-1:256] := 0;
/// FOR j := 0 to 3
///     i := j * 64;
///     IF MASK[63+i] THEN
///         MASK[i +63:i] := FFFFFFFF_FFFFFFFFH; // extend from most significant bit
///     ELSE
///         MASK[i +63:i] := 0;
///     FI;
/// ENDFOR
/// FOR j := 0 to 3
///     i := j * 64;
///     DATA_ADDR := BASE_ADDR + (SignExtend(VINDEX1[i+63:i])*SCALE + DISP);
///     IF MASK[63+i] THEN
///         DEST[i +63:i] := FETCH_64BITS(DATA_ADDR); // a fault exits the instruction
///     FI;
///     MASK[i +63:i] := 0;
/// ENDFOR
/// DEST[MAXVL-1:256] := 0;
/// VPGATHERDQ (VEX.256 version)
/// MASK[MAXVL-1:256] := 0;
/// FOR j := 0 to 3
///     i := j * 64;
///     IF MASK[63+i] THEN
///         MASK[i +63:i] := FFFFFFFF_FFFFFFFFH; // extend from most significant bit
///     ELSE
///         MASK[i +63:i] := 0;
///     FI;
/// ENDFOR
/// FOR j := 0 to 3
///     k := j * 32;
///     i := j * 64;
///     IF MASK[63+i] THEN
///         DEST[i +63:i] := FETCH_64BITS(DATA_ADDR); // a fault exits the instruction
///     FI;
///     MASK[i +63:i] := 0;
/// ENDFOR
/// DEST[MAXVL-1:256] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpgatherdq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VPGATHERQD (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j]
///         THEN DEST[i+31:i] := MEM[BASE_ADDR + (VINDEX[k+63:k]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+31:i] := remains unchanged*
///                 ; Only merging masking is allowed
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL/2] := 0
/// VPGATHERQQ (EVEX encoded version)
/// (KL, VL) = (2, 64), (4, 128), (8, 256)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j]
///         THEN DEST[i+63:i] :=
///             MEM[BASE_ADDR + (VINDEX[i+63:i]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+63:i] := remains unchanged*
///                 ; Only merging masking is allowed
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpgatherqd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VPGATHERQD (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j]
///         THEN DEST[i+31:i] := MEM[BASE_ADDR + (VINDEX[k+63:k]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+31:i] := remains unchanged*
///                 ; Only merging masking is allowed
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL/2] := 0
/// VPGATHERQQ (EVEX encoded version)
/// (KL, VL) = (2, 64), (4, 128), (8, 256)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j]
///         THEN DEST[i+63:i] :=
///             MEM[BASE_ADDR + (VINDEX[i+63:i]) * SCALE + DISP]
///             k1[j] := 0
///         ELSE *DEST[i+63:i] := remains unchanged*
///                 ; Only merging masking is allowed
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpgatherqq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPLZCNTD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             temp := 32
/// 
///             DEST[i+31:i] := 0
/// 
///             WHILE (temp > 0) AND (SRC[i+temp-1] = 0)
///         DO
///                 temp := temp - 1
///                 DEST[i+31:i] := DEST[i+31:i] + 1
///         OD
///         ELSE
///         IF *merging-masking*
///             THEN *DEST[i+31:i] remains unchanged*
///             ELSE DEST[i+31:i] := 0
///         FI
///     FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPLZCNTQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j*64
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             temp := 64
/// 
///             DEST[i+63:i] := 0
/// 
///             WHILE (temp > 0) AND (SRC[i+temp-1] = 0)
///         DO
///                 temp := temp - 1
///                 DEST[i+63:i] := DEST[i+63:i] + 1
///         OD
///         ELSE
///         IF *merging-masking*
///             THEN *DEST[i+63:i] remains unchanged*
///             ELSE DEST[i+63:i] := 0
///         FI
///     FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vplzcntd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPLZCNTD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             temp := 32
/// 
///             DEST[i+31:i] := 0
/// 
///             WHILE (temp > 0) AND (SRC[i+temp-1] = 0)
///         DO
///                 temp := temp - 1
///                 DEST[i+31:i] := DEST[i+31:i] + 1
///         OD
///         ELSE
///         IF *merging-masking*
///             THEN *DEST[i+31:i] remains unchanged*
///             ELSE DEST[i+31:i] := 0
///         FI
///     FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPLZCNTQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j*64
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             temp := 64
/// 
///             DEST[i+63:i] := 0
/// 
///             WHILE (temp > 0) AND (SRC[i+temp-1] = 0)
///         DO
///                 temp := temp - 1
///                 DEST[i+63:i] := DEST[i+63:i] + 1
///         OD
///         ELSE
///         IF *merging-masking*
///             THEN *DEST[i+63:i] remains unchanged*
///             ELSE DEST[i+63:i] := 0
///         FI
///     FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vplzcntq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMADD52HUQ (EVEX encoded)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64;
///     IF k1[j] OR *no writemask* THEN
///         IF src2 is Memory AND EVEX.b=1 THEN
///             tsrc2[63:0] := ZeroExtend64(src2[51:0]);
///         ELSE
///             tsrc2[63:0] := ZeroExtend64(src2[i+51:i];
///         FI;
///         Temp128[127:0] := ZeroExtend64(src1[i+51:i]) * tsrc2[63:0];
///         Temp2[63:0] := DEST[i+63:i] + ZeroExtend64(temp128[103:52]) ;
///         DEST[i+63:i] := Temp2[63:0];
///     ELSE
///         IF *zeroing-masking* THEN
///             DEST[i+63:i] := 0;
///         ELSE *merge-masking*
///             DEST[i+63:i] is unchanged;
///         FI;
///     FI;
/// ENDFOR
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmadd52huq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMADD52LUQ (EVEX encoded)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64;
///     IF k1[j] OR *no writemask* THEN
///         IF src2 is Memory AND EVEX.b=1 THEN
///             tsrc2[63:0] := ZeroExtend64(src2[51:0]);
///         ELSE
///             tsrc2[63:0] := ZeroExtend64(src2[i+51:i];
///         FI;
///         Temp128[127:0] := ZeroExtend64(src1[i+51:i]) * tsrc2[63:0];
///         Temp2[63:0] := DEST[i+63:i] + ZeroExtend64(temp128[51:0]) ;
///         DEST[i+63:i] := Temp2[63:0];
///     ELSE
///         IF *zeroing-masking* THEN
///             DEST[i+63:i] := 0;
///         ELSE *merge-masking*
///             DEST[i+63:i] is unchanged;
///         FI;
///     FI;
/// ENDFOR
/// DEST[MAX_VL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpmadd52luq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMASKMOVD - 256-bit load
/// DEST[31:0] := IF (SRC1[31]) Load_32(mem) ELSE 0
/// DEST[63:32] := IF (SRC1[63]) Load_32(mem + 4) ELSE 0
/// DEST[95:64] := IF (SRC1[95]) Load_32(mem + 8) ELSE 0
/// DEST[127:96] := IF (SRC1[127]) Load_32(mem + 12) ELSE 0
/// DEST[159:128] := IF (SRC1[159]) Load_32(mem + 16) ELSE 0
/// DEST[191:160] := IF (SRC1[191]) Load_32(mem + 20) ELSE 0
/// DEST[223:192] := IF (SRC1[223]) Load_32(mem + 24) ELSE 0
/// DEST[255:224] := IF (SRC1[255]) Load_32(mem + 28) ELSE 0
/// VPMASKMOVD -128-bit load
/// DEST[31:0] := IF (SRC1[31]) Load_32(mem) ELSE 0
/// DEST[63:32] := IF (SRC1[63]) Load_32(mem + 4) ELSE 0
/// DEST[95:64] := IF (SRC1[95]) Load_32(mem + 8) ELSE 0
/// DEST[127:97] := IF (SRC1[127]) Load_32(mem + 12) ELSE 0
/// DEST[MAXVL-1:128] := 0
/// VPMASKMOVQ - 256-bit load
/// DEST[63:0] := IF (SRC1[63]) Load_64(mem) ELSE 0
/// DEST[127:64] := IF (SRC1[127]) Load_64(mem + 8) ELSE 0
/// DEST[195:128] := IF (SRC1[191]) Load_64(mem + 16) ELSE 0
/// DEST[255:196] := IF (SRC1[255]) Load_64(mem + 24) ELSE 0
/// VPMASKMOVQ - 128-bit load
/// DEST[63:0] := IF (SRC1[63]) Load_64(mem) ELSE 0
/// DEST[127:64] := IF (SRC1[127]) Load_64(mem + 16) ELSE 0
/// DEST[MAXVL-1:128] := 0
/// VPMASKMOVD - 256-bit store
/// IF (SRC1[31]) DEST[31:0] := SRC2[31:0]
/// IF (SRC1[63]) DEST[63:32] := SRC2[63:32]
/// IF (SRC1[95]) DEST[95:64] := SRC2[95:64]
/// IF (SRC1[127]) DEST[127:96] := SRC2[127:96]
/// IF (SRC1[159]) DEST[159:128] :=SRC2[159:128]
/// IF (SRC1[191]) DEST[191:160] := SRC2[191:160]
/// IF (SRC1[223]) DEST[223:192] := SRC2[223:192]
/// IF (SRC1[255]) DEST[255:224] := SRC2[255:224]
/// VPMASKMOVD - 128-bit store
/// IF (SRC1[31]) DEST[31:0] := SRC2[31:0]
/// IF (SRC1[63]) DEST[63:32] := SRC2[63:32]
/// IF (SRC1[95]) DEST[95:64] := SRC2[95:64]
/// IF (SRC1[127]) DEST[127:96] := SRC2[127:96]
/// VPMASKMOVQ - 256-bit store
/// IF (SRC1[63]) DEST[63:0] := SRC2[63:0]
/// IF (SRC1[127]) DEST[127:64] :=SRC2[127:64]
/// IF (SRC1[191]) DEST[191:128] := SRC2[191:128]
/// IF (SRC1[255]) DEST[255:192] := SRC2[255:192]
/// VPMASKMOVQ - 128-bit store
/// IF (SRC1[63]) DEST[63:0] := SRC2[63:0]
/// IF (SRC1[127]) DEST[127:64] :=SRC2[127:64]
/// ```
#[box_to_static_reference]
pub(super) fn vpmaskmov() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVB2M (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF SRC[i+7]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVW2M (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF SRC[i+15]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVD2M (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF SRC[i+31]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVQ2M (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF SRC[i+63]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmovb2m() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVB2M (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF SRC[i+7]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVW2M (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF SRC[i+15]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVD2M (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF SRC[i+31]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVQ2M (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF SRC[i+63]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmovd2m() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVSDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVUSDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovdb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovdw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVM2B (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF SRC[j]
///         THEN DEST[i+7:i] := -1
///         ELSE DEST[i+7:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF SRC[j]
///         THEN DEST[i+15:i] := -1
///         ELSE DEST[i+15:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2D (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF SRC[j]
///         THEN DEST[i+31:i] := -1
///         ELSE DEST[i+31:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2Q (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF SRC[j]
///         THEN DEST[i+63:i] := -1
///         ELSE DEST[i+63:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmovm2b() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVM2B (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF SRC[j]
///         THEN DEST[i+7:i] := -1
///         ELSE DEST[i+7:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF SRC[j]
///         THEN DEST[i+15:i] := -1
///         ELSE DEST[i+15:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2D (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF SRC[j]
///         THEN DEST[i+31:i] := -1
///         ELSE DEST[i+31:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2Q (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF SRC[j]
///         THEN DEST[i+63:i] := -1
///         ELSE DEST[i+63:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmovm2d() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVM2B (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF SRC[j]
///         THEN DEST[i+7:i] := -1
///         ELSE DEST[i+7:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF SRC[j]
///         THEN DEST[i+15:i] := -1
///         ELSE DEST[i+15:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2D (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF SRC[j]
///         THEN DEST[i+31:i] := -1
///         ELSE DEST[i+31:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2Q (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF SRC[j]
///         THEN DEST[i+63:i] := -1
///         ELSE DEST[i+63:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmovm2q() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVM2B (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF SRC[j]
///         THEN DEST[i+7:i] := -1
///         ELSE DEST[i+7:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2W (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF SRC[j]
///         THEN DEST[i+15:i] := -1
///         ELSE DEST[i+15:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2D (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF SRC[j]
///         THEN DEST[i+31:i] := -1
///         ELSE DEST[i+31:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVM2Q (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF SRC[j]
///         THEN DEST[i+63:i] := -1
///         ELSE DEST[i+63:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmovm2w() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVB2M (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF SRC[i+7]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVW2M (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF SRC[i+15]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVD2M (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF SRC[i+31]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVQ2M (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF SRC[i+63]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmovq2m() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVSQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVUSQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovqb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := TruncateQuadWordToDWord (SRC[m+63:m])
///             ELSE *zeroing-masking*
///                             ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := TruncateQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateSignedQuadWordToDWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                     ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateSignedQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateUnsignedQuadWordToDWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                     ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateUnsignedQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovqd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVSQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVUSQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovqw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVSDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVUSDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovsdb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovsdw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVSQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVUSQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovsqb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := TruncateQuadWordToDWord (SRC[m+63:m])
///             ELSE *zeroing-masking*
///                             ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := TruncateQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateSignedQuadWordToDWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                     ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateSignedQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateUnsignedQuadWordToDWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                     ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateUnsignedQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovsqd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVSQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVUSQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovsqw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovswb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVSDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSDB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedDoubleWordToByte (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVUSDB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedDoubleWordToByte (SRC[m+31:m])
///             ELSE *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovusdb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSDW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSDW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (4, 128), (8, 256), (16, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 32
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedDoubleWordToWord (SRC[m+31:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovusdw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVSQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/8] := 0;
/// VPMOVUSQB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 8
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedQuadWordToByte (SRC[m+63:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*
///                                 ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovusqb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := TruncateQuadWordToDWord (SRC[m+63:m])
///             ELSE *zeroing-masking*
///                             ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := TruncateQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateSignedQuadWordToDWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                     ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateSignedQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQD instruction (EVEX encoded version) reg-reg form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateUnsignedQuadWordToDWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                                 ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                     ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSQD instruction (EVEX encoded version) memory form
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 32
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SaturateUnsignedQuadWordToDWord (SRC[m+63:m])
///             ELSE *DEST[i+31:i] remains unchanged*
///                                     ; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovusqd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := TruncateQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVSQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateSignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSQW instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/4] := 0;
/// VPMOVUSQW instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (2, 128), (4, 256), (8, 512)
///     FOR j := 0 TO KL-1
///         i := j * 16
///         m := j * 64
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+15:i] := SaturateUnsignedQuadWordToWord (SRC[m+63:m])
///             ELSE
///                 *DEST[i+15:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovusqw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovuswb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVB2M (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF SRC[i+7]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVW2M (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF SRC[i+15]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVD2M (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF SRC[i+31]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPMOVQ2M (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF SRC[i+63]
///         THEN DEST:1[= j]
///         ELSE DES0T:=[j ]
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpmovw2m() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMOVWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := TruncateWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVSWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVSWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateSignedWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// VPMOVUSWB instruction (EVEX encoded versions) when dest is a register
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedWordToByte (SRC[m+15:m])
///             ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///                     ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                         DEST[i+7:i] = 0
///                 FI
///         FI;
///     ENDFOR
///     DEST[MAXVL-1:VL/2] := 0;
/// VPMOVUSWB instruction (EVEX encoded versions) when dest is memory
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
///     FOR j := 0 TO Kl-1
///         i := j * 8
///         m := j * 16
///         IF k1[j] OR *no writemask*
///             THEN DEST[i+7:i] := SaturateUnsignedWordToByte (SRC[m+15:m])
///             ELSE
///                 *DEST[i+7:i] remains unchanged*; merging-masking
///         FI;
///     ENDFOR
/// ```
#[box_to_static_reference]
pub(super) fn vpmovwb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMULTISHIFTQB DEST, SRC1, SRC2 (EVEX encoded version)
/// (KL, VL) = (2, 128),(4, 256), (8, 512)
/// FOR i := 0 TO KL-1
///     IF EVEX.b=1 AND src2 is memory THEN
///             tcur := src2.qword[0]; //broadcasting
///     ELSE
///             tcur := src2.qword[i];
///     FI;
///     FOR j := 0 to 7
///         ctrl := src1.qword[i].byte[j] & 63;
///         FOR k := 0 to 7
///             res.bit[k] := tcur.bit[ (ctrl+k) mod 64 ];
///         ENDFOR
///         IF k1[i*8+j] or no writemask THEN
///             DEST.qword[i].byte[j] := res;
///         ELSE IF zeroing-masking THEN
///             DEST.qword[i].byte[j] := 0;
///     ENDFOR
/// ENDFOR
/// DEST.qword[MAX_VL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpmultishiftqb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPOPCNTB
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1:
///     IF MaskBit(j) OR *no writemask*:
///         DEST.byte[j] := POPCNT(SRC.byte[j])
///     ELSE IF *merging-masking*:
///         *DEST.byte[j] remains unchanged*
///     ELSE:
///         DEST.byte[j] := 0
/// DEST[MAX_VL-1:VL] := 0
/// VPOPCNTW
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1:
///     IF MaskBit(j) OR *no writemask*:
///         DEST.word[j] := POPCNT(SRC.word[j])
///     ELSE IF *merging-masking*:
///         *DEST.word[j] remains unchanged*
///     ELSE:
///         DEST.word[j] := 0
/// DEST[MAX_VL-1:VL] := 0
/// VPOPCNTD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1:
///     IF MaskBit(j) OR *no writemask*:
///         IF SRC is broadcast memop:
///             t := SRC.dword[0]
///         ELSE:
///             t := SRC.dword[j]
///         DEST.dword[j] := POPCNT(t)
///     ELSE IF *merging-masking*:
///         *DEST..dword[j] remains unchanged*
///     ELSE:
///         DEST..dword[j] := 0
/// DEST[MAX_VL-1:VL] := 0
/// VPOPCNTQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1:
///     IF MaskBit(j) OR *no writemask*:
///         IF SRC is broadcast memop:
///             t := SRC.qword[0]
///         ELSE:
///             t := SRC.qword[j]
///         DEST.qword[j] := POPCNT(t)
///     ELSE IF *merging-masking*:
///         *DEST..qword[j] remains unchanged*
///     ELSE:
///         DEST..qword[j] := 0
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpopcnt() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// LEFT_ROTATE_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 32;
/// DEST[31:0] := (SRC << COUNT) | (SRC >> (32 - COUNT));
/// LEFT_ROTATE_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 64;
/// DEST[63:0] := (SRC << COUNT) | (SRC >> (64 - COUNT));
/// VPROLD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLVD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[31:0])
///                 ELSE DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[i+31:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLVQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[63:0])
///                 ELSE DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vprold() -> &'static [IrStatement] {
    let assignment = assign(b::or(b::shl(o2(), o3()), b::shr(o2(), b::sub(bit_size_of_o2(), o3()))), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// LEFT_ROTATE_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 32;
/// DEST[31:0] := (SRC << COUNT) | (SRC >> (32 - COUNT));
/// LEFT_ROTATE_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 64;
/// DEST[63:0] := (SRC << COUNT) | (SRC >> (64 - COUNT));
/// VPROLD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLVD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[31:0])
///                 ELSE DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[i+31:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLVQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[63:0])
///                 ELSE DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vprolq() -> &'static [IrStatement] {
    let assignment = assign(b::or(b::shl(o2(), o3()), b::shr(o2(), b::sub(bit_size_of_o2(), o3()))), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// LEFT_ROTATE_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 32;
/// DEST[31:0] := (SRC << COUNT) | (SRC >> (32 - COUNT));
/// LEFT_ROTATE_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 64;
/// DEST[63:0] := (SRC << COUNT) | (SRC >> (64 - COUNT));
/// VPROLD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLVD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[31:0])
///                 ELSE DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[i+31:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLVQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[63:0])
///                 ELSE DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vprolvd() -> &'static [IrStatement] {
    let assignment = assign(b::or(b::shl(o2(), o3()), b::shr(o2(), b::sub(bit_size_of_o2(), o3()))), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// LEFT_ROTATE_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 32;
/// DEST[31:0] := (SRC << COUNT) | (SRC >> (32 - COUNT));
/// LEFT_ROTATE_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 64;
/// DEST[63:0] := (SRC << COUNT) | (SRC >> (64 - COUNT));
/// VPROLD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLVD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[31:0])
///                 ELSE DEST[i+31:i] := LEFT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[i+31:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPROLVQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[63:0])
///                 ELSE DEST[i+63:i] := LEFT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vprolvq() -> &'static [IrStatement] {
    let assignment = assign(b::or(b::shl(o2(), o3()), b::shr(o2(), b::sub(bit_size_of_o2(), o3()))), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RIGHT_ROTATE_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 32;
/// DEST[31:0] := (SRC >> COUNT) | (SRC << (32 - COUNT));
/// RIGHT_ROTATE_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 64;
/// DEST[63:0] := (SRC >> COUNT) | (SRC << (64 - COUNT));
/// VPRORD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := RIGHT_ROTATE_DWORDS( SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORVD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[31:0])
///                 ELSE DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[i+31:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], imm8])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORVQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[63:0])
///                 ELSE DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vprord() -> &'static [IrStatement] {
    let assignment = assign(b::or(b::shl(o2(), o3()), b::shr(o2(), b::sub(bit_size_of_o2(), o3()))), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RIGHT_ROTATE_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 32;
/// DEST[31:0] := (SRC >> COUNT) | (SRC << (32 - COUNT));
/// RIGHT_ROTATE_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 64;
/// DEST[63:0] := (SRC >> COUNT) | (SRC << (64 - COUNT));
/// VPRORD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := RIGHT_ROTATE_DWORDS( SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORVD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[31:0])
///                 ELSE DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[i+31:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], imm8])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORVQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[63:0])
///                 ELSE DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vprorq() -> &'static [IrStatement] {
    let assignment = assign(b::or(b::shl(o2(), o3()), b::shr(o2(), b::sub(bit_size_of_o2(), o3()))), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RIGHT_ROTATE_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 32;
/// DEST[31:0] := (SRC >> COUNT) | (SRC << (32 - COUNT));
/// RIGHT_ROTATE_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 64;
/// DEST[63:0] := (SRC >> COUNT) | (SRC << (64 - COUNT));
/// VPRORD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := RIGHT_ROTATE_DWORDS( SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORVD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[31:0])
///                 ELSE DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[i+31:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], imm8])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORVQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[63:0])
///                 ELSE DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vprorvd() -> &'static [IrStatement] {
    let assignment = assign(b::or(b::shl(o2(), o3()), b::shr(o2(), b::sub(bit_size_of_o2(), o3()))), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RIGHT_ROTATE_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 32;
/// DEST[31:0] := (SRC >> COUNT) | (SRC << (32 - COUNT));
/// RIGHT_ROTATE_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC modulo 64;
/// DEST[63:0] := (SRC >> COUNT) | (SRC << (64 - COUNT));
/// VPRORD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := RIGHT_ROTATE_DWORDS( SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], imm8)
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORVD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[31:0])
///                 ELSE DEST[i+31:i] := RIGHT_ROTATE_DWORDS(SRC1[i+31:i], SRC2[i+31:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], imm8])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPRORVQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[63:0])
///                 ELSE DEST[i+63:i] := RIGHT_ROTATE_QWORDS(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vprorvq() -> &'static [IrStatement] {
    let assignment = assign(b::or(b::shl(o2(), o3()), b::shr(o2(), b::sub(bit_size_of_o2(), o3()))), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VPSCATTERDD (EVEX encoded versions)
/// (KL, VL)= (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR +SignExtend(VINDEX[i+31:i]) * SCALE + DISP] := SRC[i+31:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERDQ (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR +SignExtend(VINDEX[k+31:k]) * SCALE + DISP] := SRC[i+63:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERQD (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR + (VINDEX[k+63:k]) * SCALE + DISP] := SRC[i+31:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERQQ (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR + (VINDEX[j+63:j]) * SCALE + DISP] := SRC[i+63:i]
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpscatterdd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VPSCATTERDD (EVEX encoded versions)
/// (KL, VL)= (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR +SignExtend(VINDEX[i+31:i]) * SCALE + DISP] := SRC[i+31:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERDQ (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR +SignExtend(VINDEX[k+31:k]) * SCALE + DISP] := SRC[i+63:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERQD (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR + (VINDEX[k+63:k]) * SCALE + DISP] := SRC[i+31:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERQQ (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR + (VINDEX[j+63:j]) * SCALE + DISP] := SRC[i+63:i]
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpscatterdq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VPSCATTERDD (EVEX encoded versions)
/// (KL, VL)= (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR +SignExtend(VINDEX[i+31:i]) * SCALE + DISP] := SRC[i+31:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERDQ (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR +SignExtend(VINDEX[k+31:k]) * SCALE + DISP] := SRC[i+63:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERQD (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR + (VINDEX[k+63:k]) * SCALE + DISP] := SRC[i+31:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERQQ (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR + (VINDEX[j+63:j]) * SCALE + DISP] := SRC[i+63:i]
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpscatterqd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// BASE_ADDR stands for the memory operand base address (a GPR); may not exist
/// VINDEX stands for the memory operand vector of indices (a ZMM register)
/// SCALE stands for the memory operand scalar (1, 2, 4 or 8)
/// DISP is the optional 1 or 4 byte displacement
/// VPSCATTERDD (EVEX encoded versions)
/// (KL, VL)= (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR +SignExtend(VINDEX[i+31:i]) * SCALE + DISP] := SRC[i+31:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERDQ (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     k := j * 32
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR +SignExtend(VINDEX[k+31:k]) * SCALE + DISP] := SRC[i+63:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERQD (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     k := j * 64
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR + (VINDEX[k+63:k]) * SCALE + DISP] := SRC[i+31:i]
///             k1[j] := 0
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// VPSCATTERQQ (EVEX encoded versions)
/// (KL, VL)= (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN MEM[BASE_ADDR + (VINDEX[j+63:j]) * SCALE + DISP] := SRC[i+63:i]
///     FI;
/// ENDFOR
/// k1[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpscatterqq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSHLDW DEST, SRC2, SRC3, imm8
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1:
///     IF MaskBit(j) OR *no writemask*:
///         tmp := concat(SRC2.word[j], SRC3.word[j]) << (imm8 & 15)
///         DEST.word[j] := tmp.word[1]
///     ELSE IF *zeroing*:
///         DEST.word[j] := 0
///     *ELSE DEST.word[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VPSHLDD DEST, SRC2, SRC3, imm8
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC3 is broadcast memop:
///         tsrc3 := SRC3.dword[0]
///     ELSE:
///         tsrc3 := SRC3.dword[j]
///     IF MaskBit(j) OR *no writemask*:
///         tmp := concat(SRC2.dword[j], tsrc3) << (imm8 & 31)
///         DEST.dword[j] := tmp.dword[1]
///     ELSE IF *zeroing*:
///         DEST.dword[j] := 0
///     *ELSE DEST.dword[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VPSHLDQ DEST, SRC2, SRC3, imm8
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC3 is broadcast memop:
///         tsrc3 := SRC3.qword[0]
///     ELSE:
///         tsrc3 := SRC3.qword[j]
///     IF MaskBit(j) OR *no writemask*:
///         tmp := concat(SRC2.qword[j], tsrc3) << (imm8 & 63)
///         DEST.qword[j] := tmp.qword[1]
///     ELSE IF *zeroing*:
///         DEST.qword[j] := 0
///     *ELSE DEST.qword[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpshld() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// FUNCTION concat(a,b):
///     IF words:
///         d.word[1] := a
///         d.word[0] := b
///         return d
///     ELSE IF dwords:
///         q.dword[1] := a
///         q.dword[0] := b
///         return q
///     ELSE IF qwords:
///         o.qword[1] := a
///         o.qword[0] := b
///         return o
/// VPSHLDVW DEST, SRC2, SRC3
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1:
///     IF MaskBit(j) OR *no writemask*:
///         tmp := concat(DEST.word[j], SRC2.word[j]) << (SRC3.word[j] & 15)
///         DEST.word[j] := tmp.word[1]
///     ELSE IF *zeroing*:
///         DEST.word[j] := 0
///     *ELSE DEST.word[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VPSHLDVD DEST, SRC2, SRC3
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC3 is broadcast memop:
///         tsrc3 := SRC3.dword[0]
///     ELSE:
///         tsrc3 := SRC3.dword[j]
///     IF MaskBit(j) OR *no writemask*:
///         tmp := concat(DEST.dword[j], SRC2.dword[j]) << (tsrc3 & 31)
///         DEST.dword[j] := tmp.dword[1]
///     ELSE IF *zeroing*:
///         DEST.dword[j] := 0
///     *ELSE DEST.dword[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VPSHLDVQ DEST, SRC2, SRC3
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC3 is broadcast memop:
///         tsrc3 := SRC3.qword[0]
///     ELSE:
///         tsrc3 := SRC3.qword[j]
///     IF MaskBit(j) OR *no writemask*:
///         tmp := concat(DEST.qword[j], SRC2.qword[j]) << (tsrc3 & 63)
///         DEST.qword[j] := tmp.qword[1]
///     ELSE IF *zeroing*:
///         DEST.qword[j] := 0
///     *ELSE DEST.qword[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpshldv() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSHRDW DEST, SRC2, SRC3, imm8
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1:
///     IF MaskBit(j) OR *no writemask*:
///         DEST.word[j] := concat(SRC3.word[j], SRC2.word[j]) >> (imm8 & 15)
///     ELSE IF *zeroing*:
///         DEST.word[j] := 0
///     *ELSE DEST.word[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VPSHRDD DEST, SRC2, SRC3, imm8
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC3 is broadcast memop:
///         tsrc3 := SRC3.dword[0]
///     ELSE:
///         tsrc3 := SRC3.dword[j]
///     IF MaskBit(j) OR *no writemask*:
///         DEST.dword[j] := concat(tsrc3, SRC2.dword[j]) >> (imm8 & 31)
///     ELSE IF *zeroing*:
///         DEST.dword[j] := 0
///     *ELSE DEST.dword[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VPSHRDQ DEST, SRC2, SRC3, imm8
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC3 is broadcast memop:
///         tsrc3 := SRC3.qword[0]
///     ELSE:
///         tsrc3 := SRC3.qword[j]
///     IF MaskBit(j) OR *no writemask*:
///         DEST.qword[j] := concat(tsrc3, SRC2.qword[j]) >> (imm8 & 63)
///     ELSE IF *zeroing*:
///         DEST.qword[j] := 0
///     *ELSE DEST.qword[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpshrd() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSHRDVW DEST, SRC2, SRC3
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1:
///     IF MaskBit(j) OR *no writemask*:
///         DEST.word[j] := concat(SRC2.word[j], DEST.word[j]) >> (SRC3.word[j] & 15)
///     ELSE IF *zeroing*:
///         DEST.word[j] := 0
///     *ELSE DEST.word[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VPSHRDVD DEST, SRC2, SRC3
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC3 is broadcast memop:
///         tsrc3 := SRC3.dword[0]
///     ELSE:
///         tsrc3 := SRC3.dword[j]
///     IF MaskBit(j) OR *no writemask*:
///         DEST.dword[j] := concat(SRC2.dword[j], DEST.dword[j]) >> (tsrc3 & 31)
///     ELSE IF *zeroing*:
///         DEST.dword[j] := 0
///     *ELSE DEST.dword[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VPSHRDVQ DEST, SRC2, SRC3
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC3 is broadcast memop:
///         tsrc3 := SRC3.qword[0]
///     ELSE:
///         tsrc3 := SRC3.qword[j]
///     IF MaskBit(j) OR *no writemask*:
///         DEST.qword[j] := concat(SRC2.qword[j], DEST.qword[j]) >> (tsrc3 & 63)
///     ELSE IF *zeroing*:
///         DEST.qword[j] := 0
///     *ELSE DEST.qword[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpshrdv() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSHUFBITQMB DEST, SRC1, SRC2
/// (KL, VL) = (16,128), (32,256), (64, 512)
/// FOR i := 0 TO KL/8-1:
///                 //Qword
///     FOR j := 0 to 7:
///                 // Byte
///         IF k2[i*8+j] or *no writemask*:
///             m := SRC2.qword[i].byte[j] & 0x3F
///             k1[i*8+j] := SRC1.qword[i].bit[m]
///         ELSE:
///             k1[i*8+j] := 0
/// k1[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpshufbitqmb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSLLVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := ZeroExtend(SRC1[i+15:i] << SRC2[i+15:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSLLVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] << COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 32 THEN
/// DEST[127:96] := ZeroExtend(SRC1[127:96] << COUNT_3);
/// ELSE
/// DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSLLVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 7th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] << COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// IF COUNT_7 < 32 THEN
/// DEST[255:224] := ZeroExtend(SRC1[255:224] << COUNT_7);
/// ELSE
/// DEST[255:224] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSLLVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] << SRC2[31:0])
///                     ELSE DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] << SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSLLVQ (VEX.128 version)
/// COUNT_0 := SRC2[63 : 0];
/// COUNT_1 := SRC2[127 : 64];
/// IF COUNT_0 < 64THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] << COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
/// IF COUNT_1 < 64 THEN
/// DEST[127:64] := ZeroExtend(SRC1[127:64] << COUNT_1);
/// ELSE
/// DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSLLVQ (VEX.256 version)
/// COUNT_0 := SRC2[63 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[255 : 192];
/// IF COUNT_0 < 64THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] << COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 64 THEN
/// DEST[255:192] := ZeroExtend(SRC1[255:192] << COUNT_3);
/// ELSE
/// DEST[255:192] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSLLVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] << SRC2[63:0])
///                     ELSE DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] << SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsllvd() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSLLVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := ZeroExtend(SRC1[i+15:i] << SRC2[i+15:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSLLVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] << COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 32 THEN
/// DEST[127:96] := ZeroExtend(SRC1[127:96] << COUNT_3);
/// ELSE
/// DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSLLVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 7th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] << COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// IF COUNT_7 < 32 THEN
/// DEST[255:224] := ZeroExtend(SRC1[255:224] << COUNT_7);
/// ELSE
/// DEST[255:224] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSLLVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] << SRC2[31:0])
///                     ELSE DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] << SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSLLVQ (VEX.128 version)
/// COUNT_0 := SRC2[63 : 0];
/// COUNT_1 := SRC2[127 : 64];
/// IF COUNT_0 < 64THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] << COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
/// IF COUNT_1 < 64 THEN
/// DEST[127:64] := ZeroExtend(SRC1[127:64] << COUNT_1);
/// ELSE
/// DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSLLVQ (VEX.256 version)
/// COUNT_0 := SRC2[63 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[255 : 192];
/// IF COUNT_0 < 64THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] << COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 64 THEN
/// DEST[255:192] := ZeroExtend(SRC1[255:192] << COUNT_3);
/// ELSE
/// DEST[255:192] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSLLVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] << SRC2[63:0])
///                     ELSE DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] << SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsllvq() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSLLVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := ZeroExtend(SRC1[i+15:i] << SRC2[i+15:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSLLVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] << COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 32 THEN
/// DEST[127:96] := ZeroExtend(SRC1[127:96] << COUNT_3);
/// ELSE
/// DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSLLVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 7th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] << COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// IF COUNT_7 < 32 THEN
/// DEST[255:224] := ZeroExtend(SRC1[255:224] << COUNT_7);
/// ELSE
/// DEST[255:224] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSLLVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] << SRC2[31:0])
///                     ELSE DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] << SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSLLVQ (VEX.128 version)
/// COUNT_0 := SRC2[63 : 0];
/// COUNT_1 := SRC2[127 : 64];
/// IF COUNT_0 < 64THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] << COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
/// IF COUNT_1 < 64 THEN
/// DEST[127:64] := ZeroExtend(SRC1[127:64] << COUNT_1);
/// ELSE
/// DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSLLVQ (VEX.256 version)
/// COUNT_0 := SRC2[63 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[255 : 192];
/// IF COUNT_0 < 64THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] << COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 64 THEN
/// DEST[255:192] := ZeroExtend(SRC1[255:192] << COUNT_3);
/// ELSE
/// DEST[255:192] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSLLVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] << SRC2[63:0])
///                     ELSE DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] << SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsllvw() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSRAVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///             THEN
///                     COUNT := SRC2[i+3:i]
///                     IF COUNT < 16
///                         THEN DEST[i+15:i] := SignExtend(SRC1[i+15:i] >> COUNT)
///                         ELSE
///                             FOR k := 0 TO 15
///                                 DEST[i+k] := SRC1[i+15]
///                             ENDFOR;
///                     FI
///             ELSE
///                     IF *merging-masking*
///                                         ; merging-masking
///                         THEN *DEST[i+15:i] remains unchanged*
///                 ELSE ; zeroing-masking
///                             DEST[i+15:i] := 0
///                     FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRAVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// DEST[31:0] := SignExtend(SRC1[31:0] >> COUNT_0);
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// DEST[127:96] := SignExtend(SRC1[127:96] >> COUNT_3);
/// DEST[MAXVL-1:128] := 0;
/// VPSRAVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 8th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// DEST[31:0] := SignExtend(SRC1[31:0] >> COUNT_0);
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// DEST[255:224] := SignExtend(SRC1[255:224] >> COUNT_7);
/// DEST[MAXVL-1:256] := 0;
/// VPSRAVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                         THEN
///                             COUNT := SRC2[4:0]
///                             IF COUNT < 32
///                                 THEN DEST[i+31:i] := SignExtend(SRC1[i+31:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 31
///                                         DEST[i+k] := SRC1[i+31]
///                                     ENDFOR;
///                             FI
///                         ELSE
///                             COUNT := SRC2[i+4:i]
///                             IF COUNT < 32
///                                 THEN DEST[i+31:i] := SignExtend(SRC1[i+31:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 31
///                                         DEST[i+k] := SRC1[i+31]
///                                     ENDFOR;
///                             FI
///                 FI;
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[31:0] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRAVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                         THEN
///                             COUNT := SRC2[5:0]
///                             IF COUNT < 64
///                                 THEN DEST[i+63S::i=i] g n
///                                         Extend(SRC1[i+63:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 63
///                                         DEST[i+k] := SRC1[i+63]
///                                     ENDFOR;
///                             FI
///                         ELSE
///                             COUNT := SRC2[i+5:i]
///                             IF COUNT < 64
///                                 THEN DEST[i+63S::i=i] g n
///                                         Extend(SRC1[i+63:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 63
///                                         DEST[i+k] := SRC1[i+63]
///                                     ENDFOR;
///                             FI
///                     FI;
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                     THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[63:0] := 0
///                     FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsravd() -> &'static [IrStatement] {
    let assignment = assign(b::sar(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSRAVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///             THEN
///                     COUNT := SRC2[i+3:i]
///                     IF COUNT < 16
///                         THEN DEST[i+15:i] := SignExtend(SRC1[i+15:i] >> COUNT)
///                         ELSE
///                             FOR k := 0 TO 15
///                                 DEST[i+k] := SRC1[i+15]
///                             ENDFOR;
///                     FI
///             ELSE
///                     IF *merging-masking*
///                                         ; merging-masking
///                         THEN *DEST[i+15:i] remains unchanged*
///                 ELSE ; zeroing-masking
///                             DEST[i+15:i] := 0
///                     FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRAVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// DEST[31:0] := SignExtend(SRC1[31:0] >> COUNT_0);
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// DEST[127:96] := SignExtend(SRC1[127:96] >> COUNT_3);
/// DEST[MAXVL-1:128] := 0;
/// VPSRAVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 8th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// DEST[31:0] := SignExtend(SRC1[31:0] >> COUNT_0);
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// DEST[255:224] := SignExtend(SRC1[255:224] >> COUNT_7);
/// DEST[MAXVL-1:256] := 0;
/// VPSRAVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                         THEN
///                             COUNT := SRC2[4:0]
///                             IF COUNT < 32
///                                 THEN DEST[i+31:i] := SignExtend(SRC1[i+31:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 31
///                                         DEST[i+k] := SRC1[i+31]
///                                     ENDFOR;
///                             FI
///                         ELSE
///                             COUNT := SRC2[i+4:i]
///                             IF COUNT < 32
///                                 THEN DEST[i+31:i] := SignExtend(SRC1[i+31:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 31
///                                         DEST[i+k] := SRC1[i+31]
///                                     ENDFOR;
///                             FI
///                 FI;
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[31:0] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRAVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                         THEN
///                             COUNT := SRC2[5:0]
///                             IF COUNT < 64
///                                 THEN DEST[i+63S::i=i] g n
///                                         Extend(SRC1[i+63:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 63
///                                         DEST[i+k] := SRC1[i+63]
///                                     ENDFOR;
///                             FI
///                         ELSE
///                             COUNT := SRC2[i+5:i]
///                             IF COUNT < 64
///                                 THEN DEST[i+63S::i=i] g n
///                                         Extend(SRC1[i+63:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 63
///                                         DEST[i+k] := SRC1[i+63]
///                                     ENDFOR;
///                             FI
///                     FI;
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                     THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[63:0] := 0
///                     FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsravq() -> &'static [IrStatement] {
    let assignment = assign(b::sar(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSRAVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///             THEN
///                     COUNT := SRC2[i+3:i]
///                     IF COUNT < 16
///                         THEN DEST[i+15:i] := SignExtend(SRC1[i+15:i] >> COUNT)
///                         ELSE
///                             FOR k := 0 TO 15
///                                 DEST[i+k] := SRC1[i+15]
///                             ENDFOR;
///                     FI
///             ELSE
///                     IF *merging-masking*
///                                         ; merging-masking
///                         THEN *DEST[i+15:i] remains unchanged*
///                 ELSE ; zeroing-masking
///                             DEST[i+15:i] := 0
///                     FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRAVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// DEST[31:0] := SignExtend(SRC1[31:0] >> COUNT_0);
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// DEST[127:96] := SignExtend(SRC1[127:96] >> COUNT_3);
/// DEST[MAXVL-1:128] := 0;
/// VPSRAVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 8th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// DEST[31:0] := SignExtend(SRC1[31:0] >> COUNT_0);
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// DEST[255:224] := SignExtend(SRC1[255:224] >> COUNT_7);
/// DEST[MAXVL-1:256] := 0;
/// VPSRAVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                         THEN
///                             COUNT := SRC2[4:0]
///                             IF COUNT < 32
///                                 THEN DEST[i+31:i] := SignExtend(SRC1[i+31:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 31
///                                         DEST[i+k] := SRC1[i+31]
///                                     ENDFOR;
///                             FI
///                         ELSE
///                             COUNT := SRC2[i+4:i]
///                             IF COUNT < 32
///                                 THEN DEST[i+31:i] := SignExtend(SRC1[i+31:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 31
///                                         DEST[i+k] := SRC1[i+31]
///                                     ENDFOR;
///                             FI
///                 FI;
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[31:0] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRAVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                         THEN
///                             COUNT := SRC2[5:0]
///                             IF COUNT < 64
///                                 THEN DEST[i+63S::i=i] g n
///                                         Extend(SRC1[i+63:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 63
///                                         DEST[i+k] := SRC1[i+63]
///                                     ENDFOR;
///                             FI
///                         ELSE
///                             COUNT := SRC2[i+5:i]
///                             IF COUNT < 64
///                                 THEN DEST[i+63S::i=i] g n
///                                         Extend(SRC1[i+63:i] >> COUNT)
///                                 ELSE
///                                     FOR k := 0 TO 63
///                                         DEST[i+k] := SRC1[i+63]
///                                     ENDFOR;
///                             FI
///                     FI;
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                     THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[63:0] := 0
///                     FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsravw() -> &'static [IrStatement] {
    let assignment = assign(b::sar(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSRLVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := ZeroExtend(SRC1[i+15:i] >> SRC2[i+15:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRLVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// IF COUNT_0 < 32 THEN
///     DEST[31:0] := ZeroExtend(SRC1[31:0] >> COUNT_0);
/// ELSE
///     DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 32 THEN
///     DEST[127:96] := ZeroExtend(SRC1[127:96] >> COUNT_3);
/// ELSE
///     DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSRLVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 7th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] >> COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// IF COUNT_7 < 32 THEN
///     DEST[255:224] := ZeroExtend(SRC1[255:224] >> COUNT_7);
/// ELSE
///     DEST[255:224] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSRLVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] >> SRC2[31:0])
///                     ELSE DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] >> SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRLVQ (VEX.128 version)
/// COUNT_0 := SRC2[63 : 0];
/// COUNT_1 := SRC2[127 : 64];
/// IF COUNT_0 < 64 THEN
///     DEST[63:0] := ZeroExtend(SRC1[63:0] >> COUNT_0);
/// ELSE
///     DEST[63:0] := 0;
/// IF COUNT_1 < 64 THEN
///     DEST[127:64] := ZeroExtend(SRC1[127:64] >> COUNT_1);
/// ELSE
///     DEST[127:64] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSRLVQ (VEX.256 version)
/// COUNT_0 := SRC2[63 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[255 : 192];
/// IF COUNT_0 < 64 THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] >> COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 64 THEN
///     DEST[255:192] := ZeroExtend(SRC1[255:192] >> COUNT_3);
/// ELSE
///     DEST[255:192] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSRLVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] >> SRC2[63:0])
///                     ELSE DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] >> SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsrlvd() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSRLVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := ZeroExtend(SRC1[i+15:i] >> SRC2[i+15:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRLVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// IF COUNT_0 < 32 THEN
///     DEST[31:0] := ZeroExtend(SRC1[31:0] >> COUNT_0);
/// ELSE
///     DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 32 THEN
///     DEST[127:96] := ZeroExtend(SRC1[127:96] >> COUNT_3);
/// ELSE
///     DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSRLVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 7th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] >> COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// IF COUNT_7 < 32 THEN
///     DEST[255:224] := ZeroExtend(SRC1[255:224] >> COUNT_7);
/// ELSE
///     DEST[255:224] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSRLVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] >> SRC2[31:0])
///                     ELSE DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] >> SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRLVQ (VEX.128 version)
/// COUNT_0 := SRC2[63 : 0];
/// COUNT_1 := SRC2[127 : 64];
/// IF COUNT_0 < 64 THEN
///     DEST[63:0] := ZeroExtend(SRC1[63:0] >> COUNT_0);
/// ELSE
///     DEST[63:0] := 0;
/// IF COUNT_1 < 64 THEN
///     DEST[127:64] := ZeroExtend(SRC1[127:64] >> COUNT_1);
/// ELSE
///     DEST[127:64] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSRLVQ (VEX.256 version)
/// COUNT_0 := SRC2[63 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[255 : 192];
/// IF COUNT_0 < 64 THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] >> COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 64 THEN
///     DEST[255:192] := ZeroExtend(SRC1[255:192] >> COUNT_3);
/// ELSE
///     DEST[255:192] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSRLVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] >> SRC2[63:0])
///                     ELSE DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] >> SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsrlvq() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSRLVW (EVEX encoded version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := ZeroExtend(SRC1[i+15:i] >> SRC2[i+15:i])
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRLVD (VEX.128 version)
/// COUNT_0 := SRC2[31 : 0]
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[127 : 96];
/// IF COUNT_0 < 32 THEN
///     DEST[31:0] := ZeroExtend(SRC1[31:0] >> COUNT_0);
/// ELSE
///     DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 32 THEN
///     DEST[127:96] := ZeroExtend(SRC1[127:96] >> COUNT_3);
/// ELSE
///     DEST[127:96] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSRLVD (VEX.256 version)
/// COUNT_0 := SRC2[31 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 7th dwords of SRC2*)
/// COUNT_7 := SRC2[255 : 224];
/// IF COUNT_0 < 32 THEN
/// DEST[31:0] := ZeroExtend(SRC1[31:0] >> COUNT_0);
/// ELSE
/// DEST[31:0] := 0;
///     (* Repeat shift operation for 2nd through 7th dwords *)
/// IF COUNT_7 < 32 THEN
///     DEST[255:224] := ZeroExtend(SRC1[255:224] >> COUNT_7);
/// ELSE
///     DEST[255:224] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSRLVD (EVEX encoded version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] >> SRC2[31:0])
///                     ELSE DEST[i+31:i] := ZeroExtend(SRC1[i+31:i] >> SRC2[i+31:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSRLVQ (VEX.128 version)
/// COUNT_0 := SRC2[63 : 0];
/// COUNT_1 := SRC2[127 : 64];
/// IF COUNT_0 < 64 THEN
///     DEST[63:0] := ZeroExtend(SRC1[63:0] >> COUNT_0);
/// ELSE
///     DEST[63:0] := 0;
/// IF COUNT_1 < 64 THEN
///     DEST[127:64] := ZeroExtend(SRC1[127:64] >> COUNT_1);
/// ELSE
///     DEST[127:64] := 0;
/// DEST[MAXVL-1:128] := 0;
/// VPSRLVQ (VEX.256 version)
/// COUNT_0 := SRC2[63 : 0];
///     (* Repeat Each COUNT_i for the 2nd through 4th dwords of SRC2*)
/// COUNT_3 := SRC2[255 : 192];
/// IF COUNT_0 < 64 THEN
/// DEST[63:0] := ZeroExtend(SRC1[63:0] >> COUNT_0);
/// ELSE
/// DEST[63:0] := 0;
///     (* Repeat shift operation for 2nd through 4th dwords *)
/// IF COUNT_3 < 64 THEN
///     DEST[255:192] := ZeroExtend(SRC1[255:192] >> COUNT_3);
/// ELSE
///     DEST[255:192] := 0;
/// DEST[MAXVL-1:256] := 0;
/// VPSRLVQ (EVEX encoded version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] >> SRC2[63:0])
///                     ELSE DEST[i+63:i] := ZeroExtend(SRC1[i+63:i] >> SRC2[i+63:i])
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vpsrlvw() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPTERNLOGD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///             THEN
///                     FOR k := 0 TO 31
///                         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                             THEN DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ k ]]
///                             ELSE DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ i+k ]]
///                         FI;
///                                 ; table lookup of immediate bellow;
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                     THEN *DEST[31+i:i] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[31+i:i] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPTERNLOGQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                     FOR k := 0 TO 63
///                         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                             THEN DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ k ]]
///                             ELSE DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ i+k ]]
///                         FI;
///                                     ; table lookup of immediate bellow;
///         ELSE
///                     IF *merging-masking*
///                                         ; merging-masking
///                         THEN *DEST[63+i:i] remains unchanged*
///                 ELSE ; zeroing-masking
///                             DEST[63+i:i] := 0
///                     FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpternlogd() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPTERNLOGD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///             THEN
///                     FOR k := 0 TO 31
///                         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                             THEN DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ k ]]
///                             ELSE DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ i+k ]]
///                         FI;
///                                 ; table lookup of immediate bellow;
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                     THEN *DEST[31+i:i] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[31+i:i] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPTERNLOGQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                     FOR k := 0 TO 63
///                         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                             THEN DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ k ]]
///                             ELSE DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ i+k ]]
///                         FI;
///                                     ; table lookup of immediate bellow;
///         ELSE
///                     IF *merging-masking*
///                                         ; merging-masking
///                         THEN *DEST[63+i:i] remains unchanged*
///                 ELSE ; zeroing-masking
///                             DEST[63+i:i] := 0
///                     FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vpternlogq() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPTESTMB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] != 0)? 1 : 0;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] != 0)? 1 : 0;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] != 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] != 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] != 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] != 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vptestmb() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VPTESTMB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] != 0)? 1 : 0;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] != 0)? 1 : 0;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] != 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] != 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] != 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] != 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vptestmd() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VPTESTMB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] != 0)? 1 : 0;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] != 0)? 1 : 0;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] != 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] != 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] != 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] != 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vptestmq() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VPTESTMB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] != 0)? 1 : 0;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMW (EVEX encoded versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] != 0)? 1 : 0;
///         ELSE DEST[j] = 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMD (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] != 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] != 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTMQ (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] != 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] != 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vptestmw() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VPTESTNMB
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j*8
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMW
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j*16
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     IF MaskBit(j) OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
/// 
///             THEN DEST[i+31:i] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] == 0)? 1 : 0
///                 ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j*64
///     IF MaskBit(j) OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] == 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] == 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vptestnmb() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VPTESTNMB
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j*8
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMW
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j*16
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     IF MaskBit(j) OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
/// 
///             THEN DEST[i+31:i] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] == 0)? 1 : 0
///                 ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j*64
///     IF MaskBit(j) OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] == 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] == 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vptestnmd() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VPTESTNMB
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j*8
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMW
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j*16
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     IF MaskBit(j) OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
/// 
///             THEN DEST[i+31:i] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] == 0)? 1 : 0
///                 ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j*64
///     IF MaskBit(j) OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] == 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] == 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vptestnmq() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// VPTESTNMB
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j*8
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMW
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j*16
///     IF MaskBit(j) OR *no writemask*
///         THEN
/// 
///             DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMD
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j*32
///     IF MaskBit(j) OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
/// 
///             THEN DEST[i+31:i] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] == 0)? 1 : 0
///                 ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] == 0)? 1 : 0
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// VPTESTNMQ
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j*64
///     IF MaskBit(j) OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] == 0)? 1 : 0;
///                 ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] == 0)? 1 : 0;
///             FI;
///         ELSE DEST[j] := 0; zeroing masking only
///     FI
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vptestnmw() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// RangeDP(SRC1[63:0], SRC2[63:0], CmpOpCtl[1:0], SignSelCtl[1:0])
/// {
///     // Check if SNAN and report IE, see also Table 5-13
///     IF (SRC1 = SNAN) THEN RETURN (QNAN(SRC1), set IE);
///     IF (SRC2 = SNAN) THEN RETURN (QNAN(SRC2), set IE);
///     Src1.exp := SRC1[62:52];
///     Src1.fraction := SRC1[51:0];
///     IF ((Src1.exp = 0 ) and (Src1.fraction != 0)) THEN// Src1 is a denormal number
///         IF DAZ THEN Src1.fraction := 0;
///         ELSE IF (SRC2 <> QNAN) Set DE; FI;
///     Src2.exp := SRC2[62:52];
///     Src2.fraction := SRC2[51:0];
///     IF ((Src2.exp = 0) and (Src2.fraction !=0 )) THEN// Src2 is a denormal number
///             IF DAZ THEN Src2.fraction := 0;
///             ELSE IF (SRC1 <> QNAN) Set DE; FI;
///     FI;
///     IF (SRC2 = QNAN) THEN{TMP[63:0] := SRC1[63:0]}
///     ELSE IF(SRC1 = QNAN) THEN{TMP[63:0] := SRC2[63:0]}
///     ELSE IF (Both SRC1, SRC2 are magnitude-0 and opposite-signed) TMP[63:0] := from Table 5-14
///     ELSE IF (Both SRC1, SRC2 are magnitude-equal and opposite-signed and CmpOpCtl[1:0] > 01) TMP[63:0] := from Table 5-15
///     ELSE
///             Case(CmpOpCtl[1:0])
///             00: TMP[63:0] := (SRC1[63:0]  S≤RC2[63:0]) ? SRC1[63:0] : SRC2[63:0];
///             01: TMP[63:0] := (SRC1[63:0]  S≤RC2[63:0]) ? SRC2[63:0] : SRC1[63:0];
///             10: TMP[63:0] := (ABS(SRC1[63:0])  ≤ABS(SRC2[63:0])) ? SRC1[63:0] : SRC2[63:0];
///             11: TMP[63:0] := (ABS(SRC1[63:0])  ≤ABS(SRC2[63:0])) ? SRC2[63:0] : SRC1[63:0];
///             ESAC;
///     FI;
///     Case(SignSelCtl[1:0])
///     00: dest := (SRC1[63] << 63) OR (TMP[62:0]);// Preserve Src1 sign bit
///     01: dest := TMP[63:0];// Preserve sign of compare result
///     10: dest := (0 << 63) OR (TMP[62:0]);// Zero out sign bit
///     11: dest := (1 << 63) OR (TMP[62:0]);// Set the sign bit
///     ESAC;
///     RETURN dest[63:0];
/// }
/// CmpOpCtl[1:0]= imm8[1:0];
/// SignSelCtl[1:0]=imm8[3:2];
/// VRANGEPD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                     IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                         THEN DEST[i+63:i] := RangeDP (SRC1[i+63:i], SRC2[63:0], CmpOpCtl[1:0], SignSelCtl[1:0]);
///                         ELSE DEST[i+63:i] := RangeDP (SRC1[i+63:i], SRC2[i+63:i], CmpOpCtl[1:0], SignSelCtl[1:0]);
///                     FI;
///     ELSE
///             IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///         ELSE ; zeroing-masking
///                         DEST[i+63:i] = 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// The following example describes a common usage of this instruction for checking that the input operand is
/// bounded between ±1023.
/// VRANGEPD zmm_dst, zmm_src, zmm_1023, 02h;
/// Where:
///                 zmm_dst is the destination operand.
///                 zmm_src is the input operand to compare against ±1023 (this is SRC1).
///                 zmm_1023 is the reference operand, contains the value of 1023 (and this is SRC2).
///                 IMM=02(imm8[1:0]='10) selects the Min Absolute value operation with selection of SRC1.sign.
/// In case |zmm_src| < 1023 (i.e., SRC1 is smaller than 1023 in magnitude), then its value will be written into
/// zmm_dst. Otherwise, the value stored in zmm_dst will get the value of 1023 (received on zmm_1023, which is
/// SRC2).
/// However, the sign control (imm8[3:2]='00) instructs to sleect the sign of SRC1 received from zmm_src. So, even
/// in the case of |zmm_src|  ≥1023, the selected sign of SRC1 is kept.
/// Thus, if zmm_src < -1023, the resulto f VRANGEPD will be the minimal value of -1023 while if zmm_src > +1023,
/// the result of VRANGE will be the maximal value of +1023.
/// ```
#[box_to_static_reference]
pub(super) fn vrangepd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RangeSP(SRC1[31:0], SRC2[31:0], CmpOpCtl[1:0], SignSelCtl[1:0])
/// {
///     // Check if SNAN and report IE, see also Table 5-13
///     IF (SRC1=SNAN) THEN RETURN (QNAN(SRC1), set IE);
///     IF (SRC2=SNAN) THEN RETURN (QNAN(SRC2), set IE);
///     Src1.exp := SRC1[30:23];
///     Src1.fraction := SRC1[22:0];
///     IF ((Src1.exp = 0 ) and (Src1.fraction != 0 )) THEN// Src1 is a denormal number
///         IF DAZ THEN Src1.fraction := 0;
///         ELSE IF (SRC2 <> QNAN) Set DE; FI;
///     FI;
///     Src2.exp := SRC2[30:23];
///     Src2.fraction := SRC2[22:0];
///     IF ((Src2.exp = 0 ) and (Src2.fraction != 0 )) THEN// Src2 is a denormal number
///         IF DAZ THEN Src2.fraction := 0;
///         ELSE IF (SRC1 <> QNAN) Set DE; FI;
///     FI;
///     IF (SRC2 = QNAN) THEN{TMP[31:0] := SRC1[31:0]}
///     ELSE IF(SRC1 = QNAN) THEN{TMP[31:0] := SRC2[31:0]}
///     ELSE IF (Both SRC1, SRC2 are magnitude-0 and opposite-signed) TMP[31:0] := from Table 5-14
///     ELSE IF (Both SRC1, SRC2 are magnitude-equal and opposite-signed and CmpOpCtl[1:0] > 01) TMP[31:0] := from Table 5-15
///     ELSE
///         Case(CmpOpCtl[1:0])
///         00: TMP[31:0] := (SRC1[31:0]  S≤RC2[31:0]) ? SRC1[31:0] : SRC2[31:0];
///         01: TMP[31:0] := (SRC1[31:0]  S≤RC2[31:0]) ? SRC2[31:0] : SRC1[31:0];
///         10: TMP[31:0] := (ABS(SRC1[31:0])  ≤ABS(SRC2[31:0])) ? SRC1[31:0] : SRC2[31:0];
///         11: TMP[31:0] := (ABS(SRC1[31:0])  ≤ABS(SRC2[31:0])) ? SRC2[31:0] : SRC1[31:0];
///         ESAC;
///     FI;
///     Case(SignSelCtl[1:0])
///     00: dest := (SRC1[31] << 31) OR (TMP[30:0]);// Preserve Src1 sign bit
///     01: dest := TMP[31:0];// Preserve sign of compare result
///     10: dest := (0 << 31) OR (TMP[30:0]);// Zero out sign bit
///     11: dest := (1 << 31) OR (TMP[30:0]);// Set the sign bit
///     ESAC;
///     RETURN dest[31:0];
/// }
/// CmpOpCtl[1:0]= imm8[1:0];
/// SignSelCtl[1:0]=imm8[3:2];
/// VRANGEPS
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := RangeSP (SRC1[i+31:i], SRC2[31:0], CmpOpCtl[1:0], SignSelCtl[1:0]);
///                     ELSE DEST[i+31:i] := RangeSP (SRC1[i+31:i], SRC2[i+31:i], CmpOpCtl[1:0], SignSelCtl[1:0]);
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+31:i] = 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// The following example describes a common usage of this instruction for checking that the input operand is
/// bounded between ±150.
/// VRANGEPS zmm_dst, zmm_src, zmm_150, 02h;
/// Where:
/// zmm_dst is the destination operand.
/// zmm_src is the input operand to compare against ±150.
/// zmm_150 is the reference operand, contains the value of 150.
/// IMM=02(imm8[1:0]='10) selects the Min Absolute value operation with selection of src1.sign.
/// In case |zmm_src| < 150, then its value will be written into zmm_dst. Otherwise, the value stored in zmm_dst
/// will get the value of 150 (received on zmm_150).
/// However, the sign control (imm8[3:2]='00) instructs to sleect the sign of SRC1 received from zmm_src. So, even
/// in the case of |zmm_src|  ≥150, the selected sign of SRC1 is kept.
/// Thus, if zmm_src < -150, the result of VRANGEPS will be the minimal value of -150 while if zmm_src > +150,
/// the result of VRANGE will be the maximal value of +150.
/// ```
#[box_to_static_reference]
pub(super) fn vrangeps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RangeDP(SRC1[63:0], SRC2[63:0], CmpOpCtl[1:0], SignSelCtl[1:0])
/// {
///     // Check if SNAN and report IE, see also Table 5-13
///     IF (SRC1 = SNAN) THEN RETURN (QNAN(SRC1), set IE);
///     IF (SRC2 = SNAN) THEN RETURN (QNAN(SRC2), set IE);
///     Src1.exp := SRC1[62:52];
///     Src1.fraction := SRC1[51:0];
///     IF ((Src1.exp = 0 ) and (Src1.fraction != 0)) THEN// Src1 is a denormal number
///             IF DAZ THEN Src1.fraction := 0;
///             ELSE IF (SRC2 <> QNAN) Set DE; FI;
///     FI;
///     Src2.exp := SRC2[62:52];
///     Src2.fraction := SRC2[51:0];
///     IF ((Src2.exp = 0) and (Src2.fraction !=0 )) THEN// Src2 is a denormal number
///             IF DAZ THEN Src2.fraction := 0;
///             ELSE IF (SRC1 <> QNAN) Set DE; FI;
///     FI;
///     IF (SRC2 = QNAN) THEN{TMP[63:0] := SRC1[63:0]}
///     ELSE IF(SRC1 = QNAN) THEN{TMP[63:0] := SRC2[63:0]}
///     ELSE IF (Both SRC1, SRC2 are magnitude-0 and opposite-signed) TMP[63:0] := from Table 5-14
///     ELSE IF (Both SRC1, SRC2 are magnitude-equal and opposite-signed and CmpOpCtl[1:0] > 01) TMP[63:0] := from Table 5-15
///     ELSE
///             Case(CmpOpCtl[1:0])
///             00: TMP[63:0] := (SRC1[63:0]  S≤RC2[63:0]) ? SRC1[63:0] : SRC2[63:0];
///             01: TMP[63:0] := (SRC1[63:0]  S≤RC2[63:0]) ? SRC2[63:0] : SRC1[63:0];
///             10: TMP[63:0] := (ABS(SRC1[63:0])  ≤ABS(SRC2[63:0])) ? SRC1[63:0] : SRC2[63:0];
///             11: TMP[63:0] := (ABS(SRC1[63:0])  ≤ABS(SRC2[63:0])) ? SRC2[63:0] : SRC1[63:0];
///             ESAC;
///     FI;
///     Case(SignSelCtl[1:0])
///     00: dest := (SRC1[63] << 63) OR (TMP[62:0]);// Preserve Src1 sign bit
///     01: dest := TMP[63:0];// Preserve sign of compare result
///     10: dest := (0 << 63) OR (TMP[62:0]);// Zero out sign bit
///     11: dest := (1 << 63) OR (TMP[62:0]);// Set the sign bit
///     ESAC;
///     RETURN dest[63:0];
/// }
/// CmpOpCtl[1:0]= imm8[1:0];
/// SignSelCtl[1:0]=imm8[3:2];
/// VRANGESD
/// IF k1[0] OR *no writemask*
///             THEN DEST[63:0] := RangeDP (SRC1[63:0], SRC2[63:0], CmpOpCtl[1:0], SignSelCtl[1:0]);
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[63:0] = 0
///             FI;
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// The following example describes a common usage of this isntruction for checking that the input operand is bound-
/// ed between ±1023.
/// VRANGESD xmm_dst, xmm_src, xmm_1023, 02h;
/// Where:
/// xmm_dst is the destination operand.
/// xmm_src is the input operand to compare against ±1023.
/// xmm_1023 is the reference operand, contains the value of 1023.
/// IMM=02(imm8[1:0]='10) selects the Min Absolute value operation with selection of src1.sign.
/// In case |xmm_src| < 1023, then its value will be written into xmm_dst. Otherwise, the value stored in xmm_dst
/// will get the value of 1023 (received on xmm_1023).
/// However, the sign control (imm8[3:2]='00) instructs to sleect the sign of SRC1 received from xmm_src. So, even
/// in the case of |xmm_src|  ≥1023, the selected sign of SRC1 is kept.
/// Thus, if xmm_src < -1023, the result of VRANGEPD willb e the minimal value of -1023while if xmm_src > +1023,
/// the result of VRANGE will be the maximal value of +1023.
/// ```
#[box_to_static_reference]
pub(super) fn vrangesd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RangeSP(SRC1[31:0], SRC2[31:0], CmpOpCtl[1:0], SignSelCtl[1:0])
/// {
///     // Check if SNAN and report IE, see also Table 5-13
///     IF (SRC1=SNAN) THEN RETURN (QNAN(SRC1), set IE);
///     IF (SRC2=SNAN) THEN RETURN (QNAN(SRC2), set IE);
///     Src1.exp := SRC1[30:23];
///     Src1.fraction := SRC1[22:0];
///     IF ((Src1.exp = 0 ) and (Src1.fraction != 0 )) THEN// Src1 is a denormal number
///         IF DAZ THEN Src1.fraction := 0;
///         ELSE IF (SRC2 <> QNAN) Set DE; FI;
///     FI;
///     Src2.exp := SRC2[30:23];
///     Src2.fraction := SRC2[22:0];
///     IF ((Src2.exp = 0 ) and (Src2.fraction != 0 )) THEN// Src2 is a denormal number
///         IF DAZ THEN Src2.fraction := 0;
///         ELSE IF (SRC1 <> QNAN) Set DE; FI;
///     FI;
///     IF (SRC2 = QNAN) THEN{TMP[31:0] := SRC1[31:0]}
///     ELSE IF(SRC1 = QNAN) THEN{TMP[31:0] := SRC2[31:0]}
///     ELSE IF (Both SRC1, SRC2 are magnitude-0 and opposite-signed) TMP[31:0] := from Table 5-14
///     ELSE IF (Both SRC1, SRC2 are magnitude-equal and opposite-signed and CmpOpCtl[1:0] > 01) TMP[31:0] := from Table 5-15
///     ELSE
///         Case(CmpOpCtl[1:0])
///         00: TMP[31:0] := (SRC1[31:0]  S≤RC2[31:0]) ? SRC1[31:0] : SRC2[31:0];
///         01: TMP[31:0] := (SRC1[31:0]  S≤RC2[31:0]) ? SRC2[31:0] : SRC1[31:0];
///         10: TMP[31:0] := (ABS(SRC1[31:0])  ≤ABS(SRC2[31:0])) ? SRC1[31:0] : SRC2[31:0];
///         11: TMP[31:0] := (ABS(SRC1[31:0])  ≤ABS(SRC2[31:0])) ? SRC2[31:0] : SRC1[31:0];
///         ESAC;
///     FI;
///     Case(SignSelCtl[1:0])
///     00: dest := (SRC1[31] << 31) OR (TMP[30:0]);// Preserve Src1 sign bit
///     01: dest := TMP[31:0];// Preserve sign of compare result
///     10: dest := (0 << 31) OR (TMP[30:0]);// Zero out sign bit
///     11: dest := (1 << 31) OR (TMP[30:0]);// Set the sign bit
///     ESAC;
///     RETURN dest[31:0];
/// }
/// CmpOpCtl[1:0]= imm8[1:0];
/// SignSelCtl[1:0]=imm8[3:2];
/// VRANGESS
/// IF k1[0] OR *no writemask*
///             THEN DEST[31:0] := RangeSP (SRC1[31:0], SRC2[31:0], CmpOpCtl[1:0], SignSelCtl[1:0]);
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[31:0] = 0
///             FI;
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// The following example describes a common usage of this instruction for checking that the input operand is
/// bounded between ±150.
/// VRANGESS zmm_dst, zmm_src, zmm_150, 02h;
/// Where:
/// xmm_dst is the destination operand.
/// xmm_src is the input operand to compare against ±150.
/// xmm_150 is the reference operand, contains the value of 150.
/// IMM=02(imm8[1:0]='10) selects the Min Absolute value operation with selection of src1.sign.
/// In case |xmm_src| < 150, then its value will be written into zmm_dst. Otherwise, the value stored in xmm_dst
/// will get the value of 150 (received on zmm_150).
/// However, the sign control (imm8[3:2]='00) instructs to sleect the sign of SRC1 received from xmm_src. So, even
/// in the case of |xmm_src|  ≥150, the selected sign of SRC1 is kept.
/// Thus, if xmm_src < -150, the result of VRANGESS will be the minimal value of -150 while if xmm_src > +150,
/// the result of VRANGE will be the maximal value of +150.
/// ```
#[box_to_static_reference]
pub(super) fn vrangess() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRCP14PD ((EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC *is memory*)
///                     THEN DEST[i+63:i] := APPROXIMATE(1.0/SRC[63:0]);
///                     ELSE DEST[i+63:i] := APPROXIMATE(1.0/SRC[i+63:i]);
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrcp14pd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRCP14PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC *is memory*)
///                     THEN DEST[i+31:i] := APPROXIMATE(1.0/SRC[31:0]);
///                     ELSE DEST[i+31:i] := APPROXIMATE(1.0/SRC[i+31:i]);
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrcp14ps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRCP14SD (EVEX version)
/// IF k1[0] OR *no writemask*
///             THEN DEST[63:0] := APPROXIMATE(1.0/SRC2[63:0]);
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrcp14sd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRCP14SS (EVEX version)
/// IF k1[0] OR *no writemask*
///             THEN DEST[31:0] := APPROXIMATE(1.0/SRC2[31:0]);
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrcp14ss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRCPPH dest{k1}, src
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.fp16[0]
///         ELSE:
///             tsrc := src.fp16[i]
///         DEST.fp16[i] := APPROXIMATE(1.0 / tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[i] := 0
///     //else DEST.fp16[i] remains unchanged
/// ```
#[box_to_static_reference]
pub(super) fn vrcpph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRCPSH dest{k1}, src1, src2
/// IF k1[0] or *no writemask*:
///     DEST.fp16[0] := APPROXIMATE(1.0 / src2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// //else DEST.fp16[0] remains unchanged
/// DEST[127:16] := src1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrcpsh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ReduceArgumentDP(SRC[63:0], imm8[7:0])
/// {
///     // Check for NaN
///     IF (SRC [63:0] = NAN) THEN
///             RETURN (Convert SRC[63:0] to QNaN); FI;
///     M := imm8[7:4]; // Number of fraction bits of the normalized significand to be subtracted
///     RC := imm8[1:0];// Round Control for ROUND() operation
///     RC source := imm[2];
///     SPE := imm[3];// Suppress Precision Exception
///     TMP[63:0] := 2⁻M *{ROUND(2M*SRC[63:0], SPE, RC_source, RC)}; // ROUND() treats SRC and 2M as standard binary FP values
///     TMP[63:0] := SRC[63:0] - TMP[63:0]; // subtraction under the same RC,SPE controls
///     RETURN TMP[63:0]; // binary encoded FP with biased exponent and normalized significand
/// }
/// VREDUCEPD
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b == 1) AND (SRC *is memory*)
///                     THEN DEST[i+63:i] := ReduceArgumentDP(SRC[63:0], imm8[7:0]);
///                     ELSE DEST[i+63:i] := ReduceArgumentDP(SRC[i+63:i], imm8[7:0]);
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+63:i] = 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vreducepd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def reduce_fp16(src, imm8):
///     nan := (src.exp = 0x1F) and (src.fraction != 0)
///     if nan:
///         return QNAN(src)
///     m := imm8[7:4]
///     rc := imm8[1:0]
///     rc_source := imm8[2]
///     spe := imm[3] // suppress precision exception
///     tmp := 2^(-m) * ROUND(2^m * src, spe, rc_source, rc)
///     tmp := src - tmp // using same RC, SPE controls
///     return tmp
/// VREDUCEPH dest{k1}, src, imm8
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.fp16[0]
///         ELSE:
///             tsrc := src.fp16[i]
///         DEST.fp16[i] := reduce_fp16(tsrc, imm8)
///     ELSE IF *zeroing*:
///         DEST.fp16[i] := 0
///     //else DEST.fp16[i] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vreduceph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ReduceArgumentSP(SRC[31:0], imm8[7:0])
/// {
///     // Check for NaN
///     IF (SRC [31:0] = NAN) THEN
///             RETURN (Convert SRC[31:0] to QNaN); FI
///     M := imm8[7:4]; // Number of fraction bits of the normalized significand to be subtracted
///     RC := imm8[1:0];// Round Control for ROUND() operation
///     RC source := imm[2];
///     SPE := imm[3];// Suppress Precision Exception
///     TMP[31:0] := 2⁻M *{ROUND(2M*SRC[31:0], SPE, RC_source, RC)}; // ROUND() treats SRC and 2M as standard binary FP values
///     TMP[31:0] := SRC[31:0] - TMP[31:0]; // subtraction under the same RC,SPE controls
/// RETURN TMP[31:0]; // binary encoded FP with biased exponent and normalized significand
/// }
/// VREDUCEPS
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b == 1) AND (SRC *is memory*)
///                     THEN DEST[i+31:i] := ReduceArgumentSP(SRC[31:0], imm8[7:0]);
///                     ELSE DEST[i+31:i] := ReduceArgumentSP(SRC[i+31:i], imm8[7:0]);
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+31:i] = 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vreduceps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ReduceArgumentDP(SRC[63:0], imm8[7:0])
/// {
///     // Check for NaN
///     IF (SRC [63:0] = NAN) THEN
///             RETURN (Convert SRC[63:0] to QNaN); FI;
///     M := imm8[7:4]; // Number of fraction bits of the normalized significand to be subtracted
///     RC := imm8[1:0];// Round Control for ROUND() operation
///     RC source := imm[2];
///     SPE := imm[3];// Suppress Precision Exception
///     TMP[63:0] := 2⁻M *{ROUND(2M*SRC[63:0], SPE, RC_source, RC)}; // ROUND() treats SRC and 2M as standard binary FP values
///     TMP[63:0] := SRC[63:0] - TMP[63:0]; // subtraction under the same RC,SPE controls
///     RETURN TMP[63:0]; // binary encoded FP with biased exponent and normalized significand
/// }
/// VREDUCESD
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := ReduceArgumentDP(SRC2[63:0], imm8[7:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] = 0
///             FI;
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vreducesd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VREDUCESH dest{k1}, src, imm8
/// IF k1[0] or *no writemask*:
///     dest.fp16[0] := reduce_fp16(src2.fp16[0], imm8) // see VREDUCEPH
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
/// //else dest.fp16[0] remains unchanged
/// DEST[127:16] := src1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vreducesh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ReduceArgumentSP(SRC[31:0], imm8[7:0])
/// {
///     // Check for NaN
///     IF (SRC [31:0] = NAN) THEN
///             RETURN (Convert SRC[31:0] to QNaN); FI
///     M := imm8[7:4]; // Number of fraction bits of the normalized significand to be subtracted
///     RC := imm8[1:0];// Round Control for ROUND() operation
///     RC source := imm[2];
///     SPE := imm[3];// Suppress Precision Exception
///     TMP[31:0] := 2⁻M *{ROUND(2M*SRC[31:0], SPE, RC_source, RC)}; // ROUND() treats SRC and 2M as standard binary FP values
///     TMP[31:0] := SRC[31:0] - TMP[31:0]; // subtraction under the same RC,SPE controls
/// RETURN TMP[31:0]; // binary encoded FP with biased exponent and normalized significand
/// }
/// VREDUCESS
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := ReduceArgumentSP(SRC2[31:0], imm8[7:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] = 0
///             FI;
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vreducess() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RoundToIntegerDP(SRC[63:0], imm8[7:0]) {
///     if (imm8[2] = 1)
///         rounding_direction := MXCSR:RC
///                                 ; get round control from MXCSR
///     else
///         rounding_direction := imm8[1:0]
///                                 ; get round control from imm8[1:0]
///     FI
///     M := imm8[7:4]
///                         ; get the scaling factor
///     case (rounding_direction)
///     00: TMP[63:0] := round_to_nearest_even_integer(2M*SRC[63:0])
///     01: TMP[63:0] := round_to_equal_or_smaller_integer(2M*SRC[63:0])
///     10: TMP[63:0] := round_to_equal_or_larger_integer(2M*SRC[63:0])
///     11: TMP[63:0] := round_to_nearest_smallest_magnitude_integer(2M*SRC[63:0])
///     ESAC
///     Dest[63:0] := 2⁻M* TMP[63:0]
///                             ; scale down back to 2
///     if (imm8[3] = 0) Then; check SPE
///         if (SRC[63:0] != Dest[63:0]) Then; check precision lost
///                 set_precision()
///                             ; set #PE
///         FI;
///     FI;
///     return(Dest[63:0])
/// }
/// VRNDSCALEPD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF *src is a memory operand*
///     THEN TMP_SRC := BROADCAST64(SRC, VL, k1)
///     ELSE TMP_SRC := SRC
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///             THEN DEST[i+63:i] := RoundToIntegerDP((TMP_SRC[i+63:i], imm8[7:0])
///     ELSE
///             IF *merging-masking*
///                                 ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrndscalepd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def round_fp16_to_integer(src, imm8):
///     if imm8[2] = 1:
///         rounding_direction := MXCSR.RC
///     else:
///         rounding_direction := imm8[1:0]
///     m := imm8[7:4] // scaling factor
///     tsrc1 := 2^m * src
///     if rounding_direction = 0b00:
///         tmp := round_to_nearest_even_integer(trc1)
///     else if rounding_direction = 0b01:
///         tmp := round_to_equal_or_smaller_integer(trc1)
///     else if rounding_direction = 0b10:
///         tmp := round_to_equal_or_larger_integer(trc1)
///     else if rounding_direction = 0b11:
///         tmp := round_to_smallest_magnitude_integer(trc1)
///     dst := 2^(-m) * tmp
///     if imm8[3]==0: // check SPE
///         if src != dst:
///             MXCSR.PE := 1
///     return dst
/// VRNDSCALEPH dest{k1}, src, imm8
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.fp16[0]
///         ELSE:
///             tsrc := src.fp16[i]
///         DEST.fp16[i] := round_fp16_to_integer(tsrc, imm8)
///     ELSE IF *zeroing*:
///         DEST.fp16[i] := 0
///     //else DEST.fp16[i] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrndscaleph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RoundToIntegerSP(SRC[31:0], imm8[7:0]) {
///     if (imm8[2] = 1)
///             rounding_direction := MXCSR:RC
///                                         ; get round control from MXCSR
///     else
///             rounding_direction := imm8[1:0]
///                                         ; get round control from imm8[1:0]
///     FI
///     M := imm8[7:4]
///                         ; get the scaling factor
///     case (rounding_direction)
///     00: TMP[31:0] := round_to_nearest_even_integer(2M*SRC[31:0])
///     01: TMP[31:0] := round_to_equal_or_smaller_integer(2M*SRC[31:0])
///     10: TMP[31:0] := round_to_equal_or_larger_integer(2M*SRC[31:0])
///     11: TMP[31:0] := round_to_nearest_smallest_magnitude_integer(2M*SRC[31:0])
///     ESAC;
///     Dest[31:0] := 2⁻M* TMP[31:0]
///                                     ; scale down back to 2
///     if (imm8[3] = 0) Then
///                             ; check SPE
///             if (SRC[31:0] != Dest[31:0]) Then; check precision lost
///                 set_precision()
///                                 ; set #PE
///             FI;
///     FI;
///     return(Dest[31:0])
/// }
/// VRNDSCALEPS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF *src is a memory operand*
///     THEN TMP_SRC := BROADCAST32(SRC, VL, k1)
///     ELSE TMP_SRC := SRC
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := RoundToIntegerSP(TMP_SRC[i+31:i]), imm8[7:0])
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrndscaleps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RoundToIntegerDP(SRC[63:0], imm8[7:0]) {
///     if (imm8[2] = 1)
///             rounding_direction := MXCSR:RC
///                                 ; get round control from MXCSR
///     else
///             rounding_direction := imm8[1:0]
///                                 ; get round control from imm8[1:0]
///     FI
///     M := imm8[7:4]
///                         ; get the scaling factor
///     case (rounding_direction)
///     00: TMP[63:0] := round_to_nearest_even_integer(2M*SRC[63:0])
///     01: TMP[63:0] := round_to_equal_or_smaller_integer(2M*SRC[63:0])
///     10: TMP[63:0] := round_to_equal_or_larger_integer(2M*SRC[63:0])
///     11: TMP[63:0] := round_to_nearest_smallest_magnitude_integer(2M*SRC[63:0])
///     ESAC
///     Dest[63:0] := 2⁻M* TMP[63:0]
///                             ; scale down back to 2
///     if (imm8[3] = 0) Then; check SPE
///             if (SRC[63:0] != Dest[63:0]) Then; check precision lost
///                 set_precision()
///                             ; set #PE
///             FI;
///     FI;
///     return(Dest[63:0])
/// }
/// VRNDSCALESD (EVEX encoded version)
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := RoundToIntegerDP(SRC2[63:0], Zero_upper_imm[7:0])
///     ELSE
///             IF *merging-masking*
///                                 ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrndscalesd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRNDSCALESH dest{k1}, src1, src2, imm8
/// IF k1[0] or *no writemask*:
///     DEST.fp16[0] := round_fp16_to_integer(src2.fp16[0], imm8) // see VRNDSCALEPH
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// //else DEST.fp16[0] remains unchanged
/// DEST[127:16] = src1[127:16]
/// ```
#[box_to_static_reference]
pub(super) fn vrndscalesh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RoundToIntegerSP(SRC[31:0], imm8[7:0]) {
///     if (imm8[2] = 1)
///             rounding_direction := MXCSR:RC
///                                         ; get round control from MXCSR
///     else
///             rounding_direction := imm8[1:0]
///                                         ; get round control from imm8[1:0]
///     FI
///     M := imm8[7:4]
///                         ; get the scaling factor
///     case (rounding_direction)
///     00: TMP[31:0] := round_to_nearest_even_integer(2M*SRC[31:0])
///     01: TMP[31:0] := round_to_equal_or_smaller_integer(2M*SRC[31:0])
///     10: TMP[31:0] := round_to_equal_or_larger_integer(2M*SRC[31:0])
///     11: TMP[31:0] := round_to_nearest_smallest_magnitude_integer(2M*SRC[31:0])
///     ESAC;
///     Dest[31:0] := 2⁻M* TMP[31:0]
///                                     ; scale down back to 2
///     if (imm8[3] = 0) Then
///                             ; check SPE
///             if (SRC[31:0] != Dest[31:0]) Then; check precision lost
///                 set_precision()
///                                 ; set #PE
///             FI;
///     FI;
///     return(Dest[31:0])
/// }
/// VRNDSCALESS (EVEX encoded version)
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := RoundToIntegerSP(SRC2[31:0], Zero_upper_imm[7:0])
///     ELSE
///             IF *merging-masking*
///                                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrndscaless() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRSQRT14PD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC *is memory*)
///                     THEN DEST[i+63:i] := APPROXIMATE(1.0/ SQRT(SRC[63:0]));
///                     ELSE DEST[i+63:i] := APPROXIMATE(1.0/ SQRT(SRC[i+63:i]));
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrsqrt14pd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRSQRT14PS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC *is memory*)
///                     THEN DEST[i+31:i] := APPROXIMATE(1.0/ SQRT(SRC[31:0]));
///                     ELSE DEST[i+31:i] := APPROXIMATE(1.0/ SQRT(SRC[i+31:i]));
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[i+31:i] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrsqrt14ps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRSQRT14SD (EVEX version)
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := APPROXIMATE(1.0/ SQRT(SRC2[63:0]))
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrsqrt14sd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRSQRT14SS (EVEX version)
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := APPROXIMATE(1.0/ SQRT(SRC2[31:0]))
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrsqrt14ss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRSQRTPH dest{k1}, src
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.fp16[0]
///         ELSE:
///             tsrc := src.fp16[i]
///         DEST.fp16[i] := APPROXIMATE(1.0 / SQRT(tsrc) )
///     ELSE IF *zeroing*:
///         DEST.fp16[i] := 0
///     //else DEST.fp16[i] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrsqrtph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VRSQRTSH dest{k1}, src1, src2
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF k1[0] or *no writemask*:
///     DEST.fp16[0] := APPROXIMATE(1.0 / SQRT(src2.fp16[0]))
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// //else DEST.fp16[0] remains unchanged
/// DEST[127:16] := src1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vrsqrtsh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// SCALE(SRC1, SRC2)
/// {
/// TMP_SRC2 := SRC2
/// TMP_SRC1 := SRC1
/// IF (SRC2 is denormal AND MXCSR.DAZ) THEN TMP_SRC2=0
/// IF (SRC1 is denormal AND MXCSR.DAZ) THEN TMP_SRC1=0
/// /* SRC2 is a 64 bits floating-point value */
/// DEST[63:0] := TMP_SRC1[63:0] * POW(2, Floor(TMP_SRC2[63:0]))
/// }
/// VSCALEFPD (EVEX encoded versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1) AND (SRC2 *is register*)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := SCALE(SRC1[i+63:i], SRC2[63:0]);
///                     ELSE DEST[i+63:i] := SCALE(SRC1[i+63:i], SRC2[i+63:i]);
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscalefpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def scale_fp16(src1,src2):
///     tmp1 := src1
///     tmp2 := src2
///     return tmp1 * POW(2, FLOOR(tmp2))
/// VSCALEFPH dest{k1}, src1, src2
/// VL = 128, 256, or 512
/// KL := VL / 16
/// IF (VL = 512) AND (EVEX.b = 1) and no memory operand:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC2 is memory and (EVEX.b = 1):
///             tsrc := src2.fp16[0]
///         ELSE:
///             tsrc := src2.fp16[i]
///         dest.fp16[i] := scale_fp16(src1.fp16[i],tsrc)
///     ELSE IF *zeroing*:
///         dest.fp16[i] := 0
///     //else dest.fp16[i] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscalefph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// SCALE(SRC1, SRC2)
/// {
///                 ; Check for denormal operands
/// TMP_SRC2 := SRC2
/// TMP_SRC1 := SRC1
/// IF (SRC2 is denormal AND MXCSR.DAZ) THEN TMP_SRC2=0
/// IF (SRC1 is denormal AND MXCSR.DAZ) THEN TMP_SRC1=0
/// /* SRC2 is a 32 bits floating-point value */
/// DEST[31:0] := TMP_SRC1[31:0] * POW(2, Floor(TMP_SRC2[31:0]))
/// }
/// VSCALEFPS (EVEX encoded versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1) AND (SRC2 *is register*)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := SCALE(SRC1[i+31:i], SRC2[31:0]);
///                     ELSE DEST[i+31:i] := SCALE(SRC1[i+31:i], SRC2[i+31:i]);
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vscalefps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// SCALE(SRC1, SRC2)
/// {
///     ; Check for denormal operands
/// TMP_SRC2 := SRC2
/// TMP_SRC1 := SRC1
/// IF (SRC2 is denormal AND MXCSR.DAZ) THEN TMP_SRC2=0
/// IF (SRC1 is denormal AND MXCSR.DAZ) THEN TMP_SRC1=0
/// /* SRC2 is a 64 bits floating-point value */
/// DEST[63:0] := TMP_SRC1[63:0] * POW(2, Floor(TMP_SRC2[63:0]))
/// }
/// VSCALEFSD (EVEX encoded version)
/// IF (EVEX.b= 1) and SRC2 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] OR *no writemask*
///     THEN DEST[63:0] := SCALE(SRC1[63:0], SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[63:0] := 0
///             FI
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscalefsd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSCALEFSH dest{k1}, src1, src2
/// IF (EVEX.b = 1) and no memory operand:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] or *no writemask*:
///     dest.fp16[0] := scale_fp16(src1.fp16[0], src2.fp16[0]) // see VSCALEFPH
/// ELSE IF *zeroing*:
///     dest.fp16[0] := 0
/// //else DEST.fp16[0] remains unchanged
/// DEST[127:16] := src1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscalefsh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// SCALE(SRC1, SRC2)
/// {
///                 ; Check for denormal operands
/// TMP_SRC2 := SRC2
/// TMP_SRC1 := SRC1
/// IF (SRC2 is denormal AND MXCSR.DAZ) THEN TMP_SRC2=0
/// IF (SRC1 is denormal AND MXCSR.DAZ) THEN TMP_SRC1=0
/// /* SRC2 is a 32 bits floating-point value */
/// DEST[31:0] := TMP_SRC1[31:0] * POW(2, Floor(TMP_SRC2[31:0]))
/// }
/// VSCALEFSS (EVEX encoded version)
/// IF (EVEX.b= 1) and SRC2 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] OR *no writemask*
///     THEN DEST[31:0] := SCALE(SRC1[31:0], SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[31:0] := 0
///             FI
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscalefss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Select2(SRC, control) {
/// CASE (control[0]) OF
///     0: TMP := SRC[127:0];
///     1: TMP := SRC[255:128];
/// ESAC;
/// RETURN TMP
/// }
/// Select4(SRC, control) {
/// CASE (control[1:0]) OF
///     0: TMP := SRC[127:0];
///     1: TMP := SRC[255:128];
///     2: TMP := SRC[383:256];
///     3: TMP := SRC[511:384];
/// ESAC;
/// RETURN TMP
/// }
/// VSHUFF32x4 (EVEX versions)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+31:i] := 0
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFF64x2 (EVEX 512-bit version)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFI32x4 (EVEX 512-bit version)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFI64x2 (EVEX 512-bit version)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscatterdpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Select2(SRC, control) {
/// CASE (control[0]) OF
///     0: TMP := SRC[127:0];
///     1: TMP := SRC[255:128];
/// ESAC;
/// RETURN TMP
/// }
/// Select4(SRC, control) {
/// CASE (control[1:0]) OF
///     0: TMP := SRC[127:0];
///     1: TMP := SRC[255:128];
///     2: TMP := SRC[383:256];
///     3: TMP := SRC[511:384];
/// ESAC;
/// RETURN TMP
/// }
/// VSHUFF32x4 (EVEX versions)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+31:i] := 0
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFF64x2 (EVEX 512-bit version)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFI32x4 (EVEX 512-bit version)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFI64x2 (EVEX 512-bit version)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscatterdps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Select2(SRC, control) {
/// CASE (control[0]) OF
///     0: TMP := SRC[127:0];
///     1: TMP := SRC[255:128];
/// ESAC;
/// RETURN TMP
/// }
/// Select4(SRC, control) {
/// CASE (control[1:0]) OF
///     0: TMP := SRC[127:0];
///     1: TMP := SRC[255:128];
///     2: TMP := SRC[383:256];
///     3: TMP := SRC[511:384];
/// ESAC;
/// RETURN TMP
/// }
/// VSHUFF32x4 (EVEX versions)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+31:i] := 0
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFF64x2 (EVEX 512-bit version)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFI32x4 (EVEX 512-bit version)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFI64x2 (EVEX 512-bit version)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscatterqpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Select2(SRC, control) {
/// CASE (control[0]) OF
///     0: TMP := SRC[127:0];
///     1: TMP := SRC[255:128];
/// ESAC;
/// RETURN TMP
/// }
/// Select4(SRC, control) {
/// CASE (control[1:0]) OF
///     0: TMP := SRC[127:0];
///     1: TMP := SRC[255:128];
///     2: TMP := SRC[383:256];
///     3: TMP := SRC[511:384];
/// ESAC;
/// RETURN TMP
/// }
/// VSHUFF32x4 (EVEX versions)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+31:i] := 0
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFF64x2 (EVEX 512-bit version)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFI32x4 (EVEX 512-bit version)
/// (KL, VL) = (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_DEST[i+31:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSHUFI64x2 (EVEX 512-bit version)
/// (KL, VL) = (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 256
///     TMP_DEST[127:0] := Select2(SRC1[255:0], imm8[0]);
///     TMP_DEST[255:128] := Select2(SRC2[255:0], imm8[1]);
/// FI;
/// IF VL = 512
///     TMP_DEST[127:0] := Select4(SRC1[511:0], imm8[1:0]);
///     TMP_DEST[255:128] := Select4(SRC1[511:0], imm8[3:2]);
///     TMP_DEST[383:256] := Select4(TMP_SRC2[511:0], imm8[5:4]);
///     TMP_DEST[511:384] := Select4(TMP_SRC2[511:0], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     THEN DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vscatterqps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSQRTPH dest{k1}, src
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR i := 0 to KL-1:
///     IF k1[i] or *no writemask*:
///         IF SRC is memory and (EVEX.b = 1):
///             tsrc := src.fp16[0]
///         ELSE:
///             tsrc := src.fp16[i]
///         DEST.fp16[i] := SQRT(tsrc)
///     ELSE IF *zeroing*:
///         DEST.fp16[i] := 0
///     //else DEST.fp16[i] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vsqrtph() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSQRTSH dest{k1}, src1, src2
/// IF k1[0] or *no writemask*:
///     DEST.fp16[0] := SQRT(src2.fp16[0])
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// //else DEST.fp16[0] remains unchanged
/// DEST[127:16] := src1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vsqrtsh() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSUBPH (EVEX encoded versions) when src2 operand is a register
/// VL = 128, 256 or 512
/// KL := VL/16
/// IF (VL = 512) AND (EVEX.b = 1):
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.fp16[j] := SRC1.fp16[j] - SRC2.fp16[j]
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// VSUBPH (EVEX encoded versions) when src2 operand is a memory source
/// VL = 128, 256 or 512
/// KL := VL/16
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         IF EVEX.b = 1:
///             DEST.fp16[j] := SRC1.fp16[j] - SRC2.fp16[0]
///         ELSE:
///             DEST.fp16[j] := SRC1.fp16[j] - SRC2.fp16[j]
///     ELSE IF *zeroing*:
///         DEST.fp16[j] := 0
///     // else dest.fp16[j] remains unchanged
/// DEST[MAXVL-1:VL] := 0
/// IVnStUelB PC/HC++ C
/// ```
#[box_to_static_reference]
pub(super) fn vsubph() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSUBSH (EVEX encoded versions)
/// IF EVEX.b = 1 and SRC2 is a register:
///     SET_RM(EVEX.RC)
/// ELSE
///     SET_RM(MXCSR.RC)
/// IF k1[0] OR *no writemask*:
///     DEST.fp16[0] := SRC1.fp16[0] - SRC2.fp16[0]
/// ELSE IF *zeroing*:
///     DEST.fp16[0] := 0
/// // else dest.fp16[0] remains unchanged
/// DEST[127:16] := SRC1[127:16]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vsubsh() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VTESTPS (128-bit version)
/// TEMP[127:0] := SRC[127:0] AND DEST[127:0]
/// IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// TEMP[127:0] := SRC[127:0] AND NOT DEST[127:0]
/// IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// VTESTPS (VEX.256 encoded version)
/// TEMP[255:0] := SRC[255:0] AND DEST[255:0]
/// IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127]= TEMP[160] =TEMP[191] = TEMP[224] = TEMP[255] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// TEMP[255:0] := SRC[255:0] AND NOT DEST[255:0]
/// IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127]= TEMP[160] =TEMP[191] = TEMP[224] = TEMP[255] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// VTESTPD (128-bit version)
/// TEMP[127:0] := SRC[127:0] AND DEST[127:0]
/// IF ( TEMP[63] = TEMP[127] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// TEMP[127:0] := SRC[127:0] AND NOT DEST[127:0]
/// IF ( TEMP[63] = TEMP[127] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// VTESTPD (VEX.256 encoded version)
/// TEMP[255:0] := SRC[255:0] AND DEST[255:0]
/// IF (TEMP[63] = TEMP[127] = TEMP[191] = TEMP[255] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// TEMP[255:0] := SRC[255:0] AND NOT DEST[255:0]
/// IF (TEMP[63] = TEMP[127] = TEMP[191] = TEMP[255] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vtestpd() -> &'static [IrStatement] {
    let assignment = assign(b::and(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VTESTPS (128-bit version)
/// TEMP[127:0] := SRC[127:0] AND DEST[127:0]
/// IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// TEMP[127:0] := SRC[127:0] AND NOT DEST[127:0]
/// IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// VTESTPS (VEX.256 encoded version)
/// TEMP[255:0] := SRC[255:0] AND DEST[255:0]
/// IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127]= TEMP[160] =TEMP[191] = TEMP[224] = TEMP[255] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// TEMP[255:0] := SRC[255:0] AND NOT DEST[255:0]
/// IF (TEMP[31] = TEMP[63] = TEMP[95] = TEMP[127]= TEMP[160] =TEMP[191] = TEMP[224] = TEMP[255] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// VTESTPD (128-bit version)
/// TEMP[127:0] := SRC[127:0] AND DEST[127:0]
/// IF ( TEMP[63] = TEMP[127] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// TEMP[127:0] := SRC[127:0] AND NOT DEST[127:0]
/// IF ( TEMP[63] = TEMP[127] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// VTESTPD (VEX.256 encoded version)
/// TEMP[255:0] := SRC[255:0] AND DEST[255:0]
/// IF (TEMP[63] = TEMP[127] = TEMP[191] = TEMP[255] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// TEMP[255:0] := SRC[255:0] AND NOT DEST[255:0]
/// IF (TEMP[63] = TEMP[127] = TEMP[191] = TEMP[255] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn vtestps() -> &'static [IrStatement] {
    let assignment = assign(b::and(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VUCOMISH
/// RESULT := UnorderedCompare(SRC1.fp16[0],SRC2.fp16[0])
/// if RESULT is UNORDERED:
///     ZF, PF, CF := 1, 1, 1
/// else if RESULT is GREATER_THAN:
///     ZF, PF, CF := 0, 0, 0
/// else if RESULT is LESS_THAN:
///     ZF, PF, CF := 0, 0, 1
/// else: // RESULT is EQUALS
///     ZF, PF, CF := 1, 0, 0
/// OF, AF, SF := 0, 0, 0
/// ```
#[box_to_static_reference]
pub(super) fn vucomish() -> &'static [IrStatement] {
    let sub = b::sub(o1(), o2());
    let calc_flags = calc_flags_automatically(sub, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// simd_reg_file[][] is a two dimensional array representing the SIMD register file containing all the overlapping xmm, ymm, and zmm
/// registers present in that implementation. The major dimension is the register number: 0 for xmm0, ymm0, and zmm0; 1 for xmm1,
/// ymm1, and zmm1; etc. The minor dimension size is the width of the implemented SIMD state measured in bits. On a machine
/// supporting Intel AVX-512, the width is 512.
/// VZEROALL (VEX.256 encoded version)
/// IF (64-bit mode)
///     limit :=15
/// ELSE
///     limit := 7
/// FOR i in 0 .. limit:
///     simd_reg_file[i][MAXVL-1:0] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vzeroall() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// simd_reg_file[][] is a two dimensional array representing the SIMD register file containing all the overlapping xmm, ymm, and zmm
/// registers present in that implementation. The major dimension is the register number: 0 for xmm0, ymm0, and zmm0; 1 for xmm1,
/// ymm1, and zmm1; etc. The minor dimension size is the width of the implemented SIMD state measured in bits.
/// VZEROUPPER
/// IF (64-bit mode)
///     limit :=15
/// ELSE
///     limit := 7
/// FOR i in 0 .. limit:
///     simd_reg_file[i][MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn vzeroupper() -> &'static [IrStatement] {
    [].into()
}
