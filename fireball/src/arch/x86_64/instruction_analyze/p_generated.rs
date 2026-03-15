use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// PABSB With 128-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 15th bytes
///     Unsigned DEST[127:120] := ABS(SRC[127:120])
/// VPABSB With 128-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 15th bytes
///     Unsigned DEST[127:120] := ABS(SRC[127:120])
/// VPABSB With 256-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 31st bytes
///     Unsigned DEST[255:248] := ABS(SRC[255:248])
/// VPABSB (EVEX Encoded Versions)
///     (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN
///             Unsigned DEST[i+7:i] := ABS(SRC[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PABSW With 128-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 7th 16-bit words
///     Unsigned DEST[127:112] := ABS(SRC[127:112])
/// VPABSW With 128-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 7th 16-bit words
///     Unsigned DEST[127:112] := ABS(SRC[127:112])
/// VPABSW With 256-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 15th 16-bit words
///     Unsigned DEST[255:240] := ABS(SRC[255:240])
/// VPABSW (EVEX Encoded Versions)
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN
///             Unsigned DEST[i+15:i] := ABS(SRC[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PABSD With 128-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 3rd 32-bit double words
///     Unsigned DEST[127:96] := ABS(SRC[127:96])
/// VPABSD With 128-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 3rd 32-bit double words
///     Unsigned DEST[127:96] := ABS(SRC[127:96])
/// VPABSD With 256-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 7th 32-bit double words
///     Unsigned DEST[255:224] := ABS(SRC[255:224])
/// VPABSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC *is memory*)
///                 THEN
///                     Unsigned DEST[i+31:i] := ABS(SRC[31:0])
///                 ELSE
///                     Unsigned DEST[i+31:i] := ABS(SRC[i+31:i])
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
/// ENDFOR;
/// VPABSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC *is memory*)
///                 THEN
///                     Unsigned DEST[i+63:i] := ABS(SRC[63:0])
///                 ELSE
///                     Unsigned DEST[i+63:i] := ABS(SRC[i+63:i])
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pabsb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PABSB With 128-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 15th bytes
///     Unsigned DEST[127:120] := ABS(SRC[127:120])
/// VPABSB With 128-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 15th bytes
///     Unsigned DEST[127:120] := ABS(SRC[127:120])
/// VPABSB With 256-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 31st bytes
///     Unsigned DEST[255:248] := ABS(SRC[255:248])
/// VPABSB (EVEX Encoded Versions)
///     (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN
///             Unsigned DEST[i+7:i] := ABS(SRC[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PABSW With 128-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 7th 16-bit words
///     Unsigned DEST[127:112] := ABS(SRC[127:112])
/// VPABSW With 128-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 7th 16-bit words
///     Unsigned DEST[127:112] := ABS(SRC[127:112])
/// VPABSW With 256-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 15th 16-bit words
///     Unsigned DEST[255:240] := ABS(SRC[255:240])
/// VPABSW (EVEX Encoded Versions)
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN
///             Unsigned DEST[i+15:i] := ABS(SRC[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PABSD With 128-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 3rd 32-bit double words
///     Unsigned DEST[127:96] := ABS(SRC[127:96])
/// VPABSD With 128-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 3rd 32-bit double words
///     Unsigned DEST[127:96] := ABS(SRC[127:96])
/// VPABSD With 256-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 7th 32-bit double words
///     Unsigned DEST[255:224] := ABS(SRC[255:224])
/// VPABSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC *is memory*)
///                 THEN
///                     Unsigned DEST[i+31:i] := ABS(SRC[31:0])
///                 ELSE
///                     Unsigned DEST[i+31:i] := ABS(SRC[i+31:i])
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
/// ENDFOR;
/// VPABSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC *is memory*)
///                 THEN
///                     Unsigned DEST[i+63:i] := ABS(SRC[63:0])
///                 ELSE
///                     Unsigned DEST[i+63:i] := ABS(SRC[i+63:i])
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pabsd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PABSB With 128-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 15th bytes
///     Unsigned DEST[127:120] := ABS(SRC[127:120])
/// VPABSB With 128-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 15th bytes
///     Unsigned DEST[127:120] := ABS(SRC[127:120])
/// VPABSB With 256-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 31st bytes
///     Unsigned DEST[255:248] := ABS(SRC[255:248])
/// VPABSB (EVEX Encoded Versions)
///     (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN
///             Unsigned DEST[i+7:i] := ABS(SRC[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PABSW With 128-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 7th 16-bit words
///     Unsigned DEST[127:112] := ABS(SRC[127:112])
/// VPABSW With 128-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 7th 16-bit words
///     Unsigned DEST[127:112] := ABS(SRC[127:112])
/// VPABSW With 256-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 15th 16-bit words
///     Unsigned DEST[255:240] := ABS(SRC[255:240])
/// VPABSW (EVEX Encoded Versions)
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN
///             Unsigned DEST[i+15:i] := ABS(SRC[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PABSD With 128-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 3rd 32-bit double words
///     Unsigned DEST[127:96] := ABS(SRC[127:96])
/// VPABSD With 128-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 3rd 32-bit double words
///     Unsigned DEST[127:96] := ABS(SRC[127:96])
/// VPABSD With 256-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 7th 32-bit double words
///     Unsigned DEST[255:224] := ABS(SRC[255:224])
/// VPABSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC *is memory*)
///                 THEN
///                     Unsigned DEST[i+31:i] := ABS(SRC[31:0])
///                 ELSE
///                     Unsigned DEST[i+31:i] := ABS(SRC[i+31:i])
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
/// ENDFOR;
/// VPABSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC *is memory*)
///                 THEN
///                     Unsigned DEST[i+63:i] := ABS(SRC[63:0])
///                 ELSE
///                     Unsigned DEST[i+63:i] := ABS(SRC[i+63:i])
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pabsq() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PABSB With 128-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 15th bytes
///     Unsigned DEST[127:120] := ABS(SRC[127:120])
/// VPABSB With 128-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 15th bytes
///     Unsigned DEST[127:120] := ABS(SRC[127:120])
/// VPABSB With 256-bit Operands:
///     Unsigned DEST[7:0] := ABS(SRC[7: 0])
///     Repeat operation for 2nd through 31st bytes
///     Unsigned DEST[255:248] := ABS(SRC[255:248])
/// VPABSB (EVEX Encoded Versions)
///     (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN
///             Unsigned DEST[i+7:i] := ABS(SRC[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PABSW With 128-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 7th 16-bit words
///     Unsigned DEST[127:112] := ABS(SRC[127:112])
/// VPABSW With 128-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 7th 16-bit words
///     Unsigned DEST[127:112] := ABS(SRC[127:112])
/// VPABSW With 256-bit Operands:
///     Unsigned DEST[15:0] := ABS(SRC[15:0])
///     Repeat operation for 2nd through 15th 16-bit words
///     Unsigned DEST[255:240] := ABS(SRC[255:240])
/// VPABSW (EVEX Encoded Versions)
///     (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN
///             Unsigned DEST[i+15:i] := ABS(SRC[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PABSD With 128-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 3rd 32-bit double words
///     Unsigned DEST[127:96] := ABS(SRC[127:96])
/// VPABSD With 128-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 3rd 32-bit double words
///     Unsigned DEST[127:96] := ABS(SRC[127:96])
/// VPABSD With 256-bit Operands:
///     Unsigned DEST[31:0] := ABS(SRC[31:0])
///     Repeat operation for 2nd through 7th 32-bit double words
///     Unsigned DEST[255:224] := ABS(SRC[255:224])
/// VPABSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC *is memory*)
///                 THEN
///                     Unsigned DEST[i+31:i] := ABS(SRC[31:0])
///                 ELSE
///                     Unsigned DEST[i+31:i] := ABS(SRC[i+31:i])
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
/// ENDFOR;
/// VPABSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC *is memory*)
///                 THEN
///                     Unsigned DEST[i+63:i] := ABS(SRC[63:0])
///                 ELSE
///                     Unsigned DEST[i+63:i] := ABS(SRC[i+63:i])
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pabsw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PACKSSWB Instruction (128-bit Legacy SSE Version)
///     DEST[7:0] := SaturateSignedWordToSignedByte (DEST[15:0]);
///     DEST[15:8] := SaturateSignedWordToSignedByte (DEST[31:16]);
///     DEST[23:16] := SaturateSignedWordToSignedByte (DEST[47:32]);
///     DEST[31:24] := SaturateSignedWordToSignedByte (DEST[63:48]);
///     DEST[39:32] := SaturateSignedWordToSignedByte (DEST[79:64]);
///     DEST[47:40] := SaturateSignedWordToSignedByte (DEST[95:80]);
///     DEST[55:48] := SaturateSignedWordToSignedByte (DEST[111:96]);
///     DEST[63:56] := SaturateSignedWordToSignedByte (DEST[127:112]);
///     DEST[71:64] := SaturateSignedWordToSignedByte (SRC[15:0]);
///     DEST[79:72] := SaturateSignedWordToSignedByte (SRC[31:16]);
///     DEST[87:80] := SaturateSignedWordToSignedByte (SRC[47:32]);
///     DEST[95:88] := SaturateSignedWordToSignedByte (SRC[63:48]);
///     DEST[103:96] := SaturateSignedWordToSignedByte (SRC[79:64]);
///     DEST[111:104] := SaturateSignedWordToSignedByte (SRC[95:80]);
///     DEST[119:112] := SaturateSignedWordToSignedByte (SRC[111:96]);
///     DEST[127:120] := SaturateSignedWordToSignedByte (SRC[127:112]);
///     DEST[MAXVL-1:128] (Unmodified)
/// PACKSSDW Instruction (128-bit Legacy SSE Version)
///     DEST[15:0] := SaturateSignedDwordToSignedWord (DEST[31:0]);
///     DEST[31:16] := SaturateSignedDwordToSignedWord (DEST[63:32]);
///     DEST[47:32] := SaturateSignedDwordToSignedWord (DEST[95:64]);
///     DEST[63:48] := SaturateSignedDwordToSignedWord (DEST[127:96]);
///     DEST[79:64] := SaturateSignedDwordToSignedWord (SRC[31:0]);
///     DEST[95:80] := SaturateSignedDwordToSignedWord (SRC[63:32]);
///     DEST[111:96] := SaturateSignedDwordToSignedWord (SRC[95:64]);
///     DEST[127:112] := SaturateSignedDwordToSignedWord (SRC[127:96]);
///     DEST[MAXVL-1:128] (Unmodified)
/// VPACKSSWB Instruction (VEX.128 Encoded Version)
///     DEST[7:0] := SaturateSignedWordToSignedByte (SRC1[15:0]);
///     DEST[15:8] := SaturateSignedWordToSignedByte (SRC1[31:16]);
///     DEST[23:16] := SaturateSignedWordToSignedByte (SRC1[47:32]);
///     DEST[31:24] := SaturateSignedWordToSignedByte (SRC1[63:48]);
///     DEST[39:32] := SaturateSignedWordToSignedByte (SRC1[79:64]);
///     DEST[47:40] := SaturateSignedWordToSignedByte (SRC1[95:80]);
///     DEST[55:48] := SaturateSignedWordToSignedByte (SRC1[111:96]);
///     DEST[63:56] := SaturateSignedWordToSignedByte (SRC1[127:112]);
///     DEST[71:64] := SaturateSignedWordToSignedByte (SRC2[15:0]);
///     DEST[79:72] := SaturateSignedWordToSignedByte (SRC2[31:16]);
///     DEST[87:80] := SaturateSignedWordToSignedByte (SRC2[47:32]);
///     DEST[95:88] := SaturateSignedWordToSignedByte (SRC2[63:48]);
///     DEST[103:96] := SaturateSignedWordToSignedByte (SRC2[79:64]);
///     DEST[111:104] := SaturateSignedWordToSignedByte (SRC2[95:80]);
///     DEST[119:112] := SaturateSignedWordToSignedByte (SRC2[111:96]);
///     DEST[127:120] := SaturateSignedWordToSignedByte (SRC2[127:112]);
///     DEST[MAXVL-1:128] := 0;
/// VPACKSSDW Instruction (VEX.128 Encoded Version)
///     DEST[15:0] := SaturateSignedDwordToSignedWord (SRC1[31:0]);
///     DEST[31:16] := SaturateSignedDwordToSignedWord (SRC1[63:32]);
///     DEST[47:32] := SaturateSignedDwordToSignedWord (SRC1[95:64]);
///     DEST[63:48] := SaturateSignedDwordToSignedWord (SRC1[127:96]);
///     DEST[79:64] := SaturateSignedDwordToSignedWord (SRC2[31:0]);
///     DEST[95:80] := SaturateSignedDwordToSignedWord (SRC2[63:32]);
///     DEST[111:96] := SaturateSignedDwordToSignedWord (SRC2[95:64]);
///     DEST[127:112] := SaturateSignedDwordToSignedWord (SRC2[127:96]);
///     DEST[MAXVL-1:128] := 0;
/// VPACKSSWB Instruction (VEX.256 Encoded Version)
///     DEST[7:0] := SaturateSignedWordToSignedByte (SRC1[15:0]);
///     DEST[15:8] := SaturateSignedWordToSignedByte (SRC1[31:16]);
///     DEST[23:16] := SaturateSignedWordToSignedByte (SRC1[47:32]);
///     DEST[31:24] := SaturateSignedWordToSignedByte (SRC1[63:48]);
///     DEST[39:32] := SaturateSignedWordToSignedByte (SRC1[79:64]);
///     DEST[47:40] := SaturateSignedWordToSignedByte (SRC1[95:80]);
///     DEST[55:48] := SaturateSignedWordToSignedByte (SRC1[111:96]);
///     DEST[63:56] := SaturateSignedWordToSignedByte (SRC1[127:112]);
///     DEST[71:64] := SaturateSignedWordToSignedByte (SRC2[15:0]);
///     DEST[79:72] := SaturateSignedWordToSignedByte (SRC2[31:16]);
///     DEST[87:80] := SaturateSignedWordToSignedByte (SRC2[47:32]);
///     DEST[95:88] := SaturateSignedWordToSignedByte (SRC2[63:48]);
///     DEST[103:96] := SaturateSignedWordToSignedByte (SRC2[79:64]);
///     DEST[111:104] := SaturateSignedWordToSignedByte (SRC2[95:80]);
///     DEST[119:112] := SaturateSignedWordToSignedByte (SRC2[111:96]);
///     DEST[127:120] := SaturateSignedWordToSignedByte (SRC2[127:112]);
///     DEST[135:128] := SaturateSignedWordToSignedByte (SRC1[143:128]);
///     DEST[143:136] := SaturateSignedWordToSignedByte (SRC1[159:144]);
///     DEST[151:144] := SaturateSignedWordToSignedByte (SRC1[175:160]);
///     DEST[159:152] := SaturateSignedWordToSignedByte (SRC1[191:176]);
///     DEST[167:160] := SaturateSignedWordToSignedByte (SRC1[207:192]);
///     DEST[175:168] := SaturateSignedWordToSignedByte (SRC1[223:208]);
///     DEST[191:184] := SaturateSignedWordToSignedByte (SRC1[255:240]);
///     DEST[199:192] := SaturateSignedWordToSignedByte (SRC2[143:128]);
///     DEST[207:200] := SaturateSignedWordToSignedByte (SRC2[159:144]);
///     DEST[215:208] := SaturateSignedWordToSignedByte (SRC2[175:160]);
///     DEST[223:216] := SaturateSignedWordToSignedByte (SRC2[191:176]);
///     DEST[231:224] := SaturateSignedWordToSignedByte (SRC2[207:192]);
///     DEST[239:232] := SaturateSignedWordToSignedByte (SRC2[223:208]);
///     DEST[247:240] := SaturateSignedWordToSignedByte (SRC2[239:224]);
///     DEST[255:248] := SaturateSignedWordToSignedByte (SRC2[255:240]);
///     DEST[MAXVL-1:256] := 0;
/// VPACKSSDW Instruction (VEX.256 Encoded Version)
///     DEST[15:0] := SaturateSignedDwordToSignedWord (SRC1[31:0]);
///     DEST[31:16] := SaturateSignedDwordToSignedWord (SRC1[63:32]);
///     DEST[47:32] := SaturateSignedDwordToSignedWord (SRC1[95:64]);
///     DEST[63:48] := SaturateSignedDwordToSignedWord (SRC1[127:96]);
///     DEST[79:64] := SaturateSignedDwordToSignedWord (SRC2[31:0]);
///     DEST[95:80] := SaturateSignedDwordToSignedWord (SRC2[63:32]);
///     DEST[111:96] := SaturateSignedDwordToSignedWord (SRC2[95:64]);
///     DEST[127:112] := SaturateSignedDwordToSignedWord (SRC2[127:96]);
///     DEST[143:128] := SaturateSignedDwordToSignedWord (SRC1[159:128]);
///     DEST[159:144] := SaturateSignedDwordToSignedWord (SRC1[191:160]);
///     DEST[175:160] := SaturateSignedDwordToSignedWord (SRC1[223:192]);
///     DEST[191:176] := SaturateSignedDwordToSignedWord (SRC1[255:224]);
///     DEST[207:192] := SaturateSignedDwordToSignedWord (SRC2[159:128]);
///     DEST[223:208] := SaturateSignedDwordToSignedWord (SRC2[191:160]);
///     DEST[239:224] := SaturateSignedDwordToSignedWord (SRC2[223:192]);
///     DEST[255:240] := SaturateSignedDwordToSignedWord (SRC2[255:224]);
///     DEST[MAXVL-1:256] := 0;
/// VPACKSSWB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// TMP_DEST[7:0] := SaturateSignedWordToSignedByte (SRC1[15:0]);
/// TMP_DEST[15:8] := SaturateSignedWordToSignedByte (SRC1[31:16]);
/// TMP_DEST[23:16] := SaturateSignedWordToSignedByte (SRC1[47:32]);
/// TMP_DEST[31:24] := SaturateSignedWordToSignedByte (SRC1[63:48]);
/// TMP_DEST[39:32] := SaturateSignedWordToSignedByte (SRC1[79:64]);
/// TMP_DEST[47:40] := SaturateSignedWordToSignedByte (SRC1[95:80]);
/// TMP_DEST[55:48] := SaturateSignedWordToSignedByte (SRC1[111:96]);
/// TMP_DEST[63:56] := SaturateSignedWordToSignedByte (SRC1[127:112]);
/// TMP_DEST[71:64] := SaturateSignedWordToSignedByte (SRC2[15:0]);
/// TMP_DEST[79:72] := SaturateSignedWordToSignedByte (SRC2[31:16]);
/// TMP_DEST[87:80] := SaturateSignedWordToSignedByte (SRC2[47:32]);
/// TMP_DEST[95:88] := SaturateSignedWordToSignedByte (SRC2[63:48]);
/// TMP_DEST[103:96] := SaturateSignedWordToSignedByte (SRC2[79:64]);
/// TMP_DEST[111:104] := SaturateSignedWordToSignedByte (SRC2[95:80]);
/// TMP_DEST[119:112] := SaturateSignedWordToSignedByte (SRC2[111:96]);
/// TMP_DEST[127:120] := SaturateSignedWordToSignedByte (SRC2[127:112]);
/// IF VL >= 256
///     TMP_DEST[135:128] := SaturateSignedWordToSignedByte (SRC1[143:128]);
///     TMP_DEST[143:136] := SaturateSignedWordToSignedByte (SRC1[159:144]);
///     TMP_DEST[151:144] := SaturateSignedWordToSignedByte (SRC1[175:160]);
///     TMP_DEST[159:152] := SaturateSignedWordToSignedByte (SRC1[191:176]);
///     TMP_DEST[175:168] := SaturateSignedWordToSignedByte (SRC1[223:208]);
///     TMP_DEST[183:176] := SaturateSignedWordToSignedByte (SRC1[239:224]);
///     TMP_DEST[191:184] := SaturateSignedWordToSignedByte (SRC1[255:240]);
///     TMP_DEST[199:192] := SaturateSignedWordToSignedByte (SRC2[143:128]);
///     TMP_DEST[207:200] := SaturateSignedWordToSignedByte (SRC2[159:144]);
///     TMP_DEST[215:208] := SaturateSignedWordToSignedByte (SRC2[175:160]);
///     TMP_DEST[223:216] := SaturateSignedWordToSignedByte (SRC2[191:176]);
///     TMP_DEST[231:224] := SaturateSignedWordToSignedByte (SRC2[207:192]);
///     TMP_DEST[239:232] := SaturateSignedWordToSignedByte (SRC2[223:208]);
///     TMP_DEST[247:240] := SaturateSignedWordToSignedByte (SRC2[239:224]);
///     TMP_DEST[255:248] := SaturateSignedWordToSignedByte (SRC2[255:240]);
/// FI;
/// IF VL >= 512
///     TMP_DEST[263:256] := SaturateSignedWordToSignedByte (SRC1[271:256]);
///     TMP_DEST[271:264] := SaturateSignedWordToSignedByte (SRC1[287:272]);
///     TMP_DEST[279:272] := SaturateSignedWordToSignedByte (SRC1[303:288]);
///     TMP_DEST[287:280] := SaturateSignedWordToSignedByte (SRC1[319:304]);
///     TMP_DEST[295:288] := SaturateSignedWordToSignedByte (SRC1[335:320]);
///     TMP_DEST[303:296] := SaturateSignedWordToSignedByte (SRC1[351:336]);
///     TMP_DEST[311:304] := SaturateSignedWordToSignedByte (SRC1[367:352]);
///     TMP_DEST[319:312] := SaturateSignedWordToSignedByte (SRC1[383:368]);
///     TMP_DEST[327:320] := SaturateSignedWordToSignedByte (SRC2[271:256]);
///     TMP_DEST[335:328] := SaturateSignedWordToSignedByte (SRC2[287:272]);
///     TMP_DEST[343:336] := SaturateSignedWordToSignedByte (SRC2[303:288]);
///     TMP_DEST[351:344] := SaturateSignedWordToSignedByte (SRC2[319:304]);
///     TMP_DEST[359:352] := SaturateSignedWordToSignedByte (SRC2[335:320]);
///     TMP_DEST[367:360] := SaturateSignedWordToSignedByte (SRC2[351:336]);
///     TMP_DEST[375:368] := SaturateSignedWordToSignedByte (SRC2[367:352]);
///     TMP_DEST[383:376] := SaturateSignedWordToSignedByte (SRC2[383:368]);
///     TMP_DEST[391:384] := SaturateSignedWordToSignedByte (SRC1[399:384]);
///     TMP_DEST[399:392] := SaturateSignedWordToSignedByte (SRC1[415:400]);
///     TMP_DEST[407:400] := SaturateSignedWordToSignedByte (SRC1[431:416]);
///     TMP_DEST[415:408] := SaturateSignedWordToSignedByte (SRC1[447:432]);
///     TMP_DEST[423:416] := SaturateSignedWordToSignedByte (SRC1[463:448]);
///     TMP_DEST[431:424] := SaturateSignedWordToSignedByte (SRC1[479:464]);
///     TMP_DEST[439:432] := SaturateSignedWordToSignedByte (SRC1[495:480]);
///     TMP_DEST[447:440] := SaturateSignedWordToSignedByte (SRC1[511:496]);
///     TMP_DEST[455:448] := SaturateSignedWordToSignedByte (SRC2[399:384]);
///     TMP_DEST[463:456] := SaturateSignedWordToSignedByte (SRC2[415:400]);
///     TMP_DEST[471:464] := SaturateSignedWordToSignedByte (SRC2[431:416]);
///     TMP_DEST[479:472] := SaturateSignedWordToSignedByte (SRC2[447:432]);
///     TMP_DEST[487:480] := SaturateSignedWordToSignedByte (SRC2[463:448]);
///     TMP_DEST[495:488] := SaturateSignedWordToSignedByte (SRC2[479:464]);
///     TMP_DEST[503:496] := SaturateSignedWordToSignedByte (SRC2[495:480]);
///     TMP_DEST[511:504] := SaturateSignedWordToSignedByte (SRC2[511:496]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPACKSSDW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO ((KL/2) - 1)
///     i := j * 32
///     IF (EVEX.b == 1) AND (SRC2 *is memory*)
///         THEN
///             TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE
///             TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// TMP_DEST[15:0] := SaturateSignedDwordToSignedWord (SRC1[31:0]);
/// TMP_DEST[31:16] := SaturateSignedDwordToSignedWord (SRC1[63:32]);
/// TMP_DEST[47:32] := SaturateSignedDwordToSignedWord (SRC1[95:64]);
/// TMP_DEST[63:48] := SaturateSignedDwordToSignedWord (SRC1[127:96]);
/// TMP_DEST[79:64] := SaturateSignedDwordToSignedWord (TMP_SRC2[31:0]);
/// TMP_DEST[95:80] := SaturateSignedDwordToSignedWord (TMP_SRC2[63:32]);
/// TMP_DEST[111:96] := SaturateSignedDwordToSignedWord (TMP_SRC2[95:64]);
/// TMP_DEST[127:112] := SaturateSignedDwordToSignedWord (TMP_SRC2[127:96]);
/// IF VL >= 256
///     TMP_DEST[143:128] := SaturateSignedDwordToSignedWord (SRC1[159:128]);
///     TMP_DEST[159:144] := SaturateSignedDwordToSignedWord (SRC1[191:160]);
///     TMP_DEST[175:160] := SaturateSignedDwordToSignedWord (SRC1[223:192]);
///     TMP_DEST[191:176] := SaturateSignedDwordToSignedWord (SRC1[255:224]);
///     TMP_DEST[207:192] := SaturateSignedDwordToSignedWord (TMP_SRC2[159:128]);
///     TMP_DEST[223:208] := SaturateSignedDwordToSignedWord (TMP_SRC2[191:160]);
///     TMP_DEST[239:224] := SaturateSignedDwordToSignedWord (TMP_SRC2[223:192]);
///     TMP_DEST[255:240] := SaturateSignedDwordToSignedWord (TMP_SRC2[255:224]);
/// FI;
/// IF VL >= 512
///     TMP_DEST[271:256] := SaturateSignedDwordToSignedWord (SRC1[287:256]);
///     TMP_DEST[287:272] := SaturateSignedDwordToSignedWord (SRC1[319:288]);
///     TMP_DEST[303:288] := SaturateSignedDwordToSignedWord (SRC1[351:320]);
///     TMP_DEST[319:304] := SaturateSignedDwordToSignedWord (SRC1[383:352]);
///     TMP_DEST[335:320] := SaturateSignedDwordToSignedWord (TMP_SRC2[287:256]);
///     TMP_DEST[351:336] := SaturateSignedDwordToSignedWord (TMP_SRC2[319:288]);
///     TMP_DEST[367:352] := SaturateSignedDwordToSignedWord (TMP_SRC2[351:320]);
///     TMP_DEST[383:368] := SaturateSignedDwordToSignedWord (TMP_SRC2[383:352]);
///     TMP_DEST[399:384] := SaturateSignedDwordToSignedWord (SRC1[415:384]);
///     TMP_DEST[415:400] := SaturateSignedDwordToSignedWord (SRC1[447:416]);
///     TMP_DEST[447:432] := SaturateSignedDwordToSignedWord (SRC1[511:480]);
///     TMP_DEST[463:448] := SaturateSignedDwordToSignedWord (TMP_SRC2[415:384]);
///     TMP_DEST[479:464] := SaturateSignedDwordToSignedWord (TMP_SRC2[447:416]);
///     TMP_DEST[495:480] := SaturateSignedDwordToSignedWord (TMP_SRC2[479:448]);
///     TMP_DEST[511:496] := SaturateSignedDwordToSignedWord (TMP_SRC2[511:480]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn packssdw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PACKSSWB Instruction (128-bit Legacy SSE Version)
///     DEST[7:0] := SaturateSignedWordToSignedByte (DEST[15:0]);
///     DEST[15:8] := SaturateSignedWordToSignedByte (DEST[31:16]);
///     DEST[23:16] := SaturateSignedWordToSignedByte (DEST[47:32]);
///     DEST[31:24] := SaturateSignedWordToSignedByte (DEST[63:48]);
///     DEST[39:32] := SaturateSignedWordToSignedByte (DEST[79:64]);
///     DEST[47:40] := SaturateSignedWordToSignedByte (DEST[95:80]);
///     DEST[55:48] := SaturateSignedWordToSignedByte (DEST[111:96]);
///     DEST[63:56] := SaturateSignedWordToSignedByte (DEST[127:112]);
///     DEST[71:64] := SaturateSignedWordToSignedByte (SRC[15:0]);
///     DEST[79:72] := SaturateSignedWordToSignedByte (SRC[31:16]);
///     DEST[87:80] := SaturateSignedWordToSignedByte (SRC[47:32]);
///     DEST[95:88] := SaturateSignedWordToSignedByte (SRC[63:48]);
///     DEST[103:96] := SaturateSignedWordToSignedByte (SRC[79:64]);
///     DEST[111:104] := SaturateSignedWordToSignedByte (SRC[95:80]);
///     DEST[119:112] := SaturateSignedWordToSignedByte (SRC[111:96]);
///     DEST[127:120] := SaturateSignedWordToSignedByte (SRC[127:112]);
///     DEST[MAXVL-1:128] (Unmodified)
/// PACKSSDW Instruction (128-bit Legacy SSE Version)
///     DEST[15:0] := SaturateSignedDwordToSignedWord (DEST[31:0]);
///     DEST[31:16] := SaturateSignedDwordToSignedWord (DEST[63:32]);
///     DEST[47:32] := SaturateSignedDwordToSignedWord (DEST[95:64]);
///     DEST[63:48] := SaturateSignedDwordToSignedWord (DEST[127:96]);
///     DEST[79:64] := SaturateSignedDwordToSignedWord (SRC[31:0]);
///     DEST[95:80] := SaturateSignedDwordToSignedWord (SRC[63:32]);
///     DEST[111:96] := SaturateSignedDwordToSignedWord (SRC[95:64]);
///     DEST[127:112] := SaturateSignedDwordToSignedWord (SRC[127:96]);
///     DEST[MAXVL-1:128] (Unmodified)
/// VPACKSSWB Instruction (VEX.128 Encoded Version)
///     DEST[7:0] := SaturateSignedWordToSignedByte (SRC1[15:0]);
///     DEST[15:8] := SaturateSignedWordToSignedByte (SRC1[31:16]);
///     DEST[23:16] := SaturateSignedWordToSignedByte (SRC1[47:32]);
///     DEST[31:24] := SaturateSignedWordToSignedByte (SRC1[63:48]);
///     DEST[39:32] := SaturateSignedWordToSignedByte (SRC1[79:64]);
///     DEST[47:40] := SaturateSignedWordToSignedByte (SRC1[95:80]);
///     DEST[55:48] := SaturateSignedWordToSignedByte (SRC1[111:96]);
///     DEST[63:56] := SaturateSignedWordToSignedByte (SRC1[127:112]);
///     DEST[71:64] := SaturateSignedWordToSignedByte (SRC2[15:0]);
///     DEST[79:72] := SaturateSignedWordToSignedByte (SRC2[31:16]);
///     DEST[87:80] := SaturateSignedWordToSignedByte (SRC2[47:32]);
///     DEST[95:88] := SaturateSignedWordToSignedByte (SRC2[63:48]);
///     DEST[103:96] := SaturateSignedWordToSignedByte (SRC2[79:64]);
///     DEST[111:104] := SaturateSignedWordToSignedByte (SRC2[95:80]);
///     DEST[119:112] := SaturateSignedWordToSignedByte (SRC2[111:96]);
///     DEST[127:120] := SaturateSignedWordToSignedByte (SRC2[127:112]);
///     DEST[MAXVL-1:128] := 0;
/// VPACKSSDW Instruction (VEX.128 Encoded Version)
///     DEST[15:0] := SaturateSignedDwordToSignedWord (SRC1[31:0]);
///     DEST[31:16] := SaturateSignedDwordToSignedWord (SRC1[63:32]);
///     DEST[47:32] := SaturateSignedDwordToSignedWord (SRC1[95:64]);
///     DEST[63:48] := SaturateSignedDwordToSignedWord (SRC1[127:96]);
///     DEST[79:64] := SaturateSignedDwordToSignedWord (SRC2[31:0]);
///     DEST[95:80] := SaturateSignedDwordToSignedWord (SRC2[63:32]);
///     DEST[111:96] := SaturateSignedDwordToSignedWord (SRC2[95:64]);
///     DEST[127:112] := SaturateSignedDwordToSignedWord (SRC2[127:96]);
///     DEST[MAXVL-1:128] := 0;
/// VPACKSSWB Instruction (VEX.256 Encoded Version)
///     DEST[7:0] := SaturateSignedWordToSignedByte (SRC1[15:0]);
///     DEST[15:8] := SaturateSignedWordToSignedByte (SRC1[31:16]);
///     DEST[23:16] := SaturateSignedWordToSignedByte (SRC1[47:32]);
///     DEST[31:24] := SaturateSignedWordToSignedByte (SRC1[63:48]);
///     DEST[39:32] := SaturateSignedWordToSignedByte (SRC1[79:64]);
///     DEST[47:40] := SaturateSignedWordToSignedByte (SRC1[95:80]);
///     DEST[55:48] := SaturateSignedWordToSignedByte (SRC1[111:96]);
///     DEST[63:56] := SaturateSignedWordToSignedByte (SRC1[127:112]);
///     DEST[71:64] := SaturateSignedWordToSignedByte (SRC2[15:0]);
///     DEST[79:72] := SaturateSignedWordToSignedByte (SRC2[31:16]);
///     DEST[87:80] := SaturateSignedWordToSignedByte (SRC2[47:32]);
///     DEST[95:88] := SaturateSignedWordToSignedByte (SRC2[63:48]);
///     DEST[103:96] := SaturateSignedWordToSignedByte (SRC2[79:64]);
///     DEST[111:104] := SaturateSignedWordToSignedByte (SRC2[95:80]);
///     DEST[119:112] := SaturateSignedWordToSignedByte (SRC2[111:96]);
///     DEST[127:120] := SaturateSignedWordToSignedByte (SRC2[127:112]);
///     DEST[135:128] := SaturateSignedWordToSignedByte (SRC1[143:128]);
///     DEST[143:136] := SaturateSignedWordToSignedByte (SRC1[159:144]);
///     DEST[151:144] := SaturateSignedWordToSignedByte (SRC1[175:160]);
///     DEST[159:152] := SaturateSignedWordToSignedByte (SRC1[191:176]);
///     DEST[167:160] := SaturateSignedWordToSignedByte (SRC1[207:192]);
///     DEST[175:168] := SaturateSignedWordToSignedByte (SRC1[223:208]);
///     DEST[191:184] := SaturateSignedWordToSignedByte (SRC1[255:240]);
///     DEST[199:192] := SaturateSignedWordToSignedByte (SRC2[143:128]);
///     DEST[207:200] := SaturateSignedWordToSignedByte (SRC2[159:144]);
///     DEST[215:208] := SaturateSignedWordToSignedByte (SRC2[175:160]);
///     DEST[223:216] := SaturateSignedWordToSignedByte (SRC2[191:176]);
///     DEST[231:224] := SaturateSignedWordToSignedByte (SRC2[207:192]);
///     DEST[239:232] := SaturateSignedWordToSignedByte (SRC2[223:208]);
///     DEST[247:240] := SaturateSignedWordToSignedByte (SRC2[239:224]);
///     DEST[255:248] := SaturateSignedWordToSignedByte (SRC2[255:240]);
///     DEST[MAXVL-1:256] := 0;
/// VPACKSSDW Instruction (VEX.256 Encoded Version)
///     DEST[15:0] := SaturateSignedDwordToSignedWord (SRC1[31:0]);
///     DEST[31:16] := SaturateSignedDwordToSignedWord (SRC1[63:32]);
///     DEST[47:32] := SaturateSignedDwordToSignedWord (SRC1[95:64]);
///     DEST[63:48] := SaturateSignedDwordToSignedWord (SRC1[127:96]);
///     DEST[79:64] := SaturateSignedDwordToSignedWord (SRC2[31:0]);
///     DEST[95:80] := SaturateSignedDwordToSignedWord (SRC2[63:32]);
///     DEST[111:96] := SaturateSignedDwordToSignedWord (SRC2[95:64]);
///     DEST[127:112] := SaturateSignedDwordToSignedWord (SRC2[127:96]);
///     DEST[143:128] := SaturateSignedDwordToSignedWord (SRC1[159:128]);
///     DEST[159:144] := SaturateSignedDwordToSignedWord (SRC1[191:160]);
///     DEST[175:160] := SaturateSignedDwordToSignedWord (SRC1[223:192]);
///     DEST[191:176] := SaturateSignedDwordToSignedWord (SRC1[255:224]);
///     DEST[207:192] := SaturateSignedDwordToSignedWord (SRC2[159:128]);
///     DEST[223:208] := SaturateSignedDwordToSignedWord (SRC2[191:160]);
///     DEST[239:224] := SaturateSignedDwordToSignedWord (SRC2[223:192]);
///     DEST[255:240] := SaturateSignedDwordToSignedWord (SRC2[255:224]);
///     DEST[MAXVL-1:256] := 0;
/// VPACKSSWB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// TMP_DEST[7:0] := SaturateSignedWordToSignedByte (SRC1[15:0]);
/// TMP_DEST[15:8] := SaturateSignedWordToSignedByte (SRC1[31:16]);
/// TMP_DEST[23:16] := SaturateSignedWordToSignedByte (SRC1[47:32]);
/// TMP_DEST[31:24] := SaturateSignedWordToSignedByte (SRC1[63:48]);
/// TMP_DEST[39:32] := SaturateSignedWordToSignedByte (SRC1[79:64]);
/// TMP_DEST[47:40] := SaturateSignedWordToSignedByte (SRC1[95:80]);
/// TMP_DEST[55:48] := SaturateSignedWordToSignedByte (SRC1[111:96]);
/// TMP_DEST[63:56] := SaturateSignedWordToSignedByte (SRC1[127:112]);
/// TMP_DEST[71:64] := SaturateSignedWordToSignedByte (SRC2[15:0]);
/// TMP_DEST[79:72] := SaturateSignedWordToSignedByte (SRC2[31:16]);
/// TMP_DEST[87:80] := SaturateSignedWordToSignedByte (SRC2[47:32]);
/// TMP_DEST[95:88] := SaturateSignedWordToSignedByte (SRC2[63:48]);
/// TMP_DEST[103:96] := SaturateSignedWordToSignedByte (SRC2[79:64]);
/// TMP_DEST[111:104] := SaturateSignedWordToSignedByte (SRC2[95:80]);
/// TMP_DEST[119:112] := SaturateSignedWordToSignedByte (SRC2[111:96]);
/// TMP_DEST[127:120] := SaturateSignedWordToSignedByte (SRC2[127:112]);
/// IF VL >= 256
///     TMP_DEST[135:128] := SaturateSignedWordToSignedByte (SRC1[143:128]);
///     TMP_DEST[143:136] := SaturateSignedWordToSignedByte (SRC1[159:144]);
///     TMP_DEST[151:144] := SaturateSignedWordToSignedByte (SRC1[175:160]);
///     TMP_DEST[159:152] := SaturateSignedWordToSignedByte (SRC1[191:176]);
///     TMP_DEST[175:168] := SaturateSignedWordToSignedByte (SRC1[223:208]);
///     TMP_DEST[183:176] := SaturateSignedWordToSignedByte (SRC1[239:224]);
///     TMP_DEST[191:184] := SaturateSignedWordToSignedByte (SRC1[255:240]);
///     TMP_DEST[199:192] := SaturateSignedWordToSignedByte (SRC2[143:128]);
///     TMP_DEST[207:200] := SaturateSignedWordToSignedByte (SRC2[159:144]);
///     TMP_DEST[215:208] := SaturateSignedWordToSignedByte (SRC2[175:160]);
///     TMP_DEST[223:216] := SaturateSignedWordToSignedByte (SRC2[191:176]);
///     TMP_DEST[231:224] := SaturateSignedWordToSignedByte (SRC2[207:192]);
///     TMP_DEST[239:232] := SaturateSignedWordToSignedByte (SRC2[223:208]);
///     TMP_DEST[247:240] := SaturateSignedWordToSignedByte (SRC2[239:224]);
///     TMP_DEST[255:248] := SaturateSignedWordToSignedByte (SRC2[255:240]);
/// FI;
/// IF VL >= 512
///     TMP_DEST[263:256] := SaturateSignedWordToSignedByte (SRC1[271:256]);
///     TMP_DEST[271:264] := SaturateSignedWordToSignedByte (SRC1[287:272]);
///     TMP_DEST[279:272] := SaturateSignedWordToSignedByte (SRC1[303:288]);
///     TMP_DEST[287:280] := SaturateSignedWordToSignedByte (SRC1[319:304]);
///     TMP_DEST[295:288] := SaturateSignedWordToSignedByte (SRC1[335:320]);
///     TMP_DEST[303:296] := SaturateSignedWordToSignedByte (SRC1[351:336]);
///     TMP_DEST[311:304] := SaturateSignedWordToSignedByte (SRC1[367:352]);
///     TMP_DEST[319:312] := SaturateSignedWordToSignedByte (SRC1[383:368]);
///     TMP_DEST[327:320] := SaturateSignedWordToSignedByte (SRC2[271:256]);
///     TMP_DEST[335:328] := SaturateSignedWordToSignedByte (SRC2[287:272]);
///     TMP_DEST[343:336] := SaturateSignedWordToSignedByte (SRC2[303:288]);
///     TMP_DEST[351:344] := SaturateSignedWordToSignedByte (SRC2[319:304]);
///     TMP_DEST[359:352] := SaturateSignedWordToSignedByte (SRC2[335:320]);
///     TMP_DEST[367:360] := SaturateSignedWordToSignedByte (SRC2[351:336]);
///     TMP_DEST[375:368] := SaturateSignedWordToSignedByte (SRC2[367:352]);
///     TMP_DEST[383:376] := SaturateSignedWordToSignedByte (SRC2[383:368]);
///     TMP_DEST[391:384] := SaturateSignedWordToSignedByte (SRC1[399:384]);
///     TMP_DEST[399:392] := SaturateSignedWordToSignedByte (SRC1[415:400]);
///     TMP_DEST[407:400] := SaturateSignedWordToSignedByte (SRC1[431:416]);
///     TMP_DEST[415:408] := SaturateSignedWordToSignedByte (SRC1[447:432]);
///     TMP_DEST[423:416] := SaturateSignedWordToSignedByte (SRC1[463:448]);
///     TMP_DEST[431:424] := SaturateSignedWordToSignedByte (SRC1[479:464]);
///     TMP_DEST[439:432] := SaturateSignedWordToSignedByte (SRC1[495:480]);
///     TMP_DEST[447:440] := SaturateSignedWordToSignedByte (SRC1[511:496]);
///     TMP_DEST[455:448] := SaturateSignedWordToSignedByte (SRC2[399:384]);
///     TMP_DEST[463:456] := SaturateSignedWordToSignedByte (SRC2[415:400]);
///     TMP_DEST[471:464] := SaturateSignedWordToSignedByte (SRC2[431:416]);
///     TMP_DEST[479:472] := SaturateSignedWordToSignedByte (SRC2[447:432]);
///     TMP_DEST[487:480] := SaturateSignedWordToSignedByte (SRC2[463:448]);
///     TMP_DEST[495:488] := SaturateSignedWordToSignedByte (SRC2[479:464]);
///     TMP_DEST[503:496] := SaturateSignedWordToSignedByte (SRC2[495:480]);
///     TMP_DEST[511:504] := SaturateSignedWordToSignedByte (SRC2[511:496]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPACKSSDW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO ((KL/2) - 1)
///     i := j * 32
///     IF (EVEX.b == 1) AND (SRC2 *is memory*)
///         THEN
///             TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE
///             TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// TMP_DEST[15:0] := SaturateSignedDwordToSignedWord (SRC1[31:0]);
/// TMP_DEST[31:16] := SaturateSignedDwordToSignedWord (SRC1[63:32]);
/// TMP_DEST[47:32] := SaturateSignedDwordToSignedWord (SRC1[95:64]);
/// TMP_DEST[63:48] := SaturateSignedDwordToSignedWord (SRC1[127:96]);
/// TMP_DEST[79:64] := SaturateSignedDwordToSignedWord (TMP_SRC2[31:0]);
/// TMP_DEST[95:80] := SaturateSignedDwordToSignedWord (TMP_SRC2[63:32]);
/// TMP_DEST[111:96] := SaturateSignedDwordToSignedWord (TMP_SRC2[95:64]);
/// TMP_DEST[127:112] := SaturateSignedDwordToSignedWord (TMP_SRC2[127:96]);
/// IF VL >= 256
///     TMP_DEST[143:128] := SaturateSignedDwordToSignedWord (SRC1[159:128]);
///     TMP_DEST[159:144] := SaturateSignedDwordToSignedWord (SRC1[191:160]);
///     TMP_DEST[175:160] := SaturateSignedDwordToSignedWord (SRC1[223:192]);
///     TMP_DEST[191:176] := SaturateSignedDwordToSignedWord (SRC1[255:224]);
///     TMP_DEST[207:192] := SaturateSignedDwordToSignedWord (TMP_SRC2[159:128]);
///     TMP_DEST[223:208] := SaturateSignedDwordToSignedWord (TMP_SRC2[191:160]);
///     TMP_DEST[239:224] := SaturateSignedDwordToSignedWord (TMP_SRC2[223:192]);
///     TMP_DEST[255:240] := SaturateSignedDwordToSignedWord (TMP_SRC2[255:224]);
/// FI;
/// IF VL >= 512
///     TMP_DEST[271:256] := SaturateSignedDwordToSignedWord (SRC1[287:256]);
///     TMP_DEST[287:272] := SaturateSignedDwordToSignedWord (SRC1[319:288]);
///     TMP_DEST[303:288] := SaturateSignedDwordToSignedWord (SRC1[351:320]);
///     TMP_DEST[319:304] := SaturateSignedDwordToSignedWord (SRC1[383:352]);
///     TMP_DEST[335:320] := SaturateSignedDwordToSignedWord (TMP_SRC2[287:256]);
///     TMP_DEST[351:336] := SaturateSignedDwordToSignedWord (TMP_SRC2[319:288]);
///     TMP_DEST[367:352] := SaturateSignedDwordToSignedWord (TMP_SRC2[351:320]);
///     TMP_DEST[383:368] := SaturateSignedDwordToSignedWord (TMP_SRC2[383:352]);
///     TMP_DEST[399:384] := SaturateSignedDwordToSignedWord (SRC1[415:384]);
///     TMP_DEST[415:400] := SaturateSignedDwordToSignedWord (SRC1[447:416]);
///     TMP_DEST[447:432] := SaturateSignedDwordToSignedWord (SRC1[511:480]);
///     TMP_DEST[463:448] := SaturateSignedDwordToSignedWord (TMP_SRC2[415:384]);
///     TMP_DEST[479:464] := SaturateSignedDwordToSignedWord (TMP_SRC2[447:416]);
///     TMP_DEST[495:480] := SaturateSignedDwordToSignedWord (TMP_SRC2[479:448]);
///     TMP_DEST[511:496] := SaturateSignedDwordToSignedWord (TMP_SRC2[511:480]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn packsswb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PACKUSDW (Legacy SSE Instruction)
/// TMP[15:0] := (DEST[31:0] < 0) ? 0 : DEST[15:0];
/// DEST[15:0] := (DEST[31:0] > FFFFH) ? FFFFH : TMP[15:0] ;
/// TMP[31:16] := (DEST[63:32] < 0) ? 0 : DEST[47:32];
/// DEST[31:16] := (DEST[63:32] > FFFFH) ? FFFFH : TMP[31:16] ;
/// TMP[47:32] := (DEST[95:64] < 0) ? 0 : DEST[79:64];
/// DEST[47:32] := (DEST[95:64] > FFFFH) ? FFFFH : TMP[47:32] ;
/// TMP[63:48] := (DEST[127:96] < 0) ? 0 : DEST[111:96];
/// DEST[63:48] := (DEST[127:96] > FFFFH) ? FFFFH : TMP[63:48] ;
/// TMP[79:64] := (SRC[31:0] < 0) ? 0 : SRC[15:0];
/// DEST[79:64] := (SRC[31:0] > FFFFH) ? FFFFH : TMP[79:64] ;
/// TMP[95:80] := (SRC[63:32] < 0) ? 0 : SRC[47:32];
/// DEST[95:80] := (SRC[63:32] > FFFFH) ? FFFFH : TMP[95:80] ;
/// TMP[111:96] := (SRC[95:64] < 0) ? 0 : SRC[79:64];
/// DEST[111:96] := (SRC[95:64] > FFFFH) ? FFFFH : TMP[111:96] ;
/// TMP[127:112] := (SRC[127:96] < 0) ? 0 : SRC[111:96];
/// DEST[127:112] := (SRC[127:96] > FFFFH) ? FFFFH : TMP[127:112] ;
/// DEST[MAXVL-1:128] (Unmodified)
/// PACKUSDW (VEX.128 Encoded Version)
/// TMP[15:0] := (SRC1[31:0] < 0) ? 0 : SRC1[15:0];
/// DEST[15:0] := (SRC1[31:0] > FFFFH) ? FFFFH : TMP[15:0] ;
/// TMP[31:16] := (SRC1[63:32] < 0) ? 0 : SRC1[47:32];
/// DEST[31:16] := (SRC1[63:32] > FFFFH) ? FFFFH : TMP[31:16] ;
/// TMP[47:32] := (SRC1[95:64] < 0) ? 0 : SRC1[79:64];
/// DEST[47:32] := (SRC1[95:64] > FFFFH) ? FFFFH : TMP[47:32] ;
/// TMP[63:48] := (SRC1[127:96] < 0) ? 0 : SRC1[111:96];
/// DEST[63:48] := (SRC1[127:96] > FFFFH) ? FFFFH : TMP[63:48] ;
/// TMP[79:64] := (SRC2[31:0] < 0) ? 0 : SRC2[15:0];
/// DEST[79:64] := (SRC2[31:0] > FFFFH) ? FFFFH : TMP[79:64] ;
/// TMP[95:80] := (SRC2[63:32] < 0) ? 0 : SRC2[47:32];
/// DEST[95:80] := (SRC2[63:32] > FFFFH) ? FFFFH : TMP[95:80] ;
/// TMP[111:96] := (SRC2[95:64] < 0) ? 0 : SRC2[79:64];
/// DEST[111:96] := (SRC2[95:64] > FFFFH) ? FFFFH : TMP[111:96] ;
/// TMP[127:112] := (SRC2[127:96] < 0) ? 0 : SRC2[111:96];
/// DEST[127:112] := (SRC2[127:96] > FFFFH) ? FFFFH : TMP[127:112];
/// DEST[MAXVL-1:128] := 0;
/// VPACKUSDW (VEX.256 Encoded Version)
/// TMP[15:0] := (SRC1[31:0] < 0) ? 0 : SRC1[15:0];
/// DEST[15:0] := (SRC1[31:0] > FFFFH) ? FFFFH : TMP[15:0] ;
/// TMP[31:16] := (SRC1[63:32] < 0) ? 0 : SRC1[47:32];
/// DEST[31:16] := (SRC1[63:32] > FFFFH) ? FFFFH : TMP[31:16] ;
/// TMP[47:32] := (SRC1[95:64] < 0) ? 0 : SRC1[79:64];
/// DEST[47:32] := (SRC1[95:64] > FFFFH) ? FFFFH : TMP[47:32] ;
/// TMP[63:48] := (SRC1[127:96] < 0) ? 0 : SRC1[111:96];
/// DEST[63:48] := (SRC1[127:96] > FFFFH) ? FFFFH : TMP[63:48] ;
/// TMP[79:64] := (SRC2[31:0] < 0) ? 0 : SRC2[15:0];
/// TMP[95:80] := (SRC2[63:32] < 0) ? 0 : SRC2[47:32];
/// DEST[95:80] := (SRC2[63:32] > FFFFH) ? FFFFH : TMP[95:80] ;
/// TMP[111:96] := (SRC2[95:64] < 0) ? 0 : SRC2[79:64];
/// DEST[111:96] := (SRC2[95:64] > FFFFH) ? FFFFH : TMP[111:96] ;
/// TMP[127:112] := (SRC2[127:96] < 0) ? 0 : SRC2[111:96];
/// DEST[127:112] := (SRC2[127:96] > FFFFH) ? FFFFH : TMP[127:112] ;
/// TMP[143:128] := (SRC1[159:128] < 0) ? 0 : SRC1[143:128];
/// DEST[143:128] := (SRC1[159:128] > FFFFH) ? FFFFH : TMP[143:128] ;
/// TMP[159:144] := (SRC1[191:160] < 0) ? 0 : SRC1[175:160];
/// DEST[159:144] := (SRC1[191:160] > FFFFH) ? FFFFH : TMP[159:144] ;
/// TMP[175:160] := (SRC1[223:192] < 0) ? 0 : SRC1[207:192];
/// DEST[175:160] := (SRC1[223:192] > FFFFH) ? FFFFH : TMP[175:160] ;
/// TMP[191:176] := (SRC1[255:224] < 0) ? 0 : SRC1[239:224];
/// DEST[191:176] := (SRC1[255:224] > FFFFH) ? FFFFH : TMP[191:176] ;
/// TMP[207:192] := (SRC2[159:128] < 0) ? 0 : SRC2[143:128];
/// DEST[207:192] := (SRC2[159:128] > FFFFH) ? FFFFH : TMP[207:192] ;
/// TMP[223:208] := (SRC2[191:160] < 0) ? 0 : SRC2[175:160];
/// DEST[223:208] := (SRC2[191:160] > FFFFH) ? FFFFH : TMP[223:208] ;
/// TMP[239:224] := (SRC2[223:192] < 0) ? 0 : SRC2[207:192];
/// DEST[239:224] := (SRC2[223:192] > FFFFH) ? FFFFH : TMP[239:224] ;
/// TMP[255:240] := (SRC2[255:224] < 0) ? 0 : SRC2[239:224];
/// DEST[255:240] := (SRC2[255:224] > FFFFH) ? FFFFH : TMP[255:240] ;
/// DEST[MAXVL-1:256] := 0;
/// VPACKUSDW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO ((KL/2) - 1)
///     i := j * 32
///     IF (EVEX.b == 1) AND (SRC2 *is memory*)
///         THEN
///             TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE
///             TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// TMP[15:0] := (SRC1[31:0] < 0) ? 0 : SRC1[15:0];
/// DEST[15:0] := (SRC1[31:0] > FFFFH) ? FFFFH : TMP[15:0] ;
/// TMP[31:16] := (SRC1[63:32] < 0) ? 0 : SRC1[47:32];
/// DEST[31:16] := (SRC1[63:32] > FFFFH) ? FFFFH : TMP[31:16] ;
/// TMP[47:32] := (SRC1[95:64] < 0) ? 0 : SRC1[79:64];
/// DEST[47:32] := (SRC1[95:64] > FFFFH) ? FFFFH : TMP[47:32] ;
/// TMP[63:48] := (SRC1[127:96] < 0) ? 0 : SRC1[111:96];
/// DEST[63:48] := (SRC1[127:96] > FFFFH) ? FFFFH : TMP[63:48] ;
/// TMP[79:64] := (TMP_SRC2[31:0] < 0) ? 0 : TMP_SRC2[15:0];
/// DEST[79:64] := (TMP_SRC2[31:0] > FFFFH) ? FFFFH : TMP[79:64] ;
/// TMP[95:80] := (TMP_SRC2[63:32] < 0) ? 0 : TMP_SRC2[47:32];
/// DEST[95:80] := (TMP_SRC2[63:32] > FFFFH) ? FFFFH : TMP[95:80] ;
/// TMP[111:96] := (TMP_SRC2[95:64] < 0) ? 0 : TMP_SRC2[79:64];
/// DEST[111:96] := (TMP_SRC2[95:64] > FFFFH) ? FFFFH : TMP[111:96] ;
/// TMP[127:112] := (TMP_SRC2[127:96] < 0) ? 0 : TMP_SRC2[111:96];
/// DEST[127:112] := (TMP_SRC2[127:96] > FFFFH) ? FFFFH : TMP[127:112] ;
///     TMP[143:128] := (SRC1[159:128] < 0) ? 0 : SRC1[143:128];
///     DEST[143:128] := (SRC1[159:128] > FFFFH) ? FFFFH : TMP[143:128] ;
///     TMP[159:144] := (SRC1[191:160] < 0) ? 0 : SRC1[175:160];
///     DEST[159:144] := (SRC1[191:160] > FFFFH) ? FFFFH : TMP[159:144] ;
///     TMP[175:160] := (SRC1[223:192] < 0) ? 0 : SRC1[207:192];
///     DEST[175:160] := (SRC1[223:192] > FFFFH) ? FFFFH : TMP[175:160] ;
///     TMP[191:176] := (SRC1[255:224] < 0) ? 0 : SRC1[239:224];
///     DEST[191:176] := (SRC1[255:224] > FFFFH) ? FFFFH : TMP[191:176] ;
///     TMP[207:192] := (TMP_SRC2[159:128] < 0) ? 0 : TMP_SRC2[143:128];
///     DEST[207:192] := (TMP_SRC2[159:128] > FFFFH) ? FFFFH : TMP[207:192] ;
///     TMP[223:208] := (TMP_SRC2[191:160] < 0) ? 0 : TMP_SRC2[175:160];
///     DEST[223:208] := (TMP_SRC2[191:160] > FFFFH) ? FFFFH : TMP[223:208] ;
///     TMP[239:224] := (TMP_SRC2[223:192] < 0) ? 0 : TMP_SRC2[207:192];
///     DEST[239:224] := (TMP_SRC2[223:192] > FFFFH) ? FFFFH : TMP[239:224] ;
///     TMP[255:240] := (TMP_SRC2[255:224] < 0) ? 0 : TMP_SRC2[239:224];
///     DEST[255:240] := (TMP_SRC2[255:224] > FFFFH) ? FFFFH : TMP[255:240] ;
/// FI;
/// IF VL >= 512
///     TMP[271:256] := (SRC1[287:256] < 0) ? 0 : SRC1[271:256];
///     DEST[271:256] := (SRC1[287:256] > FFFFH) ? FFFFH : TMP[271:256] ;
///     TMP[287:272] := (SRC1[319:288] < 0) ? 0 : SRC1[303:288];
///     DEST[287:272] := (SRC1[319:288] > FFFFH) ? FFFFH : TMP[287:272] ;
///     TMP[303:288] := (SRC1[351:320] < 0) ? 0 : SRC1[335:320];
///     DEST[303:288] := (SRC1[351:320] > FFFFH) ? FFFFH : TMP[303:288] ;
///     TMP[319:304] := (SRC1[383:352] < 0) ? 0 : SRC1[367:352];
///     DEST[319:304] := (SRC1[383:352] > FFFFH) ? FFFFH : TMP[319:304] ;
///     TMP[335:320] := (TMP_SRC2[287:256] < 0) ? 0 : TMP_SRC2[271:256];
///     DEST[335:304] := (TMP_SRC2[287:256] > FFFFH) ? FFFFH : TMP[79:64] ;
///     TMP[351:336] := (TMP_SRC2[319:288] < 0) ? 0 : TMP_SRC2[303:288];
///     DEST[351:336] := (TMP_SRC2[319:288] > FFFFH) ? FFFFH : TMP[351:336] ;
///     TMP[367:352] := (TMP_SRC2[351:320] < 0) ? 0 : TMP_SRC2[315:320];
///     DEST[367:352] := (TMP_SRC2[351:320] > FFFFH) ? FFFFH : TMP[367:352] ;
///     TMP[383:368] := (TMP_SRC2[383:352] < 0) ? 0 : TMP_SRC2[367:352];
///     DEST[383:368] := (TMP_SRC2[383:352] > FFFFH) ? FFFFH : TMP[383:368] ;
///     TMP[399:384] := (SRC1[415:384] < 0) ? 0 : SRC1[399:384];
///     DEST[399:384] := (SRC1[415:384] > FFFFH) ? FFFFH : TMP[399:384] ;
///     TMP[415:400] := (SRC1[447:416] < 0) ? 0 : SRC1[431:416];
///     DEST[415:400] := (SRC1[447:416] > FFFFH) ? FFFFH : TMP[415:400] ;
///     TMP[431:416] := (SRC1[479:448] < 0) ? 0 : SRC1[463:448];
///     DEST[431:416] := (SRC1[479:448] > FFFFH) ? FFFFH : TMP[431:416] ;
///     TMP[447:432] := (SRC1[511:480] < 0) ? 0 : SRC1[495:480];
///     DEST[447:432] := (SRC1[511:480] > FFFFH) ? FFFFH : TMP[447:432] ;
///     TMP[463:448] := (TMP_SRC2[415:384] < 0) ? 0 : TMP_SRC2[399:384];
///     DEST[463:448] := (TMP_SRC2[415:384] > FFFFH) ? FFFFH : TMP[463:448] ;
///     TMP[475:464] := (TMP_SRC2[447:416] < 0) ? 0 : TMP_SRC2[431:416];
///     DEST[475:464] := (TMP_SRC2[447:416] > FFFFH) ? FFFFH : TMP[475:464] ;
///     TMP[491:476] := (TMP_SRC2[479:448] < 0) ? 0 : TMP_SRC2[463:448];
///     DEST[491:476] := (TMP_SRC2[479:448] > FFFFH) ? FFFFH : TMP[491:476] ;
///     TMP[511:492] := (TMP_SRC2[511:480] < 0) ? 0 : TMP_SRC2[495:480];
///     DEST[511:492] := (TMP_SRC2[511:480] > FFFFH) ? FFFFH : TMP[511:492] ;
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN
///             DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn packusdw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PACKUSWB (With 64-bit Operands)
///     DEST[7:0] := SaturateSignedWordToUnsignedByte DEST[15:0];
///     DEST[15:8] := SaturateSignedWordToUnsignedByte DEST[31:16];
///     DEST[23:16] := SaturateSignedWordToUnsignedByte DEST[47:32];
///     DEST[31:24] := SaturateSignedWordToUnsignedByte DEST[63:48];
///     DEST[39:32] := SaturateSignedWordToUnsignedByte SRC[15:0];
///     DEST[47:40] := SaturateSignedWordToUnsignedByte SRC[31:16];
///     DEST[55:48] := SaturateSignedWordToUnsignedByte SRC[47:32];
///     DEST[63:56] := SaturateSignedWordToUnsignedByte SRC[63:48];
/// PACKUSWB (Legacy SSE Instruction)
///     DEST[7:0] := SaturateSignedWordToUnsignedByte (DEST[15:0]);
///     DEST[15:8] := SaturateSignedWordToUnsignedByte (DEST[31:16]);
///     DEST[23:16] := SaturateSignedWordToUnsignedByte (DEST[47:32]);
///     DEST[31:24] := SaturateSignedWordToUnsignedByte (DEST[63:48]);
///     DEST[39:32] := SaturateSignedWordToUnsignedByte (DEST[79:64]);
///     DEST[47:40] := SaturateSignedWordToUnsignedByte (DEST[95:80]);
///     DEST[55:48] := SaturateSignedWordToUnsignedByte (DEST[111:96]);
///     DEST[63:56] := SaturateSignedWordToUnsignedByte (DEST[127:112]);
///     DEST[71:64] := SaturateSignedWordToUnsignedByte (SRC[15:0]);
///     DEST[79:72] := SaturateSignedWordToUnsignedByte (SRC[31:16]);
///     DEST[87:80] := SaturateSignedWordToUnsignedByte (SRC[47:32]);
///     DEST[95:88] := SaturateSignedWordToUnsignedByte (SRC[63:48]);
///     DEST[103:96] := SaturateSignedWordToUnsignedByte (SRC[79:64]);
///     DEST[111:104] := SaturateSignedWordToUnsignedByte (SRC[95:80]);
///     DEST[119:112] := SaturateSignedWordToUnsignedByte (SRC[111:96]);
///     DEST[127:120] := SaturateSignedWordToUnsignedByte (SRC[127:112]);
/// PACKUSWB (VEX.128 Encoded Version)
///     DEST[7:0] := SaturateSignedWordToUnsignedByte (SRC1[15:0]);
///     DEST[15:8] := SaturateSignedWordToUnsignedByte (SRC1[31:16]);
///     DEST[23:16] := SaturateSignedWordToUnsignedByte (SRC1[47:32]);
///     DEST[31:24] := SaturateSignedWordToUnsignedByte (SRC1[63:48]);
///     DEST[39:32] := SaturateSignedWordToUnsignedByte (SRC1[79:64]);
///     DEST[47:40] := SaturateSignedWordToUnsignedByte (SRC1[95:80]);
///     DEST[55:48] := SaturateSignedWordToUnsignedByte (SRC1[111:96]);
///     DEST[63:56] := SaturateSignedWordToUnsignedByte (SRC1[127:112]);
///     DEST[71:64] := SaturateSignedWordToUnsignedByte (SRC2[15:0]);
///     DEST[79:72] := SaturateSignedWordToUnsignedByte (SRC2[31:16]);
///     DEST[87:80] := SaturateSignedWordToUnsignedByte (SRC2[47:32]);
///     DEST[95:88] := SaturateSignedWordToUnsignedByte (SRC2[63:48]);
///     DEST[103:96] := SaturateSignedWordToUnsignedByte (SRC2[79:64]);
///     DEST[119:112] := SaturateSignedWordToUnsignedByte (SRC2[111:96]);
///     DEST[127:120] := SaturateSignedWordToUnsignedByte (SRC2[127:112]);
///     DEST[MAXVL-1:128] := 0;
/// VPACKUSWB (VEX.256 Encoded Version)
///     DEST[7:0] := SaturateSignedWordToUnsignedByte (SRC1[15:0]);
///     DEST[15:8] := SaturateSignedWordToUnsignedByte (SRC1[31:16]);
///     DEST[23:16] := SaturateSignedWordToUnsignedByte (SRC1[47:32]);
///     DEST[31:24] := SaturateSignedWordToUnsignedByte (SRC1[63:48]);
///     DEST[39:32] := SaturateSignedWordToUnsignedByte (SRC1[79:64]);
///     DEST[47:40] := SaturateSignedWordToUnsignedByte (SRC1[95:80]);
///     DEST[55:48] := SaturateSignedWordToUnsignedByte (SRC1[111:96]);
///     DEST[63:56] := SaturateSignedWordToUnsignedByte (SRC1[127:112]);
///     DEST[71:64] := SaturateSignedWordToUnsignedByte (SRC2[15:0]);
///     DEST[79:72] := SaturateSignedWordToUnsignedByte (SRC2[31:16]);
///     DEST[87:80] := SaturateSignedWordToUnsignedByte (SRC2[47:32]);
///     DEST[95:88] := SaturateSignedWordToUnsignedByte (SRC2[63:48]);
///     DEST[103:96] := SaturateSignedWordToUnsignedByte (SRC2[79:64]);
///     DEST[111:104] := SaturateSignedWordToUnsignedByte (SRC2[95:80]);
///     DEST[119:112] := SaturateSignedWordToUnsignedByte (SRC2[111:96]);
///     DEST[127:120] := SaturateSignedWordToUnsignedByte (SRC2[127:112]);
///     DEST[135:128] := SaturateSignedWordToUnsignedByte (SRC1[143:128]);
///     DEST[143:136] := SaturateSignedWordToUnsignedByte (SRC1[159:144]);
///     DEST[151:144] := SaturateSignedWordToUnsignedByte (SRC1[175:160]);
///     DEST[159:152] := SaturateSignedWordToUnsignedByte (SRC1[191:176]);
///     DEST[167:160] := SaturateSignedWordToUnsignedByte (SRC1[207:192]);
///     DEST[175:168] := SaturateSignedWordToUnsignedByte (SRC1[223:208]);
///     DEST[183:176] := SaturateSignedWordToUnsignedByte (SRC1[239:224]);
///     DEST[191:184] := SaturateSignedWordToUnsignedByte (SRC1[255:240]);
///     DEST[199:192] := SaturateSignedWordToUnsignedByte (SRC2[143:128]);
///     DEST[207:200] := SaturateSignedWordToUnsignedByte (SRC2[159:144]);
///     DEST[215:208] := SaturateSignedWordToUnsignedByte (SRC2[175:160]);
///     DEST[223:216] := SaturateSignedWordToUnsignedByte (SRC2[191:176]);
///     DEST[231:224] := SaturateSignedWordToUnsignedByte (SRC2[207:192]);
///     DEST[239:232] := SaturateSignedWordToUnsignedByte (SRC2[223:208]);
///     DEST[247:240] := SaturateSignedWordToUnsignedByte (SRC2[239:224]);
///     DEST[255:248] := SaturateSignedWordToUnsignedByte (SRC2[255:240]);
/// VPACKUSWB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// TMP_DEST[7:0] := SaturateSignedWordToUnsignedByte (SRC1[15:0]);
/// TMP_DEST[15:8] := SaturateSignedWordToUnsignedByte (SRC1[31:16]);
/// TMP_DEST[23:16] := SaturateSignedWordToUnsignedByte (SRC1[47:32]);
/// TMP_DEST[31:24] := SaturateSignedWordToUnsignedByte (SRC1[63:48]);
/// TMP_DEST[39:32] := SaturateSignedWordToUnsignedByte (SRC1[79:64]);
/// TMP_DEST[47:40] := SaturateSignedWordToUnsignedByte (SRC1[95:80]);
/// TMP_DEST[55:48] := SaturateSignedWordToUnsignedByte (SRC1[111:96]);
/// TMP_DEST[63:56] := SaturateSignedWordToUnsignedByte (SRC1[127:112]);
/// TMP_DEST[71:64] := SaturateSignedWordToUnsignedByte (SRC2[15:0]);
/// TMP_DEST[79:72] := SaturateSignedWordToUnsignedByte (SRC2[31:16]);
/// TMP_DEST[87:80] := SaturateSignedWordToUnsignedByte (SRC2[47:32]);
/// TMP_DEST[95:88] := SaturateSignedWordToUnsignedByte (SRC2[63:48]);
/// TMP_DEST[103:96] := SaturateSignedWordToUnsignedByte (SRC2[79:64]);
/// TMP_DEST[119:112] := SaturateSignedWordToUnsignedByte (SRC2[111:96]);
/// TMP_DEST[127:120] := SaturateSignedWordToUnsignedByte (SRC2[127:112]);
/// IF VL >= 256
///     TMP_DEST[135:128] := SaturateSignedWordToUnsignedByte (SRC1[143:128]);
///     TMP_DEST[143:136] := SaturateSignedWordToUnsignedByte (SRC1[159:144]);
///     TMP_DEST[151:144] := SaturateSignedWordToUnsignedByte (SRC1[175:160]);
///     TMP_DEST[159:152] := SaturateSignedWordToUnsignedByte (SRC1[191:176]);
///     TMP_DEST[167:160] := SaturateSignedWordToUnsignedByte (SRC1[207:192]);
///     TMP_DEST[175:168] := SaturateSignedWordToUnsignedByte (SRC1[223:208]);
///     TMP_DEST[183:176] := SaturateSignedWordToUnsignedByte (SRC1[239:224]);
///     TMP_DEST[191:184] := SaturateSignedWordToUnsignedByte (SRC1[255:240]);
///     TMP_DEST[199:192] := SaturateSignedWordToUnsignedByte (SRC2[143:128]);
///     TMP_DEST[207:200] := SaturateSignedWordToUnsignedByte (SRC2[159:144]);
///     TMP_DEST[215:208] := SaturateSignedWordToUnsignedByte (SRC2[175:160]);
///     TMP_DEST[223:216] := SaturateSignedWordToUnsignedByte (SRC2[191:176]);
///     TMP_DEST[231:224] := SaturateSignedWordToUnsignedByte (SRC2[207:192]);
///     TMP_DEST[239:232] := SaturateSignedWordToUnsignedByte (SRC2[223:208]);
///     TMP_DEST[247:240] := SaturateSignedWordToUnsignedByte (SRC2[239:224]);
///     TMP_DEST[255:248] := SaturateSignedWordToUnsignedByte (SRC2[255:240]);
/// FI;
/// IF VL >= 512
///     TMP_DEST[263:256] := SaturateSignedWordToUnsignedByte (SRC1[271:256]);
///     TMP_DEST[271:264] := SaturateSignedWordToUnsignedByte (SRC1[287:272]);
///     TMP_DEST[279:272] := SaturateSignedWordToUnsignedByte (SRC1[303:288]);
///     TMP_DEST[287:280] := SaturateSignedWordToUnsignedByte (SRC1[319:304]);
///     TMP_DEST[295:288] := SaturateSignedWordToUnsignedByte (SRC1[335:320]);
///     TMP_DEST[303:296] := SaturateSignedWordToUnsignedByte (SRC1[351:336]);
///     TMP_DEST[311:304] := SaturateSignedWordToUnsignedByte (SRC1[367:352]);
///     TMP_DEST[319:312] := SaturateSignedWordToUnsignedByte (SRC1[383:368]);
///     TMP_DEST[327:320] := SaturateSignedWordToUnsignedByte (SRC2[271:256]);
///     TMP_DEST[335:328] := SaturateSignedWordToUnsignedByte (SRC2[287:272]);
///     TMP_DEST[343:336] := SaturateSignedWordToUnsignedByte (SRC2[303:288]);
///     TMP_DEST[351:344] := SaturateSignedWordToUnsignedByte (SRC2[319:304]);
///     TMP_DEST[359:352] := SaturateSignedWordToUnsignedByte (SRC2[335:320]);
///     TMP_DEST[367:360] := SaturateSignedWordToUnsignedByte (SRC2[351:336]);
///     TMP_DEST[375:368] := SaturateSignedWordToUnsignedByte (SRC2[367:352]);
///     TMP_DEST[383:376] := SaturateSignedWordToUnsignedByte (SRC2[383:368]);
///     TMP_DEST[391:384] := SaturateSignedWordToUnsignedByte (SRC1[399:384]);
///     TMP_DEST[399:392] := SaturateSignedWordToUnsignedByte (SRC1[415:400]);
///     TMP_DEST[407:400] := SaturateSignedWordToUnsignedByte (SRC1[431:416]);
///     TMP_DEST[415:408] := SaturateSignedWordToUnsignedByte (SRC1[447:432]);
///     TMP_DEST[423:416] := SaturateSignedWordToUnsignedByte (SRC1[463:448]);
///     TMP_DEST[431:424] := SaturateSignedWordToUnsignedByte (SRC1[479:464]);
///     TMP_DEST[439:432] := SaturateSignedWordToUnsignedByte (SRC1[495:480]);
///     TMP_DEST[447:440] := SaturateSignedWordToUnsignedByte (SRC1[511:496]);
///     TMP_DEST[455:448] := SaturateSignedWordToUnsignedByte (SRC2[399:384]);
///     TMP_DEST[463:456] := SaturateSignedWordToUnsignedByte (SRC2[415:400]);
///     TMP_DEST[471:464] := SaturateSignedWordToUnsignedByte (SRC2[431:416]);
///     TMP_DEST[479:472] := SaturateSignedWordToUnsignedByte (SRC2[447:432]);
///     TMP_DEST[487:480] := SaturateSignedWordToUnsignedByte (SRC2[463:448]);
///     TMP_DEST[503:496] := SaturateSignedWordToUnsignedByte (SRC2[495:480]);
///     TMP_DEST[511:504] := SaturateSignedWordToUnsignedByte (SRC2[511:496]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN
///             DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn packuswb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PADDB (With 64-bit Operands)
///     DEST[7:0] := DEST[7:0] + SRC[7:0];
///     (* Repeat add operation for 2nd through 7th byte *)
///     DEST[63:56] := DEST[63:56] + SRC[63:56];
/// PADDW (With 64-bit Operands)
///     DEST[15:0] := DEST[15:0] + SRC[15:0];
///     (* Repeat add operation for 2nd and 3th word *)
///     DEST[63:48] := DEST[63:48] + SRC[63:48];
/// PADDD (With 64-bit Operands)
///     DEST[31:0] := DEST[31:0] + SRC[31:0];
///     DEST[63:32] := DEST[63:32] + SRC[63:32];
/// PADDQ (With 64-Bit Operands)
///     DEST[63:0] := DEST[63:0] + SRC[63:0];
/// PADDB (Legacy SSE Instruction)
///     DEST[7:0] := DEST[7:0] + SRC[7:0];
///     (* Repeat add operation for 2nd through 15th byte *)
///     DEST[127:120] := DEST[127:120] + SRC[127:120];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDW (Legacy SSE Instruction)
///     DEST[15:0] := DEST[15:0] + SRC[15:0];
///     (* Repeat add operation for 2nd through 7th word *)
///     DEST[127:112] := DEST[127:112] + SRC[127:112];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDD (Legacy SSE Instruction)
///     DEST[31:0] := DEST[31:0] + SRC[31:0];
///     (* Repeat add operation for 2nd and 3th doubleword *)
///     DEST[127:96] := DEST[127:96] + SRC[127:96];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDQ (Legacy SSE Instruction)
///     DEST[63:0] := DEST[63:0] + SRC[63:0];
///     DEST[127:64] := DEST[127:64] + SRC[127:64];
///     DEST[MAXVL-1:128] (Unmodified)
/// VPADDB (VEX.128 Encoded Instruction)
///     DEST[7:0] := SRC1[7:0] + SRC2[7:0];
///     (* Repeat add operation for 2nd through 15th byte *)
///     DEST[127:120] := SRC1[127:120] + SRC2[127:120];
///     DEST[MAXVL-1:128] := 0;
/// VPADDW (VEX.128 Encoded Instruction)
///     DEST[15:0] := SRC1[15:0] + SRC2[15:0];
///     (* Repeat add operation for 2nd through 7th word *)
///     DEST[127:112] := SRC1[127:112] + SRC2[127:112];
///     DEST[MAXVL-1:128] := 0;
/// VPADDD (VEX.128 Encoded Instruction)
///     DEST[31:0] := SRC1[31:0] + SRC2[31:0];
///     (* Repeat add operation for 2nd and 3th doubleword *)
///     DEST[127:96] := SRC1[127:96] + SRC2[127:96];
///     DEST[MAXVL-1:128] := 0;
/// VPADDQ (VEX.128 Encoded Instruction)
///     DEST[63:0] := SRC1[63:0] + SRC2[63:0];
///     DEST[127:64] := SRC1[127:64] + SRC2[127:64];
///     DEST[MAXVL-1:128] := 0;
/// VPADDB (VEX.256 Encoded Instruction)
///     DEST[7:0] := SRC1[7:0] + SRC2[7:0];
///     (* Repeat add operation for 2nd through 31th byte *)
///     DEST[255:248] := SRC1[255:248] + SRC2[255:248];
/// VPADDW (VEX.256 Encoded Instruction)
///     DEST[15:0] := SRC1[15:0] + SRC2[15:0];
///     (* Repeat add operation for 2nd through 15th word *)
///     DEST[255:240] := SRC1[255:240] + SRC2[255:240];
/// VPADDD (VEX.256 Encoded Instruction)
///     DEST[31:0] := SRC1[31:0] + SRC2[31:0];
///     (* Repeat add operation for 2nd and 7th doubleword *)
///     DEST[255:224] := SRC1[255:224] + SRC2[255:224];
/// VPADDQ (VEX.256 Encoded Instruction)
///     DEST[63:0] := SRC1[63:0] + SRC2[63:0];
///     DEST[127:64] := SRC1[127:64] + SRC2[127:64];
///     DEST[191:128] := SRC1[191:128] + SRC2[191:128];
///     DEST[255:192] := SRC1[255:192] + SRC2[255:192];
/// VPADDB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC1[i+7:i] + SRC2[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC1[i+15:i] + SRC2[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := SRC1[i+31:i] + SRC2[31:0]
///                 ELSE DEST[i+31:i] := SRC1[i+31:i] + SRC2[i+31:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := SRC1[i+63:i] + SRC2[63:0]
///                 ELSE DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn paddb() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PADDB (With 64-bit Operands)
///     DEST[7:0] := DEST[7:0] + SRC[7:0];
///     (* Repeat add operation for 2nd through 7th byte *)
///     DEST[63:56] := DEST[63:56] + SRC[63:56];
/// PADDW (With 64-bit Operands)
///     DEST[15:0] := DEST[15:0] + SRC[15:0];
///     (* Repeat add operation for 2nd and 3th word *)
///     DEST[63:48] := DEST[63:48] + SRC[63:48];
/// PADDD (With 64-bit Operands)
///     DEST[31:0] := DEST[31:0] + SRC[31:0];
///     DEST[63:32] := DEST[63:32] + SRC[63:32];
/// PADDQ (With 64-Bit Operands)
///     DEST[63:0] := DEST[63:0] + SRC[63:0];
/// PADDB (Legacy SSE Instruction)
///     DEST[7:0] := DEST[7:0] + SRC[7:0];
///     (* Repeat add operation for 2nd through 15th byte *)
///     DEST[127:120] := DEST[127:120] + SRC[127:120];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDW (Legacy SSE Instruction)
///     DEST[15:0] := DEST[15:0] + SRC[15:0];
///     (* Repeat add operation for 2nd through 7th word *)
///     DEST[127:112] := DEST[127:112] + SRC[127:112];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDD (Legacy SSE Instruction)
///     DEST[31:0] := DEST[31:0] + SRC[31:0];
///     (* Repeat add operation for 2nd and 3th doubleword *)
///     DEST[127:96] := DEST[127:96] + SRC[127:96];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDQ (Legacy SSE Instruction)
///     DEST[63:0] := DEST[63:0] + SRC[63:0];
///     DEST[127:64] := DEST[127:64] + SRC[127:64];
///     DEST[MAXVL-1:128] (Unmodified)
/// VPADDB (VEX.128 Encoded Instruction)
///     DEST[7:0] := SRC1[7:0] + SRC2[7:0];
///     (* Repeat add operation for 2nd through 15th byte *)
///     DEST[127:120] := SRC1[127:120] + SRC2[127:120];
///     DEST[MAXVL-1:128] := 0;
/// VPADDW (VEX.128 Encoded Instruction)
///     DEST[15:0] := SRC1[15:0] + SRC2[15:0];
///     (* Repeat add operation for 2nd through 7th word *)
///     DEST[127:112] := SRC1[127:112] + SRC2[127:112];
///     DEST[MAXVL-1:128] := 0;
/// VPADDD (VEX.128 Encoded Instruction)
///     DEST[31:0] := SRC1[31:0] + SRC2[31:0];
///     (* Repeat add operation for 2nd and 3th doubleword *)
///     DEST[127:96] := SRC1[127:96] + SRC2[127:96];
///     DEST[MAXVL-1:128] := 0;
/// VPADDQ (VEX.128 Encoded Instruction)
///     DEST[63:0] := SRC1[63:0] + SRC2[63:0];
///     DEST[127:64] := SRC1[127:64] + SRC2[127:64];
///     DEST[MAXVL-1:128] := 0;
/// VPADDB (VEX.256 Encoded Instruction)
///     DEST[7:0] := SRC1[7:0] + SRC2[7:0];
///     (* Repeat add operation for 2nd through 31th byte *)
///     DEST[255:248] := SRC1[255:248] + SRC2[255:248];
/// VPADDW (VEX.256 Encoded Instruction)
///     DEST[15:0] := SRC1[15:0] + SRC2[15:0];
///     (* Repeat add operation for 2nd through 15th word *)
///     DEST[255:240] := SRC1[255:240] + SRC2[255:240];
/// VPADDD (VEX.256 Encoded Instruction)
///     DEST[31:0] := SRC1[31:0] + SRC2[31:0];
///     (* Repeat add operation for 2nd and 7th doubleword *)
///     DEST[255:224] := SRC1[255:224] + SRC2[255:224];
/// VPADDQ (VEX.256 Encoded Instruction)
///     DEST[63:0] := SRC1[63:0] + SRC2[63:0];
///     DEST[127:64] := SRC1[127:64] + SRC2[127:64];
///     DEST[191:128] := SRC1[191:128] + SRC2[191:128];
///     DEST[255:192] := SRC1[255:192] + SRC2[255:192];
/// VPADDB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC1[i+7:i] + SRC2[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC1[i+15:i] + SRC2[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := SRC1[i+31:i] + SRC2[31:0]
///                 ELSE DEST[i+31:i] := SRC1[i+31:i] + SRC2[i+31:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := SRC1[i+63:i] + SRC2[63:0]
///                 ELSE DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn paddd() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PADDB (With 64-bit Operands)
///     DEST[7:0] := DEST[7:0] + SRC[7:0];
///     (* Repeat add operation for 2nd through 7th byte *)
///     DEST[63:56] := DEST[63:56] + SRC[63:56];
/// PADDW (With 64-bit Operands)
///     DEST[15:0] := DEST[15:0] + SRC[15:0];
///     (* Repeat add operation for 2nd and 3th word *)
///     DEST[63:48] := DEST[63:48] + SRC[63:48];
/// PADDD (With 64-bit Operands)
///     DEST[31:0] := DEST[31:0] + SRC[31:0];
///     DEST[63:32] := DEST[63:32] + SRC[63:32];
/// PADDQ (With 64-Bit Operands)
///     DEST[63:0] := DEST[63:0] + SRC[63:0];
/// PADDB (Legacy SSE Instruction)
///     DEST[7:0] := DEST[7:0] + SRC[7:0];
///     (* Repeat add operation for 2nd through 15th byte *)
///     DEST[127:120] := DEST[127:120] + SRC[127:120];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDW (Legacy SSE Instruction)
///     DEST[15:0] := DEST[15:0] + SRC[15:0];
///     (* Repeat add operation for 2nd through 7th word *)
///     DEST[127:112] := DEST[127:112] + SRC[127:112];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDD (Legacy SSE Instruction)
///     DEST[31:0] := DEST[31:0] + SRC[31:0];
///     (* Repeat add operation for 2nd and 3th doubleword *)
///     DEST[127:96] := DEST[127:96] + SRC[127:96];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDQ (Legacy SSE Instruction)
///     DEST[63:0] := DEST[63:0] + SRC[63:0];
///     DEST[127:64] := DEST[127:64] + SRC[127:64];
///     DEST[MAXVL-1:128] (Unmodified)
/// VPADDB (VEX.128 Encoded Instruction)
///     DEST[7:0] := SRC1[7:0] + SRC2[7:0];
///     (* Repeat add operation for 2nd through 15th byte *)
///     DEST[127:120] := SRC1[127:120] + SRC2[127:120];
///     DEST[MAXVL-1:128] := 0;
/// VPADDW (VEX.128 Encoded Instruction)
///     DEST[15:0] := SRC1[15:0] + SRC2[15:0];
///     (* Repeat add operation for 2nd through 7th word *)
///     DEST[127:112] := SRC1[127:112] + SRC2[127:112];
///     DEST[MAXVL-1:128] := 0;
/// VPADDD (VEX.128 Encoded Instruction)
///     DEST[31:0] := SRC1[31:0] + SRC2[31:0];
///     (* Repeat add operation for 2nd and 3th doubleword *)
///     DEST[127:96] := SRC1[127:96] + SRC2[127:96];
///     DEST[MAXVL-1:128] := 0;
/// VPADDQ (VEX.128 Encoded Instruction)
///     DEST[63:0] := SRC1[63:0] + SRC2[63:0];
///     DEST[127:64] := SRC1[127:64] + SRC2[127:64];
///     DEST[MAXVL-1:128] := 0;
/// VPADDB (VEX.256 Encoded Instruction)
///     DEST[7:0] := SRC1[7:0] + SRC2[7:0];
///     (* Repeat add operation for 2nd through 31th byte *)
///     DEST[255:248] := SRC1[255:248] + SRC2[255:248];
/// VPADDW (VEX.256 Encoded Instruction)
///     DEST[15:0] := SRC1[15:0] + SRC2[15:0];
///     (* Repeat add operation for 2nd through 15th word *)
///     DEST[255:240] := SRC1[255:240] + SRC2[255:240];
/// VPADDD (VEX.256 Encoded Instruction)
///     DEST[31:0] := SRC1[31:0] + SRC2[31:0];
///     (* Repeat add operation for 2nd and 7th doubleword *)
///     DEST[255:224] := SRC1[255:224] + SRC2[255:224];
/// VPADDQ (VEX.256 Encoded Instruction)
///     DEST[63:0] := SRC1[63:0] + SRC2[63:0];
///     DEST[127:64] := SRC1[127:64] + SRC2[127:64];
///     DEST[191:128] := SRC1[191:128] + SRC2[191:128];
///     DEST[255:192] := SRC1[255:192] + SRC2[255:192];
/// VPADDB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC1[i+7:i] + SRC2[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC1[i+15:i] + SRC2[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := SRC1[i+31:i] + SRC2[31:0]
///                 ELSE DEST[i+31:i] := SRC1[i+31:i] + SRC2[i+31:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := SRC1[i+63:i] + SRC2[63:0]
///                 ELSE DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn paddq() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PADDSB (With 64-bit Operands)
///     DEST[7:0] := SaturateToSignedByte(DEST[7:0] + SRC (7:0]);
///     (* Repeat add operation for 2nd through 7th bytes *)
///     DEST[63:56] := SaturateToSignedByte(DEST[63:56] + SRC[63:56] );
/// PADDSB (With 128-bit Operands)
///     DEST[7:0] := SaturateToSignedByte (DEST[7:0] + SRC[7:0]);
///     (* Repeat add operation for 2nd through 14th bytes *)
///     DEST[127:120] := SaturateToSignedByte (DEST[111:120] + SRC[127:120]);
/// VPADDSB (VEX.128 Encoded Version)
///     DEST[7:0] := SaturateToSignedByte (SRC1[7:0] + SRC2[7:0]);
///     (* Repeat subtract operation for 2nd through 14th bytes *)
///     DEST[127:120] := SaturateToSignedByte (SRC1[111:120] + SRC2[127:120]);
///     DEST[MAXVL-1:128] := 0
/// VPADDSB (VEX.256 Encoded Version)
///     DEST[7:0] := SaturateToSignedByte (SRC1[7:0] + SRC2[7:0]);
///     (* Repeat add operation for 2nd through 31st bytes *)
///     DEST[255:248] := SaturateToSignedByte (SRC1[255:248] + SRC2[255:248]);
/// VPADDSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SaturateToSignedByte (SRC1[i+7:i] + SRC2[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PADDSW (with 64-bit operands)
///     DEST[15:0] := SaturateToSignedWord(DEST[15:0] + SRC[15:0] );
///     (* Repeat add operation for 2nd and 7th words *)
///     DEST[63:48] := SaturateToSignedWord(DEST[63:48] + SRC[63:48] );
/// PADDSW (with 128-bit operands)
///     DEST[15:0] := SaturateToSignedWord (DEST[15:0] + SRC[15:0]);
///     (* Repeat add operation for 2nd through 7th words *)
///     DEST[127:112] := SaturateToSignedWord (DEST[127:112] + SRC[127:112]);
/// VPADDSW (VEX.128 Encoded Version)
///     DEST[15:0] := SaturateToSignedWord (SRC1[15:0] + SRC2[15:0]);
///     (* Repeat subtract operation for 2nd through 7th words *)
///     DEST[127:112] := SaturateToSignedWord (SRC1[127:112] + SRC2[127:112]);
///     DEST[MAXVL-1:128] := 0
/// VPADDSW (VEX.256 Encoded Version)
///     DEST[15:0] := SaturateToSignedWord (SRC1[15:0] + SRC2[15:0]);
///     (* Repeat add operation for 2nd through 15th words *)
///     DEST[255:240] := SaturateToSignedWord (SRC1[255:240] + SRC2[255:240])
/// VPADDSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToSignedWord (SRC1[i+15:i] + SRC2[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn paddsb() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PADDSB (With 64-bit Operands)
///     DEST[7:0] := SaturateToSignedByte(DEST[7:0] + SRC (7:0]);
///     (* Repeat add operation for 2nd through 7th bytes *)
///     DEST[63:56] := SaturateToSignedByte(DEST[63:56] + SRC[63:56] );
/// PADDSB (With 128-bit Operands)
///     DEST[7:0] := SaturateToSignedByte (DEST[7:0] + SRC[7:0]);
///     (* Repeat add operation for 2nd through 14th bytes *)
///     DEST[127:120] := SaturateToSignedByte (DEST[111:120] + SRC[127:120]);
/// VPADDSB (VEX.128 Encoded Version)
///     DEST[7:0] := SaturateToSignedByte (SRC1[7:0] + SRC2[7:0]);
///     (* Repeat subtract operation for 2nd through 14th bytes *)
///     DEST[127:120] := SaturateToSignedByte (SRC1[111:120] + SRC2[127:120]);
///     DEST[MAXVL-1:128] := 0
/// VPADDSB (VEX.256 Encoded Version)
///     DEST[7:0] := SaturateToSignedByte (SRC1[7:0] + SRC2[7:0]);
///     (* Repeat add operation for 2nd through 31st bytes *)
///     DEST[255:248] := SaturateToSignedByte (SRC1[255:248] + SRC2[255:248]);
/// VPADDSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SaturateToSignedByte (SRC1[i+7:i] + SRC2[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PADDSW (with 64-bit operands)
///     DEST[15:0] := SaturateToSignedWord(DEST[15:0] + SRC[15:0] );
///     (* Repeat add operation for 2nd and 7th words *)
///     DEST[63:48] := SaturateToSignedWord(DEST[63:48] + SRC[63:48] );
/// PADDSW (with 128-bit operands)
///     DEST[15:0] := SaturateToSignedWord (DEST[15:0] + SRC[15:0]);
///     (* Repeat add operation for 2nd through 7th words *)
///     DEST[127:112] := SaturateToSignedWord (DEST[127:112] + SRC[127:112]);
/// VPADDSW (VEX.128 Encoded Version)
///     DEST[15:0] := SaturateToSignedWord (SRC1[15:0] + SRC2[15:0]);
///     (* Repeat subtract operation for 2nd through 7th words *)
///     DEST[127:112] := SaturateToSignedWord (SRC1[127:112] + SRC2[127:112]);
///     DEST[MAXVL-1:128] := 0
/// VPADDSW (VEX.256 Encoded Version)
///     DEST[15:0] := SaturateToSignedWord (SRC1[15:0] + SRC2[15:0]);
///     (* Repeat add operation for 2nd through 15th words *)
///     DEST[255:240] := SaturateToSignedWord (SRC1[255:240] + SRC2[255:240])
/// VPADDSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToSignedWord (SRC1[i+15:i] + SRC2[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn paddsw() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PADDUSB (With 64-bit Operands)
///     DEST[7:0] := SaturateToUnsignedByte(DEST[7:0] + SRC (7:0] );
///     (* Repeat add operation for 2nd through 7th bytes *)
///     DEST[63:56] := SaturateToUnsignedByte(DEST[63:56] + SRC[63:56]
/// PADDUSB (With 128-bit Operands)
///     DEST[7:0] := SaturateToUnsignedByte (DEST[7:0] + SRC[7:0]);
///     (* Repeat add operation for 2nd through 14th bytes *)
///     DEST[127:120] := SaturateToUnSignedByte (DEST[127:120] + SRC[127:120]);
/// VPADDUSB (VEX.128 Encoded Version)
///     DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] + SRC2[7:0]);
///     (* Repeat subtract operation for 2nd through 14th bytes *)
///     DEST[127:120] := SaturateToUnsignedByte (SRC1[111:120] + SRC2[127:120]);
///     DEST[MAXVL-1:128] := 0
/// VPADDUSB (VEX.256 Encoded Version)
///     DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] + SRC2[7:0]);
///     (* Repeat add operation for 2nd through 31st bytes *)
///     DEST[255:248] := SaturateToUnsignedByte (SRC1[255:248] + SRC2[255:248]);
/// PADDUSW (With 64-bit Operands)
///     DEST[15:0] := SaturateToUnsignedWord(DEST[15:0] + SRC[15:0] );
///     (* Repeat add operation for 2nd and 3rd words *)
///     DEST[63:48] := SaturateToUnsignedWord(DEST[63:48] + SRC[63:48] );
/// PADDUSW (With 128-bit Operands)
///     DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] + SRC[15:0]);
///     (* Repeat add operation for 2nd through 7th words *)
///     DEST[127:112] := SaturateToUnSignedWord (DEST[127:112] + SRC[127:112]);
/// VPADDUSW (VEX.128 Encoded Version)
///     DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] + SRC2[15:0]);
///     (* Repeat subtract operation for 2nd through 7th words *)
///     DEST[127:112] := SaturateToUnsignedWord (SRC1[127:112] + SRC2[127:112]);
///     DEST[MAXVL-1:128] := 0
/// VPADDUSW (VEX.256 Encoded Version)
///     DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] + SRC2[15:0]);
///     (* Repeat add operation for 2nd through 15th words *)
///     DEST[255:240] := SaturateToUnsignedWord (SRC1[255:240] + SRC2[255:240])
/// VPADDUSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SaturateToUnsignedByte (SRC1[i+7:i] + SRC2[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDUSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToUnsignedWord (SRC1[i+15:i] + SRC2[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn paddusb() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PADDUSB (With 64-bit Operands)
///     DEST[7:0] := SaturateToUnsignedByte(DEST[7:0] + SRC (7:0] );
///     (* Repeat add operation for 2nd through 7th bytes *)
///     DEST[63:56] := SaturateToUnsignedByte(DEST[63:56] + SRC[63:56]
/// PADDUSB (With 128-bit Operands)
///     DEST[7:0] := SaturateToUnsignedByte (DEST[7:0] + SRC[7:0]);
///     (* Repeat add operation for 2nd through 14th bytes *)
///     DEST[127:120] := SaturateToUnSignedByte (DEST[127:120] + SRC[127:120]);
/// VPADDUSB (VEX.128 Encoded Version)
///     DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] + SRC2[7:0]);
///     (* Repeat subtract operation for 2nd through 14th bytes *)
///     DEST[127:120] := SaturateToUnsignedByte (SRC1[111:120] + SRC2[127:120]);
///     DEST[MAXVL-1:128] := 0
/// VPADDUSB (VEX.256 Encoded Version)
///     DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] + SRC2[7:0]);
///     (* Repeat add operation for 2nd through 31st bytes *)
///     DEST[255:248] := SaturateToUnsignedByte (SRC1[255:248] + SRC2[255:248]);
/// PADDUSW (With 64-bit Operands)
///     DEST[15:0] := SaturateToUnsignedWord(DEST[15:0] + SRC[15:0] );
///     (* Repeat add operation for 2nd and 3rd words *)
///     DEST[63:48] := SaturateToUnsignedWord(DEST[63:48] + SRC[63:48] );
/// PADDUSW (With 128-bit Operands)
///     DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] + SRC[15:0]);
///     (* Repeat add operation for 2nd through 7th words *)
///     DEST[127:112] := SaturateToUnSignedWord (DEST[127:112] + SRC[127:112]);
/// VPADDUSW (VEX.128 Encoded Version)
///     DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] + SRC2[15:0]);
///     (* Repeat subtract operation for 2nd through 7th words *)
///     DEST[127:112] := SaturateToUnsignedWord (SRC1[127:112] + SRC2[127:112]);
///     DEST[MAXVL-1:128] := 0
/// VPADDUSW (VEX.256 Encoded Version)
///     DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] + SRC2[15:0]);
///     (* Repeat add operation for 2nd through 15th words *)
///     DEST[255:240] := SaturateToUnsignedWord (SRC1[255:240] + SRC2[255:240])
/// VPADDUSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SaturateToUnsignedByte (SRC1[i+7:i] + SRC2[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDUSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToUnsignedWord (SRC1[i+15:i] + SRC2[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn paddusw() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PADDB (With 64-bit Operands)
///     DEST[7:0] := DEST[7:0] + SRC[7:0];
///     (* Repeat add operation for 2nd through 7th byte *)
///     DEST[63:56] := DEST[63:56] + SRC[63:56];
/// PADDW (With 64-bit Operands)
///     DEST[15:0] := DEST[15:0] + SRC[15:0];
///     (* Repeat add operation for 2nd and 3th word *)
///     DEST[63:48] := DEST[63:48] + SRC[63:48];
/// PADDD (With 64-bit Operands)
///     DEST[31:0] := DEST[31:0] + SRC[31:0];
///     DEST[63:32] := DEST[63:32] + SRC[63:32];
/// PADDQ (With 64-Bit Operands)
///     DEST[63:0] := DEST[63:0] + SRC[63:0];
/// PADDB (Legacy SSE Instruction)
///     DEST[7:0] := DEST[7:0] + SRC[7:0];
///     (* Repeat add operation for 2nd through 15th byte *)
///     DEST[127:120] := DEST[127:120] + SRC[127:120];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDW (Legacy SSE Instruction)
///     DEST[15:0] := DEST[15:0] + SRC[15:0];
///     (* Repeat add operation for 2nd through 7th word *)
///     DEST[127:112] := DEST[127:112] + SRC[127:112];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDD (Legacy SSE Instruction)
///     DEST[31:0] := DEST[31:0] + SRC[31:0];
///     (* Repeat add operation for 2nd and 3th doubleword *)
///     DEST[127:96] := DEST[127:96] + SRC[127:96];
///     DEST[MAXVL-1:128] (Unmodified)
/// PADDQ (Legacy SSE Instruction)
///     DEST[63:0] := DEST[63:0] + SRC[63:0];
///     DEST[127:64] := DEST[127:64] + SRC[127:64];
///     DEST[MAXVL-1:128] (Unmodified)
/// VPADDB (VEX.128 Encoded Instruction)
///     DEST[7:0] := SRC1[7:0] + SRC2[7:0];
///     (* Repeat add operation for 2nd through 15th byte *)
///     DEST[127:120] := SRC1[127:120] + SRC2[127:120];
///     DEST[MAXVL-1:128] := 0;
/// VPADDW (VEX.128 Encoded Instruction)
///     DEST[15:0] := SRC1[15:0] + SRC2[15:0];
///     (* Repeat add operation for 2nd through 7th word *)
///     DEST[127:112] := SRC1[127:112] + SRC2[127:112];
///     DEST[MAXVL-1:128] := 0;
/// VPADDD (VEX.128 Encoded Instruction)
///     DEST[31:0] := SRC1[31:0] + SRC2[31:0];
///     (* Repeat add operation for 2nd and 3th doubleword *)
///     DEST[127:96] := SRC1[127:96] + SRC2[127:96];
///     DEST[MAXVL-1:128] := 0;
/// VPADDQ (VEX.128 Encoded Instruction)
///     DEST[63:0] := SRC1[63:0] + SRC2[63:0];
///     DEST[127:64] := SRC1[127:64] + SRC2[127:64];
///     DEST[MAXVL-1:128] := 0;
/// VPADDB (VEX.256 Encoded Instruction)
///     DEST[7:0] := SRC1[7:0] + SRC2[7:0];
///     (* Repeat add operation for 2nd through 31th byte *)
///     DEST[255:248] := SRC1[255:248] + SRC2[255:248];
/// VPADDW (VEX.256 Encoded Instruction)
///     DEST[15:0] := SRC1[15:0] + SRC2[15:0];
///     (* Repeat add operation for 2nd through 15th word *)
///     DEST[255:240] := SRC1[255:240] + SRC2[255:240];
/// VPADDD (VEX.256 Encoded Instruction)
///     DEST[31:0] := SRC1[31:0] + SRC2[31:0];
///     (* Repeat add operation for 2nd and 7th doubleword *)
///     DEST[255:224] := SRC1[255:224] + SRC2[255:224];
/// VPADDQ (VEX.256 Encoded Instruction)
///     DEST[63:0] := SRC1[63:0] + SRC2[63:0];
///     DEST[127:64] := SRC1[127:64] + SRC2[127:64];
///     DEST[191:128] := SRC1[191:128] + SRC2[191:128];
///     DEST[255:192] := SRC1[255:192] + SRC2[255:192];
/// VPADDB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC1[i+7:i] + SRC2[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC1[i+15:i] + SRC2[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := SRC1[i+31:i] + SRC2[31:0]
///                 ELSE DEST[i+31:i] := SRC1[i+31:i] + SRC2[i+31:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPADDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := SRC1[i+63:i] + SRC2[63:0]
///                 ELSE DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn paddw() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PALIGNR (With 64-bit Operands)
///     temp1[127:0] = CONCATENATE(DEST,SRC)>>(imm8*8)
///     DEST[63:0] = temp1[63:0]
/// PALIGNR (With 128-bit Operands)
/// temp1[255:0] := ((DEST[127:0] << 128) OR SRC[127:0])>>(imm8*8);
/// DEST[127:0] := temp1[127:0]
/// DEST[MAXVL-1:128] (Unmodified)0
/// VPALIGNR (VEX.128 Encoded Version)
/// temp1[255:0] := ((SRC1[127:0] << 128) OR SRC2[127:0])>>(imm8*8);
/// DEST[127:0] := temp1[127:0]
/// DEST[MAXVL-1:128] := 0
/// VPALIGNR (VEX.256 Encoded Version)
/// temp1[255:0] := ((SRC1[127:0] << 128) OR SRC2[127:0])>>(imm8[7:0]*8);
/// DEST[127:0] := temp1[127:0]
/// temp1[255:0] := ((SRC1[255:128] << 128) OR SRC2[255:128])>>(imm8[7:0]*8);
/// DEST[MAXVL-1:128] := temp1[127:0]
/// VPALIGNR (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR l := 0 TO VL-1 with increments of 128
///     temp1[255:0] := ((SRC1[l+127:l] << 128) OR SRC2[l+127:l])>>(imm8[7:0]*8);
///     TMP_DEST[l+127:l] := temp1[127:0]
/// ENDFOR;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn palignr() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PAND (64-bit Operand)
/// DEST := DEST AND SRC
/// PAND (128-bit Legacy SSE Version)
/// DEST := DEST AND SRC
/// DEST[MAXVL-1:128] (Unmodified)
/// VPAND (VEX.128 Encoded Version)
/// DEST := SRC1 AND SRC2
/// DEST[MAXVL-1:128] := 0
/// VPAND (VEX.256 Encoded Instruction)
/// DEST[255:0] := (SRC1[255:0] AND SRC2[255:0])
/// DEST[MAXVL-1:256] := 0
/// VPANDD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := SRC1[i+31:i] BITWISE AND SRC2[31:0]
///                     ELSE DEST[i+31:i] := SRC1[i+31:i] BITWISE AND SRC2[i+31:i]
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
/// VPANDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := SRC1[i+63:i] BITWISE AND SRC2[63:0]
///                     ELSE DEST[i+63:i] := SRC1[i+63:i] BITWISE AND SRC2[i+63:i]
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
pub(super) fn pand() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PANDN (64-bit Operand)
/// DEST := NOT(DEST) AND SRC
/// PANDN (128-bit Legacy SSE Version)
/// DEST := NOT(DEST) AND SRC
/// DEST[MAXVL-1:128] (Unmodified)
/// VPANDN (VEX.128 Encoded Version)
/// DEST := NOT(SRC1) AND SRC2
/// DEST[MAXVL-1:128] := 0
/// VPANDN (VEX.256 Encoded Instruction)
/// DEST[255:0] := ((NOT SRC1[255:0]) AND SRC2[255:0])
/// DEST[MAXVL-1:256] := 0
/// VPANDND (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := ((NOT SRC1[i+31:i]) AND SRC2[31:0])
///                     ELSE DEST[i+31:i] := ((NOT SRC1[i+31:i]) AND SRC2[i+31:i])
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
/// VPANDNQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := ((NOT SRC1[i+63:i]) AND SRC2[63:0])
///                     ELSE DEST[i+63:i] := ((NOT SRC1[i+63:i]) AND SRC2[i+63:i])
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
pub(super) fn pandn() -> &'static [IrStatement] {
    let assignment = assign(b::and(u::not(o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Execute_Next_Instruction(DELAY);
/// ```
#[box_to_static_reference]
pub(super) fn pause() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// PAVGB (With 64-bit Operands)
///     DEST[7:0] := (SRC[7:0] + DEST[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)
///     (* Repeat operation performed for bytes 2 through 6 *)
///     DEST[63:56] := (SRC[63:56] + DEST[63:56] + 1) >> 1;
/// PAVGW (With 64-bit Operands)
///     DEST[15:0] := (SRC[15:0] + DEST[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)
///     (* Repeat operation performed for words 2 and 3 *)
///     DEST[63:48] := (SRC[63:48] + DEST[63:48] + 1) >> 1;
/// PAVGB (With 128-bit Operands)
///     DEST[7:0] := (SRC[7:0] + DEST[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)
///     (* Repeat operation performed for bytes 2 through 14 *)
///     DEST[127:120] := (SRC[127:120] + DEST[127:120] + 1) >> 1;
/// PAVGW (With 128-bit Operands)
///     DEST[15:0] := (SRC[15:0] + DEST[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)
///     (* Repeat operation performed for words 2 through 6 *)
///     DEST[127:112] := (SRC[127:112] + DEST[127:112] + 1) >> 1;
/// VPAVGB (VEX.128 Encoded Version)
///     DEST[7:0] := (SRC1[7:0] + SRC2[7:0] + 1) >> 1;
///     (* Repeat operation performed for bytes 2 through 15 *)
///     DEST[127:120] := (SRC1[127:120] + SRC2[127:120] + 1) >> 1
///     DEST[MAXVL-1:128] := 0
/// VPAVGW (VEX.128 Encoded Version)
///     DEST[15:0] := (SRC1[15:0] + SRC2[15:0] + 1) >> 1;
///     (* Repeat operation performed for 16-bit words 2 through 7 *)
///     DEST[127:112] := (SRC1[127:112] + SRC2[127:112] + 1) >> 1
///     DEST[MAXVL-1:128] := 0
/// VPAVGB (VEX.256 Encoded Instruction)
///     DEST[7:0] := (SRC1[7:0] + SRC2[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)
///     (* Repeat operation performed for bytes 2 through 31)
///     DEST[255:248] := (SRC1[255:248] + SRC2[255:248] + 1) >> 1;
/// VPAVGW (VEX.256 Encoded Instruction)
///     DEST[15:0] := (SRC1[15:0] + SRC2[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)
///     (* Repeat operation performed for words 2 through 15)
///     DEST[255:14]) := (SRC1[255:240] + SRC2[255:240] + 1) >> 1;
/// VPAVGB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := (SRC1[i+7:i] + SRC2[i+7:i] + 1) >> 1; (* Temp sum before shifting is 9 bits *)
///         ELSE
///             IF *merging-masking*
///                             ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPAVGW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := (SRC1[i+15:i] + SRC2[i+15:i] + 1) >> 1
///                         ; (* Temp sum before shifting is 17 bits *)
///         ELSE
///             IF *merging-masking*
///                             ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pavgb() -> &'static [IrStatement] {
    let assignment = assign(b::add(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PAVGB (With 64-bit Operands)
///     DEST[7:0] := (SRC[7:0] + DEST[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)
///     (* Repeat operation performed for bytes 2 through 6 *)
///     DEST[63:56] := (SRC[63:56] + DEST[63:56] + 1) >> 1;
/// PAVGW (With 64-bit Operands)
///     DEST[15:0] := (SRC[15:0] + DEST[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)
///     (* Repeat operation performed for words 2 and 3 *)
///     DEST[63:48] := (SRC[63:48] + DEST[63:48] + 1) >> 1;
/// PAVGB (With 128-bit Operands)
///     DEST[7:0] := (SRC[7:0] + DEST[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)
///     (* Repeat operation performed for bytes 2 through 14 *)
///     DEST[127:120] := (SRC[127:120] + DEST[127:120] + 1) >> 1;
/// PAVGW (With 128-bit Operands)
///     DEST[15:0] := (SRC[15:0] + DEST[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)
///     (* Repeat operation performed for words 2 through 6 *)
///     DEST[127:112] := (SRC[127:112] + DEST[127:112] + 1) >> 1;
/// VPAVGB (VEX.128 Encoded Version)
///     DEST[7:0] := (SRC1[7:0] + SRC2[7:0] + 1) >> 1;
///     (* Repeat operation performed for bytes 2 through 15 *)
///     DEST[127:120] := (SRC1[127:120] + SRC2[127:120] + 1) >> 1
///     DEST[MAXVL-1:128] := 0
/// VPAVGW (VEX.128 Encoded Version)
///     DEST[15:0] := (SRC1[15:0] + SRC2[15:0] + 1) >> 1;
///     (* Repeat operation performed for 16-bit words 2 through 7 *)
///     DEST[127:112] := (SRC1[127:112] + SRC2[127:112] + 1) >> 1
///     DEST[MAXVL-1:128] := 0
/// VPAVGB (VEX.256 Encoded Instruction)
///     DEST[7:0] := (SRC1[7:0] + SRC2[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)
///     (* Repeat operation performed for bytes 2 through 31)
///     DEST[255:248] := (SRC1[255:248] + SRC2[255:248] + 1) >> 1;
/// VPAVGW (VEX.256 Encoded Instruction)
///     DEST[15:0] := (SRC1[15:0] + SRC2[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)
///     (* Repeat operation performed for words 2 through 15)
///     DEST[255:14]) := (SRC1[255:240] + SRC2[255:240] + 1) >> 1;
/// VPAVGB (EVEX encoded versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := (SRC1[i+7:i] + SRC2[i+7:i] + 1) >> 1; (* Temp sum before shifting is 9 bits *)
///         ELSE
///             IF *merging-masking*
///                             ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPAVGW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := (SRC1[i+15:i] + SRC2[i+15:i] + 1) >> 1
///                         ; (* Temp sum before shifting is 17 bits *)
///         ELSE
///             IF *merging-masking*
///                             ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                                 ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pavgw() -> &'static [IrStatement] {
    let assignment = assign(b::add(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PBLENDVB (128-bit Legacy SSE Version)
/// MASK := XMM0
/// IF (MASK[7] = 1) THEN DEST[7:0] := SRC[7:0];
/// ELSE DEST[7:0] := DEST[7:0];
/// IF (MASK[15] = 1) THEN DEST[15:8] := SRC[15:8];
/// ELSE DEST[15:8] := DEST[15:8];
/// IF (MASK[23] = 1) THEN DEST[23:16] := SRC[23:16]
/// ELSE DEST[23:16] := DEST[23:16];
/// IF (MASK[31] = 1) THEN DEST[31:24] := SRC[31:24]
/// ELSE DEST[31:24] := DEST[31:24];
/// IF (MASK[39] = 1) THEN DEST[39:32] := SRC[39:32]
/// ELSE DEST[39:32] := DEST[39:32];
/// IF (MASK[47] = 1) THEN DEST[47:40] := SRC[47:40]
/// ELSE DEST[47:40] := DEST[47:40];
/// IF (MASK[55] = 1) THEN DEST[55:48] := SRC[55:48]
/// ELSE DEST[55:48] := DEST[55:48];
/// IF (MASK[63] = 1) THEN DEST[63:56] := SRC[63:56]
/// ELSE DEST[63:56] := DEST[63:56];
/// IF (MASK[71] = 1) THEN DEST[71:64] := SRC[71:64]
/// ELSE DEST[71:64] := DEST[71:64];
/// IF (MASK[79] = 1) THEN DEST[79:72] := SRC[79:72]
/// ELSE DEST[79:72] := DEST[79:72];
/// IF (MASK[87] = 1) THEN DEST[87:80] := SRC[87:80]
/// ELSE DEST[87:80] := DEST[87:80];
/// IF (MASK[95] = 1) THEN DEST[95:88] := SRC[95:88]
/// ELSE DEST[95:88] := DEST[95:88];
/// IF (MASK[103] = 1) THEN DEST[103:96] := SRC[103:96]
/// ELSE DEST[103:96] := DEST[103:96];
/// IF (MASK[111] = 1) THEN DEST[111:104] := SRC[111:104]
/// ELSE DEST[111:104] := DEST[111:104];
/// IF (MASK[119] = 1) THEN DEST[119:112] := SRC[119:112]
/// ELSE DEST[119:112] := DEST[119:112];
/// IF (MASK[127] = 1) THEN DEST[127:120] := SRC[127:120]
/// ELSE DEST[127:120] := DEST[127:120])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPBLENDVB (VEX.128 Encoded Version)
/// MASK := SRC3
/// IF (MASK[7] = 1) THEN DEST[7:0] := SRC2[7:0];
/// ELSE DEST[7:0] := SRC1[7:0];
/// IF (MASK[15] = 1) THEN DEST[15:8] := SRC2[15:8];
/// ELSE DEST[15:8] := SRC1[15:8];
/// IF (MASK[23] = 1) THEN DEST[23:16] := SRC2[23:16]
/// ELSE DEST[23:16] := SRC1[23:16];
/// IF (MASK[31] = 1) THEN DEST[31:24] := SRC2[31:24]
/// ELSE DEST[31:24] := SRC1[31:24];
/// IF (MASK[39] = 1) THEN DEST[39:32] := SRC2[39:32]
/// ELSE DEST[39:32] := SRC1[39:32];
/// IF (MASK[47] = 1) THEN DEST[47:40] := SRC2[47:40]
/// ELSE DEST[47:40] := SRC1[47:40];
/// IF (MASK[55] = 1) THEN DEST[55:48] := SRC2[55:48]
/// ELSE DEST[55:48] := SRC1[55:48];
/// IF (MASK[63] = 1) THEN DEST[63:56] := SRC2[63:56]
/// ELSE DEST[63:56] := SRC1[63:56];
/// IF (MASK[71] = 1) THEN DEST[71:64] := SRC2[71:64]
/// ELSE DEST[71:64] := SRC1[71:64];
/// IF (MASK[79] = 1) THEN DEST[79:72] := SRC2[79:72]
/// ELSE DEST[79:72] := SRC1[79:72];
/// IF (MASK[87] = 1) THEN DEST[87:80] := SRC2[87:80]
/// ELSE DEST[87:80] := SRC1[87:80];
/// ELSE DEST[95:88] := SRC1[95:88];
/// IF (MASK[103] = 1) THEN DEST[103:96] := SRC2[103:96]
/// ELSE DEST[103:96] := SRC1[103:96];
/// IF (MASK[111] = 1) THEN DEST[111:104] := SRC2[111:104]
/// ELSE DEST[111:104] := SRC1[111:104];
/// IF (MASK[119] = 1) THEN DEST[119:112] := SRC2[119:112]
/// ELSE DEST[119:112] := SRC1[119:112];
/// IF (MASK[127] = 1) THEN DEST[127:120] := SRC2[127:120]
/// ELSE DEST[127:120] := SRC1[127:120])
/// DEST[MAXVL-1:128] := 0
/// VPBLENDVB (VEX.256 Encoded Version)
/// MASK := SRC3
/// IF (MASK[7] == 1) THEN DEST[7:0] := SRC2[7:0];
/// ELSE DEST[7:0] := SRC1[7:0];
/// IF (MASK[15] == 1) THEN DEST[15:8] := SRC2[15:8];
/// ELSE DEST[15:8] := SRC1[15:8];
/// IF (MASK[23] == 1) THEN DEST[23:16] := SRC2[23:16]
/// ELSE DEST[23:16] := SRC1[23:16];
/// IF (MASK[31] == 1) THEN DEST[31:24] := SRC2[31:24]
/// ELSE DEST[31:24] := SRC1[31:24];
/// IF (MASK[39] == 1) THEN DEST[39:32] := SRC2[39:32]
/// ELSE DEST[39:32] := SRC1[39:32];
/// IF (MASK[47] == 1) THEN DEST[47:40] := SRC2[47:40]
/// ELSE DEST[47:40] := SRC1[47:40];
/// IF (MASK[55] == 1) THEN DEST[55:48] := SRC2[55:48]
/// ELSE DEST[55:48] := SRC1[55:48];
/// IF (MASK[63] == 1) THEN DEST[63:56] := SRC2[63:56]
/// ELSE DEST[63:56] := SRC1[63:56];
/// IF (MASK[71] == 1) THEN DEST[71:64] := SRC2[71:64]
/// ELSE DEST[71:64] := SRC1[71:64];
/// IF (MASK[79] == 1) THEN DEST[79:72] := SRC2[79:72]
/// ELSE DEST[79:72] := SRC1[79:72];
/// IF (MASK[87] == 1) THEN DEST[87:80] := SRC2[87:80]
/// ELSE DEST[87:80] := SRC1[87:80];
/// IF (MASK[95] == 1) THEN DEST[95:88] := SRC2[95:88]
/// ELSE DEST[95:88] := SRC1[95:88];
/// IF (MASK[103] == 1) THEN DEST[103:96] := SRC2[103:96]
/// ELSE DEST[103:96] := SRC1[103:96];
/// IF (MASK[111] == 1) THEN DEST[111:104] := SRC2[111:104]
/// ELSE DEST[111:104] := SRC1[111:104];
/// IF (MASK[119] == 1) THEN DEST[119:112] := SRC2[119:112]
/// ELSE DEST[119:112] := SRC1[119:112];
/// IF (MASK[127] == 1) THEN DEST[127:120] := SRC2[127:120]
/// ELSE DEST[127:120] := SRC1[127:120])
/// IF (MASK[135] == 1) THEN DEST[135:128] := SRC2[135:128];
/// ELSE DEST[135:128] := SRC1[135:128];
/// IF (MASK[143] == 1) THEN DEST[143:136] := SRC2[143:136];
/// ELSE DEST[[143:136] := SRC1[143:136];
/// IF (MASK[151] == 1) THEN DEST[151:144] := SRC2[151:144]
/// ELSE DEST[151:144] := SRC1[151:144];
/// IF (MASK[159] == 1) THEN DEST[159:152] := SRC2[159:152]
/// ELSE DEST[159:152] := SRC1[159:152];
/// ELSE DEST[167:160] := SRC1[167:160];
/// IF (MASK[175] == 1) THEN DEST[175:168] := SRC2[175:168]
/// ELSE DEST[175:168] := SRC1[175:168];
/// IF (MASK[183] == 1) THEN DEST[183:176] := SRC2[183:176]
/// ELSE DEST[183:176] := SRC1[183:176];
/// IF (MASK[191] == 1) THEN DEST[191:184] := SRC2[191:184]
/// ELSE DEST[191:184] := SRC1[191:184];
/// IF (MASK[199] == 1) THEN DEST[199:192] := SRC2[199:192]
/// ELSE DEST[199:192] := SRC1[199:192];
/// IF (MASK[207] == 1) THEN DEST[207:200] := SRC2[207:200]
/// ELSE DEST[207:200] := SRC1[207:200]
/// IF (MASK[215] == 1) THEN DEST[215:208] := SRC2[215:208]
/// ELSE DEST[215:208] := SRC1[215:208];
/// IF (MASK[223] == 1) THEN DEST[223:216] := SRC2[223:216]
/// ELSE DEST[223:216] := SRC1[223:216];
/// IF (MASK[231] == 1) THEN DEST[231:224] := SRC2[231:224]
/// ELSE DEST[231:224] := SRC1[231:224];
/// IF (MASK[239] == 1) THEN DEST[239:232] := SRC2[239:232]
/// ELSE DEST[239:232] := SRC1[239:232];
/// IF (MASK[247] == 1) THEN DEST[247:240] := SRC2[247:240]
/// ELSE DEST[247:240] := SRC1[247:240];
/// IF (MASK[255] == 1) THEN DEST[255:248] := SRC2[255:248]
/// ELSE DEST[255:248] := SRC1[255:248]
/// ```
#[box_to_static_reference]
pub(super) fn pblendvb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PBLENDW (128-bit Legacy SSE Version)
/// IF (imm8[0] = 1) THEN DEST[15:0] := SRC[15:0]
/// ELSE DEST[15:0] := DEST[15:0]
/// IF (imm8[1] = 1) THEN DEST[31:16] := SRC[31:16]
/// ELSE DEST[31:16] := DEST[31:16]
/// IF (imm8[2] = 1) THEN DEST[47:32] := SRC[47:32]
/// ELSE DEST[47:32] := DEST[47:32]
/// IF (imm8[3] = 1) THEN DEST[63:48] := SRC[63:48]
/// ELSE DEST[63:48] := DEST[63:48]
/// IF (imm8[4] = 1) THEN DEST[79:64] := SRC[79:64]
/// ELSE DEST[79:64] := DEST[79:64]
/// IF (imm8[5] = 1) THEN DEST[95:80] := SRC[95:80]
/// ELSE DEST[95:80] := DEST[95:80]
/// IF (imm8[6] = 1) THEN DEST[111:96] := SRC[111:96]
/// ELSE DEST[111:96] := DEST[111:96]
/// ELSE DEST[127:112] := DEST[127:112]
/// VPBLENDW (VEX.128 Encoded Version)
/// IF (imm8[0] = 1) THEN DEST[15:0] := SRC2[15:0]
/// ELSE DEST[15:0] := SRC1[15:0]
/// IF (imm8[1] = 1) THEN DEST[31:16] := SRC2[31:16]
/// ELSE DEST[31:16] := SRC1[31:16]
/// IF (imm8[2] = 1) THEN DEST[47:32] := SRC2[47:32]
/// ELSE DEST[47:32] := SRC1[47:32]
/// IF (imm8[3] = 1) THEN DEST[63:48] := SRC2[63:48]
/// ELSE DEST[63:48] := SRC1[63:48]
/// IF (imm8[4] = 1) THEN DEST[79:64] := SRC2[79:64]
/// ELSE DEST[79:64] := SRC1[79:64]
/// IF (imm8[5] = 1) THEN DEST[95:80] := SRC2[95:80]
/// ELSE DEST[95:80] := SRC1[95:80]
/// IF (imm8[6] = 1) THEN DEST[111:96] := SRC2[111:96]
/// ELSE DEST[111:96] := SRC1[111:96]
/// IF (imm8[7] = 1) THEN DEST[127:112] := SRC2[127:112]
/// ELSE DEST[127:112] := SRC1[127:112]
/// DEST[MAXVL-1:128] := 0
/// VPBLENDW (VEX.256 Encoded Version)
/// IF (imm8[0] == 1) THEN DEST[15:0] := SRC2[15:0]
/// ELSE DEST[15:0] := SRC1[15:0]
/// IF (imm8[1] == 1) THEN DEST[31:16] := SRC2[31:16]
/// ELSE DEST[31:16] := SRC1[31:16]
/// IF (imm8[2] == 1) THEN DEST[47:32] := SRC2[47:32]
/// ELSE DEST[47:32] := SRC1[47:32]
/// IF (imm8[3] == 1) THEN DEST[63:48] := SRC2[63:48]
/// ELSE DEST[63:48] := SRC1[63:48]
/// IF (imm8[4] == 1) THEN DEST[79:64] := SRC2[79:64]
/// ELSE DEST[79:64] := SRC1[79:64]
/// IF (imm8[5] == 1) THEN DEST[95:80] := SRC2[95:80]
/// ELSE DEST[95:80] := SRC1[95:80]
/// IF (imm8[6] == 1) THEN DEST[111:96] := SRC2[111:96]
/// ELSE DEST[111:96] := SRC1[111:96]
/// IF (imm8[7] == 1) THEN DEST[127:112] := SRC2[127:112]
/// ELSE DEST[127:112] := SRC1[127:112]
/// IF (imm8[0] == 1) THEN DEST[143:128] := SRC2[143:128]
/// ELSE DEST[143:128] := SRC1[143:128]
/// IF (imm8[1] == 1) THEN DEST[159:144] := SRC2[159:144]
/// ELSE DEST[159:144] := SRC1[159:144]
/// IF (imm8[2] == 1) THEN DEST[175:160] := SRC2[175:160]
/// ELSE DEST[175:160] := SRC1[175:160]
/// IF (imm8[3] == 1) THEN DEST[191:176] := SRC2[191:176]
/// ELSE DEST[191:176] := SRC1[191:176]
/// IF (imm8[4] == 1) THEN DEST[207:192] := SRC2[207:192]
/// ELSE DEST[207:192] := SRC1[207:192]
/// IF (imm8[5] == 1) THEN DEST[223:208] := SRC2[223:208]
/// ELSE DEST[223:208] := SRC1[223:208]
/// IF (imm8[6] == 1) THEN DEST[239:224] := SRC2[239:224]
/// ELSE DEST[239:224] := SRC1[239:224]
/// IF (imm8[7] == 1) THEN DEST[255:240] := SRC2[255:240]
/// ```
#[box_to_static_reference]
pub(super) fn pblendw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// define PCLMUL128(X,Y):
///                 // helper function
///     FOR i := 0 to 63:
///         TMP [ i ] := X[ 0 ] and Y[ i ]
///         FOR j := 1 to i:
///             TMP [ i ] := TMP [ i ] xor (X[ j ] and Y[ i - j ])
///         DEST[ i ] := TMP[ i ]
///     FOR i := 64 to 126:
///         TMP [ i ] := 0
///         FOR j := i - 63 to 63:
///             TMP [ i ] := TMP [ i ] xor (X[ j ] and Y[ i - j ])
///         DEST[ i ] := TMP[ i ]
///     DEST[127] := 0;
///     RETURN DEST
///                 // 128b vector
/// PCLMULQDQ (SSE Version)
/// IF imm8[0] = 0:
///     TEMP1 := SRC1.qword[0]
/// ELSE:
///     TEMP1 := SRC1.qword[1]
/// IF imm8[4] = 0:
///     TEMP2 := SRC2.qword[0]
/// ELSE:
///     TEMP2 := SRC2.qword[1]
/// DEST[127:0] := PCLMUL128(TEMP1, TEMP2)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCLMULQDQ (128b and 256b VEX Encoded Versions)
/// (KL,VL) = (1,128), (2,256)
/// FOR i= 0 to KL-1:
///     IF imm8[0] = 0:
///         TEMP1 := SRC1.xmm[i].qword[0]
///     ELSE:
///         TEMP1 := SRC1.xmm[i].qword[1]
///     IF imm8[4] = 0:
///         TEMP2 := SRC2.xmm[i].qword[0]
///     ELSE:
///         TEMP2 := SRC2.xmm[i].qword[1]
///     DEST.xmm[i] := PCLMUL128(TEMP1, TEMP2)
/// DEST[MAXVL-1:VL] := 0
/// VPCLMULQDQ (EVEX Encoded Version)
/// (KL,VL) = (1,128), (2,256), (4,512)
/// FOR i = 0 to KL-1:
///     IF imm8[0] = 0:
///         TEMP1 := SRC1.xmm[i].qword[0]
///     ELSE:
///         TEMP1 := SRC1.xmm[i].qword[1]
///     IF imm8[4] = 0:
///         TEMP2 := SRC2.xmm[i].qword[0]
///     ELSE:
///         TEMP2 := SRC2.xmm[i].qword[1]
///     DEST.xmm[i] := PCLMUL128(TEMP1, TEMP2)
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pclmulqdq() -> &'static [IrStatement] {
    [exception("pclmulqdq")].into()
}

/// # Pseudocode
/// ```text
/// PCMPEQB (With 64-bit Operands)
///     IF DEST[7:0] = SRC[7:0]
///         THEN DEST[7:0) := FFH;
///         ELSE DEST[7:0] := 0; FI;
///     (* Continue comparison of 2nd through 7th bytes in DEST and SRC *)
///     IF DEST[63:56] = SRC[63:56]
///         THEN DEST[63:56] := FFH;
///         ELSE DEST[63:56] := 0; FI;
/// COMPARE_BYTES_EQUAL (SRC1, SRC2)
///     IF SRC1[7:0] = SRC2[7:0]
///     THEN DEST[7:0] := FFH;
///     ELSE DEST[7:0] := 0; FI;
/// (* Continue comparison of 2nd through 15th bytes in SRC1 and SRC2 *)
///     IF SRC1[127:120] = SRC2[127:120]
///     THEN DEST[127:120] := FFH;
///     ELSE DEST[127:120] := 0; FI;
/// COMPARE_WORDS_EQUAL (SRC1, SRC2)
///     IF SRC1[15:0] = SRC2[15:0]
///     THEN DEST[15:0] := FFFFH;
///     ELSE DEST[15:0] := 0; FI;
/// (* Continue comparison of 2nd through 7th 16-bit words in SRC1 and SRC2 *)
///     IF SRC1[127:112] = SRC2[127:112]
///     THEN DEST[127:112] := FFFFH;
///     ELSE DEST[127:112] := 0; FI;
/// COMPARE_DWORDS_EQUAL (SRC1, SRC2)
///     IF SRC1[31:0] = SRC2[31:0]
///     THEN DEST[31:0] := FFFFFFFFH;
///     ELSE DEST[31:0] := 0; FI;
/// (* Continue comparison of 2nd through 3rd 32-bit dwords in SRC1 and SRC2 *)
///     IF SRC1[127:96] = SRC2[127:96]
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] := 0; FI;
/// PCMPEQB (With 128-bit Operands)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(DEST[127:0],SRC[127:0])
/// VPCMPEQB (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQB (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_BYTES_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+7:i] == SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPEQW (With 64-bit Operands)
///     IF DEST[15:0] = SRC[15:0]
///         THEN DEST[15:0] := FFFFH;
///         ELSE DEST[15:0] := 0; FI;
///     (* Continue comparison of 2nd and 3rd words in DEST and SRC *)
///     IF DEST[63:48] = SRC[63:48]
///         THEN DEST[63:48] := FFFFH;
///         ELSE DEST[63:48] := 0; FI;
/// PCMPEQW (With 128-bit Operands)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPEQW (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQW (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_WORDS_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+15:i] == SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPEQD (With 64-bit Operands)
///     IF DEST[31:0] = SRC[31:0]
///         THEN DEST[31:0] := FFFFFFFFH;
///         ELSE DEST[31:0] := 0; FI;
///     IF DEST[63:32] = SRC[63:32]
///         THEN DEST[63:32] := FFFFFFFFH;
///         ELSE DEST[63:32] := 0; FI;
/// PCMPEQD (With 128-bit Operands)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPEQD (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQD (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_DWORDS_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] = SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] = SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pcmpeqb() -> &'static [IrStatement] {
    let assignment = assign(b::equal(o2(), o3(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PCMPEQB (With 64-bit Operands)
///     IF DEST[7:0] = SRC[7:0]
///         THEN DEST[7:0) := FFH;
///         ELSE DEST[7:0] := 0; FI;
///     (* Continue comparison of 2nd through 7th bytes in DEST and SRC *)
///     IF DEST[63:56] = SRC[63:56]
///         THEN DEST[63:56] := FFH;
///         ELSE DEST[63:56] := 0; FI;
/// COMPARE_BYTES_EQUAL (SRC1, SRC2)
///     IF SRC1[7:0] = SRC2[7:0]
///     THEN DEST[7:0] := FFH;
///     ELSE DEST[7:0] := 0; FI;
/// (* Continue comparison of 2nd through 15th bytes in SRC1 and SRC2 *)
///     IF SRC1[127:120] = SRC2[127:120]
///     THEN DEST[127:120] := FFH;
///     ELSE DEST[127:120] := 0; FI;
/// COMPARE_WORDS_EQUAL (SRC1, SRC2)
///     IF SRC1[15:0] = SRC2[15:0]
///     THEN DEST[15:0] := FFFFH;
///     ELSE DEST[15:0] := 0; FI;
/// (* Continue comparison of 2nd through 7th 16-bit words in SRC1 and SRC2 *)
///     IF SRC1[127:112] = SRC2[127:112]
///     THEN DEST[127:112] := FFFFH;
///     ELSE DEST[127:112] := 0; FI;
/// COMPARE_DWORDS_EQUAL (SRC1, SRC2)
///     IF SRC1[31:0] = SRC2[31:0]
///     THEN DEST[31:0] := FFFFFFFFH;
///     ELSE DEST[31:0] := 0; FI;
/// (* Continue comparison of 2nd through 3rd 32-bit dwords in SRC1 and SRC2 *)
///     IF SRC1[127:96] = SRC2[127:96]
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] := 0; FI;
/// PCMPEQB (With 128-bit Operands)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(DEST[127:0],SRC[127:0])
/// VPCMPEQB (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQB (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_BYTES_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+7:i] == SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPEQW (With 64-bit Operands)
///     IF DEST[15:0] = SRC[15:0]
///         THEN DEST[15:0] := FFFFH;
///         ELSE DEST[15:0] := 0; FI;
///     (* Continue comparison of 2nd and 3rd words in DEST and SRC *)
///     IF DEST[63:48] = SRC[63:48]
///         THEN DEST[63:48] := FFFFH;
///         ELSE DEST[63:48] := 0; FI;
/// PCMPEQW (With 128-bit Operands)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPEQW (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQW (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_WORDS_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+15:i] == SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPEQD (With 64-bit Operands)
///     IF DEST[31:0] = SRC[31:0]
///         THEN DEST[31:0] := FFFFFFFFH;
///         ELSE DEST[31:0] := 0; FI;
///     IF DEST[63:32] = SRC[63:32]
///         THEN DEST[63:32] := FFFFFFFFH;
///         ELSE DEST[63:32] := 0; FI;
/// PCMPEQD (With 128-bit Operands)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPEQD (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQD (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_DWORDS_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] = SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] = SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pcmpeqd() -> &'static [IrStatement] {
    let assignment = assign(b::equal(o2(), o3(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PCMPEQQ (With 128-bit Operands)
/// IF (DEST[63:0] = SRC[63:0])
///     THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[63:0] := 0; FI;
/// IF (DEST[127:64] = SRC[127:64])
///     THEN DEST[127:64] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[127:64] := 0; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// COMPARE_QWORDS_EQUAL (SRC1, SRC2)
///     IF SRC1[63:0] = SRC2[63:0]
///     THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[63:0] := 0; FI;
///     IF SRC1[127:64] = SRC2[127:64]
///     THEN DEST[127:64] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[127:64] := 0; FI;
/// VPCMPEQQ (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_QWORDS_EQUAL(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQQ (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_QWORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_QWORDS_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k2[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+63:i] = SRC2[63:0];
///                 ELSE CMP := SRC1[i+63:i] = SRC2[i+63:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pcmpeqq() -> &'static [IrStatement] {
    let assignment = assign(b::equal(o2(), o3(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PCMPEQB (With 64-bit Operands)
///     IF DEST[7:0] = SRC[7:0]
///         THEN DEST[7:0) := FFH;
///         ELSE DEST[7:0] := 0; FI;
///     (* Continue comparison of 2nd through 7th bytes in DEST and SRC *)
///     IF DEST[63:56] = SRC[63:56]
///         THEN DEST[63:56] := FFH;
///         ELSE DEST[63:56] := 0; FI;
/// COMPARE_BYTES_EQUAL (SRC1, SRC2)
///     IF SRC1[7:0] = SRC2[7:0]
///     THEN DEST[7:0] := FFH;
///     ELSE DEST[7:0] := 0; FI;
/// (* Continue comparison of 2nd through 15th bytes in SRC1 and SRC2 *)
///     IF SRC1[127:120] = SRC2[127:120]
///     THEN DEST[127:120] := FFH;
///     ELSE DEST[127:120] := 0; FI;
/// COMPARE_WORDS_EQUAL (SRC1, SRC2)
///     IF SRC1[15:0] = SRC2[15:0]
///     THEN DEST[15:0] := FFFFH;
///     ELSE DEST[15:0] := 0; FI;
/// (* Continue comparison of 2nd through 7th 16-bit words in SRC1 and SRC2 *)
///     IF SRC1[127:112] = SRC2[127:112]
///     THEN DEST[127:112] := FFFFH;
///     ELSE DEST[127:112] := 0; FI;
/// COMPARE_DWORDS_EQUAL (SRC1, SRC2)
///     IF SRC1[31:0] = SRC2[31:0]
///     THEN DEST[31:0] := FFFFFFFFH;
///     ELSE DEST[31:0] := 0; FI;
/// (* Continue comparison of 2nd through 3rd 32-bit dwords in SRC1 and SRC2 *)
///     IF SRC1[127:96] = SRC2[127:96]
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] := 0; FI;
/// PCMPEQB (With 128-bit Operands)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(DEST[127:0],SRC[127:0])
/// VPCMPEQB (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQB (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_BYTES_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+7:i] == SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPEQW (With 64-bit Operands)
///     IF DEST[15:0] = SRC[15:0]
///         THEN DEST[15:0] := FFFFH;
///         ELSE DEST[15:0] := 0; FI;
///     (* Continue comparison of 2nd and 3rd words in DEST and SRC *)
///     IF DEST[63:48] = SRC[63:48]
///         THEN DEST[63:48] := FFFFH;
///         ELSE DEST[63:48] := 0; FI;
/// PCMPEQW (With 128-bit Operands)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPEQW (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQW (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_WORDS_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+15:i] == SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPEQD (With 64-bit Operands)
///     IF DEST[31:0] = SRC[31:0]
///         THEN DEST[31:0] := FFFFFFFFH;
///         ELSE DEST[31:0] := 0; FI;
///     IF DEST[63:32] = SRC[63:32]
///         THEN DEST[63:32] := FFFFFFFFH;
///         ELSE DEST[63:32] := 0; FI;
/// PCMPEQD (With 128-bit Operands)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPEQD (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPCMPEQD (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_EQUAL(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_DWORDS_EQUAL(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPEQD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] = SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] = SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pcmpeqw() -> &'static [IrStatement] {
    let assignment = assign(b::equal(o2(), o3(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PCMPGTB (With 64-bit Operands)
///     IF DEST[7:0] > SRC[7:0]
///         THEN DEST[7:0) := FFH;
///         ELSE DEST[7:0] := 0; FI;
///     (* Continue comparison of 2nd through 7th bytes in DEST and SRC *)
///     IF DEST[63:56] > SRC[63:56]
///         THEN DEST[63:56] := FFH;
///         ELSE DEST[63:56] := 0; FI;
/// COMPARE_BYTES_GREATER (SRC1, SRC2)
///     IF SRC1[7:0] > SRC2[7:0]
///     THEN DEST[7:0] := FFH;
///     ELSE DEST[7:0] := 0; FI;
/// (* Continue comparison of 2nd through 15th bytes in SRC1 and SRC2 *)
///     IF SRC1[127:120] > SRC2[127:120]
///     THEN DEST[127:120] := FFH;
///     ELSE DEST[127:120] := 0; FI;
/// COMPARE_WORDS_GREATER (SRC1, SRC2)
///     IF SRC1[15:0] > SRC2[15:0]
///     THEN DEST[15:0] := FFFFH;
///     ELSE DEST[15:0] := 0; FI;
/// (* Continue comparison of 2nd through 7th 16-bit words in SRC1 and SRC2 *)
///     IF SRC1[127:112] > SRC2[127:112]
///     THEN DEST[127:112] := FFFFH;
///     ELSE DEST[127:112] := 0; FI;
/// COMPARE_DWORDS_GREATER (SRC1, SRC2)
///     IF SRC1[31:0] > SRC2[31:0]
///     THEN DEST[31:0] := FFFFFFFFH;
///     ELSE DEST[31:0] := 0; FI;
/// (* Continue comparison of 2nd through 3rd 32-bit dwords in SRC1 and SRC2 *)
///     IF SRC1[127:96] > SRC2[127:96]
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] := 0; FI;
/// PCMPGTB (With 128-bit Operands)
/// DEST[127:0] := COMPARE_BYTES_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTB (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTB (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_BYTES_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+7:i] > SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPGTW (With 64-bit Operands)
///     IF DEST[15:0] > SRC[15:0]
///         THEN DEST[15:0] := FFFFH;
///         ELSE DEST[15:0] := 0; FI;
///     (* Continue comparison of 2nd and 3rd words in DEST and SRC *)
///     IF DEST[63:48] > SRC[63:48]
///         THEN DEST[63:48] := FFFFH;
///         ELSE DEST[63:48] := 0; FI;
/// PCMPGTW (With 128-bit Operands)
/// DEST[127:0] := COMPARE_WORDS_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTW (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTW (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_WORDS_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+15:i] > SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPGTD (With 64-bit Operands)
///     IF DEST[31:0] > SRC[31:0]
///         THEN DEST[31:0] := FFFFFFFFH;
///         ELSE DEST[31:0] := 0; FI;
///     IF DEST[63:32] > SRC[63:32]
///         THEN DEST[63:32] := FFFFFFFFH;
///         ELSE DEST[63:32] := 0; FI;
/// PCMPGTD (With 128-bit Operands)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTD (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTD (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_DWORDS_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] > SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] > SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pcmpgtb() -> &'static [IrStatement] {
    let assignment = assign(b::signed_less(o3(), o2(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PCMPGTB (With 64-bit Operands)
///     IF DEST[7:0] > SRC[7:0]
///         THEN DEST[7:0) := FFH;
///         ELSE DEST[7:0] := 0; FI;
///     (* Continue comparison of 2nd through 7th bytes in DEST and SRC *)
///     IF DEST[63:56] > SRC[63:56]
///         THEN DEST[63:56] := FFH;
///         ELSE DEST[63:56] := 0; FI;
/// COMPARE_BYTES_GREATER (SRC1, SRC2)
///     IF SRC1[7:0] > SRC2[7:0]
///     THEN DEST[7:0] := FFH;
///     ELSE DEST[7:0] := 0; FI;
/// (* Continue comparison of 2nd through 15th bytes in SRC1 and SRC2 *)
///     IF SRC1[127:120] > SRC2[127:120]
///     THEN DEST[127:120] := FFH;
///     ELSE DEST[127:120] := 0; FI;
/// COMPARE_WORDS_GREATER (SRC1, SRC2)
///     IF SRC1[15:0] > SRC2[15:0]
///     THEN DEST[15:0] := FFFFH;
///     ELSE DEST[15:0] := 0; FI;
/// (* Continue comparison of 2nd through 7th 16-bit words in SRC1 and SRC2 *)
///     IF SRC1[127:112] > SRC2[127:112]
///     THEN DEST[127:112] := FFFFH;
///     ELSE DEST[127:112] := 0; FI;
/// COMPARE_DWORDS_GREATER (SRC1, SRC2)
///     IF SRC1[31:0] > SRC2[31:0]
///     THEN DEST[31:0] := FFFFFFFFH;
///     ELSE DEST[31:0] := 0; FI;
/// (* Continue comparison of 2nd through 3rd 32-bit dwords in SRC1 and SRC2 *)
///     IF SRC1[127:96] > SRC2[127:96]
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] := 0; FI;
/// PCMPGTB (With 128-bit Operands)
/// DEST[127:0] := COMPARE_BYTES_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTB (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTB (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_BYTES_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+7:i] > SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPGTW (With 64-bit Operands)
///     IF DEST[15:0] > SRC[15:0]
///         THEN DEST[15:0] := FFFFH;
///         ELSE DEST[15:0] := 0; FI;
///     (* Continue comparison of 2nd and 3rd words in DEST and SRC *)
///     IF DEST[63:48] > SRC[63:48]
///         THEN DEST[63:48] := FFFFH;
///         ELSE DEST[63:48] := 0; FI;
/// PCMPGTW (With 128-bit Operands)
/// DEST[127:0] := COMPARE_WORDS_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTW (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTW (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_WORDS_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+15:i] > SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPGTD (With 64-bit Operands)
///     IF DEST[31:0] > SRC[31:0]
///         THEN DEST[31:0] := FFFFFFFFH;
///         ELSE DEST[31:0] := 0; FI;
///     IF DEST[63:32] > SRC[63:32]
///         THEN DEST[63:32] := FFFFFFFFH;
///         ELSE DEST[63:32] := 0; FI;
/// PCMPGTD (With 128-bit Operands)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTD (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTD (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_DWORDS_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] > SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] > SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pcmpgtd() -> &'static [IrStatement] {
    let assignment = assign(b::signed_less(o3(), o2(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// COMPARE_QWORDS_GREATER (SRC1, SRC2)
///     IF SRC1[63:0] > SRC2[63:0]
///     THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[63:0] := 0; FI;
///     IF SRC1[127:64] > SRC2[127:64]
///     THEN DEST[127:64] := FFFFFFFFFFFFFFFFH;
///     ELSE DEST[127:64] := 0; FI;
/// VPCMPGTQ (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_QWORDS_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTQ (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_QWORDS_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_QWORDS_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+63:i] > SRC2[63:0];
///                 ELSE CMP := SRC1[i+63:i] > SRC2[i+63:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pcmpgtq() -> &'static [IrStatement] {
    let assignment = assign(b::signed_less(o3(), o2(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PCMPGTB (With 64-bit Operands)
///     IF DEST[7:0] > SRC[7:0]
///         THEN DEST[7:0) := FFH;
///         ELSE DEST[7:0] := 0; FI;
///     (* Continue comparison of 2nd through 7th bytes in DEST and SRC *)
///     IF DEST[63:56] > SRC[63:56]
///         THEN DEST[63:56] := FFH;
///         ELSE DEST[63:56] := 0; FI;
/// COMPARE_BYTES_GREATER (SRC1, SRC2)
///     IF SRC1[7:0] > SRC2[7:0]
///     THEN DEST[7:0] := FFH;
///     ELSE DEST[7:0] := 0; FI;
/// (* Continue comparison of 2nd through 15th bytes in SRC1 and SRC2 *)
///     IF SRC1[127:120] > SRC2[127:120]
///     THEN DEST[127:120] := FFH;
///     ELSE DEST[127:120] := 0; FI;
/// COMPARE_WORDS_GREATER (SRC1, SRC2)
///     IF SRC1[15:0] > SRC2[15:0]
///     THEN DEST[15:0] := FFFFH;
///     ELSE DEST[15:0] := 0; FI;
/// (* Continue comparison of 2nd through 7th 16-bit words in SRC1 and SRC2 *)
///     IF SRC1[127:112] > SRC2[127:112]
///     THEN DEST[127:112] := FFFFH;
///     ELSE DEST[127:112] := 0; FI;
/// COMPARE_DWORDS_GREATER (SRC1, SRC2)
///     IF SRC1[31:0] > SRC2[31:0]
///     THEN DEST[31:0] := FFFFFFFFH;
///     ELSE DEST[31:0] := 0; FI;
/// (* Continue comparison of 2nd through 3rd 32-bit dwords in SRC1 and SRC2 *)
///     IF SRC1[127:96] > SRC2[127:96]
///     THEN DEST[127:96] := FFFFFFFFH;
///     ELSE DEST[127:96] := 0; FI;
/// PCMPGTB (With 128-bit Operands)
/// DEST[127:0] := COMPARE_BYTES_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTB (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTB (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_BYTES_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_BYTES_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+7:i] > SRC2[i+7:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPGTW (With 64-bit Operands)
///     IF DEST[15:0] > SRC[15:0]
///         THEN DEST[15:0] := FFFFH;
///         ELSE DEST[15:0] := 0; FI;
///     (* Continue comparison of 2nd and 3rd words in DEST and SRC *)
///     IF DEST[63:48] > SRC[63:48]
///         THEN DEST[63:48] := FFFFH;
///         ELSE DEST[63:48] := 0; FI;
/// PCMPGTW (With 128-bit Operands)
/// DEST[127:0] := COMPARE_WORDS_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTW (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTW (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_WORDS_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_WORDS_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             CMP := SRC1[i+15:i] > SRC2[i+15:i];
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking onlyFI;
///     FI;
/// ENDFOR
/// DEST[MAX_KL-1:KL] := 0
/// PCMPGTD (With 64-bit Operands)
///     IF DEST[31:0] > SRC[31:0]
///         THEN DEST[31:0] := FFFFFFFFH;
///         ELSE DEST[31:0] := 0; FI;
///     IF DEST[63:32] > SRC[63:32]
///         THEN DEST[63:32] := FFFFFFFFH;
///         ELSE DEST[63:32] := 0; FI;
/// PCMPGTD (With 128-bit Operands)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(DEST[127:0],SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// VPCMPGTD (VEX.128 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(SRC1,SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPCMPGTD (VEX.256 Encoded Version)
/// DEST[127:0] := COMPARE_DWORDS_GREATER(SRC1[127:0],SRC2[127:0])
/// DEST[255:128] := COMPARE_DWORDS_GREATER(SRC1[255:128],SRC2[255:128])
/// DEST[MAXVL-1:256] := 0
/// VPCMPGTD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k2[j] OR *no writemask*
///         THEN
///             /* signed comparison */
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN CMP := SRC1[i+31:i] > SRC2[31:0];
///                 ELSE CMP := SRC1[i+31:i] > SRC2[i+31:i];
///             FI;
///             IF CMP = TRUE
///                 THEN DEST[j] := 1;
///                 ELSE DEST[j] := 0; FI;
///         ELSE DEST[j] := 0
///                     ; zeroing-masking only
///     FI;
/// DEST[MAX_KL-1:KL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pcmpgtw() -> &'static [IrStatement] {
    let assignment = assign(b::signed_less(o3(), o2(), o1_size()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (* #UD if PCONFIG is not enumerated or CPL > 0 *)
/// IF CPUID.7.0:EDX[18] = 0 OR CPL > 0
///     THEN #UD; FI;
/// (* #GP(0) for an unsupported leaf function *)
/// IF EAX != 0
///     THEN #GP(0); FI;
/// CASE (EAX)
///                 (* operation based on selected leaf function *)
///     0 (MKTME_KEY_PROGRAM):
///     (* Confirm that TME-MK is properly enabled by the IA32_TME_ACTIVATE MSR *)
///     (* The MSR must be locked, encryption enabled, and a non-zero number of KeyID bits specified *)
///     IF IA32_TME_ACTIVATE[0] = 0 OR IA32_TME_ACTIVATE[1] = 0 OR IA32_TME_ACTIVATE[35:32] = 0
///             THEN #GP(0); FI;
///     IF DS:RBX is not 256-byte aligned
///         THEN #GP(0); FI;
///     Load TMP_KEY_PROGRAM_STRUCT from 192 bytes at linear address DS:RBX;
///     IF TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL sets any reserved bits
///         THEN #GP(0); FI;
///     (* Check for a valid command *)
///     IF TMP_KEY_PROGRAM_STRUCT. KEYID_CTRL.COMMAND > 3
///         THEN #GP(0); FI;
///     (* Check that the KEYID being operated upon is a valid KEYID *)
///     IF TMP_KEY_PROGRAM_STRUCT.KEYID = 0 OR
///         TMP_KEY_PROGRAM_STRUCT.KEYID > 2^IA32_TME_ACTIVATE.MK_TME_KEYID_BITS - 1 OR
///         TMP_KEY_PROGRAM_STRUCT.KEYID > IA32_TME_CAPABILITY.MK_TME_MAX_KEYS
///             THEN #GP(0); FI;
///     (* Check that only one encryption algorithm is requested for the KeyID and it is one of the activated algorithms *)
///     IF TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.ENC_ALG does not set exactly one bit OR
///         (TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.ENC_ALG & IA32_TME_ACTIVATE[63:48]) = 0
///             THEN #GP(0); FI:
///     Attempt to acquire lock to gain exclusive access to platform key table;
///     IF attempt is unsuccessful
///         THEN (* PCONFIG failure *)
///             RFLAGS.ZF := 1;
///             RAX := DEVICE_BUSY;(* failure reason 5 *)
///             GOTO EXIT;
///     FI;
///     CASE (TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.COMMAND) OF
///         0 (KEYID_SET_KEY_DIRECT):
///         Update TME-MK table for TMP_KEY_PROGRAM_STRUCT.KEYID as follows:
///             Encrypt with the selected key
///             Use the encryption algorithm selected by TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.ENC_ALG
///             (* The number of bytes used by the next two lines depends on selected encryption algorithm *)
///             DATA_KEY is TMP_KEY_PROGRAM_STRUCT.KEY_FIELD_1
///             TWEAK_KEY is TMP_KEY_PROGRAM_STRUCT.KEY_FIELD_2
///         BREAK;
///         1 (KEYID_SET_KEY_RANDOM):
///         Load TMP_RND_DATA_KEY with a random key using hardware RNG; (* key size depends on selected encryption algorithm *)
///         IF there was insufficient entropy
///             THEN (* PCONFIG failure *)
///                 RFLAGS.ZF := 1;
///                 RAX := ENTROPY_ERROR;(* failure reason 2 *)
///                 Release lock on platform key table;
///                 GOTO EXIT;
///         FI;
///         Load TMP_RND_TWEAK_KEY with a random key using hardware RNG; (* key size depends on selected encryption algorithm *)
///         IF there was insufficient entropy
///             THEN (* PCONFIG failure *)
///                 RFLAGS.ZF := 1;
///                 RAX := ENTROPY_ERROR;(* failure reason 2 *)
///                 Release lock on platform key table;
///                 GOTO EXIT;
///         FI;
///         (* Combine software-supplied entropy to the data key and tweak key *)
///         (* The number of bytes used by the next two lines depends on selected encryption algorithm *)
///         TMP_RND_DATA_KEY := TMP_RND_KEY XOR TMP_KEY_PROGRAM_STRUCT.KEY_FIELD_1;
///         TMP_RND_TWEAK_KEY := TMP_RND_TWEAK_KEY XOR TMP_KEY_PROGRAM_STRUCT.KEY_FIELD_2;
///         Update TME-MK table for TMP_KEY_PROGRAM_STRUCT.KEYID as follows:
///             Encrypt with the selected key
///             Use the encryption algorithm selected by TMP_KEY_PROGRAM_STRUCT.KEYID_CTRL.ENC_ALG
///             (* The number of bytes used by the next two lines depends on selected encryption algorithm *)
///             DATA_KEY is TMP_RND_DATA_KEY
///             TWEAK_KEY is TMP_RND_TWEAK_KEY
///         BREAK;
///         Update TME-MK table for TMP_KEY_PROGRAM_STRUCT.KEYID as follows:
///             Encrypt (or not) using the current configuration for TME
///             The specified encryption algorithm and key values are not used.
///         BREAK;
///         3 (KEYID_NO_ENCRYPT):
///         Update TME-MK table for TMP_KEY_PROGRAM_STRUCT.KEYID as follows:
///             Do not encrypt
///             The specified encryption algorithm and key values are not used.
///         BREAK;
///     ESAC;
///     Release lock on platform key table;
/// ESAC;
/// RAX := 0;
/// RFLAGS.ZF := 0;
/// EXIT:
/// RFLAGS.CF := 0;
/// RFLAGS.PF := 0;
/// RFLAGS.AF := 0;
/// RFLAGS.OF := 0;
/// RFLAGS.SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn pconfig() -> &'static [IrStatement] {
    [exception("pconfig")].into()
}

/// # Pseudocode
/// ```text
/// TEMP := SRC1;
/// MASK := SRC2;
/// DEST := 0 ;
/// m := 0, k := 0;
/// DOWHILE m < OperandSize
///     IF MASK[ m] = 1 THEN
///         DEST[ m] := TEMP[ k];
///     k := k+ 1;
///     FI
///     m := m+ 1;
/// ```
#[box_to_static_reference]
pub(super) fn pdep() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TEMP := SRC1;
/// MASK := SRC2;
/// DEST := 0 ;
/// m := 0, k := 0;
/// DOWHILE m < OperandSize
///     IF MASK[ m] = 1 THEN
///         DEST[ k] := TEMP[ m];
///     k := k+ 1;
///     FI
///     m := m+ 1;
/// OD
/// ```
#[box_to_static_reference]
pub(super) fn pext() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE of
///     PEXTRB: SEL := COUNT[3:0];
///         TEMP := (Src >> SEL*8) AND FFH;
///         IF (DEST = Mem8)
///             THEN
///             Mem8 := TEMP[7:0];
///         ELSE IF (64-Bit Mode and 64-bit register selected)
///             THEN
///                 R64[7:0] := TEMP[7:0];
///                 r64[63:8] := ZERO_FILL; };
///         ELSE
///                 R32[7:0] := TEMP[7:0];
///                 r32[31:8] := ZERO_FILL; };
///         FI;
///     PEXTRD:SEL := COUNT[1:0];
///         TEMP := (Src >> SEL*32) AND FFFF_FFFFH;
///         DEST := TEMP;
///     PEXTRQ:SEL := COUNT[0];
///         TEMP := (Src >> SEL*64);
///         DEST := TEMP;
/// EASC:
/// VPEXTRTD/VPEXTRQ
/// IF (64-Bit Mode and 64-bit dest operand)
/// THEN
///     Src_Offset := imm8[0]
///     r64/m64 := (Src >> Src_Offset * 64)
/// ELSE
///     Src_Offset := imm8[1:0]
///     r32/m32 := ((Src >> Src_Offset *32) AND 0FFFFFFFFh);
/// FI
/// VPEXTRB ( dest=m8)
/// SRC_Offset := imm8[3:0]
/// Mem8 := (Src >> Src_Offset*8)
/// VPEXTRB ( dest=reg)
/// IF (64-Bit Mode )
/// THEN
///     SRC_Offset := imm8[3:0]
///     DEST[7:0] := ((Src >> Src_Offset*8) AND 0FFh)
///     DEST[63:8] := ZERO_FILL;
/// ELSE
///     SRC_Offset := imm8[3:0];
///     DEST[7:0] := ((Src >> Src_Offset*8) AND 0FFh);
///     DEST[31:8] := ZERO_FILL;
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn pextrb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE of
///     PEXTRB: SEL := COUNT[3:0];
///         TEMP := (Src >> SEL*8) AND FFH;
///         IF (DEST = Mem8)
///             THEN
///             Mem8 := TEMP[7:0];
///         ELSE IF (64-Bit Mode and 64-bit register selected)
///             THEN
///                 R64[7:0] := TEMP[7:0];
///                 r64[63:8] := ZERO_FILL; };
///         ELSE
///                 R32[7:0] := TEMP[7:0];
///                 r32[31:8] := ZERO_FILL; };
///         FI;
///     PEXTRD:SEL := COUNT[1:0];
///         TEMP := (Src >> SEL*32) AND FFFF_FFFFH;
///         DEST := TEMP;
///     PEXTRQ:SEL := COUNT[0];
///         TEMP := (Src >> SEL*64);
///         DEST := TEMP;
/// EASC:
/// VPEXTRTD/VPEXTRQ
/// IF (64-Bit Mode and 64-bit dest operand)
/// THEN
///     Src_Offset := imm8[0]
///     r64/m64 := (Src >> Src_Offset * 64)
/// ELSE
///     Src_Offset := imm8[1:0]
///     r32/m32 := ((Src >> Src_Offset *32) AND 0FFFFFFFFh);
/// FI
/// VPEXTRB ( dest=m8)
/// SRC_Offset := imm8[3:0]
/// Mem8 := (Src >> Src_Offset*8)
/// VPEXTRB ( dest=reg)
/// IF (64-Bit Mode )
/// THEN
///     SRC_Offset := imm8[3:0]
///     DEST[7:0] := ((Src >> Src_Offset*8) AND 0FFh)
///     DEST[63:8] := ZERO_FILL;
/// ELSE
///     SRC_Offset := imm8[3:0];
///     DEST[7:0] := ((Src >> Src_Offset*8) AND 0FFh);
///     DEST[31:8] := ZERO_FILL;
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn pextrd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE of
///     PEXTRB: SEL := COUNT[3:0];
///         TEMP := (Src >> SEL*8) AND FFH;
///         IF (DEST = Mem8)
///             THEN
///             Mem8 := TEMP[7:0];
///         ELSE IF (64-Bit Mode and 64-bit register selected)
///             THEN
///                 R64[7:0] := TEMP[7:0];
///                 r64[63:8] := ZERO_FILL; };
///         ELSE
///                 R32[7:0] := TEMP[7:0];
///                 r32[31:8] := ZERO_FILL; };
///         FI;
///     PEXTRD:SEL := COUNT[1:0];
///         TEMP := (Src >> SEL*32) AND FFFF_FFFFH;
///         DEST := TEMP;
///     PEXTRQ:SEL := COUNT[0];
///         TEMP := (Src >> SEL*64);
///         DEST := TEMP;
/// EASC:
/// VPEXTRTD/VPEXTRQ
/// IF (64-Bit Mode and 64-bit dest operand)
/// THEN
///     Src_Offset := imm8[0]
///     r64/m64 := (Src >> Src_Offset * 64)
/// ELSE
///     Src_Offset := imm8[1:0]
///     r32/m32 := ((Src >> Src_Offset *32) AND 0FFFFFFFFh);
/// FI
/// VPEXTRB ( dest=m8)
/// SRC_Offset := imm8[3:0]
/// Mem8 := (Src >> Src_Offset*8)
/// VPEXTRB ( dest=reg)
/// IF (64-Bit Mode )
/// THEN
///     SRC_Offset := imm8[3:0]
///     DEST[7:0] := ((Src >> Src_Offset*8) AND 0FFh)
///     DEST[63:8] := ZERO_FILL;
/// ELSE
///     SRC_Offset := imm8[3:0];
///     DEST[7:0] := ((Src >> Src_Offset*8) AND 0FFh);
///     DEST[31:8] := ZERO_FILL;
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn pextrq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (DEST = Mem16)
/// THEN
///     SEL := COUNT[2:0];
///     TEMP := (Src >> SEL*16) AND FFFFH;
///     Mem16 := TEMP[15:0];
/// ELSE IF (64-Bit Mode and destination is a general-purpose register)
///     THEN
///         FOR (PEXTRW instruction with 64-bit source operand)
///             {SEL := COUNT[1:0];
///                 TEMP := (SRC >> (SEL * 16)) AND FFFFH;
///                 r64[15:0] := TEMP[15:0];
///                 r64[63:16] := ZERO_FILL; };
///         FOR (PEXTRW instruction with 128-bit source operand)
///         {SEL := COUNT[2:0];
///                 TEMP := (SRC >> (SEL * 16)) AND FFFFH;
///                 r64[15:0] := TEMP[15:0];
///                 r64[63:16] := ZERO_FILL; }
///     ELSE
///         FOR (PEXTRW instruction with 64-bit source operand)
///             {SEL := COUNT[1:0];
///                 TEMP := (SRC >> (SEL * 16)) AND FFFFH;
///                 r32[15:0] := TEMP[15:0];
///                 r32[31:16] := ZERO_FILL; };
///         FOR (PEXTRW instruction with 128-bit source operand)
///             {SEL := COUNT[2:0];
///                 TEMP := (SRC >> (SEL * 16)) AND FFFFH;
///                 r32[15:0] := TEMP[15:0];
///                 r32[31:16] := ZERO_FILL; };
///     FI;
/// FI;
/// VPEXTRW ( dest=m16)
/// SRC_Offset := imm8[2:0]
/// Mem16 := (Src >> Src_Offset*16)
/// VPEXTRW ( dest=reg)
/// IF (64-Bit Mode )
/// THEN
///     SRC_Offset := imm8[2:0]
///     DEST[15:0] := ((Src >> Src_Offset*16) AND 0FFFFh)
///     DEST[63:16] := ZERO_FILL;
/// ELSE
///     SRC_Offset := imm8[2:0]
///     DEST[15:0] := ((Src >> Src_Offset*16) AND 0FFFFh)
///     DEST[31:16] := ZERO_FILL;
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn pextrw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PHADDW (With 64-bit Operands)
///     mm1[15-0]  = mm1[31-16] + mm1[15-0];
///     mm1[31-16] = mm1[63-48] + mm1[47-32];
///     mm1[47-32] = mm2/m64[31-16] + mm2/m64[15-0];
///     mm1[63-48] = mm2/m64[63-48] + mm2/m64[47-32];
/// PHADDW (With 128-bit Operands)
///     xmm1[15-0] = xmm1[31-16] + xmm1[15-0];
///     xmm1[31-16] = xmm1[63-48] + xmm1[47-32];
///     xmm1[47-32] = xmm1[95-80] + xmm1[79-64];
///     xmm1[63-48] = xmm1[127-112] + xmm1[111-96];
///     xmm1[79-64] = xmm2/m128[31-16] + xmm2/m128[15-0];
///     xmm1[95-80] = xmm2/m128[63-48] + xmm2/m128[47-32];
///     xmm1[111-96] = xmm2/m128[95-80] + xmm2/m128[79-64];
///     xmm1[127-112] = xmm2/m128[127-112] + xmm2/m128[111-96];
/// VPHADDW (VEX.128 Encoded Version)
/// DEST[15:0] := SRC1[31:16] + SRC1[15:0]
/// DEST[31:16] := SRC1[63:48] + SRC1[47:32]
/// DEST[47:32] := SRC1[95:80] + SRC1[79:64]
/// DEST[63:48] := SRC1[127:112] + SRC1[111:96]
/// DEST[79:64] := SRC2[31:16] + SRC2[15:0]
/// DEST[95:80] := SRC2[63:48] + SRC2[47:32]
/// DEST[111:96] := SRC2[95:80] + SRC2[79:64]
/// DEST[127:112] := SRC2[127:112] + SRC2[111:96]
/// DEST[MAXVL-1:128] := 0
/// VPHADDW (VEX.256 Encoded Version)
/// DEST[15:0] := SRC1[31:16] + SRC1[15:0]
/// DEST[31:16] := SRC1[63:48] + SRC1[47:32]
/// DEST[47:32] := SRC1[95:80] + SRC1[79:64]
/// DEST[63:48] := SRC1[127:112] + SRC1[111:96]
/// DEST[79:64] := SRC2[31:16] + SRC2[15:0]
/// DEST[95:80] := SRC2[63:48] + SRC2[47:32]
/// DEST[111:96] := SRC2[95:80] + SRC2[79:64]
/// DEST[127:112] := SRC2[127:112] + SRC2[111:96]
/// DEST[143:128] := SRC1[159:144] + SRC1[143:128]
/// DEST[159:144] := SRC1[191:176] + SRC1[175:160]
/// DEST[175:160] := SRC1[223:208] + SRC1[207:192]
/// DEST[191:176] := SRC1[255:240] + SRC1[239:224]
/// DEST[207:192] := SRC2[127:112] + SRC2[143:128]
/// DEST[223:208] := SRC2[159:144] + SRC2[175:160]
/// DEST[239:224] := SRC2[191:176] + SRC2[207:192]
/// DEST[255:240] := SRC2[223:208] + SRC2[239:224]
/// PHADDD (With 64-bit Operands)
///     mm1[31-0]  = mm1[63-32] + mm1[31-0];
///     mm1[63-32] = mm2/m64[63-32] + mm2/m64[31-0];
/// PHADDD (With 128-bit Operands)
///     xmm1[31-0] = xmm1[63-32] + xmm1[31-0];
///     xmm1[63-32] = xmm1[127-96] + xmm1[95-64];
///     xmm1[95-64] = xmm2/m128[63-32] + xmm2/m128[31-0];
///     xmm1[127-96] = xmm2/m128[127-96] + xmm2/m128[95-64];
/// VPHADDD (VEX.128 Encoded Version)
/// DEST[31-0] := SRC1[63-32] + SRC1[31-0]
/// DEST[63-32] := SRC1[127-96] + SRC1[95-64]
/// DEST[95-64] := SRC2[63-32] + SRC2[31-0]
/// DEST[127-96] := SRC2[127-96] + SRC2[95-64]
/// DEST[MAXVL-1:128] := 0
/// VPHADDD (VEX.256 Encoded Version)
/// DEST[31-0] := SRC1[63-32] + SRC1[31-0]
/// DEST[63-32] := SRC1[127-96] + SRC1[95-64]
/// DEST[95-64] := SRC2[63-32] + SRC2[31-0]
/// DEST[127-96] := SRC2[127-96] + SRC2[95-64]
/// DEST[159-128] := SRC1[191-160] + SRC1[159-128]
/// DEST[191-160] := SRC1[255-224] + SRC1[223-192]
/// DEST[223-192] := SRC2[191-160] + SRC2[159-128]
/// DEST[255-224] := SRC2[255-224] + SRC2[223-192]
/// ```
#[box_to_static_reference]
pub(super) fn phaddd() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PHADDSW (With 64-bit Operands)
///     mm1[15-0]  = SaturateToSignedWord((mm1[31-16] + mm1[15-0]);
///     mm1[31-16] = SaturateToSignedWord(mm1[63-48] + mm1[47-32]);
///     mm1[47-32] = SaturateToSignedWord(mm2/m64[31-16] + mm2/m64[15-0]);
///     mm1[63-48] = SaturateToSignedWord(mm2/m64[63-48] + mm2/m64[47-32]);
/// PHADDSW (With 128-bit Operands)
///     xmm1[15-0]= SaturateToSignedWord(xmm1[31-16] + xmm1[15-0]);
///     xmm1[31-16] = SaturateToSignedWord(xmm1[63-48] + xmm1[47-32]);
///     xmm1[47-32] = SaturateToSignedWord(xmm1[95-80] + xmm1[79-64]);
///     xmm1[63-48] = SaturateToSignedWord(xmm1[127-112] + xmm1[111-96]);
///     xmm1[79-64] = SaturateToSignedWord(xmm2/m128[31-16] + xmm2/m128[15-0]);
///     xmm1[95-80] = SaturateToSignedWord(xmm2/m128[63-48] + xmm2/m128[47-32]);
///     xmm1[111-96] = SaturateToSignedWord(xmm2/m128[95-80] + xmm2/m128[79-64]);
///     xmm1[127-112] = SaturateToSignedWord(xmm2/m128[127-112] + xmm2/m128[111-96]);
/// VPHADDSW (VEX.128 Encoded Version)
/// DEST[15:0]= SaturateToSignedWord(SRC1[31:16] + SRC1[15:0])
/// DEST[31:16] = SaturateToSignedWord(SRC1[63:48] + SRC1[47:32])
/// DEST[47:32] = SaturateToSignedWord(SRC1[95:80] + SRC1[79:64])
/// DEST[63:48] = SaturateToSignedWord(SRC1[127:112] + SRC1[111:96])
/// DEST[79:64] = SaturateToSignedWord(SRC2[31:16] + SRC2[15:0])
/// DEST[95:80] = SaturateToSignedWord(SRC2[63:48] + SRC2[47:32])
/// DEST[111:96] = SaturateToSignedWord(SRC2[95:80] + SRC2[79:64])
/// DEST[127:112] = SaturateToSignedWord(SRC2[127:112] + SRC2[111:96])
/// DEST[MAXVL-1:128] := 0
/// VPHADDSW (VEX.256 Encoded Version)
/// DEST[15:0]= SaturateToSignedWord(SRC1[31:16] + SRC1[15:0])
/// DEST[31:16] = SaturateToSignedWord(SRC1[63:48] + SRC1[47:32])
/// DEST[47:32] = SaturateToSignedWord(SRC1[95:80] + SRC1[79:64])
/// DEST[63:48] = SaturateToSignedWord(SRC1[127:112] + SRC1[111:96])
/// DEST[79:64] = SaturateToSignedWord(SRC2[31:16] + SRC2[15:0])
/// DEST[95:80] = SaturateToSignedWord(SRC2[63:48] + SRC2[47:32])
/// DEST[111:96] = SaturateToSignedWord(SRC2[95:80] + SRC2[79:64])
/// DEST[127:112] = SaturateToSignedWord(SRC2[127:112] + SRC2[111:96])
/// DEST[143:128]= SaturateToSignedWord(SRC1[159:144] + SRC1[143:128])
/// DEST[159:144] = SaturateToSignedWord(SRC1[191:176] + SRC1[175:160])
/// DEST[175:160] = SaturateToSignedWord( SRC1[223:208] + SRC1[207:192])
/// DEST[191:176] = SaturateToSignedWord(SRC1[255:240] + SRC1[239:224])
/// DEST[207:192] = SaturateToSignedWord(SRC2[127:112] + SRC2[143:128])
/// DEST[223:208] = SaturateToSignedWord(SRC2[159:144] + SRC2[175:160])
/// DEST[239:224] = SaturateToSignedWord(SRC2[191-160] + SRC2[159-128])
/// DEST[255:240] = SaturateToSignedWord(SRC2[255:240] + SRC2[239:224])
/// ```
#[box_to_static_reference]
pub(super) fn phaddsw() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PHADDW (With 64-bit Operands)
///     mm1[15-0]  = mm1[31-16] + mm1[15-0];
///     mm1[31-16] = mm1[63-48] + mm1[47-32];
///     mm1[47-32] = mm2/m64[31-16] + mm2/m64[15-0];
///     mm1[63-48] = mm2/m64[63-48] + mm2/m64[47-32];
/// PHADDW (With 128-bit Operands)
///     xmm1[15-0] = xmm1[31-16] + xmm1[15-0];
///     xmm1[31-16] = xmm1[63-48] + xmm1[47-32];
///     xmm1[47-32] = xmm1[95-80] + xmm1[79-64];
///     xmm1[63-48] = xmm1[127-112] + xmm1[111-96];
///     xmm1[79-64] = xmm2/m128[31-16] + xmm2/m128[15-0];
///     xmm1[95-80] = xmm2/m128[63-48] + xmm2/m128[47-32];
///     xmm1[111-96] = xmm2/m128[95-80] + xmm2/m128[79-64];
///     xmm1[127-112] = xmm2/m128[127-112] + xmm2/m128[111-96];
/// VPHADDW (VEX.128 Encoded Version)
/// DEST[15:0] := SRC1[31:16] + SRC1[15:0]
/// DEST[31:16] := SRC1[63:48] + SRC1[47:32]
/// DEST[47:32] := SRC1[95:80] + SRC1[79:64]
/// DEST[63:48] := SRC1[127:112] + SRC1[111:96]
/// DEST[79:64] := SRC2[31:16] + SRC2[15:0]
/// DEST[95:80] := SRC2[63:48] + SRC2[47:32]
/// DEST[111:96] := SRC2[95:80] + SRC2[79:64]
/// DEST[127:112] := SRC2[127:112] + SRC2[111:96]
/// DEST[MAXVL-1:128] := 0
/// VPHADDW (VEX.256 Encoded Version)
/// DEST[15:0] := SRC1[31:16] + SRC1[15:0]
/// DEST[31:16] := SRC1[63:48] + SRC1[47:32]
/// DEST[47:32] := SRC1[95:80] + SRC1[79:64]
/// DEST[63:48] := SRC1[127:112] + SRC1[111:96]
/// DEST[79:64] := SRC2[31:16] + SRC2[15:0]
/// DEST[95:80] := SRC2[63:48] + SRC2[47:32]
/// DEST[111:96] := SRC2[95:80] + SRC2[79:64]
/// DEST[127:112] := SRC2[127:112] + SRC2[111:96]
/// DEST[143:128] := SRC1[159:144] + SRC1[143:128]
/// DEST[159:144] := SRC1[191:176] + SRC1[175:160]
/// DEST[175:160] := SRC1[223:208] + SRC1[207:192]
/// DEST[191:176] := SRC1[255:240] + SRC1[239:224]
/// DEST[207:192] := SRC2[127:112] + SRC2[143:128]
/// DEST[223:208] := SRC2[159:144] + SRC2[175:160]
/// DEST[239:224] := SRC2[191:176] + SRC2[207:192]
/// DEST[255:240] := SRC2[223:208] + SRC2[239:224]
/// PHADDD (With 64-bit Operands)
///     mm1[31-0]  = mm1[63-32] + mm1[31-0];
///     mm1[63-32] = mm2/m64[63-32] + mm2/m64[31-0];
/// PHADDD (With 128-bit Operands)
///     xmm1[31-0] = xmm1[63-32] + xmm1[31-0];
///     xmm1[63-32] = xmm1[127-96] + xmm1[95-64];
///     xmm1[95-64] = xmm2/m128[63-32] + xmm2/m128[31-0];
///     xmm1[127-96] = xmm2/m128[127-96] + xmm2/m128[95-64];
/// VPHADDD (VEX.128 Encoded Version)
/// DEST[31-0] := SRC1[63-32] + SRC1[31-0]
/// DEST[63-32] := SRC1[127-96] + SRC1[95-64]
/// DEST[95-64] := SRC2[63-32] + SRC2[31-0]
/// DEST[127-96] := SRC2[127-96] + SRC2[95-64]
/// DEST[MAXVL-1:128] := 0
/// VPHADDD (VEX.256 Encoded Version)
/// DEST[31-0] := SRC1[63-32] + SRC1[31-0]
/// DEST[63-32] := SRC1[127-96] + SRC1[95-64]
/// DEST[95-64] := SRC2[63-32] + SRC2[31-0]
/// DEST[127-96] := SRC2[127-96] + SRC2[95-64]
/// DEST[159-128] := SRC1[191-160] + SRC1[159-128]
/// DEST[191-160] := SRC1[255-224] + SRC1[223-192]
/// DEST[223-192] := SRC2[191-160] + SRC2[159-128]
/// DEST[255-224] := SRC2[255-224] + SRC2[223-192]
/// ```
#[box_to_static_reference]
pub(super) fn phaddw() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PHMINPOSUW (128-bit Legacy SSE Version)
/// INDEX := 0;
/// MIN := SRC[15:0]
/// IF (SRC[31:16] < MIN)
///     THEN INDEX := 1;  MIN := SRC[31:16]; FI;
/// IF (SRC[47:32] < MIN)
///     THEN INDEX := 2;  MIN := SRC[47:32]; FI;
/// * Repeat operation for words 3 through 6
/// IF (SRC[127:112] < MIN)
///     THEN INDEX := 7;  MIN := SRC[127:112]; FI;
/// DEST[15:0] := MIN;
/// DEST[18:16] := INDEX;
/// DEST[127:19] := 0000000000000000000000000000H;
/// VPHMINPOSUW (VEX.128 Encoded Version)
/// INDEX := 0
/// MIN := SRC[15:0]
/// IF (SRC[31:16] < MIN) THEN INDEX := 1; MIN := SRC[31:16]
/// IF (SRC[47:32] < MIN) THEN INDEX := 2; MIN := SRC[47:32]
/// * Repeat operation for words 3 through 6
/// IF (SRC[127:112] < MIN) THEN INDEX := 7; MIN := SRC[127:112]
/// DEST[15:0] := MIN
/// DEST[18:16] := INDEX
/// DEST[127:19] := 0000000000000000000000000000H
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn phminposuw() -> &'static [IrStatement] {
    let assignment = assign(o1(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PHSUBW (With 64-bit Operands)
///     mm1[15-0] = mm1[15-0] - mm1[31-16];
///     mm1[31-16] = mm1[47-32] - mm1[63-48];
///     mm1[47-32] = mm2/m64[15-0] - mm2/m64[31-16];
///     mm1[63-48] = mm2/m64[47-32] - mm2/m64[63-48];
/// PHSUBW (With 128-bit Operands)
///     xmm1[15-0] = xmm1[15-0] - xmm1[31-16];
///     xmm1[31-16] = xmm1[47-32] - xmm1[63-48];
///     xmm1[47-32] = xmm1[79-64] - xmm1[95-80];
///     xmm1[63-48] = xmm1[111-96] - xmm1[127-112];
///     xmm1[79-64] = xmm2/m128[15-0] - xmm2/m128[31-16];
///     xmm1[95-80] = xmm2/m128[47-32] - xmm2/m128[63-48];
///     xmm1[111-96] = xmm2/m128[79-64] - xmm2/m128[95-80];
///     xmm1[127-112] = xmm2/m128[111-96] - xmm2/m128[127-112];
/// VPHSUBW (VEX.128 Encoded Version)
/// DEST[15:0] := SRC1[15:0] - SRC1[31:16]
/// DEST[31:16] := SRC1[47:32] - SRC1[63:48]
/// DEST[47:32] := SRC1[79:64] - SRC1[95:80]
/// DEST[63:48] := SRC1[111:96] - SRC1[127:112]
/// DEST[79:64] := SRC2[15:0] - SRC2[31:16]
/// DEST[95:80] := SRC2[47:32] - SRC2[63:48]
/// DEST[111:96] := SRC2[79:64] - SRC2[95:80]
/// DEST[127:112] := SRC2[111:96] - SRC2[127:112]
/// DEST[MAXVL-1:128] := 0
/// VPHSUBW (VEX.256 Encoded Version)
/// DEST[15:0] := SRC1[15:0] - SRC1[31:16]
/// DEST[31:16] := SRC1[47:32] - SRC1[63:48]
/// DEST[47:32] := SRC1[79:64] - SRC1[95:80]
/// DEST[63:48] := SRC1[111:96] - SRC1[127:112]
/// DEST[79:64] := SRC2[15:0] - SRC2[31:16]
/// DEST[95:80] := SRC2[47:32] - SRC2[63:48]
/// DEST[111:96] := SRC2[79:64] - SRC2[95:80]
/// DEST[127:112] := SRC2[111:96] - SRC2[127:112]
/// DEST[143:128] := SRC1[143:128] - SRC1[159:144]
/// DEST[159:144] := SRC1[175:160] - SRC1[191:176]
/// DEST[175:160] := SRC1[207:192] - SRC1[223:208]
/// DEST[191:176] := SRC1[239:224] - SRC1[255:240]
/// DEST[207:192] := SRC2[143:128] - SRC2[159:144]
/// DEST[223:208] := SRC2[175:160] - SRC2[191:176]
/// DEST[239:224] := SRC2[207:192] - SRC2[223:208]
/// DEST[255:240] := SRC2[239:224] - SRC2[255:240]
/// PHSUBD (With 64-bit Operands)
///     mm1[31-0] = mm1[31-0] - mm1[63-32];
///     mm1[63-32] = mm2/m64[31-0] - mm2/m64[63-32];
/// PHSUBD (With 128-bit Operands)
///     xmm1[31-0] = xmm1[31-0] - xmm1[63-32];
///     xmm1[63-32] = xmm1[95-64] - xmm1[127-96];
///     xmm1[95-64] = xmm2/m128[31-0] - xmm2/m128[63-32];
///     xmm1[127-96] = xmm2/m128[95-64] - xmm2/m128[127-96];
/// VPHSUBD (VEX.128 Encoded Version)
/// DEST[31-0] := SRC1[31-0] - SRC1[63-32]
/// DEST[63-32] := SRC1[95-64] - SRC1[127-96]
/// DEST[95-64] := SRC2[31-0] - SRC2[63-32]
/// DEST[127-96] := SRC2[95-64] - SRC2[127-96]
/// DEST[MAXVL-1:128] := 0
/// VPHSUBD (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] - SRC1[63:32]
/// DEST[63:32] := SRC1[95:64] - SRC1[127:96]
/// DEST[95:64] := SRC2[31:0] - SRC2[63:32]
/// DEST[127:96] := SRC2[95:64] - SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] - SRC1[191:160]
/// DEST[191:160] := SRC1[223:192] - SRC1[255:224]
/// DEST[223:192] := SRC2[159:128] - SRC2[191:160]
/// DEST[255:224] := SRC2[223:192] - SRC2[255:224]
/// ```
#[box_to_static_reference]
pub(super) fn phsubd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PHSUBSW (With 64-bit Operands)
///     mm1[15-0] = SaturateToSignedWord(mm1[15-0] - mm1[31-16]);
///     mm1[31-16] = SaturateToSignedWord(mm1[47-32] - mm1[63-48]);
///     mm1[47-32] = SaturateToSignedWord(mm2/m64[15-0] - mm2/m64[31-16]);
///     mm1[63-48] = SaturateToSignedWord(mm2/m64[47-32] - mm2/m64[63-48]);
/// PHSUBSW (With 128-bit Operands)
///     xmm1[15-0] = SaturateToSignedWord(xmm1[15-0] - xmm1[31-16]);
///     xmm1[31-16] = SaturateToSignedWord(xmm1[47-32] - xmm1[63-48]);
///     xmm1[47-32] = SaturateToSignedWord(xmm1[79-64] - xmm1[95-80]);
///     xmm1[63-48] = SaturateToSignedWord(xmm1[111-96] - xmm1[127-112]);
///     xmm1[79-64] = SaturateToSignedWord(xmm2/m128[15-0] - xmm2/m128[31-16]);
///     xmm1[95-80] =SaturateToSignedWord(xmm2/m128[47-32] - xmm2/m128[63-48]);
///     xmm1[111-96] =SaturateToSignedWord(xmm2/m128[79-64]- xmm2/m128[95-80]);
///     xmm1[127-112]= SaturateToSignedWord(xmm2/m128[111-96] - xmm2/m128[127-112]);
/// VPHSUBSW (VEX.128 Encoded Version)
/// DEST[15:0]= SaturateToSignedWord(SRC1[15:0] - SRC1[31:16])
/// DEST[31:16] = SaturateToSignedWord(SRC1[47:32] - SRC1[63:48])
/// DEST[47:32] = SaturateToSignedWord(SRC1[79:64] - SRC1[95:80])
/// DEST[63:48] = SaturateToSignedWord(SRC1[111:96] - SRC1[127:112])
/// DEST[79:64] = SaturateToSignedWord(SRC2[15:0] - SRC2[31:16])
/// DEST[95:80] = SaturateToSignedWord(SRC2[47:32] - SRC2[63:48])
/// DEST[111:96] = SaturateToSignedWord(SRC2[79:64] - SRC2[95:80])
/// DEST[127:112] = SaturateToSignedWord(SRC2[111:96] - SRC2[127:112])
/// DEST[MAXVL-1:128] := 0
/// VPHSUBSW (VEX.256 Encoded Version)
/// DEST[15:0]= SaturateToSignedWord(SRC1[15:0] - SRC1[31:16])
/// DEST[31:16] = SaturateToSignedWord(SRC1[47:32] - SRC1[63:48])
/// DEST[47:32] = SaturateToSignedWord(SRC1[79:64] - SRC1[95:80])
/// DEST[63:48] = SaturateToSignedWord(SRC1[111:96] - SRC1[127:112])
/// DEST[79:64] = SaturateToSignedWord(SRC2[15:0] - SRC2[31:16])
/// DEST[95:80] = SaturateToSignedWord(SRC2[47:32] - SRC2[63:48])
/// DEST[111:96] = SaturateToSignedWord(SRC2[79:64] - SRC2[95:80])
/// DEST[127:112] = SaturateToSignedWord(SRC2[111:96] - SRC2[127:112])
/// DEST[143:128]= SaturateToSignedWord(SRC1[143:128] - SRC1[159:144])
/// DEST[159:144] = SaturateToSignedWord(SRC1[175:160] - SRC1[191:176])
/// DEST[175:160] = SaturateToSignedWord(SRC1[207:192] - SRC1[223:208])
/// DEST[191:176] = SaturateToSignedWord(SRC1[239:224] - SRC1[255:240])
/// DEST[207:192] = SaturateToSignedWord(SRC2[143:128] - SRC2[159:144])
/// DEST[223:208] = SaturateToSignedWord(SRC2[175:160] - SRC2[191:176])
/// DEST[239:224] = SaturateToSignedWord(SRC2[207:192] - SRC2[223:208])
/// DEST[255:240] = SaturateToSignedWord(SRC2[239:224] - SRC2[255:240])
/// ```
#[box_to_static_reference]
pub(super) fn phsubsw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PHSUBW (With 64-bit Operands)
///     mm1[15-0] = mm1[15-0] - mm1[31-16];
///     mm1[31-16] = mm1[47-32] - mm1[63-48];
///     mm1[47-32] = mm2/m64[15-0] - mm2/m64[31-16];
///     mm1[63-48] = mm2/m64[47-32] - mm2/m64[63-48];
/// PHSUBW (With 128-bit Operands)
///     xmm1[15-0] = xmm1[15-0] - xmm1[31-16];
///     xmm1[31-16] = xmm1[47-32] - xmm1[63-48];
///     xmm1[47-32] = xmm1[79-64] - xmm1[95-80];
///     xmm1[63-48] = xmm1[111-96] - xmm1[127-112];
///     xmm1[79-64] = xmm2/m128[15-0] - xmm2/m128[31-16];
///     xmm1[95-80] = xmm2/m128[47-32] - xmm2/m128[63-48];
///     xmm1[111-96] = xmm2/m128[79-64] - xmm2/m128[95-80];
///     xmm1[127-112] = xmm2/m128[111-96] - xmm2/m128[127-112];
/// VPHSUBW (VEX.128 Encoded Version)
/// DEST[15:0] := SRC1[15:0] - SRC1[31:16]
/// DEST[31:16] := SRC1[47:32] - SRC1[63:48]
/// DEST[47:32] := SRC1[79:64] - SRC1[95:80]
/// DEST[63:48] := SRC1[111:96] - SRC1[127:112]
/// DEST[79:64] := SRC2[15:0] - SRC2[31:16]
/// DEST[95:80] := SRC2[47:32] - SRC2[63:48]
/// DEST[111:96] := SRC2[79:64] - SRC2[95:80]
/// DEST[127:112] := SRC2[111:96] - SRC2[127:112]
/// DEST[MAXVL-1:128] := 0
/// VPHSUBW (VEX.256 Encoded Version)
/// DEST[15:0] := SRC1[15:0] - SRC1[31:16]
/// DEST[31:16] := SRC1[47:32] - SRC1[63:48]
/// DEST[47:32] := SRC1[79:64] - SRC1[95:80]
/// DEST[63:48] := SRC1[111:96] - SRC1[127:112]
/// DEST[79:64] := SRC2[15:0] - SRC2[31:16]
/// DEST[95:80] := SRC2[47:32] - SRC2[63:48]
/// DEST[111:96] := SRC2[79:64] - SRC2[95:80]
/// DEST[127:112] := SRC2[111:96] - SRC2[127:112]
/// DEST[143:128] := SRC1[143:128] - SRC1[159:144]
/// DEST[159:144] := SRC1[175:160] - SRC1[191:176]
/// DEST[175:160] := SRC1[207:192] - SRC1[223:208]
/// DEST[191:176] := SRC1[239:224] - SRC1[255:240]
/// DEST[207:192] := SRC2[143:128] - SRC2[159:144]
/// DEST[223:208] := SRC2[175:160] - SRC2[191:176]
/// DEST[239:224] := SRC2[207:192] - SRC2[223:208]
/// DEST[255:240] := SRC2[239:224] - SRC2[255:240]
/// PHSUBD (With 64-bit Operands)
///     mm1[31-0] = mm1[31-0] - mm1[63-32];
///     mm1[63-32] = mm2/m64[31-0] - mm2/m64[63-32];
/// PHSUBD (With 128-bit Operands)
///     xmm1[31-0] = xmm1[31-0] - xmm1[63-32];
///     xmm1[63-32] = xmm1[95-64] - xmm1[127-96];
///     xmm1[95-64] = xmm2/m128[31-0] - xmm2/m128[63-32];
///     xmm1[127-96] = xmm2/m128[95-64] - xmm2/m128[127-96];
/// VPHSUBD (VEX.128 Encoded Version)
/// DEST[31-0] := SRC1[31-0] - SRC1[63-32]
/// DEST[63-32] := SRC1[95-64] - SRC1[127-96]
/// DEST[95-64] := SRC2[31-0] - SRC2[63-32]
/// DEST[127-96] := SRC2[95-64] - SRC2[127-96]
/// DEST[MAXVL-1:128] := 0
/// VPHSUBD (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] - SRC1[63:32]
/// DEST[63:32] := SRC1[95:64] - SRC1[127:96]
/// DEST[95:64] := SRC2[31:0] - SRC2[63:32]
/// DEST[127:96] := SRC2[95:64] - SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] - SRC1[191:160]
/// DEST[191:160] := SRC1[223:192] - SRC1[255:224]
/// DEST[223:192] := SRC2[159:128] - SRC2[191:160]
/// DEST[255:224] := SRC2[223:192] - SRC2[255:224]
/// ```
#[box_to_static_reference]
pub(super) fn phsubw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE OF
///     PINSRB:SEL := COUNT[3:0];
///             MASK := (0FFH << (SEL * 8));
///             TEMP := (((SRC[7:0] << (SEL *8)) AND MASK);
///     PINSRD:SEL := COUNT[1:0];
///             MASK := (0FFFFFFFFH << (SEL * 32));
///             TEMP := (((SRC << (SEL *32)) AND MASK);
///     PINSRQ:SEL := COUNT[0]
///             MASK := (0FFFFFFFFFFFFFFFFH << (SEL * 64));
///             TEMP := (((SRC << (SEL *64)) AND MASK);
/// ESAC;
///         DEST := ((DEST AND NOT MASK) OR TEMP);
/// VPINSRB (VEX/EVEX Encoded Version)
/// SEL := imm8[3:0]
/// DEST[127:0] := write_b_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// VPINSRD (VEX/EVEX Encoded Version)
/// SEL := imm8[1:0]
/// DEST[127:0] := write_d_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// VPINSRQ (VEX/EVEX Encoded Version)
/// SEL := imm8[0]
/// DEST[127:0] := write_q_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pinsrb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE OF
///     PINSRB:SEL := COUNT[3:0];
///             MASK := (0FFH << (SEL * 8));
///             TEMP := (((SRC[7:0] << (SEL *8)) AND MASK);
///     PINSRD:SEL := COUNT[1:0];
///             MASK := (0FFFFFFFFH << (SEL * 32));
///             TEMP := (((SRC << (SEL *32)) AND MASK);
///     PINSRQ:SEL := COUNT[0]
///             MASK := (0FFFFFFFFFFFFFFFFH << (SEL * 64));
///             TEMP := (((SRC << (SEL *64)) AND MASK);
/// ESAC;
///         DEST := ((DEST AND NOT MASK) OR TEMP);
/// VPINSRB (VEX/EVEX Encoded Version)
/// SEL := imm8[3:0]
/// DEST[127:0] := write_b_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// VPINSRD (VEX/EVEX Encoded Version)
/// SEL := imm8[1:0]
/// DEST[127:0] := write_d_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// VPINSRQ (VEX/EVEX Encoded Version)
/// SEL := imm8[0]
/// DEST[127:0] := write_q_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pinsrd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// CASE OF
///     PINSRB:SEL := COUNT[3:0];
///             MASK := (0FFH << (SEL * 8));
///             TEMP := (((SRC[7:0] << (SEL *8)) AND MASK);
///     PINSRD:SEL := COUNT[1:0];
///             MASK := (0FFFFFFFFH << (SEL * 32));
///             TEMP := (((SRC << (SEL *32)) AND MASK);
///     PINSRQ:SEL := COUNT[0]
///             MASK := (0FFFFFFFFFFFFFFFFH << (SEL * 64));
///             TEMP := (((SRC << (SEL *64)) AND MASK);
/// ESAC;
///         DEST := ((DEST AND NOT MASK) OR TEMP);
/// VPINSRB (VEX/EVEX Encoded Version)
/// SEL := imm8[3:0]
/// DEST[127:0] := write_b_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// VPINSRD (VEX/EVEX Encoded Version)
/// SEL := imm8[1:0]
/// DEST[127:0] := write_d_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// VPINSRQ (VEX/EVEX Encoded Version)
/// SEL := imm8[0]
/// DEST[127:0] := write_q_element(SEL, SRC2, SRC1)
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pinsrq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PINSRW dest, src, imm8 (MMX)
///     SEL := imm8[1:0]
/// DEST.word[SEL] := src.word[0]
/// PINSRW dest, src, imm8 (SSE)
///     SEL := imm8[2:0]
/// DEST.word[SEL] := src.word[0]
/// VPINSRW dest, src1, src2, imm8 (AVX/AVX512)
///     SEL := imm8[2:0]
///     DEST := src1
/// DEST.word[SEL] := src2.word[0]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pinsrw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMADDUBSW (With 64-bit Operands)
///     DEST[15-0] = SaturateToSignedWord(SRC[15-8]*DEST[15-8]+SRC[7-0]*DEST[7-0]);
///     DEST[31-16] = SaturateToSignedWord(SRC[31-24]*DEST[31-24]+SRC[23-16]*DEST[23-16]);
///     DEST[47-32] = SaturateToSignedWord(SRC[47-40]*DEST[47-40]+SRC[39-32]*DEST[39-32]);
///     DEST[63-48] = SaturateToSignedWord(SRC[63-56]*DEST[63-56]+SRC[55-48]*DEST[55-48]);
/// PMADDUBSW (With 128-bit Operands)
///     DEST[15-0] = SaturateToSignedWord(SRC[15-8]* DEST[15-8]+SRC[7-0]*DEST[7-0]);
///     // Repeat operation for 2nd through 7th word
///     SRC1/DEST[127-112] = SaturateToSignedWord(SRC[127-120]*DEST[127-120]+ SRC[119-112]* DEST[119-112]);
/// VPMADDUBSW (VEX.128 Encoded Version)
/// DEST[15:0] := SaturateToSignedWord(SRC2[15:8]* SRC1[15:8]+SRC2[7:0]*SRC1[7:0])
/// // Repeat operation for 2nd through 7th word
/// DEST[127:112] := SaturateToSignedWord(SRC2[127:120]*SRC1[127:120]+ SRC2[119:112]* SRC1[119:112])
/// DEST[MAXVL-1:128] := 0
/// VPMADDUBSW (VEX.256 Encoded Version)
/// DEST[15:0] := SaturateToSignedWord(SRC2[15:8]* SRC1[15:8]+SRC2[7:0]*SRC1[7:0])
/// // Repeat operation for 2nd through 15th word
/// DEST[255:240] := SaturateToSignedWord(SRC2[255:248]*SRC1[255:248]+ SRC2[247:240]* SRC1[247:240])
/// DEST[MAXVL-1:256] := 0
/// VPMADDUBSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToSignedWord(SRC2[i+15:i+8]* SRC1[i+15:i+8] + SRC2[i+7:i]*SRC1[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaddubsw() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMADDWD (With 64-bit Operands)
///     DEST[31:0] := (DEST[15:0] * SRC[15:0]) + (DEST[31:16] * SRC[31:16]);
///     DEST[63:32] := (DEST[47:32] * SRC[47:32]) + (DEST[63:48] * SRC[63:48]);
/// PMADDWD (With 128-bit Operands)
///     DEST[31:0] := (DEST[15:0] * SRC[15:0]) + (DEST[31:16] * SRC[31:16]);
///     DEST[63:32] := (DEST[47:32] * SRC[47:32]) + (DEST[63:48] * SRC[63:48]);
///     DEST[95:64] := (DEST[79:64] * SRC[79:64]) + (DEST[95:80] * SRC[95:80]);
///     DEST[127:96] := (DEST[111:96] * SRC[111:96]) + (DEST[127:112] * SRC[127:112]);
/// VPMADDWD (VEX.128 Encoded Version)
/// DEST[31:0] := (SRC1[15:0] * SRC2[15:0]) + (SRC1[31:16] * SRC2[31:16])
/// DEST[63:32] := (SRC1[47:32] * SRC2[47:32]) + (SRC1[63:48] * SRC2[63:48])
/// DEST[95:64] := (SRC1[79:64] * SRC2[79:64]) + (SRC1[95:80] * SRC2[95:80])
/// DEST[127:96] := (SRC1[111:96] * SRC2[111:96]) + (SRC1[127:112] * SRC2[127:112])
/// DEST[MAXVL-1:128] := 0
/// VPMADDWD (VEX.256 Encoded Version)
/// DEST[31:0] := (SRC1[15:0] * SRC2[15:0]) + (SRC1[31:16] * SRC2[31:16])
/// DEST[63:32] := (SRC1[47:32] * SRC2[47:32]) + (SRC1[63:48] * SRC2[63:48])
/// DEST[95:64] := (SRC1[79:64] * SRC2[79:64]) + (SRC1[95:80] * SRC2[95:80])
/// DEST[127:96] := (SRC1[111:96] * SRC2[111:96]) + (SRC1[127:112] * SRC2[127:112])
/// DEST[159:128] := (SRC1[143:128] * SRC2[143:128]) + (SRC1[159:144] * SRC2[159:144])
/// DEST[191:160] := (SRC1[175:160] * SRC2[175:160]) + (SRC1[191:176] * SRC2[191:176])
/// DEST[223:192] := (SRC1[207:192] * SRC2[207:192]) + (SRC1[223:208] * SRC2[223:208])
/// DEST[255:224] := (SRC1[239:224] * SRC2[239:224]) + (SRC1[255:240] * SRC2[255:240])
/// DEST[MAXVL-1:256] := 0
/// VPMADDWD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := (SRC2[i+31:i+16]* SRC1[i+31:i+16]) + (SRC2[i+15:i]*SRC1[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+31:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaddwd() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMAXSW (64-bit Operands)
///     IF DEST[15:0] > SRC[15:0]) THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd and 3rd words in source and destination operands *)
///     IF DEST[63:48] > SRC[63:48]) THEN
///         DEST[63:48] := DEST[63:48];
///     ELSE
///         DEST[63:48] := SRC[63:48]; FI;
/// PMAXSB (128-bit Legacy SSE Version)
///     IF DEST[7:0] > SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[7:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] >SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSB (VEX.128 Encoded Version)
///     IF SRC1[7:0] > SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] >SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSB (VEX.256 Encoded Version)
///     IF SRC1[7:0] > SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] >SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMAXSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] > SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXSW (128-bit Legacy SSE Version)
///     IF DEST[15:0] >SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] >SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSW (VEX.128 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] >SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSW (VEX.256 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] >SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// VPMAXSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] > SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXSD (128-bit Legacy SSE Version)
///     IF DEST[31:0] >SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] >SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSD (VEX.128 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] > SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSD (VEX.256 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] > SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// VPMAXSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] > SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] > SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE  DEST[i+31:i] := 0
///                             ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMAXSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] > SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+63:i] > SRC2[i+63:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaxsb() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMAXSW (64-bit Operands)
///     IF DEST[15:0] > SRC[15:0]) THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd and 3rd words in source and destination operands *)
///     IF DEST[63:48] > SRC[63:48]) THEN
///         DEST[63:48] := DEST[63:48];
///     ELSE
///         DEST[63:48] := SRC[63:48]; FI;
/// PMAXSB (128-bit Legacy SSE Version)
///     IF DEST[7:0] > SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[7:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] >SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSB (VEX.128 Encoded Version)
///     IF SRC1[7:0] > SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] >SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSB (VEX.256 Encoded Version)
///     IF SRC1[7:0] > SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] >SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMAXSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] > SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXSW (128-bit Legacy SSE Version)
///     IF DEST[15:0] >SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] >SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSW (VEX.128 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] >SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSW (VEX.256 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] >SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// VPMAXSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] > SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXSD (128-bit Legacy SSE Version)
///     IF DEST[31:0] >SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] >SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSD (VEX.128 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] > SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSD (VEX.256 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] > SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// VPMAXSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] > SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] > SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE  DEST[i+31:i] := 0
///                             ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMAXSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] > SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+63:i] > SRC2[i+63:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaxsd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMAXSW (64-bit Operands)
///     IF DEST[15:0] > SRC[15:0]) THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd and 3rd words in source and destination operands *)
///     IF DEST[63:48] > SRC[63:48]) THEN
///         DEST[63:48] := DEST[63:48];
///     ELSE
///         DEST[63:48] := SRC[63:48]; FI;
/// PMAXSB (128-bit Legacy SSE Version)
///     IF DEST[7:0] > SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[7:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] >SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSB (VEX.128 Encoded Version)
///     IF SRC1[7:0] > SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] >SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSB (VEX.256 Encoded Version)
///     IF SRC1[7:0] > SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] >SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMAXSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] > SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXSW (128-bit Legacy SSE Version)
///     IF DEST[15:0] >SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] >SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSW (VEX.128 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] >SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSW (VEX.256 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] >SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// VPMAXSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] > SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXSD (128-bit Legacy SSE Version)
///     IF DEST[31:0] >SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] >SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSD (VEX.128 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] > SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSD (VEX.256 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] > SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// VPMAXSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] > SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] > SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE  DEST[i+31:i] := 0
///                             ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMAXSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] > SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+63:i] > SRC2[i+63:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaxsq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMAXSW (64-bit Operands)
///     IF DEST[15:0] > SRC[15:0]) THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd and 3rd words in source and destination operands *)
///     IF DEST[63:48] > SRC[63:48]) THEN
///         DEST[63:48] := DEST[63:48];
///     ELSE
///         DEST[63:48] := SRC[63:48]; FI;
/// PMAXSB (128-bit Legacy SSE Version)
///     IF DEST[7:0] > SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[7:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] >SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSB (VEX.128 Encoded Version)
///     IF SRC1[7:0] > SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] >SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSB (VEX.256 Encoded Version)
///     IF SRC1[7:0] > SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] >SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMAXSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] > SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXSW (128-bit Legacy SSE Version)
///     IF DEST[15:0] >SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] >SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSW (VEX.128 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] >SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSW (VEX.256 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] >SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// VPMAXSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] > SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXSD (128-bit Legacy SSE Version)
///     IF DEST[31:0] >SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] >SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXSD (VEX.128 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] > SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXSD (VEX.256 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] > SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// VPMAXSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] > SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] > SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///                     ELSE  DEST[i+31:i] := 0
///                             ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMAXSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] > SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+63:i] > SRC2[i+63:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaxsw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMAXUB (64-bit Operands)
///     IF DEST[7:0] > SRC[17:0]) THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[7:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 7th bytes in source and destination operands *)
///     IF DEST[63:56] > SRC[63:56]) THEN
///         DEST[63:56] := DEST[63:56];
///     ELSE
///         DEST[63:56] := SRC[63:56]; FI;
/// PMAXUB (128-bit Legacy SSE Version)
///     IF DEST[7:0] >SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[15:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] >SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXUB (VEX.128 Encoded Version)
///     IF SRC1[7:0] >SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] >SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUB (VEX.256 Encoded Version)
///     IF SRC1[7:0] >SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[15:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] >SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] > SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXUW (128-bit Legacy SSE Version)
///     IF DEST[15:0] >SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] >SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// VPMAXUW (VEX.128 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] >SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUW (VEX.256 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] >SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] > SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaxub() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMAXUD (128-bit Legacy SSE Version)
///     IF DEST[31:0] >SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] >SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXUD (VEX.128 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] > SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUD (VEX.256 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] > SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMAXUD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] > SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] > SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPMAXUQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] > SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] > SRC2[i+31:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaxud() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMAXUD (128-bit Legacy SSE Version)
///     IF DEST[31:0] >SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] >SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXUD (VEX.128 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] > SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUD (VEX.256 Encoded Version)
///     IF SRC1[31:0] > SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] > SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMAXUD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] > SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] > SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPMAXUQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] > SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] > SRC2[i+31:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         THEN DEST[i+63:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaxuq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMAXUB (64-bit Operands)
///     IF DEST[7:0] > SRC[17:0]) THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[7:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 7th bytes in source and destination operands *)
///     IF DEST[63:56] > SRC[63:56]) THEN
///         DEST[63:56] := DEST[63:56];
///     ELSE
///         DEST[63:56] := SRC[63:56]; FI;
/// PMAXUB (128-bit Legacy SSE Version)
///     IF DEST[7:0] >SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[15:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] >SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMAXUB (VEX.128 Encoded Version)
///     IF SRC1[7:0] >SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] >SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUB (VEX.256 Encoded Version)
///     IF SRC1[7:0] >SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[15:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] >SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] > SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMAXUW (128-bit Legacy SSE Version)
///     IF DEST[15:0] >SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] >SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// VPMAXUW (VEX.128 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] >SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUW (VEX.256 Encoded Version)
///     IF SRC1[15:0] > SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] >SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMAXUW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] > SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmaxuw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMINSW (64-bit Operands)
///     IF DEST[15:0] < SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd and 3rd words in source and destination operands *)
///     IF DEST[63:48] < SRC[63:48] THEN
///         DEST[63:48] := DEST[63:48];
///     ELSE
///         DEST[63:48] := SRC[63:48]; FI;
/// PMINSB (128-bit Legacy SSE Version)
///     IF DEST[7:0] < SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[15:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] < SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINSB (VEX.128 Encoded Version)
///     IF SRC1[7:0] < SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] < SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINSB (VEX.256 Encoded Version)
///     IF SRC1[7:0] < SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[15:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] < SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] < SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMINSW (128-bit Legacy SSE Version)
///     IF DEST[15:0] < SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] < SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINSW (VEX.128 Encoded Version)
///     IF SRC1[15:0] < SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] < SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// VPMINSW (VEX.256 Encoded Version)
///     IF SRC1[15:0] < SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] < SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] < SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pminsb() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMINSD (128-bit Legacy SSE Version)
///     IF DEST[31:0] < SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] < SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINSD (VEX.128 Encoded Version)
///     IF SRC1[31:0] < SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] < SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINSD (VEX.256 Encoded Version)
///     IF SRC1[31:0] < SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] < SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] < SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] < SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
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
/// VPMINSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] < SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+63:i] < SRC2[i+63:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
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
pub(super) fn pminsd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMINSD (128-bit Legacy SSE Version)
///     IF DEST[31:0] < SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] < SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINSD (VEX.128 Encoded Version)
///     IF SRC1[31:0] < SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] < SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINSD (VEX.256 Encoded Version)
///     IF SRC1[31:0] < SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] < SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINSD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] < SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] < SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
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
/// VPMINSQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] < SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+63:i] < SRC2[i+63:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
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
pub(super) fn pminsq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMINSW (64-bit Operands)
///     IF DEST[15:0] < SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd and 3rd words in source and destination operands *)
///     IF DEST[63:48] < SRC[63:48] THEN
///         DEST[63:48] := DEST[63:48];
///     ELSE
///         DEST[63:48] := SRC[63:48]; FI;
/// PMINSB (128-bit Legacy SSE Version)
///     IF DEST[7:0] < SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[15:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] < SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINSB (VEX.128 Encoded Version)
///     IF SRC1[7:0] < SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] < SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINSB (VEX.256 Encoded Version)
///     IF SRC1[7:0] < SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[15:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] < SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] < SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMINSW (128-bit Legacy SSE Version)
///     IF DEST[15:0] < SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] < SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINSW (VEX.128 Encoded Version)
///     IF SRC1[15:0] < SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] < SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// VPMINSW (VEX.256 Encoded Version)
///     IF SRC1[15:0] < SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] < SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] < SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pminsw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMINUB (64-bit Operands)
///     IF DEST[7:0] < SRC[17:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[7:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 7th bytes in source and destination operands *)
///     IF DEST[63:56] < SRC[63:56] THEN
///         DEST[63:56] := DEST[63:56];
///     ELSE
///         DEST[63:56] := SRC[63:56]; FI;
/// PMINUB (128-bit Operands)
///     IF DEST[7:0] < SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[15:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] < SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINUB (VEX.128 Encoded Version)
///     IF SRC1[7:0] < SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] < SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINUB (VEX.256 Encoded Version)
///     IF SRC1[7:0] < SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[15:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] < SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINUB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] < SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMINUW (128-bit Operands)
///     IF DEST[15:0] < SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] < SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// VPMINUW (VEX.128 Encoded Version)
///     IF SRC1[15:0] < SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] < SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINUW (VEX.256 Encoded Version)
///     IF SRC1[15:0] < SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] < SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINUW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] < SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pminub() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMINUD (128-bit Legacy SSE Version)
/// PMINUD instruction for 128-bit operands:
///     IF DEST[31:0] < SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] < SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINUD (VEX.128 Encoded Version)
/// VPMINUD instruction for 128-bit operands:
///     IF SRC1[31:0] < SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] < SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINUD (VEX.256 Encoded Version)
/// VPMINUD instruction for 128-bit operands:
///     IF SRC1[31:0] < SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] < SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINUD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] < SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] < SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
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
/// VPMINUQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] < SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+63:i] < SRC2[i+63:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
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
pub(super) fn pminud() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMINUD (128-bit Legacy SSE Version)
/// PMINUD instruction for 128-bit operands:
///     IF DEST[31:0] < SRC[31:0] THEN
///         DEST[31:0] := DEST[31:0];
///     ELSE
///         DEST[31:0] := SRC[31:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:96] < SRC[127:96] THEN
///         DEST[127:96] := DEST[127:96];
///     ELSE
///         DEST[127:96] := SRC[127:96]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINUD (VEX.128 Encoded Version)
/// VPMINUD instruction for 128-bit operands:
///     IF SRC1[31:0] < SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 3rd dwords in source and destination operands *)
///     IF SRC1[127:96] < SRC2[127:96] THEN
///         DEST[127:96] := SRC1[127:96];
///     ELSE
///         DEST[127:96] := SRC2[127:96]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINUD (VEX.256 Encoded Version)
/// VPMINUD instruction for 128-bit operands:
///     IF SRC1[31:0] < SRC2[31:0] THEN
///         DEST[31:0] := SRC1[31:0];
///     ELSE
///         DEST[31:0] := SRC2[31:0]; FI;
///     (* Repeat operation for 2nd through 7th dwords in source and destination operands *)
///     IF SRC1[255:224] < SRC2[255:224] THEN
///         DEST[255:224] := SRC1[255:224];
///     ELSE
///         DEST[255:224] := SRC2[255:224]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINUD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+31:i] < SRC2[31:0]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[31:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+31:i] < SRC2[i+31:i]
///                         THEN DEST[i+31:i] := SRC1[i+31:i];
///                         ELSE DEST[i+31:i] := SRC2[i+31:i];
///                 FI;
///         FI;
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
/// VPMINUQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///         IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     IF SRC1[i+63:i] < SRC2[63:0]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[63:0];
///                     FI;
///                 ELSE
///                     IF SRC1[i+63:i] < SRC2[i+63:i]
///                         THEN DEST[i+63:i] := SRC1[i+63:i];
///                         ELSE DEST[i+63:i] := SRC2[i+63:i];
///                 FI;
///         FI;
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
pub(super) fn pminuq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMINUB (64-bit Operands)
///     IF DEST[7:0] < SRC[17:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[7:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 7th bytes in source and destination operands *)
///     IF DEST[63:56] < SRC[63:56] THEN
///         DEST[63:56] := DEST[63:56];
///     ELSE
///         DEST[63:56] := SRC[63:56]; FI;
/// PMINUB (128-bit Operands)
///     IF DEST[7:0] < SRC[7:0] THEN
///         DEST[7:0] := DEST[7:0];
///     ELSE
///         DEST[15:0] := SRC[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF DEST[127:120] < SRC[127:120] THEN
///         DEST[127:120] := DEST[127:120];
///     ELSE
///         DEST[127:120] := SRC[127:120]; FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// VPMINUB (VEX.128 Encoded Version)
///     IF SRC1[7:0] < SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[7:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 15th bytes in source and destination operands *)
///     IF SRC1[127:120] < SRC2[127:120] THEN
///         DEST[127:120] := SRC1[127:120];
///     ELSE
///         DEST[127:120] := SRC2[127:120]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINUB (VEX.256 Encoded Version)
///     IF SRC1[7:0] < SRC2[7:0] THEN
///         DEST[7:0] := SRC1[7:0];
///     ELSE
///         DEST[15:0] := SRC2[7:0]; FI;
///     (* Repeat operation for 2nd through 31st bytes in source and destination operands *)
///     IF SRC1[255:248] < SRC2[255:248] THEN
///         DEST[255:248] := SRC1[255:248];
///     ELSE
///         DEST[255:248] := SRC2[255:248]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINUB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+7:i] < SRC2[i+7:i]
///                 THEN DEST[i+7:i] := SRC1[i+7:i];
///                 ELSE DEST[i+7:i] := SRC2[i+7:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+7:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+7:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// PMINUW (128-bit Operands)
///     IF DEST[15:0] < SRC[15:0] THEN
///         DEST[15:0] := DEST[15:0];
///     ELSE
///         DEST[15:0] := SRC[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF DEST[127:112] < SRC[127:112] THEN
///         DEST[127:112] := DEST[127:112];
///     ELSE
///         DEST[127:112] := SRC[127:112]; FI;
/// VPMINUW (VEX.128 Encoded Version)
///     IF SRC1[15:0] < SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 7th words in source and destination operands *)
///     IF SRC1[127:112] < SRC2[127:112] THEN
///         DEST[127:112] := SRC1[127:112];
///     ELSE
///         DEST[127:112] := SRC2[127:112]; FI;
/// DEST[MAXVL-1:128] := 0
/// VPMINUW (VEX.256 Encoded Version)
///     IF SRC1[15:0] < SRC2[15:0] THEN
///         DEST[15:0] := SRC1[15:0];
///     ELSE
///         DEST[15:0] := SRC2[15:0]; FI;
///     (* Repeat operation for 2nd through 15th words in source and destination operands *)
///     IF SRC1[255:240] < SRC2[255:240] THEN
///         DEST[255:240] := SRC1[255:240];
///     ELSE
///         DEST[255:240] := SRC2[255:240]; FI;
/// DEST[MAXVL-1:256] := 0
/// VPMINUW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask* THEN
///         IF SRC1[i+15:i] < SRC2[i+15:i]
///                 THEN DEST[i+15:i] := SRC1[i+15:i];
///                 ELSE DEST[i+15:i] := SRC2[i+15:i];
///         FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+15:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+15:i] := 0
///                 FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pminuw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMOVMSKB (With 64-bit Source Operand and r32)
///     r32[0] := SRC[7];
///     r32[1] := SRC[15];
///     (* Repeat operation for bytes 2 through 6 *)
///     r32[7] := SRC[63];
///     r32[31:8] := ZERO_FILL;
/// (V)PMOVMSKB (With 128-bit Source Operand and r32)
///     r32[0] := SRC[7];
///     r32[1] := SRC[15];
///     (* Repeat operation for bytes 2 through 14 *)
///     r32[15] := SRC[127];
///     r32[31:16] := ZERO_FILL;
/// VPMOVMSKB (With 256-bit Source Operand and r32)
/// r32[0] := SRC[7];
/// r32[1] := SRC[15];
/// (* Repeat operation for bytes 3rd through 31*)
/// r32[31] := SRC[255];
/// PMOVMSKB (With 64-bit Source Operand and r64)
///     r64[0] := SRC[7];
///     r64[1] := SRC[15];
///     (* Repeat operation for bytes 2 through 6 *)
///     r64[7] := SRC[63];
///     r64[63:8] := ZERO_FILL;
/// (V)PMOVMSKB (With 128-bit Source Operand and r64)
///     r64[0] := SRC[7];
///     r64[1] := SRC[15];
///     (* Repeat operation for bytes 2 through 14 *)
///     r64[15] := SRC[127];
///     r64[63:16] := ZERO_FILL;
/// VPMOVMSKB (With 256-bit Source Operand and r64)
/// r64[0] := SRC[7];
/// r64[1] := SRC[15];
/// (* Repeat operation for bytes 2 through 31*)
/// r64[31] := SRC[255];
/// r64[63:32] := ZERO_FILL;
/// ```
#[box_to_static_reference]
pub(super) fn pmovmskb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Packed_Sign_Extend_BYTE_to_WORD(DEST, SRC)
/// DEST[15:0] := SignExtend(SRC[7:0]);
/// DEST[31:16] := SignExtend(SRC[15:8]);
/// DEST[47:32] := SignExtend(SRC[23:16]);
/// DEST[63:48] := SignExtend(SRC[31:24]);
/// DEST[79:64] := SignExtend(SRC[39:32]);
/// DEST[95:80] := SignExtend(SRC[47:40]);
/// DEST[111:96] := SignExtend(SRC[55:48]);
/// DEST[127:112] := SignExtend(SRC[63:56]);
/// Packed_Sign_Extend_BYTE_to_DWORD(DEST, SRC)
/// DEST[31:0] := SignExtend(SRC[7:0]);
/// DEST[63:32] := SignExtend(SRC[15:8]);
/// DEST[95:64] := SignExtend(SRC[23:16]);
/// DEST[127:96] := SignExtend(SRC[31:24]);
/// Packed_Sign_Extend_BYTE_to_QWORD(DEST, SRC)
/// DEST[63:0] := SignExtend(SRC[7:0]);
/// DEST[127:64] := SignExtend(SRC[15:8]);
/// Packed_Sign_Extend_WORD_to_DWORD(DEST, SRC)
/// DEST[31:0] := SignExtend(SRC[15:0]);
/// DEST[63:32] := SignExtend(SRC[31:16]);
/// DEST[95:64] := SignExtend(SRC[47:32]);
/// DEST[127:96] := SignExtend(SRC[63:48]);
/// Packed_Sign_Extend_WORD_to_QWORD(DEST, SRC)
/// DEST[63:0] := SignExtend(SRC[15:0]);
/// Packed_Sign_Extend_DWORD_to_QWORD(DEST, SRC)
/// DEST[63:0] := SignExtend(SRC[31:0]);
/// DEST[127:64] := SignExtend(SRC[63:32]);
/// VPMOVSXBW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// Packed_Sign_Extend_BYTE_to_WORD(TMP_DEST[127:0], SRC[63:0])
/// IF VL >= 256
///     Packed_Sign_Extend_BYTE_to_WORD(TMP_DEST[255:128], SRC[127:64])
/// FI;
/// IF VL >= 512
///     Packed_Sign_Extend_BYTE_to_WORD(TMP_DEST[383:256], SRC[191:128])
///     Packed_Sign_Extend_BYTE_to_WORD(TMP_DEST[511:384], SRC[255:192])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TEMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVSXBD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// Packed_Sign_Extend_BYTE_to_DWORD(TMP_DEST[127:0], SRC[31:0])
/// IF VL >= 256
///     Packed_Sign_Extend_BYTE_to_DWORD(TMP_DEST[255:128], SRC[63:32])
/// FI;
/// IF VL >= 512
///     Packed_Sign_Extend_BYTE_to_DWORD(TMP_DEST[383:256], SRC[95:64])
///     Packed_Sign_Extend_BYTE_to_DWORD(TMP_DEST[511:384], SRC[127:96])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TEMP_DEST[i+31:i]
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
/// VPMOVSXBQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// Packed_Sign_Extend_BYTE_to_QWORD(TMP_DEST[127:0], SRC[15:0])
/// IF VL >= 256
///     Packed_Sign_Extend_BYTE_to_QWORD(TMP_DEST[255:128], SRC[31:16])
/// FI;
/// IF VL >= 512
///     Packed_Sign_Extend_BYTE_to_QWORD(TMP_DEST[383:256], SRC[47:32])
///     Packed_Sign_Extend_BYTE_to_QWORD(TMP_DEST[511:384], SRC[63:48])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TEMP_DEST[i+63:i]
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
/// VPMOVSXWD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// Packed_Sign_Extend_WORD_to_DWORD(TMP_DEST[127:0], SRC[63:0])
/// IF VL >= 256
///     Packed_Sign_Extend_WORD_to_DWORD(TMP_DEST[255:128], SRC[127:64])
/// FI;
/// IF VL >= 512
///     Packed_Sign_Extend_WORD_to_DWORD(TMP_DEST[383:256], SRC[191:128])
///     Packed_Sign_Extend_WORD_to_DWORD(TMP_DEST[511:384], SRC[256:192])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TEMP_DEST[i+31:i]
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
/// VPMOVSXWQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// Packed_Sign_Extend_WORD_to_QWORD(TMP_DEST[127:0], SRC[31:0])
/// IF VL >= 256
///     Packed_Sign_Extend_WORD_to_QWORD(TMP_DEST[255:128], SRC[63:32])
/// FI;
/// IF VL >= 512
///     Packed_Sign_Extend_WORD_to_QWORD(TMP_DEST[383:256], SRC[95:64])
///     Packed_Sign_Extend_WORD_to_QWORD(TMP_DEST[511:384], SRC[127:96])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TEMP_DEST[i+63:i]
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
/// VPMOVSXDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// Packed_Sign_Extend_DWORD_to_QWORD(TEMP_DEST[127:0], SRC[63:0])
/// IF VL >= 256
///     Packed_Sign_Extend_DWORD_to_QWORD(TEMP_DEST[255:128], SRC[127:64])
/// FI;
/// IF VL >= 512
///     Packed_Sign_Extend_DWORD_to_QWORD(TEMP_DEST[383:256], SRC[191:128])
///     Packed_Sign_Extend_DWORD_to_QWORD(TEMP_DEST[511:384], SRC[255:192])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TEMP_DEST[i+63:i]
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
/// VPMOVSXBW (VEX.256 Encoded Version)
/// Packed_Sign_Extend_BYTE_to_WORD(DEST[127:0], SRC[63:0])
/// Packed_Sign_Extend_BYTE_to_WORD(DEST[255:128], SRC[127:64])
/// DEST[MAXVL-1:256] := 0
/// VPMOVSXBD (VEX.256 Encoded Version)
/// Packed_Sign_Extend_BYTE_to_DWORD(DEST[127:0], SRC[31:0])
/// Packed_Sign_Extend_BYTE_to_DWORD(DEST[255:128], SRC[63:32])
/// DEST[MAXVL-1:256] := 0
/// VPMOVSXBQ (VEX.256 Encoded Version)
/// Packed_Sign_Extend_BYTE_to_QWORD(DEST[127:0], SRC[15:0])
/// Packed_Sign_Extend_BYTE_to_QWORD(DEST[255:128], SRC[31:16])
/// DEST[MAXVL-1:256] := 0
/// VPMOVSXWD (VEX.256 Encoded Version)
/// Packed_Sign_Extend_WORD_to_DWORD(DEST[127:0], SRC[63:0])
/// Packed_Sign_Extend_WORD_to_DWORD(DEST[255:128], SRC[127:64])
/// DEST[MAXVL-1:256] := 0
/// VPMOVSXWQ (VEX.256 Encoded Version)
/// Packed_Sign_Extend_WORD_to_QWORD(DEST[127:0], SRC[31:0])
/// Packed_Sign_Extend_WORD_to_QWORD(DEST[255:128], SRC[63:32])
/// DEST[MAXVL-1:256] := 0
/// VPMOVSXDQ (VEX.256 Encoded Version)
/// Packed_Sign_Extend_DWORD_to_QWORD(DEST[127:0], SRC[63:0])
/// Packed_Sign_Extend_DWORD_to_QWORD(DEST[255:128], SRC[127:64])
/// DEST[MAXVL-1:256] := 0
/// VPMOVSXBW (VEX.128 Encoded Version)
/// Packed_Sign_Extend_BYTE_to_WORDDEST[127:0], SRC[127:0]()
/// DEST[MAXVL-1:128] := 0
/// VPMOVSXBD (VEX.128 Encoded Version)
/// Packed_Sign_Extend_BYTE_to_DWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPMOVSXBQ (VEX.128 Encoded Version)
/// Packed_Sign_Extend_BYTE_to_QWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPMOVSXWD (VEX.128 Encoded Version)
/// Packed_Sign_Extend_WORD_to_DWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPMOVSXWQ (VEX.128 Encoded Version)
/// Packed_Sign_Extend_WORD_to_QWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] := 0
/// VPMOVSXDQ (VEX.128 Encoded Version)
/// Packed_Sign_Extend_DWORD_to_QWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] := 0
/// PMOVSXBW
/// Packed_Sign_Extend_BYTE_to_WORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVSXBD
/// Packed_Sign_Extend_BYTE_to_DWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVSXBQ
/// Packed_Sign_Extend_BYTE_to_QWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVSXWD
/// Packed_Sign_Extend_WORD_to_DWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVSXWQ
/// Packed_Sign_Extend_WORD_to_QWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVSXDQ
/// Packed_Sign_Extend_DWORD_to_QWORD(DEST[127:0], SRC[127:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn pmovsx() -> &'static [IrStatement] {
    let assignment = assign(u::sign_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Packed_Zero_Extend_BYTE_to_WORD(DEST, SRC)
/// DEST[15:0] := ZeroExtend(SRC[7:0]);
/// DEST[31:16] := ZeroExtend(SRC[15:8]);
/// DEST[47:32] := ZeroExtend(SRC[23:16]);
/// DEST[63:48] := ZeroExtend(SRC[31:24]);
/// DEST[79:64] := ZeroExtend(SRC[39:32]);
/// DEST[95:80] := ZeroExtend(SRC[47:40]);
/// DEST[111:96] := ZeroExtend(SRC[55:48]);
/// DEST[127:112] := ZeroExtend(SRC[63:56]);
/// Packed_Zero_Extend_BYTE_to_DWORD(DEST, SRC)
/// DEST[31:0] := ZeroExtend(SRC[7:0]);
/// DEST[63:32] := ZeroExtend(SRC[15:8]);
/// DEST[95:64] := ZeroExtend(SRC[23:16]);
/// DEST[127:96] := ZeroExtend(SRC[31:24]);
/// Packed_Zero_Extend_BYTE_to_QWORD(DEST, SRC)
/// DEST[63:0] := ZeroExtend(SRC[7:0]);
/// DEST[127:64] := ZeroExtend(SRC[15:8]);
/// Packed_Zero_Extend_WORD_to_DWORD(DEST, SRC)
/// DEST[31:0] := ZeroExtend(SRC[15:0]);
/// DEST[63:32] := ZeroExtend(SRC[31:16]);
/// DEST[95:64] := ZeroExtend(SRC[47:32]);
/// DEST[127:96] := ZeroExtend(SRC[63:48]);
/// Packed_Zero_Extend_WORD_to_QWORD(DEST, SRC)
/// DEST[63:0] := ZeroExtend(SRC[15:0]);
/// DEST[127:64] := ZeroExtend(SRC[31:16]);
/// Packed_Zero_Extend_DWORD_to_QWORD(DEST, SRC)
/// DEST[63:0] := ZeroExtend(SRC[31:0]);
/// DEST[127:64] := ZeroExtend(SRC[63:32]);
/// VPMOVZXBW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// Packed_Zero_Extend_BYTE_to_WORD(TMP_DEST[127:0], SRC[63:0])
/// IF VL >= 256
///     Packed_Zero_Extend_BYTE_to_WORD(TMP_DEST[255:128], SRC[127:64])
/// FI;
/// IF VL >= 512
///     Packed_Zero_Extend_BYTE_to_WORD(TMP_DEST[383:256], SRC[191:128])
///     Packed_Zero_Extend_BYTE_to_WORD(TMP_DEST[511:384], SRC[255:192])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TEMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMOVZXBD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// Packed_Zero_Extend_BYTE_to_DWORD(TMP_DEST[127:0], SRC[31:0])
/// IF VL >= 256
///     Packed_Zero_Extend_BYTE_to_DWORD(TMP_DEST[255:128], SRC[63:32])
/// FI;
/// IF VL >= 512
///     Packed_Zero_Extend_BYTE_to_DWORD(TMP_DEST[383:256], SRC[95:64])
///     Packed_Zero_Extend_BYTE_to_DWORD(TMP_DEST[511:384], SRC[127:96])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TEMP_DEST[i+31:i]
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
/// VPMOVZXBQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// Packed_Zero_Extend_BYTE_to_QWORD(TMP_DEST[127:0], SRC[15:0])
/// IF VL >= 256
///     Packed_Zero_Extend_BYTE_to_QWORD(TMP_DEST[255:128], SRC[31:16])
/// FI;
/// IF VL >= 512
///     Packed_Zero_Extend_BYTE_to_QWORD(TMP_DEST[383:256], SRC[47:32])
///     Packed_Zero_Extend_BYTE_to_QWORD(TMP_DEST[511:384], SRC[63:48])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TEMP_DEST[i+63:i]
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
/// VPMOVZXWD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// Packed_Zero_Extend_WORD_to_DWORD(TMP_DEST[127:0], SRC[63:0])
/// IF VL >= 256
///     Packed_Zero_Extend_WORD_to_DWORD(TMP_DEST[255:128], SRC[127:64])
/// FI;
/// IF VL >= 512
///     Packed_Zero_Extend_WORD_to_DWORD(TMP_DEST[383:256], SRC[191:128])
///     Packed_Zero_Extend_WORD_to_DWORD(TMP_DEST[511:384], SRC[256:192])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TEMP_DEST[i+31:i]
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
/// VPMOVZXWQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// Packed_Zero_Extend_WORD_to_QWORD(TMP_DEST[127:0], SRC[31:0])
/// IF VL >= 256
///     Packed_Zero_Extend_WORD_to_QWORD(TMP_DEST[255:128], SRC[63:32])
/// FI;
/// IF VL >= 512
///     Packed_Zero_Extend_WORD_to_QWORD(TMP_DEST[383:256], SRC[95:64])
///     Packed_Zero_Extend_WORD_to_QWORD(TMP_DEST[511:384], SRC[127:96])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TEMP_DEST[i+63:i]
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
/// VPMOVZXDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// Packed_Zero_Extend_DWORD_to_QWORD(TEMP_DEST[127:0], SRC[63:0])
/// IF VL >= 256
///     Packed_Zero_Extend_DWORD_to_QWORD(TEMP_DEST[255:128], SRC[127:64])
/// FI;
/// IF VL >= 512
///     Packed_Zero_Extend_DWORD_to_QWORD(TEMP_DEST[383:256], SRC[191:128])
///     Packed_Zero_Extend_DWORD_to_QWORD(TEMP_DEST[511:384], SRC[255:192])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TEMP_DEST[i+63:i]
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
/// VPMOVZXBW (VEX.256 Encoded Version)
/// Packed_Zero_Extend_BYTE_to_WORD(DEST[127:0], SRC[63:0])
/// Packed_Zero_Extend_BYTE_to_WORD(DEST[255:128], SRC[127:64])
/// DEST[MAXVL-1:256] := 0
/// VPMOVZXBD (VEX.256 Encoded Version)
/// Packed_Zero_Extend_BYTE_to_DWORD(DEST[127:0], SRC[31:0])
/// Packed_Zero_Extend_BYTE_to_DWORD(DEST[255:128], SRC[63:32])
/// DEST[MAXVL-1:256] := 0
/// VPMOVZXBQ (VEX.256 Encoded Version)
/// Packed_Zero_Extend_BYTE_to_QWORD(DEST[127:0], SRC[15:0])
/// Packed_Zero_Extend_BYTE_to_QWORD(DEST[255:128], SRC[31:16])
/// DEST[MAXVL-1:256] := 0
/// VPMOVZXWD (VEX.256 Encoded Version)
/// Packed_Zero_Extend_WORD_to_DWORD(DEST[127:0], SRC[63:0])
/// Packed_Zero_Extend_WORD_to_DWORD(DEST[255:128], SRC[127:64])
/// DEST[MAXVL-1:256] := 0
/// VPMOVZXWQ (VEX.256 Encoded Version)
/// Packed_Zero_Extend_WORD_to_QWORD(DEST[127:0], SRC[31:0])
/// Packed_Zero_Extend_WORD_to_QWORD(DEST[255:128], SRC[63:32])
/// DEST[MAXVL-1:256] := 0
/// VPMOVZXDQ (VEX.256 Encoded Version)
/// Packed_Zero_Extend_DWORD_to_QWORD(DEST[127:0], SRC[63:0])
/// Packed_Zero_Extend_DWORD_to_QWORD(DEST[255:128], SRC[127:64])
/// DEST[MAXVL-1:256] := 0
/// VPMOVZXBW (VEX.128 Encoded Version)
/// Packed_Zero_Extend_BYTE_to_WORD()
/// DEST[MAXVL-1:128] := 0
/// VPMOVZXBD (VEX.128 Encoded Version)
/// Packed_Zero_Extend_BYTE_to_DWORD()
/// DEST[MAXVL-1:128] := 0
/// VPMOVZXBQ (VEX.128 Encoded Version)
/// Packed_Zero_Extend_BYTE_to_QWORD()
/// DEST[MAXVL-1:128] := 0
/// VPMOVZXWD (VEX.128 Encoded Version)
/// Packed_Zero_Extend_WORD_to_DWORD()
/// DEST[MAXVL-1:128] := 0
/// VPMOVZXWQ (VEX.128 Encoded Version)
/// Packed_Zero_Extend_WORD_to_QWORD()
/// DEST[MAXVL-1:128] := 0
/// VPMOVZXDQ (VEX.128 Encoded Version)
/// Packed_Zero_Extend_DWORD_to_QWORD()
/// DEST[MAXVL-1:128] := 0
/// PMOVZXBW
/// Packed_Zero_Extend_BYTE_to_WORD()
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVZXBD
/// Packed_Zero_Extend_BYTE_to_DWORD()
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVZXBQ
/// Packed_Zero_Extend_BYTE_to_QWORD()
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVZXWD
/// Packed_Zero_Extend_WORD_to_DWORD()
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVZXWQ
/// Packed_Zero_Extend_WORD_to_QWORD()
/// DEST[MAXVL-1:128] (Unmodified)
/// PMOVZXDQ
/// Packed_Zero_Extend_DWORD_to_QWORD()
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn pmovzx() -> &'static [IrStatement] {
    let assignment = assign(u::sign_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMULDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := SignExtend64( SRC1[i+31:i]) * SignExtend64( SRC2[31:0])
///                 ELSE DEST[i+63:i] := SignExtend64( SRC1[i+31:i]) * SignExtend64( SRC2[i+31:i])
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
/// VPMULDQ (VEX.256 Encoded Version)
/// DEST[63:0] := SignExtend64( SRC1[31:0]) * SignExtend64( SRC2[31:0])
/// DEST[127:64] := SignExtend64( SRC1[95:64]) * SignExtend64( SRC2[95:64])
/// DEST[191:128] := SignExtend64( SRC1[159:128]) * SignExtend64( SRC2[159:128])
/// DEST[255:192] := SignExtend64( SRC1[223:192]) * SignExtend64( SRC2[223:192])
/// DEST[MAXVL-1:256] := 0
/// VPMULDQ (VEX.128 Encoded Version)
/// DEST[63:0] := SignExtend64( SRC1[31:0]) * SignExtend64( SRC2[31:0])
/// DEST[127:64] := SignExtend64( SRC1[95:64]) * SignExtend64( SRC2[95:64])
/// DEST[MAXVL-1:128] := 0
/// PMULDQ (128-bit Legacy SSE Version)
/// DEST[63:0] := SignExtend64( DEST[31:0]) * SignExtend64( SRC[31:0])
/// DEST[127:64] := SignExtend64( DEST[95:64]) * SignExtend64( SRC[95:64])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn pmuldq() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMULHRSW (With 64-bit Operands)
///     temp0[31:0] = INT32 ((DEST[15:0] * SRC[15:0]) >>14) + 1;
///     temp1[31:0] = INT32 ((DEST[31:16] * SRC[31:16]) >>14) + 1;
///     temp2[31:0] = INT32 ((DEST[47:32] * SRC[47:32]) >> 14) + 1;
///     temp3[31:0] = INT32 ((DEST[63:48] * SRc[63:48]) >> 14) + 1;
///     DEST[15:0] = temp0[16:1];
///     DEST[31:16] = temp1[16:1];
///     DEST[47:32] = temp2[16:1];
///     DEST[63:48] = temp3[16:1];
/// PMULHRSW (With 128-bit Operands)
///     temp0[31:0] = INT32 ((DEST[15:0] * SRC[15:0]) >>14) + 1;
///     temp1[31:0] = INT32 ((DEST[31:16] * SRC[31:16]) >>14) + 1;
///     temp2[31:0] = INT32 ((DEST[47:32] * SRC[47:32]) >>14) + 1;
///     temp3[31:0] = INT32 ((DEST[63:48] * SRC[63:48]) >>14) + 1;
///     temp4[31:0] = INT32 ((DEST[79:64] * SRC[79:64]) >>14) + 1;
///     temp5[31:0] = INT32 ((DEST[95:80] * SRC[95:80]) >>14) + 1;
///     temp6[31:0] = INT32 ((DEST[111:96] * SRC[111:96]) >>14) + 1;
///     temp7[31:0] = INT32 ((DEST[127:112] * SRC[127:112) >>14) + 1;
///     DEST[15:0] = temp0[16:1];
///     DEST[31:16] = temp1[16:1];
///     DEST[47:32] = temp2[16:1];
///     DEST[63:48] = temp3[16:1];
///     DEST[79:64] = temp4[16:1];
///     DEST[95:80] = temp5[16:1];
///     DEST[111:96] = temp6[16:1];
///     DEST[127:112] = temp7[16:1];
/// VPMULHRSW (VEX.128 Encoded Version)
/// temp0[31:0] := INT32 ((SRC1[15:0] * SRC2[15:0]) >>14) + 1
/// temp1[31:0] := INT32 ((SRC1[31:16] * SRC2[31:16]) >>14) + 1
/// temp2[31:0] := INT32 ((SRC1[47:32] * SRC2[47:32]) >>14) + 1
/// temp3[31:0] := INT32 ((SRC1[63:48] * SRC2[63:48]) >>14) + 1
/// temp4[31:0] := INT32 ((SRC1[79:64] * SRC2[79:64]) >>14) + 1
/// temp5[31:0] := INT32 ((SRC1[95:80] * SRC2[95:80]) >>14) + 1
/// temp6[31:0] := INT32 ((SRC1[111:96] * SRC2[111:96]) >>14) + 1
/// temp7[31:0] := INT32 ((SRC1[127:112] * SRC2[127:112) >>14) + 1
/// DEST[15:0] := temp0[16:1]
/// DEST[31:16] := temp1[16:1]
/// DEST[47:32] := temp2[16:1]
/// DEST[63:48] := temp3[16:1]
/// DEST[79:64] := temp4[16:1]
/// DEST[95:80] := temp5[16:1]
/// DEST[111:96] := temp6[16:1]
/// DEST[127:112] := temp7[16:1]
/// DEST[MAXVL-1:128] := 0
/// VPMULHRSW (VEX.256 Encoded Version)
/// temp0[31:0] := INT32 ((SRC1[15:0] * SRC2[15:0]) >>14) + 1
/// temp1[31:0] := INT32 ((SRC1[31:16] * SRC2[31:16]) >>14) + 1
/// temp2[31:0] := INT32 ((SRC1[47:32] * SRC2[47:32]) >>14) + 1
/// temp3[31:0] := INT32 ((SRC1[63:48] * SRC2[63:48]) >>14) + 1
/// temp4[31:0] := INT32 ((SRC1[79:64] * SRC2[79:64]) >>14) + 1
/// temp5[31:0] := INT32 ((SRC1[95:80] * SRC2[95:80]) >>14) + 1
/// temp6[31:0] := INT32 ((SRC1[111:96] * SRC2[111:96]) >>14) + 1
/// temp7[31:0] := INT32 ((SRC1[127:112] * SRC2[127:112) >>14) + 1
/// temp8[31:0] := INT32 ((SRC1[143:128] * SRC2[143:128]) >>14) + 1
/// temp9[31:0] := INT32 ((SRC1[159:144] * SRC2[159:144]) >>14) + 1
/// temp10[31:0] := INT32 ((SRC1[75:160] * SRC2[175:160]) >>14) + 1
/// temp11[31:0] := INT32 ((SRC1[191:176] * SRC2[191:176]) >>14) + 1
/// temp12[31:0] := INT32 ((SRC1[207:192] * SRC2[207:192]) >>14) + 1
/// temp13[31:0] := INT32 ((SRC1[223:208] * SRC2[223:208]) >>14) + 1
/// temp14[31:0] := INT32 ((SRC1[239:224] * SRC2[239:224]) >>14) + 1
/// temp15[31:0] := INT32 ((SRC1[255:240] * SRC2[255:240) >>14) + 1
/// DEST[15:0] := temp0[16:1]
/// DEST[31:16] := temp1[16:1]
/// DEST[47:32] := temp2[16:1]
/// DEST[63:48] := temp3[16:1]
/// DEST[79:64] := temp4[16:1]
/// DEST[95:80] := temp5[16:1]
/// DEST[111:96] := temp6[16:1]
/// DEST[127:112] := temp7[16:1]
/// DEST[143:128] := temp8[16:1]
/// DEST[159:144] := temp9[16:1]
/// DEST[175:160] := temp10[16:1]
/// DEST[191:176] := temp11[16:1]
/// DEST[207:192] := temp12[16:1]
/// DEST[223:208] := temp13[16:1]
/// DEST[239:224] := temp14[16:1]
/// DEST[255:240] := temp15[16:1]
/// DEST[MAXVL-1:256] := 0
/// VPMULHRSW (EVEX Encoded Version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN
///             temp[31:0] := ((SRC1[i+15:i] * SRC2[i+15:i]) >>14) + 1
///             DEST[i+15:i] := tmp[16:1]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmulhrsw() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMULHUW (With 64-bit Operands)
///     TEMP0[31:0] := DEST[15:0] * SRC[15:0]; (* Unsigned multiplication *)
///     TEMP1[31:0] := DEST[31:16] * SRC[31:16];
///     TEMP2[31:0] := DEST[47:32] * SRC[47:32];
///     TEMP3[31:0] := DEST[63:48] * SRC[63:48];
///     DEST[15:0] := TEMP0[31:16];
///     DEST[31:16] := TEMP1[31:16];
///     DEST[47:32] := TEMP2[31:16];
///     DEST[63:48] := TEMP3[31:16];
/// PMULHUW (With 128-bit Operands)
///     TEMP0[31:0] := DEST[15:0] * SRC[15:0]; (* Unsigned multiplication *)
///     TEMP1[31:0] := DEST[31:16] * SRC[31:16];
///     TEMP2[31:0] := DEST[47:32] * SRC[47:32];
///     TEMP3[31:0] := DEST[63:48] * SRC[63:48];
///     TEMP4[31:0] := DEST[79:64] * SRC[79:64];
///     TEMP5[31:0] := DEST[95:80] * SRC[95:80];
///     TEMP6[31:0] := DEST[111:96] * SRC[111:96];
///     TEMP7[31:0] := DEST[127:112] * SRC[127:112];
///     DEST[15:0] := TEMP0[31:16];
///     DEST[31:16] := TEMP1[31:16];
///     DEST[47:32] := TEMP2[31:16];
///     DEST[63:48] := TEMP3[31:16];
///     DEST[79:64] := TEMP4[31:16];
///     DEST[95:80] := TEMP5[31:16];
///     DEST[111:96] := TEMP6[31:16];
///     DEST[127:112] := TEMP7[31:16];
/// VPMULHUW (VEX.128 Encoded Version)
/// TEMP0[31:0] := SRC1[15:0] * SRC2[15:0]
/// TEMP1[31:0] := SRC1[31:16] * SRC2[31:16]
/// TEMP2[31:0] := SRC1[47:32] * SRC2[47:32]
/// TEMP3[31:0] := SRC1[63:48] * SRC2[63:48]
/// TEMP4[31:0] := SRC1[79:64] * SRC2[79:64]
/// TEMP5[31:0] := SRC1[95:80] * SRC2[95:80]
/// TEMP6[31:0] := SRC1[111:96] * SRC2[111:96]
/// TEMP7[31:0] := SRC1[127:112] * SRC2[127:112]
/// DEST[15:0] := TEMP0[31:16]
/// DEST[31:16] := TEMP1[31:16]
/// DEST[47:32] := TEMP2[31:16]
/// DEST[63:48] := TEMP3[31:16]
/// DEST[79:64] := TEMP4[31:16]
/// DEST[95:80] := TEMP5[31:16]
/// DEST[111:96] := TEMP6[31:16]
/// DEST[127:112] := TEMP7[31:16]
/// DEST[MAXVL-1:128] := 0
/// PMULHUW (VEX.256 Encoded Version)
/// TEMP0[31:0] := SRC1[15:0] * SRC2[15:0]
/// TEMP1[31:0] := SRC1[31:16] * SRC2[31:16]
/// TEMP2[31:0] := SRC1[47:32] * SRC2[47:32]
/// TEMP3[31:0] := SRC1[63:48] * SRC2[63:48]
/// TEMP4[31:0] := SRC1[79:64] * SRC2[79:64]
/// TEMP5[31:0] := SRC1[95:80] * SRC2[95:80]
/// TEMP6[31:0] := SRC1[111:96] * SRC2[111:96]
/// TEMP7[31:0] := SRC1[127:112] * SRC2[127:112]
/// TEMP8[31:0] := SRC1[143:128] * SRC2[143:128]
/// TEMP9[31:0] := SRC1[159:144] * SRC2[159:144]
/// TEMP10[31:0] := SRC1[175:160] * SRC2[175:160]
/// TEMP11[31:0] := SRC1[191:176] * SRC2[191:176]
/// TEMP12[31:0] := SRC1[207:192] * SRC2[207:192]
/// TEMP13[31:0] := SRC1[223:208] * SRC2[223:208]
/// TEMP14[31:0] := SRC1[239:224] * SRC2[239:224]
/// TEMP15[31:0] := SRC1[255:240] * SRC2[255:240]
/// DEST[15:0] := TEMP0[31:16]
/// DEST[31:16] := TEMP1[31:16]
/// DEST[47:32] := TEMP2[31:16]
/// DEST[63:48] := TEMP3[31:16]
/// DEST[79:64] := TEMP4[31:16]
/// DEST[95:80] := TEMP5[31:16]
/// DEST[111:96] := TEMP6[31:16]
/// DEST[127:112] := TEMP7[31:16]
/// DEST[143:128] := TEMP8[31:16]
/// DEST[159:144] := TEMP9[31:16]
/// DEST[175:160] := TEMP10[31:16]
/// DEST[191:176] := TEMP11[31:16]
/// DEST[207:192] := TEMP12[31:16]
/// DEST[223:208] := TEMP13[31:16]
/// DEST[239:224] := TEMP14[31:16]
/// DEST[255:240] := TEMP15[31:16]
/// DEST[MAXVL-1:256] := 0
/// PMULHUW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN
///             temp[31:0] := SRC1[i+15:i] * SRC2[i+15:i]
///             DEST[i+15:i] := tmp[31:16]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmulhuw() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMULHW (With 64-bit Operands)
///     TEMP0[31:0] := DEST[15:0] * SRC[15:0]; (* Signed multiplication *)
///     TEMP1[31:0] := DEST[31:16] * SRC[31:16];
///     TEMP2[31:0] := DEST[47:32] * SRC[47:32];
///     TEMP3[31:0] := DEST[63:48] * SRC[63:48];
///     DEST[15:0] := TEMP0[31:16];
///     DEST[31:16] := TEMP1[31:16];
///     DEST[47:32] := TEMP2[31:16];
///     DEST[63:48] := TEMP3[31:16];
/// PMULHW (With 128-bit Operands)
///     TEMP0[31:0] := DEST[15:0] * SRC[15:0]; (* Signed multiplication *)
///     TEMP1[31:0] := DEST[31:16] * SRC[31:16];
///     TEMP2[31:0] := DEST[47:32] * SRC[47:32];
///     TEMP3[31:0] := DEST[63:48] * SRC[63:48];
///     TEMP4[31:0] := DEST[79:64] * SRC[79:64];
///     TEMP5[31:0] := DEST[95:80] * SRC[95:80];
///     TEMP6[31:0] := DEST[111:96] * SRC[111:96];
///     TEMP7[31:0] := DEST[127:112] * SRC[127:112];
///     DEST[15:0] := TEMP0[31:16];
///     DEST[31:16] := TEMP1[31:16];
///     DEST[47:32] := TEMP2[31:16];
///     DEST[63:48] := TEMP3[31:16];
///     DEST[79:64] := TEMP4[31:16];
///     DEST[95:80] := TEMP5[31:16];
///     DEST[111:96] := TEMP6[31:16];
///     DEST[127:112] := TEMP7[31:16];
/// VPMULHW (VEX.128 Encoded Version)
/// TEMP0[31:0] := SRC1[15:0] * SRC2[15:0] (*Signed Multiplication*)
/// TEMP1[31:0] := SRC1[31:16] * SRC2[31:16]
/// TEMP2[31:0] := SRC1[47:32] * SRC2[47:32]
/// TEMP3[31:0] := SRC1[63:48] * SRC2[63:48]
/// TEMP4[31:0] := SRC1[79:64] * SRC2[79:64]
/// TEMP5[31:0] := SRC1[95:80] * SRC2[95:80]
/// TEMP6[31:0] := SRC1[111:96] * SRC2[111:96]
/// TEMP7[31:0] := SRC1[127:112] * SRC2[127:112]
/// DEST[15:0] := TEMP0[31:16]
/// DEST[31:16] := TEMP1[31:16]
/// DEST[47:32] := TEMP2[31:16]
/// DEST[63:48] := TEMP3[31:16]
/// DEST[79:64] := TEMP4[31:16]
/// DEST[95:80] := TEMP5[31:16]
/// DEST[111:96] := TEMP6[31:16]
/// DEST[127:112] := TEMP7[31:16]
/// DEST[MAXVL-1:128] := 0
/// PMULHW (VEX.256 Encoded Version)
/// TEMP0[31:0] := SRC1[15:0] * SRC2[15:0] (*Signed Multiplication*)
/// TEMP1[31:0] := SRC1[31:16] * SRC2[31:16]
/// TEMP2[31:0] := SRC1[47:32] * SRC2[47:32]
/// TEMP3[31:0] := SRC1[63:48] * SRC2[63:48]
/// TEMP4[31:0] := SRC1[79:64] * SRC2[79:64]
/// TEMP5[31:0] := SRC1[95:80] * SRC2[95:80]
/// TEMP6[31:0] := SRC1[111:96] * SRC2[111:96]
/// TEMP7[31:0] := SRC1[127:112] * SRC2[127:112]
/// TEMP8[31:0] := SRC1[143:128] * SRC2[143:128]
/// TEMP9[31:0] := SRC1[159:144] * SRC2[159:144]
/// TEMP10[31:0] := SRC1[175:160] * SRC2[175:160]
/// TEMP11[31:0] := SRC1[191:176] * SRC2[191:176]
/// TEMP12[31:0] := SRC1[207:192] * SRC2[207:192]
/// TEMP13[31:0] := SRC1[223:208] * SRC2[223:208]
/// TEMP14[31:0] := SRC1[239:224] * SRC2[239:224]
/// TEMP15[31:0] := SRC1[255:240] * SRC2[255:240]
/// DEST[15:0] := TEMP0[31:16]
/// DEST[31:16] := TEMP1[31:16]
/// DEST[47:32] := TEMP2[31:16]
/// DEST[63:48] := TEMP3[31:16]
/// DEST[79:64] := TEMP4[31:16]
/// DEST[95:80] := TEMP5[31:16]
/// DEST[111:96] := TEMP6[31:16]
/// DEST[127:112] := TEMP7[31:16]
/// DEST[143:128] := TEMP8[31:16]
/// DEST[159:144] := TEMP9[31:16]
/// DEST[175:160] := TEMP10[31:16]
/// DEST[191:176] := TEMP11[31:16]
/// DEST[207:192] := TEMP12[31:16]
/// DEST[223:208] := TEMP13[31:16]
/// DEST[239:224] := TEMP14[31:16]
/// DEST[255:240] := TEMP15[31:16]
/// DEST[MAXVL-1:256] := 0
/// PMULHW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN
///             temp[31:0] := SRC1[i+15:i] * SRC2[i+15:i]
///             DEST[i+15:i] := tmp[31:16]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmulhw() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMULLQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                     THEN Temp[127:0] := SRC1[i+63:i] * SRC2[63:0]
///                     ELSE Temp[127:0] := SRC1[i+63:i] * SRC2[i+63:i]
///                 FI;
///                 DEST[i+63:i] := Temp[63:0]
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
/// VPMULLD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN Temp[63:0] := SRC1[i+31:i] * SRC2[31:0]
///                     ELSE Temp[63:0] := SRC1[i+31:i] * SRC2[i+31:i]
///                 FI;
///                 DEST[i+31:i] := Temp[31:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMULLD (VEX.256 Encoded Version)
/// Temp0[63:0] := SRC1[31:0] * SRC2[31:0]
/// Temp1[63:0] := SRC1[63:32] * SRC2[63:32]
/// Temp2[63:0] := SRC1[95:64] * SRC2[95:64]
/// Temp3[63:0] := SRC1[127:96] * SRC2[127:96]
/// Temp4[63:0] := SRC1[159:128] * SRC2[159:128]
/// Temp5[63:0] := SRC1[191:160] * SRC2[191:160]
/// Temp6[63:0] := SRC1[223:192] * SRC2[223:192]
/// Temp7[63:0] := SRC1[255:224] * SRC2[255:224]
/// DEST[31:0] := Temp0[31:0]
/// DEST[63:32] := Temp1[31:0]
/// DEST[95:64] := Temp2[31:0]
/// DEST[127:96] := Temp3[31:0]
/// DEST[159:128] := Temp4[31:0]
/// DEST[191:160] := Temp5[31:0]
/// DEST[223:192] := Temp6[31:0]
/// DEST[255:224] := Temp7[31:0]
/// DEST[MAXVL-1:256] := 0
/// VPMULLD (VEX.128 Encoded Version)
/// Temp0[63:0] := SRC1[31:0] * SRC2[31:0]
/// Temp1[63:0] := SRC1[63:32] * SRC2[63:32]
/// Temp2[63:0] := SRC1[95:64] * SRC2[95:64]
/// Temp3[63:0] := SRC1[127:96] * SRC2[127:96]
/// DEST[31:0] := Temp0[31:0]
/// DEST[63:32] := Temp1[31:0]
/// DEST[95:64] := Temp2[31:0]
/// DEST[127:96] := Temp3[31:0]
/// DEST[MAXVL-1:128] := 0
/// PMULLD (128-bit Legacy SSE Version)
/// Temp0[63:0] := DEST[31:0] * SRC[31:0]
/// Temp1[63:0] := DEST[63:32] * SRC[63:32]
/// Temp2[63:0] := DEST[95:64] * SRC[95:64]
/// Temp3[63:0] := DEST[127:96] * SRC[127:96]
/// DEST[31:0] := Temp0[31:0]
/// DEST[63:32] := Temp1[31:0]
/// DEST[95:64] := Temp2[31:0]
/// DEST[127:96] := Temp3[31:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn pmulld() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPMULLQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                     THEN Temp[127:0] := SRC1[i+63:i] * SRC2[63:0]
///                     ELSE Temp[127:0] := SRC1[i+63:i] * SRC2[i+63:i]
///                 FI;
///                 DEST[i+63:i] := Temp[63:0]
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
/// VPMULLD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN Temp[63:0] := SRC1[i+31:i] * SRC2[31:0]
///                     ELSE Temp[63:0] := SRC1[i+31:i] * SRC2[i+31:i]
///                 FI;
///                 DEST[i+31:i] := Temp[31:0]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPMULLD (VEX.256 Encoded Version)
/// Temp0[63:0] := SRC1[31:0] * SRC2[31:0]
/// Temp1[63:0] := SRC1[63:32] * SRC2[63:32]
/// Temp2[63:0] := SRC1[95:64] * SRC2[95:64]
/// Temp3[63:0] := SRC1[127:96] * SRC2[127:96]
/// Temp4[63:0] := SRC1[159:128] * SRC2[159:128]
/// Temp5[63:0] := SRC1[191:160] * SRC2[191:160]
/// Temp6[63:0] := SRC1[223:192] * SRC2[223:192]
/// Temp7[63:0] := SRC1[255:224] * SRC2[255:224]
/// DEST[31:0] := Temp0[31:0]
/// DEST[63:32] := Temp1[31:0]
/// DEST[95:64] := Temp2[31:0]
/// DEST[127:96] := Temp3[31:0]
/// DEST[159:128] := Temp4[31:0]
/// DEST[191:160] := Temp5[31:0]
/// DEST[223:192] := Temp6[31:0]
/// DEST[255:224] := Temp7[31:0]
/// DEST[MAXVL-1:256] := 0
/// VPMULLD (VEX.128 Encoded Version)
/// Temp0[63:0] := SRC1[31:0] * SRC2[31:0]
/// Temp1[63:0] := SRC1[63:32] * SRC2[63:32]
/// Temp2[63:0] := SRC1[95:64] * SRC2[95:64]
/// Temp3[63:0] := SRC1[127:96] * SRC2[127:96]
/// DEST[31:0] := Temp0[31:0]
/// DEST[63:32] := Temp1[31:0]
/// DEST[95:64] := Temp2[31:0]
/// DEST[127:96] := Temp3[31:0]
/// DEST[MAXVL-1:128] := 0
/// PMULLD (128-bit Legacy SSE Version)
/// Temp0[63:0] := DEST[31:0] * SRC[31:0]
/// Temp1[63:0] := DEST[63:32] * SRC[63:32]
/// Temp2[63:0] := DEST[95:64] * SRC[95:64]
/// Temp3[63:0] := DEST[127:96] * SRC[127:96]
/// DEST[31:0] := Temp0[31:0]
/// DEST[63:32] := Temp1[31:0]
/// DEST[95:64] := Temp2[31:0]
/// DEST[127:96] := Temp3[31:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn pmullq() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMULLW (With 64-bit Operands)
///     TEMP0[31:0] := DEST[15:0] * SRC[15:0]; (* Signed multiplication *)
///     TEMP1[31:0] := DEST[31:16] * SRC[31:16];
///     TEMP2[31:0] := DEST[47:32] * SRC[47:32];
///     TEMP3[31:0] := DEST[63:48] * SRC[63:48];
///     DEST[15:0] := TEMP0[15:0];
///     DEST[31:16] := TEMP1[15:0];
///     DEST[47:32] := TEMP2[15:0];
///     DEST[63:48] := TEMP3[15:0];
/// PMULLW (With 128-bit Operands)
///     TEMP0[31:0] := DEST[15:0] * SRC[15:0]; (* Signed multiplication *)
///     TEMP1[31:0] := DEST[31:16] * SRC[31:16];
///     TEMP2[31:0] := DEST[47:32] * SRC[47:32];
///     TEMP3[31:0] := DEST[63:48] * SRC[63:48];
///     TEMP4[31:0] := DEST[79:64] * SRC[79:64];
///     TEMP5[31:0] := DEST[95:80] * SRC[95:80];
///     TEMP6[31:0] := DEST[111:96] * SRC[111:96];
///     TEMP7[31:0] := DEST[127:112] * SRC[127:112];
///     DEST[15:0] := TEMP0[15:0];
///     DEST[31:16] := TEMP1[15:0];
///     DEST[47:32] := TEMP2[15:0];
///     DEST[63:48] := TEMP3[15:0];
///     DEST[79:64] := TEMP4[15:0];
///     DEST[95:80] := TEMP5[15:0];
///     DEST[111:96] := TEMP6[15:0];
///     DEST[127:112] := TEMP7[15:0];
/// DEST[MAXVL-1:256] := 0
/// VPMULLW (VEX.128 Encoded Version)
/// Temp0[31:0] := SRC1[15:0] * SRC2[15:0]
/// Temp1[31:0] := SRC1[31:16] * SRC2[31:16]
/// Temp2[31:0] := SRC1[47:32] * SRC2[47:32]
/// Temp3[31:0] := SRC1[63:48] * SRC2[63:48]
/// Temp4[31:0] := SRC1[79:64] * SRC2[79:64]
/// Temp5[31:0] := SRC1[95:80] * SRC2[95:80]
/// Temp6[31:0] := SRC1[111:96] * SRC2[111:96]
/// Temp7[31:0] := SRC1[127:112] * SRC2[127:112]
/// DEST[15:0] := Temp0[15:0]
/// DEST[31:16] := Temp1[15:0]
/// DEST[47:32] := Temp2[15:0]
/// DEST[63:48] := Temp3[15:0]
/// DEST[79:64] := Temp4[15:0]
/// DEST[95:80] := Temp5[15:0]
/// DEST[111:96] := Temp6[15:0]
/// DEST[127:112] := Temp7[15:0]
/// DEST[MAXVL-1:128] := 0
/// PMULLW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN
///             temp[31:0] := SRC1[i+15:i] * SRC2[i+15:i]
///             DEST[i+15:i] := temp[15:0]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pmullw() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PMULUDQ (With 64-Bit Operands)
///     DEST[63:0] := DEST[31:0] * SRC[31:0];
/// PMULUDQ (With 128-Bit Operands)
///     DEST[63:0] := DEST[31:0] * SRC[31:0];
///     DEST[127:64] := DEST[95:64] * SRC[95:64];
/// VPMULUDQ (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[31:0] * SRC2[31:0]
/// DEST[127:64] := SRC1[95:64] * SRC2[95:64]
/// DEST[MAXVL-1:128] := 0
/// VPMULUDQ (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[31:0] * SRC2[31:0]
/// DEST[127:64] := SRC1[95:64] * SRC2[95:64
/// DEST[191:128] := SRC1[159:128] * SRC2[159:128]
/// DEST[255:192] := SRC1[223:192] * SRC2[223:192]
/// DEST[MAXVL-1:256] := 0
/// VPMULUDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := ZeroExtend64( SRC1[i+31:i]) * ZeroExtend64( SRC2[31:0] )
///                 ELSE DEST[i+63:i] := ZeroExtend64( SRC1[i+31:i]) * ZeroExtend64( SRC2[i+31:i] )
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
pub(super) fn pmuludq() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF StackAddrSize = 32
///     THEN
///         IF OperandSize = 32
///             THEN
///                 DEST := SS:ESP; (* Copy a doubleword *)
///                 ESP := ESP + 4;
///             ELSE (* OperandSize = 16*)
///                 DEST := SS:ESP; (* Copy a word *)
///                 ESP := ESP + 2;
///         FI;
///     ELSE IF StackAddrSize = 64
///         THEN
///             IF OperandSize = 64
///                 THEN
///                     DEST := SS:RSP; (* Copy quadword *)
///                     RSP := RSP + 8;
///                 ELSE (* OperandSize = 16*)
///                     DEST := SS:RSP; (* Copy a word *)
///                     RSP := RSP + 2;
///             FI;
///         FI;
///     ELSE StackAddrSize = 16
///         THEN
///             IF OperandSize = 16
///                 THEN
///                     DEST := SS:SP; (* Copy a word *)
///                 ELSE (* OperandSize = 32 *)
///                     DEST := SS:SP; (* Copy a doubleword *)
///                     SP := SP + 4;
///             FI;
/// FI;
/// Loading a segment register while in protected mode results in special actions, as described in the following listing.
/// These checks are performed on the segment selector and the segment descriptor it points to.
/// 64-BIT_MODE
/// IF FS, or GS is loaded with non-NULL selector;
///     THEN
///         IF segment selector index is outside descriptor table limits
///             OR segment is not a data or readable code segment
///             OR ((segment is a data or nonconforming code segment)
///                 AND ((RPL > DPL) or (CPL > DPL))
///                     THEN #GP(selector);
///             IF segment not marked present
///                 THEN #NP(selector);
///         ELSE
///             SegmentRegister := segment selector;
///             SegmentRegister := segment descriptor;
///         FI;
/// FI;
/// IF FS, or GS is loaded with a NULL selector;
///         THEN
///             SegmentRegister := segment selector;
///             SegmentRegister := segment descriptor;
/// FI;
/// PREOTECTED MODE OR COMPATIBILITY MODE;
/// IF SS is loaded;
///     THEN
///         IF segment selector is NULL
///             THEN #GP(0);
///         FI;
///         IF segment selector index is outside descriptor table limits
///             or segment selector's RPL ≠ CPL
///             or segment is not a writable data segment
///             or DPL ≠ CPL
///                 THEN #GP(selector);
///         FI;
///         IF segment not marked present
///             THEN #SS(selector);
///             ELSE
///                 SS := segment selector;
///                 SS := segment descriptor;
///         FI;
/// FI;
/// IF DS, ES, FS, or GS is loaded with non-NULL selector;
///     THEN
///         IF segment selector index is outside descriptor table limits
///             or segment is not a data or readable code segment
///             or ((segment is a data or nonconforming code segment)
///             and ((RPL > DPL) or (CPL > DPL))
///                 THEN #GP(selector);
///         FI;
///         IF segment not marked present
///             THEN #NP(selector);
///             ELSE
///                 SegmentRegister := segment selector;
///                 SegmentRegister := segment descriptor;
///         FI;
/// FI;
/// IF DS, ES, FS, or GS is loaded with a NULL selector
///     THEN
///         SegmentRegister := segment selector;
///         SegmentRegister := segment descriptor;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn pop() -> &'static [IrStatement] {
    let pop = assign(d(rsp.clone()), o1(), o1_size());
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    [pop, set_sp].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///     THEN
///         #UD;
/// ELSE
///     IF OperandSize = 32 (* Instruction = POPAD *)
///     THEN
///         EDI := Pop();
///         ESI := Pop();
///         EBP := Pop();
///         Increment ESP by 4; (* Skip next 4 bytes of stack *)
///         EBX := Pop();
///         EDX := Pop();
///         ECX := Pop();
///         EAX := Pop();
///     ELSE (* OperandSize = 16, instruction = POPA *)
///         DI := Pop();
///         SI := Pop();
///         BP := Pop();
///         Increment ESP by 2; (* Skip next 2 bytes of stack *)
///         BX := Pop();
///         DX := Pop();
///         CX := Pop();
///         AX := Pop();
///     FI;
/// ```
#[box_to_static_reference]
pub(super) fn popa() -> &'static [IrStatement] {
    let pop = assign(d(rsp.clone()), o1(), o1_size());
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    [pop, set_sp].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///     THEN
///         #UD;
/// ELSE
///     IF OperandSize = 32 (* Instruction = POPAD *)
///     THEN
///         EDI := Pop();
///         ESI := Pop();
///         EBP := Pop();
///         Increment ESP by 4; (* Skip next 4 bytes of stack *)
///         EBX := Pop();
///         EDX := Pop();
///         ECX := Pop();
///         EAX := Pop();
///     ELSE (* OperandSize = 16, instruction = POPA *)
///         DI := Pop();
///         SI := Pop();
///         BP := Pop();
///         Increment ESP by 2; (* Skip next 2 bytes of stack *)
///         BX := Pop();
///         DX := Pop();
///         CX := Pop();
///         AX := Pop();
///     FI;
/// ```
#[box_to_static_reference]
pub(super) fn popad() -> &'static [IrStatement] {
    let pop = assign(d(rsp.clone()), o1(), o1_size());
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    [pop, set_sp].into()
}

/// # Pseudocode
/// ```text
/// Count = 0;
/// For (i=0; i < OperandSize; i++)
/// { IF (SRC[ i] = 1) // i'th bit
///     THEN Count++; FI;
/// }
/// DEST := Count;
/// ```
#[box_to_static_reference]
pub(super) fn popcnt() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_sf = assign(c(0), sf.clone(), size_relative(sf.clone()));
    let set_af = assign(c(0), af.clone(), size_relative(af.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    let set_pf = assign(c(0), pf.clone(), size_relative(pf.clone()));
    let set_zf = condition(b::equal(o1(), c(0), o1_size()), [assign(c(1), zf.clone(), size_relative(zf.clone()))], [assign(c(0), zf.clone(), size_relative(zf.clone()))]);
    [assignment, set_of, set_sf, set_af, set_cf, set_pf, set_zf].into()
}

/// # Pseudocode
/// ```text
/// IF EFLAGS.VM = 0 (* Not in Virtual-8086 Mode *)
///     THEN IF CPL = 0 OR CR0.PE = 0
///         THEN
///             IF OperandSize = 32;
///                 THEN
///                     EFLAGS := Pop(); (* 32-bit pop *)
///                     (* All non-reserved flags except RF, VIP, VIF, and VM can be modified;
///                     VIP, VIF, VM, and all reserved bits are unaffected. RF is cleared. *)
///                 ELSE IF (Operandsize = 64)
///                     RFLAGS = Pop(); (* 64-bit pop *)
///                     (* All non-reserved flags except RF, VIP, VIF, and VM can be modified;
///                 ELSE (* OperandSize = 16 *)
///                     EFLAGS[15:0] := Pop(); (* 16-bit pop *)
///                     (* All non-reserved flags can be modified. *)
///             FI;
///         ELSE (* CPL > 0 *)
///             IF OperandSize = 32
///                 THEN
///                     IF CPL > IOPL
///                         THEN
///                             EFLAGS := Pop(); (* 32-bit pop *)
///                             (* All non-reserved bits except IF, IOPL, VIP, VIF, VM, and RF can be modified;
///                             IF, IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                         ELSE
///                             EFLAGS := Pop(); (* 32-bit pop *)
///                             (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                             IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                     FI;
///                 ELSE IF (Operandsize = 64)
///                     IF CPL > IOPL
///                         THEN
///                             RFLAGS := Pop(); (* 64-bit pop *)
///                             (* All non-reserved bits except IF, IOPL, VIP, VIF, VM, and RF can be modified;
///                             IF, IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                         ELSE
///                             RFLAGS := Pop(); (* 64-bit pop *)
///                             (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                             IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                     FI;
///                 ELSE (* OperandSize = 16 *)
///                     EFLAGS[15:0] := Pop(); (* 16-bit pop *)
///                     (* All non-reserved bits except IOPL can be modified; IOPL and all
///                     reserved bits are unaffected. *)
///             FI;
///         FI;
///     ELSE (* In virtual-8086 mode *)
///         IF IOPL = 3
///             THEN
///                 IF OperandSize = 32
///                     THEN
///                         EFLAGS := Pop();
///                         (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                         VIP, VIF, VM, IOPL, and all reserved bits are unaffected. RF is cleared. *)
///                     ELSE
///                         EFLAGS[15:0] := Pop(); FI;
///                         (* All non-reserved bits except IOPL can be modified; IOPL and all reserved bits are unaffected. *)
///                 FI;
///             ELSE (* IOPL < 3 *)
///                 IF (Operandsize = 32) OR (CR4.VME = 0)
///                     THEN #GP(0); (* Trap to virtual-8086 monitor. *)
///                     ELSE (* Operandsize = 16 and CR4.VME = 1 *)
///                         tempFLAGS := Pop();
///                         IF (EFLAGS.VIP = 1 AND tempFLAGS[9] = 1) OR tempFLAGS[8] = 1
/// 
///                             THEN #GP(0);
/// 
///                             ELSE
/// 
///                                 EFLAGS.VIF := tempFLAGS[9];
/// 
///                                 EFLAGS[15:0] := tempFLAGS;
/// 
///                                 (* All non-reserved bits except IOPL and IF can be modified;
///                                 IOPL, IF, and all reserved bits are unaffected. *)
///                         FI;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn popf() -> &'static [IrStatement] {
    let pop = assign(d(rsp.clone()), o1(), o1_size());
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    [pop, set_sp].into()
}

/// # Pseudocode
/// ```text
/// IF EFLAGS.VM = 0 (* Not in Virtual-8086 Mode *)
///     THEN IF CPL = 0 OR CR0.PE = 0
///         THEN
///             IF OperandSize = 32;
///                 THEN
///                     EFLAGS := Pop(); (* 32-bit pop *)
///                     (* All non-reserved flags except RF, VIP, VIF, and VM can be modified;
///                     VIP, VIF, VM, and all reserved bits are unaffected. RF is cleared. *)
///                 ELSE IF (Operandsize = 64)
///                     RFLAGS = Pop(); (* 64-bit pop *)
///                     (* All non-reserved flags except RF, VIP, VIF, and VM can be modified;
///                 ELSE (* OperandSize = 16 *)
///                     EFLAGS[15:0] := Pop(); (* 16-bit pop *)
///                     (* All non-reserved flags can be modified. *)
///             FI;
///         ELSE (* CPL > 0 *)
///             IF OperandSize = 32
///                 THEN
///                     IF CPL > IOPL
///                         THEN
///                             EFLAGS := Pop(); (* 32-bit pop *)
///                             (* All non-reserved bits except IF, IOPL, VIP, VIF, VM, and RF can be modified;
///                             IF, IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                         ELSE
///                             EFLAGS := Pop(); (* 32-bit pop *)
///                             (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                             IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                     FI;
///                 ELSE IF (Operandsize = 64)
///                     IF CPL > IOPL
///                         THEN
///                             RFLAGS := Pop(); (* 64-bit pop *)
///                             (* All non-reserved bits except IF, IOPL, VIP, VIF, VM, and RF can be modified;
///                             IF, IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                         ELSE
///                             RFLAGS := Pop(); (* 64-bit pop *)
///                             (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                             IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                     FI;
///                 ELSE (* OperandSize = 16 *)
///                     EFLAGS[15:0] := Pop(); (* 16-bit pop *)
///                     (* All non-reserved bits except IOPL can be modified; IOPL and all
///                     reserved bits are unaffected. *)
///             FI;
///         FI;
///     ELSE (* In virtual-8086 mode *)
///         IF IOPL = 3
///             THEN
///                 IF OperandSize = 32
///                     THEN
///                         EFLAGS := Pop();
///                         (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                         VIP, VIF, VM, IOPL, and all reserved bits are unaffected. RF is cleared. *)
///                     ELSE
///                         EFLAGS[15:0] := Pop(); FI;
///                         (* All non-reserved bits except IOPL can be modified; IOPL and all reserved bits are unaffected. *)
///                 FI;
///             ELSE (* IOPL < 3 *)
///                 IF (Operandsize = 32) OR (CR4.VME = 0)
///                     THEN #GP(0); (* Trap to virtual-8086 monitor. *)
///                     ELSE (* Operandsize = 16 and CR4.VME = 1 *)
///                         tempFLAGS := Pop();
///                         IF (EFLAGS.VIP = 1 AND tempFLAGS[9] = 1) OR tempFLAGS[8] = 1
/// 
///                             THEN #GP(0);
/// 
///                             ELSE
/// 
///                                 EFLAGS.VIF := tempFLAGS[9];
/// 
///                                 EFLAGS[15:0] := tempFLAGS;
/// 
///                                 (* All non-reserved bits except IOPL and IF can be modified;
///                                 IOPL, IF, and all reserved bits are unaffected. *)
///                         FI;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn popfd() -> &'static [IrStatement] {
    let pop = assign(d(rsp.clone()), o1(), o1_size());
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    [pop, set_sp].into()
}

/// # Pseudocode
/// ```text
/// IF EFLAGS.VM = 0 (* Not in Virtual-8086 Mode *)
///     THEN IF CPL = 0 OR CR0.PE = 0
///         THEN
///             IF OperandSize = 32;
///                 THEN
///                     EFLAGS := Pop(); (* 32-bit pop *)
///                     (* All non-reserved flags except RF, VIP, VIF, and VM can be modified;
///                     VIP, VIF, VM, and all reserved bits are unaffected. RF is cleared. *)
///                 ELSE IF (Operandsize = 64)
///                     RFLAGS = Pop(); (* 64-bit pop *)
///                     (* All non-reserved flags except RF, VIP, VIF, and VM can be modified;
///                 ELSE (* OperandSize = 16 *)
///                     EFLAGS[15:0] := Pop(); (* 16-bit pop *)
///                     (* All non-reserved flags can be modified. *)
///             FI;
///         ELSE (* CPL > 0 *)
///             IF OperandSize = 32
///                 THEN
///                     IF CPL > IOPL
///                         THEN
///                             EFLAGS := Pop(); (* 32-bit pop *)
///                             (* All non-reserved bits except IF, IOPL, VIP, VIF, VM, and RF can be modified;
///                             IF, IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                         ELSE
///                             EFLAGS := Pop(); (* 32-bit pop *)
///                             (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                             IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                     FI;
///                 ELSE IF (Operandsize = 64)
///                     IF CPL > IOPL
///                         THEN
///                             RFLAGS := Pop(); (* 64-bit pop *)
///                             (* All non-reserved bits except IF, IOPL, VIP, VIF, VM, and RF can be modified;
///                             IF, IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                         ELSE
///                             RFLAGS := Pop(); (* 64-bit pop *)
///                             (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                             IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)
///                     FI;
///                 ELSE (* OperandSize = 16 *)
///                     EFLAGS[15:0] := Pop(); (* 16-bit pop *)
///                     (* All non-reserved bits except IOPL can be modified; IOPL and all
///                     reserved bits are unaffected. *)
///             FI;
///         FI;
///     ELSE (* In virtual-8086 mode *)
///         IF IOPL = 3
///             THEN
///                 IF OperandSize = 32
///                     THEN
///                         EFLAGS := Pop();
///                         (* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified;
///                         VIP, VIF, VM, IOPL, and all reserved bits are unaffected. RF is cleared. *)
///                     ELSE
///                         EFLAGS[15:0] := Pop(); FI;
///                         (* All non-reserved bits except IOPL can be modified; IOPL and all reserved bits are unaffected. *)
///                 FI;
///             ELSE (* IOPL < 3 *)
///                 IF (Operandsize = 32) OR (CR4.VME = 0)
///                     THEN #GP(0); (* Trap to virtual-8086 monitor. *)
///                     ELSE (* Operandsize = 16 and CR4.VME = 1 *)
///                         tempFLAGS := Pop();
///                         IF (EFLAGS.VIP = 1 AND tempFLAGS[9] = 1) OR tempFLAGS[8] = 1
/// 
///                             THEN #GP(0);
/// 
///                             ELSE
/// 
///                                 EFLAGS.VIF := tempFLAGS[9];
/// 
///                                 EFLAGS[15:0] := tempFLAGS;
/// 
///                                 (* All non-reserved bits except IOPL and IF can be modified;
///                                 IOPL, IF, and all reserved bits are unaffected. *)
///                         FI;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn popfq() -> &'static [IrStatement] {
    let pop = assign(d(rsp.clone()), o1(), o1_size());
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    [pop, set_sp].into()
}

/// # Pseudocode
/// ```text
/// FETCH (m8);
/// ```
#[box_to_static_reference]
pub(super) fn por() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// FETCH_WITH_EXCLUSIVE_OWNERSHIP (m8);
/// ```
#[box_to_static_reference]
pub(super) fn prefetchw() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// VPSADBW (EVEX Encoded Versions)
/// VL = 128, 256, 512
/// TEMP0 := ABS(SRC1[7:0] - SRC2[7:0])
/// (* Repeat operation for bytes 1 through 15 *)
/// TEMP15 := ABS(SRC1[127:120] - SRC2[127:120])
/// DEST[15:0] := SUM(TEMP0:TEMP7)
/// DEST[63:16] := 000000000000H
/// DEST[79:64] := SUM(TEMP8:TEMP15)
/// DEST[127:80] := 00000000000H
/// IF VL >= 256
///     (* Repeat operation for bytes 16 through 31*)
///     TEMP31 := ABS(SRC1[255:248] - SRC2[255:248])
///     DEST[143:128] := SUM(TEMP16:TEMP23)
///     DEST[191:144] := 000000000000H
///     DEST[207:192] := SUM(TEMP24:TEMP31)
///     DEST[223:208] := 00000000000H
/// FI;
/// IF VL >= 512
/// (* Repeat operation for bytes 32 through 63*)
///     TEMP63 := ABS(SRC1[511:504] - SRC2[511:504])
///     DEST[271:256] := SUM(TEMP0:TEMP7)
///     DEST[319:272] := 000000000000H
///     DEST[335:320] := SUM(TEMP8:TEMP15)
///     DEST[383:336] := 00000000000H
///     DEST[399:384] := SUM(TEMP16:TEMP23)
///     DEST[447:400] := 000000000000H
///     DEST[463:448] := SUM(TEMP24:TEMP31)
///     DEST[511:464] := 00000000000H
/// FI;
/// DEST[MAXVL-1:VL] := 0
/// VPSADBW (VEX.256 Encoded Version)
/// TEMP0 := ABS(SRC1[7:0] - SRC2[7:0])
/// (* Repeat operation for bytes 2 through 30*)
/// TEMP31 := ABS(SRC1[255:248] - SRC2[255:248])
/// DEST[15:0] := SUM(TEMP0:TEMP7)
/// DEST[63:16] := 000000000000H
/// DEST[79:64] := SUM(TEMP8:TEMP15)
/// DEST[127:80] := 00000000000H
/// DEST[143:128] := SUM(TEMP16:TEMP23)
/// DEST[191:144] := 000000000000H
/// DEST[207:192] := SUM(TEMP24:TEMP31)
/// DEST[223:208] := 00000000000H
/// DEST[MAXVL-1:256] := 0
/// VPSADBW (VEX.128 Encoded Version)
/// TEMP0 := ABS(SRC1[7:0] - SRC2[7:0])
/// (* Repeat operation for bytes 2 through 14 *)
/// TEMP15 := ABS(SRC1[127:120] - SRC2[127:120])
/// DEST[15:0] := SUM(TEMP0:TEMP7)
/// DEST[63:16] := 000000000000H
/// DEST[79:64] := SUM(TEMP8:TEMP15)
/// DEST[127:80] := 00000000000H
/// DEST[MAXVL-1:128] := 0
/// PSADBW (128-bit Legacy SSE Version)
/// TEMP0 := ABS(DEST[7:0] - SRC[7:0])
/// (* Repeat operation for bytes 2 through 14 *)
/// TEMP15 := ABS(DEST[127:120] - SRC[127:120])
/// DEST[15:0] := SUM(TEMP0:TEMP7)
/// DEST[63:16] := 000000000000H
/// DEST[79:64] := SUM(TEMP8:TEMP15)
/// DEST[127:80] := 00000000000
/// DEST[MAXVL-1:128] (Unmodified)
/// PSADBW (64-bit Operand)
/// TEMP0 := ABS(DEST[7:0] - SRC[7:0])
/// (* Repeat operation for bytes 2 through 6 *)
/// TEMP7 := ABS(DEST[63:56] - SRC[63:56])
/// DEST[15:0] := SUM(TEMP0:TEMP7)
/// DEST[63:16] := 000000000000H
/// ```
#[box_to_static_reference]
pub(super) fn psadbw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSHUFB (With 64-bit Operands)
/// TEMP := DEST
/// for i = 0 to 7 {
///     if (SRC[(i * 8)+7] = 1 ) then
///         DEST[(i*8)+7...(i*8)+0] := 0;
///     else
///         index[2..0] := SRC[(i*8)+2 .. (i*8)+0];
///         DEST[(i*8)+7...(i*8)+0] := TEMP[(index*8+7)..(index*8+0)];
///     endif;
/// }
/// PSHUFB (with 128 bit operands)
/// TEMP := DEST
/// for i = 0 to 15 {
///     if (SRC[(i * 8)+7] = 1 ) then
///         DEST[(i*8)+7..(i*8)+0] := 0;
///     else
///         index[3..0] := SRC[(i*8)+3 .. (i*8)+0];
///         DEST[(i*8)+7..(i*8)+0] := TEMP[(index*8+7)..(index*8+0)];
///     endif
/// }
/// VPSHUFB (VEX.128 Encoded Version)
/// for i = 0 to 15 {
///     if (SRC2[(i * 8)+7] = 1) then
///         DEST[(i*8)+7..(i*8)+0] := 0;
///         else
///         index[3..0] := SRC2[(i*8)+3 .. (i*8)+0];
///         DEST[(i*8)+7..(i*8)+0] := SRC1[(index*8+7)..(index*8+0)];
///     endif
/// }
/// DEST[MAXVL-1:128] := 0
/// VPSHUFB (VEX.256 Encoded Version)
/// for i = 0 to 15 {
///     if (SRC2[(i * 8)+7] == 1 ) then
///         DEST[(i*8)+7..(i*8)+0] := 0;
///         else
///         index[3..0] := SRC2[(i*8)+3 .. (i*8)+0];
///         DEST[(i*8)+7..(i*8)+0] := SRC1[(index*8+7)..(index*8+0)];
///     endif
///     if (SRC2[128 + (i * 8)+7] == 1 ) then
///         DEST[128 + (i*8)+7..(i*8)+0] := 0;
///         else
///         index[3..0] := SRC2[128 + (i*8)+3 .. (i*8)+0];
///         DEST[128 + (i*8)+7..(i*8)+0] := SRC1[128 + (index*8+7)..(index*8+0)];
/// }
/// VPSHUFB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// jmask := (KL-1) & ~0xF
///                     // 0x00, 0x10, 0x30 depending on the VL
/// FOR j = 0 TO KL-1
///                     // dest
///     IF kl[ i ] or no_masking
///         index := src.byte[ j ];
///         IF index & 0x80
///             Dest.byte[ j ] := 0;
///         ELSE
///             index := (index & 0xF) + (j & jmask);
///                     // 16-element in-lane lookup
///             Dest.byte[ j ] := src.byte[ index ];
///     ELSE if zeroing
///         Dest.byte[ j ] := 0;
/// DEST[MAXVL-1:VL] := 0;
///                 Figure 4-15.  PSHUFB with 64-Bit Operands
/// ```
#[box_to_static_reference]
pub(super) fn pshufb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSHUFD (128-bit Legacy SSE Version)
/// DEST[31:0] := (SRC >> (ORDER[1:0] * 32))[31:0];
/// DEST[63:32] := (SRC >> (ORDER[3:2] * 32))[31:0];
/// DEST[95:64] := (SRC >> (ORDER[5:4] * 32))[31:0];
/// DEST[127:96] := (SRC >> (ORDER[7:6] * 32))[31:0];
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSHUFD (VEX.128 Encoded Version)
/// DEST[31:0] := (SRC >> (ORDER[1:0] * 32))[31:0];
/// DEST[63:32] := (SRC >> (ORDER[3:2] * 32))[31:0];
/// DEST[95:64] := (SRC >> (ORDER[5:4] * 32))[31:0];
/// DEST[127:96] := (SRC >> (ORDER[7:6] * 32))[31:0];
/// DEST[MAXVL-1:128] := 0
/// VPSHUFD (VEX.256 Encoded Version)
/// DEST[31:0] := (SRC[127:0] >> (ORDER[1:0] * 32))[31:0];
/// DEST[63:32] := (SRC[127:0] >> (ORDER[3:2] * 32))[31:0];
/// DEST[95:64] := (SRC[127:0] >> (ORDER[5:4] * 32))[31:0];
/// DEST[127:96] := (SRC[127:0] >> (ORDER[7:6] * 32))[31:0];
/// DEST[159:128] := (SRC[255:128] >> (ORDER[1:0] * 32))[31:0];
/// DEST[191:160] := (SRC[255:128] >> (ORDER[3:2] * 32))[31:0];
/// DEST[223:192] := (SRC[255:128] >> (ORDER[5:4] * 32))[31:0];
/// DEST[255:224] := (SRC[255:128] >> (ORDER[7:6] * 32))[31:0];
/// DEST[MAXVL-1:256] := 0
/// VPSHUFD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC *is memory*)
///         THEN TMP_SRC[i+31:i] := SRC[31:0]
///         ELSE TMP_SRC[i+31:i] := SRC[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL >= 128
///     TMP_DEST[31:0] := (TMP_SRC[127:0] >> (ORDER[1:0] * 32))[31:0];
///     TMP_DEST[63:32] := (TMP_SRC[127:0] >> (ORDER[3:2] * 32))[31:0];
///     TMP_DEST[95:64] := (TMP_SRC[127:0] >> (ORDER[5:4] * 32))[31:0];
///     TMP_DEST[127:96] := (TMP_SRC[127:0] >> (ORDER[7:6] * 32))[31:0];
/// FI;
/// IF VL >= 256
///     TMP_DEST[159:128] := (TMP_SRC[255:128] >> (ORDER[1:0] * 32))[31:0];
///     TMP_DEST[191:160] := (TMP_SRC[255:128] >> (ORDER[3:2] * 32))[31:0];
///     TMP_DEST[223:192] := (TMP_SRC[255:128] >> (ORDER[5:4] * 32))[31:0];
///     TMP_DEST[255:224] := (TMP_SRC[255:128] >> (ORDER[7:6] * 32))[31:0];
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := (TMP_SRC[383:256] >> (ORDER[1:0] * 32))[31:0];
///     TMP_DEST[319:288] := (TMP_SRC[383:256] >> (ORDER[3:2] * 32))[31:0];
///     TMP_DEST[351:320] := (TMP_SRC[383:256] >> (ORDER[5:4] * 32))[31:0];
///     TMP_DEST[383:352] := (TMP_SRC[383:256] >> (ORDER[7:6] * 32))[31:0];
///     TMP_DEST[415:384] := (TMP_SRC[511:384] >> (ORDER[1:0] * 32))[31:0];
///     TMP_DEST[447:416] := (TMP_SRC[511:384] >> (ORDER[3:2] * 32))[31:0];
///     TMP_DEST[479:448] := (TMP_SRC[511:384] >> (ORDER[5:4] * 32))[31:0];
///     TMP_DEST[511:480] := (TMP_SRC[511:384] >> (ORDER[7:6] * 32))[31:0];
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pshufd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSHUFHW (128-bit Legacy SSE Version)
/// DEST[63:0] := SRC[63:0]
/// DEST[79:64] := (SRC >> (imm[1:0] *16))[79:64]
/// DEST[95:80] := (SRC >> (imm[3:2] * 16))[79:64]
/// DEST[111:96] := (SRC >> (imm[5:4] * 16))[79:64]
/// DEST[127:112] := (SRC >> (imm[7:6] * 16))[79:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSHUFHW (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0]
/// DEST[79:64] := (SRC1 >> (imm[1:0] *16))[79:64]
/// DEST[95:80] := (SRC1 >> (imm[3:2] * 16))[79:64]
/// DEST[111:96] := (SRC1 >> (imm[5:4] * 16))[79:64]
/// DEST[127:112] := (SRC1 >> (imm[7:6] * 16))[79:64]
/// DEST[MAXVL-1:128] := 0
/// VPSHUFHW (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0]
/// DEST[79:64] := (SRC1 >> (imm[1:0] *16))[79:64]
/// DEST[95:80] := (SRC1 >> (imm[3:2] * 16))[79:64]
/// DEST[111:96] := (SRC1 >> (imm[5:4] * 16))[79:64]
/// DEST[127:112] := (SRC1 >> (imm[7:6] * 16))[79:64]
/// DEST[191:128] := SRC1[191:128]
/// DEST[207192] := (SRC1 >> (imm[1:0] *16))[207:192]
/// DEST[223:208] := (SRC1 >> (imm[3:2] * 16))[207:192]
/// DEST[239:224] := (SRC1 >> (imm[5:4] * 16))[207:192]
/// DEST[255:240] := (SRC1 >> (imm[7:6] * 16))[207:192]
/// DEST[MAXVL-1:256] := 0
/// VPSHUFHW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL >= 128
///     TMP_DEST[63:0] := SRC1[63:0]
///     TMP_DEST[79:64] := (SRC1 >> (imm[1:0] *16))[79:64]
///     TMP_DEST[95:80] := (SRC1 >> (imm[3:2] * 16))[79:64]
///     TMP_DEST[111:96] := (SRC1 >> (imm[5:4] * 16))[79:64]
///     TMP_DEST[127:112] := (SRC1 >> (imm[7:6] * 16))[79:64]
/// FI;
/// IF VL >= 256
///     TMP_DEST[191:128] := SRC1[191:128]
///     TMP_DEST[207:192] := (SRC1 >> (imm[1:0] *16))[207:192]
///     TMP_DEST[223:208] := (SRC1 >> (imm[3:2] * 16))[207:192]
///     TMP_DEST[239:224] := (SRC1 >> (imm[5:4] * 16))[207:192]
///     TMP_DEST[255:240] := (SRC1 >> (imm[7:6] * 16))[207:192]
/// FI;
/// IF VL >= 512
///     TMP_DEST[319:256] := SRC1[319:256]
///     TMP_DEST[351:336] := (SRC1 >> (imm[3:2] * 16))[335:320]
///     TMP_DEST[367:352] := (SRC1 >> (imm[5:4] * 16))[335:320]
///     TMP_DEST[383:368] := (SRC1 >> (imm[7:6] * 16))[335:320]
///     TMP_DEST[447:384] := SRC1[447:384]
///     TMP_DEST[463:448] := (SRC1 >> (imm[1:0] *16))[463:448]
///     TMP_DEST[479:464] := (SRC1 >> (imm[3:2] * 16))[463:448]
///     TMP_DEST[495:480] := (SRC1 >> (imm[5:4] * 16))[463:448]
///     TMP_DEST[511:496] := (SRC1 >> (imm[7:6] * 16))[463:448]
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i];
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pshufhw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSHUFLW (128-bit Legacy SSE Version)
/// DEST[15:0] := (SRC >> (imm[1:0] *16))[15:0]
/// DEST[31:16] := (SRC >> (imm[3:2] * 16))[15:0]
/// DEST[47:32] := (SRC >> (imm[5:4] * 16))[15:0]
/// DEST[63:48] := (SRC >> (imm[7:6] * 16))[15:0]
/// DEST[127:64] := SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSHUFLW (VEX.128 Encoded Version)
/// DEST[15:0] := (SRC1 >> (imm[1:0] *16))[15:0]
/// DEST[31:16] := (SRC1 >> (imm[3:2] * 16))[15:0]
/// DEST[47:32] := (SRC1 >> (imm[5:4] * 16))[15:0]
/// DEST[63:48] := (SRC1 >> (imm[7:6] * 16))[15:0]
/// DEST[127:64] := SRC[127:64]
/// DEST[MAXVL-1:128] := 0
/// VPSHUFLW (VEX.256 Encoded Version)
/// DEST[15:0] := (SRC1 >> (imm[1:0] *16))[15:0]
/// DEST[31:16] := (SRC1 >> (imm[3:2] * 16))[15:0]
/// DEST[47:32] := (SRC1 >> (imm[5:4] * 16))[15:0]
/// DEST[63:48] := (SRC1 >> (imm[7:6] * 16))[15:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[143:128] := (SRC1 >> (imm[1:0] *16))[143:128]
/// DEST[159:144] := (SRC1 >> (imm[3:2] * 16))[143:128]
/// DEST[175:160] := (SRC1 >> (imm[5:4] * 16))[143:128]
/// DEST[191:176] := (SRC1 >> (imm[7:6] * 16))[143:128]
/// DEST[255:192] := SRC1[255:192]
/// DEST[MAXVL-1:256] := 0
/// VPSHUFLW (EVEX.U1.512 Encoded Version)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL >= 128
///     TMP_DEST[15:0] := (SRC1 >> (imm[1:0] *16))[15:0]
///     TMP_DEST[31:16] := (SRC1 >> (imm[3:2] * 16))[15:0]
///     TMP_DEST[47:32] := (SRC1 >> (imm[5:4] * 16))[15:0]
///     TMP_DEST[63:48] := (SRC1 >> (imm[7:6] * 16))[15:0]
///     TMP_DEST[127:64] := SRC1[127:64]
/// FI;
/// IF VL >= 256
///     TMP_DEST[143:128] := (SRC1 >> (imm[1:0] *16))[143:128]
///     TMP_DEST[159:144] := (SRC1 >> (imm[3:2] * 16))[143:128]
///     TMP_DEST[175:160] := (SRC1 >> (imm[5:4] * 16))[143:128]
///     TMP_DEST[191:176] := (SRC1 >> (imm[7:6] * 16))[143:128]
///     TMP_DEST[255:192] := SRC1[255:192]
/// FI;
/// IF VL >= 512
///     TMP_DEST[271:256] := (SRC1 >> (imm[1:0] *16))[271:256]
///     TMP_DEST[287:272] := (SRC1 >> (imm[3:2] * 16))[271:256]
///     TMP_DEST[303:288] := (SRC1 >> (imm[5:4] * 16))[271:256]
///     TMP_DEST[319:304] := (SRC1 >> (imm[7:6] * 16))[271:256]
///     TMP_DEST[399:384] := (SRC1 >> (imm[1:0] *16))[399:384]
///     TMP_DEST[415:400] := (SRC1 >> (imm[3:2] * 16))[399:384]
///     TMP_DEST[431:416] := (SRC1 >> (imm[5:4] * 16))[399:384]
///     TMP_DEST[447:432] := (SRC1 >> (imm[7:6] * 16))[399:384]
///     TMP_DEST[511:448] := SRC1[511:448]
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i];
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pshuflw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[15:0] := (SRC >> (ORDER[1:0] * 16))[15:0];
/// DEST[31:16] := (SRC >> (ORDER[3:2] * 16))[15:0];
/// DEST[47:32] := (SRC >> (ORDER[5:4] * 16))[15:0];
/// DEST[63:48] := (SRC >> (ORDER[7:6] * 16))[15:0];
/// ```
#[box_to_static_reference]
pub(super) fn pshufw() -> &'static [IrStatement] {
    let stmt_0 = assign(b::shr(o2(), b::mul(unknown_data(), c(16))), o1(), o1_size());
    let stmt_1 = assign(b::shr(o2(), b::mul(unknown_data(), c(16))), o1(), o1_size());
    let stmt_2 = assign(b::shr(o2(), b::mul(unknown_data(), c(16))), o1(), o1_size());
    let stmt_3 = assign(b::shr(o2(), b::mul(unknown_data(), c(16))), o1(), o1_size());
    [stmt_0, stmt_1, stmt_2, stmt_3].into()
}

/// # Pseudocode
/// ```text
/// def byte_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// 
/// def word_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// 
/// def dword_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// PSIGNB srcdest, src// MMX 64-bit Operands
/// VL=64
/// KL := VL/8
/// for i in 0...KL-1:
/// srcdest.byte[i] := byte_sign(src.byte[i], srcdest.byte[i])
/// PSIGNW srcdest, src   // MMX 64-bit Operands
/// VL=64
/// KL := VL/16
/// FOR i in 0...KL-1:
/// srcdest.word[i] := word_sign(src.word[i], srcdest.word[i])
/// PSIGND srcdest, src   // MMX 64-bit Operands
/// VL=64
/// KL := VL/32
/// FOR i in 0...KL-1:
/// srcdest.dword[i] := dword_sign(src.dword[i], srcdest.dword[i])
/// PSIGNB srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/8
/// FOR i in 0...KL-1:
/// srcdest.byte[i] := byte_sign(src.byte[i], srcdest.byte[i])
/// PSIGNW srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/16
/// FOR i in 0...KL-1:
/// srcdest.word[i] := word_sign(src.word[i], srcdest.word[i])
/// PSIGND srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/32
/// FOR i in 0...KL-1:
/// srcdest.dword[i] := dword_sign(src.dword[i], srcdest.dword[i])
/// VPSIGNB dest, src1, src2   // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/8
/// FOR i in 0...KL-1:
/// dest.byte[i] := byte_sign(src2.byte[i], src1.byte[i])
/// DEST[MAXVL-1:VL] := 0
/// VPSIGNW dest, src1, src2   // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/16
/// FOR i in 0...KL-1:
/// dest.word[i] := word_sign(src2.word[i], src1.word[i])
/// DEST[MAXVL-1:VL] := 0
/// VPSIGND dest, src1, src2    // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/32
/// FOR i in 0...KL-1:
/// dest.dword[i] := dword_sign(src2.dword[i], src1.dword[i])
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn psignb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def byte_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// 
/// def word_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// 
/// def dword_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// PSIGNB srcdest, src// MMX 64-bit Operands
/// VL=64
/// KL := VL/8
/// for i in 0...KL-1:
/// srcdest.byte[i] := byte_sign(src.byte[i], srcdest.byte[i])
/// PSIGNW srcdest, src   // MMX 64-bit Operands
/// VL=64
/// KL := VL/16
/// FOR i in 0...KL-1:
/// srcdest.word[i] := word_sign(src.word[i], srcdest.word[i])
/// PSIGND srcdest, src   // MMX 64-bit Operands
/// VL=64
/// KL := VL/32
/// FOR i in 0...KL-1:
/// srcdest.dword[i] := dword_sign(src.dword[i], srcdest.dword[i])
/// PSIGNB srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/8
/// FOR i in 0...KL-1:
/// srcdest.byte[i] := byte_sign(src.byte[i], srcdest.byte[i])
/// PSIGNW srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/16
/// FOR i in 0...KL-1:
/// srcdest.word[i] := word_sign(src.word[i], srcdest.word[i])
/// PSIGND srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/32
/// FOR i in 0...KL-1:
/// srcdest.dword[i] := dword_sign(src.dword[i], srcdest.dword[i])
/// VPSIGNB dest, src1, src2   // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/8
/// FOR i in 0...KL-1:
/// dest.byte[i] := byte_sign(src2.byte[i], src1.byte[i])
/// DEST[MAXVL-1:VL] := 0
/// VPSIGNW dest, src1, src2   // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/16
/// FOR i in 0...KL-1:
/// dest.word[i] := word_sign(src2.word[i], src1.word[i])
/// DEST[MAXVL-1:VL] := 0
/// VPSIGND dest, src1, src2    // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/32
/// FOR i in 0...KL-1:
/// dest.dword[i] := dword_sign(src2.dword[i], src1.dword[i])
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn psignd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// def byte_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// 
/// def word_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// 
/// def dword_sign(control, input_val):
/// if control<0:
/// return negate(input_val)
/// elif control==0:
/// return 0
/// return input_val
/// PSIGNB srcdest, src// MMX 64-bit Operands
/// VL=64
/// KL := VL/8
/// for i in 0...KL-1:
/// srcdest.byte[i] := byte_sign(src.byte[i], srcdest.byte[i])
/// PSIGNW srcdest, src   // MMX 64-bit Operands
/// VL=64
/// KL := VL/16
/// FOR i in 0...KL-1:
/// srcdest.word[i] := word_sign(src.word[i], srcdest.word[i])
/// PSIGND srcdest, src   // MMX 64-bit Operands
/// VL=64
/// KL := VL/32
/// FOR i in 0...KL-1:
/// srcdest.dword[i] := dword_sign(src.dword[i], srcdest.dword[i])
/// PSIGNB srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/8
/// FOR i in 0...KL-1:
/// srcdest.byte[i] := byte_sign(src.byte[i], srcdest.byte[i])
/// PSIGNW srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/16
/// FOR i in 0...KL-1:
/// srcdest.word[i] := word_sign(src.word[i], srcdest.word[i])
/// PSIGND srcdest, src   // SSE 128-bit Operands
/// VL=128
/// KL := VL/32
/// FOR i in 0...KL-1:
/// srcdest.dword[i] := dword_sign(src.dword[i], srcdest.dword[i])
/// VPSIGNB dest, src1, src2   // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/8
/// FOR i in 0...KL-1:
/// dest.byte[i] := byte_sign(src2.byte[i], src1.byte[i])
/// DEST[MAXVL-1:VL] := 0
/// VPSIGNW dest, src1, src2   // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/16
/// FOR i in 0...KL-1:
/// dest.word[i] := word_sign(src2.word[i], src1.word[i])
/// DEST[MAXVL-1:VL] := 0
/// VPSIGND dest, src1, src2    // AVX 128-bit or 256-bit Operands
/// VL=(128,256)
/// KL := VL/32
/// FOR i in 0...KL-1:
/// dest.dword[i] := dword_sign(src2.dword[i], src1.dword[i])
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn psignw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSLLW (With 64-bit Operand)
///     IF (COUNT > 15)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST[15:0] := ZeroExtend(DEST[15:0] << COUNT);
///         (* Repeat shift operation for 2nd and 3rd words *)
///         DEST[63:48] := ZeroExtend(DEST[63:48] << COUNT);
///     FI;
/// PSLLD (with 64-bit operand)
///     IF (COUNT > 31)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST[31:0] := ZeroExtend(DEST[31:0] << COUNT);
///         DEST[63:32] := ZeroExtend(DEST[63:32] << COUNT);
///     FI;
/// PSLLQ (With 64-bit Operand)
///     IF (COUNT > 63)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST := ZeroExtend(DEST << COUNT);
///     FI;
/// LOGICAL_LEFT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] << COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[127:112] := ZeroExtend(SRC[127:112] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[127:96] := ZeroExtend(SRC[127:96] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] << COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
///     DEST[255:240] := ZeroExtend(SRC[255:240] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[255:224] := ZeroExtend(SRC[255:224] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] << COUNT)
///     DEST[191:128] := ZeroExtend(SRC[191:128] << COUNT);
///     DEST[255:192] := ZeroExtend(SRC[255:192] << COUNT);
/// FI;
/// VPSLLW (EVEX Versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLW (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLW (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_WORD_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLW (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLW (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLW (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLW (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSLLD (EVEX versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LOGICAL_LEFT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LOGICAL_LEFT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSLLD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLD (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLD (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLD (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLD (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLD (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLD (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSLLQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LOGICAL_LEFT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LOGICAL_LEFT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSLLQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLQ (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLQ (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLQ (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLQ (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLQ (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLQ (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn pslld() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSLLDQ (EVEX.U1.512 Encoded Version)
/// TEMP := COUNT
/// IF (TEMP > 15) THEN TEMP := 16; FI
/// DEST[127:0] := SRC[127:0] << (TEMP * 8)
/// DEST[255:128] := SRC[255:128] << (TEMP * 8)
/// DEST[383:256] := SRC[383:256] << (TEMP * 8)
/// DEST[511:384] := SRC[511:384] << (TEMP * 8)
/// DEST[MAXVL-1:512] := 0
/// VPSLLDQ (VEX.256 and EVEX.256 Encoded Version)
/// TEMP := COUNT
/// IF (TEMP > 15) THEN TEMP := 16; FI
/// DEST[127:0] := SRC[127:0] << (TEMP * 8)
/// DEST[255:128] := SRC[255:128] << (TEMP * 8)
/// DEST[MAXVL-1:256] := 0
/// VPSLLDQ (VEX.128 and EVEX.128 Encoded Version)
/// TEMP := COUNT
/// IF (TEMP > 15) THEN TEMP := 16; FI
/// DEST := SRC << (TEMP * 8)
/// DEST[MAXVL-1:128] := 0
/// PSLLDQ(128-bit Legacy SSE Version)
/// TEMP := COUNT
/// IF (TEMP > 15) THEN TEMP := 16; FI
/// DEST := DEST << (TEMP * 8)
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn pslldq() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSLLW (With 64-bit Operand)
///     IF (COUNT > 15)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST[15:0] := ZeroExtend(DEST[15:0] << COUNT);
///         (* Repeat shift operation for 2nd and 3rd words *)
///         DEST[63:48] := ZeroExtend(DEST[63:48] << COUNT);
///     FI;
/// PSLLD (with 64-bit operand)
///     IF (COUNT > 31)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST[31:0] := ZeroExtend(DEST[31:0] << COUNT);
///         DEST[63:32] := ZeroExtend(DEST[63:32] << COUNT);
///     FI;
/// PSLLQ (With 64-bit Operand)
///     IF (COUNT > 63)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST := ZeroExtend(DEST << COUNT);
///     FI;
/// LOGICAL_LEFT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] << COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[127:112] := ZeroExtend(SRC[127:112] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[127:96] := ZeroExtend(SRC[127:96] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] << COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
///     DEST[255:240] := ZeroExtend(SRC[255:240] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[255:224] := ZeroExtend(SRC[255:224] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] << COUNT)
///     DEST[191:128] := ZeroExtend(SRC[191:128] << COUNT);
///     DEST[255:192] := ZeroExtend(SRC[255:192] << COUNT);
/// FI;
/// VPSLLW (EVEX Versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLW (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLW (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_WORD_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLW (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLW (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLW (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLW (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSLLD (EVEX versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LOGICAL_LEFT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LOGICAL_LEFT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSLLD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLD (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLD (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLD (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLD (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLD (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLD (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSLLQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LOGICAL_LEFT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LOGICAL_LEFT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSLLQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLQ (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLQ (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLQ (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLQ (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLQ (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLQ (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psllq() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSLLW (With 64-bit Operand)
///     IF (COUNT > 15)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST[15:0] := ZeroExtend(DEST[15:0] << COUNT);
///         (* Repeat shift operation for 2nd and 3rd words *)
///         DEST[63:48] := ZeroExtend(DEST[63:48] << COUNT);
///     FI;
/// PSLLD (with 64-bit operand)
///     IF (COUNT > 31)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST[31:0] := ZeroExtend(DEST[31:0] << COUNT);
///         DEST[63:32] := ZeroExtend(DEST[63:32] << COUNT);
///     FI;
/// PSLLQ (With 64-bit Operand)
///     IF (COUNT > 63)
///     THEN
///         DEST[64:0] := 0000000000000000H;
///     ELSE
///         DEST := ZeroExtend(DEST << COUNT);
///     FI;
/// LOGICAL_LEFT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] << COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[127:112] := ZeroExtend(SRC[127:112] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[127:96] := ZeroExtend(SRC[127:96] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] << COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
///     DEST[255:240] := ZeroExtend(SRC[255:240] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] << COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[255:224] := ZeroExtend(SRC[255:224] << COUNT);
/// FI;
/// LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
///     DEST[255:128] := 00000000000000000000000000000000H
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] << COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] << COUNT)
///     DEST[191:128] := ZeroExtend(SRC[191:128] << COUNT);
///     DEST[255:192] := ZeroExtend(SRC[255:192] << COUNT);
/// FI;
/// VPSLLW (EVEX Versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLW (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLW (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_WORD_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLW (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLW (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLW (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLW (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSLLD (EVEX versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LOGICAL_LEFT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LOGICAL_LEFT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSLLD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLD (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLD (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLD (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLD (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLD (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLD (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSLLQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LOGICAL_LEFT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LOGICAL_LEFT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSLLQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSLLQ (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLQ (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_LEFT_SHIFT_QWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSLLQ (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSLLQ (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSLLQ (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSLLQ (xmm, imm8)
/// DEST[127:0] := LOGICAL_LEFT_SHIFT_QWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psllw() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSRAW (With 64-bit Operand)
///     IF (COUNT > 15)
///         THEN COUNT := 16;
///     FI;
///     DEST[15:0] := SignExtend(DEST[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd and 3rd words *)
///     DEST[63:48] := SignExtend(DEST[63:48] >> COUNT);
/// PSRAD (with 64-bit operand)
///     IF (COUNT > 31)
///         THEN COUNT := 32;
///     FI;
///     DEST[31:0] := SignExtend(DEST[31:0] >> COUNT);
///     DEST[63:32] := SignExtend(DEST[63:32] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := SignBit
/// ELSE
///     DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
/// FI;
/// ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := SignBit
/// ELSE
///     DEST[63:0] := SignExtend(SRC[63:0] >> COUNT);
/// FI;
/// ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
///     THENCOUNT := 16;
/// FI;
/// DEST[15:0] := SignExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
/// DEST[255:240] := SignExtend(SRC[255:240] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
///     THENCOUNT := 32;
/// FI;
/// DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[255:224] := SignExtend(SRC[255:224] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_QWORDS(SRC, COUNT_SRC, VL) ; VL: 128b, 256b or 512b
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
///     THENCOUNT := 64;
/// FI;
/// DEST[63:0] := SignExtend(SRC[63:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[VL-1:VL-64] := SignExtend(SRC[VL-1:VL-64] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
///     THENCOUNT := 16;
/// FI;
/// DEST[15:0] := SignExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[127:112] := SignExtend(SRC[127:112] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
///     THENCOUNT := 32;
/// FI;
/// DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
/// DEST[127:96] := SignExtend(SRC[127:96] >> COUNT);
/// VPSRAW (EVEX versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAW (ymm, ymm, xmm/m128) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPSRAW (ymm, imm8) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0
/// VPSRAW (xmm, xmm, xmm/m128) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRAW (xmm, imm8) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRAW (xmm, xmm, xmm/m128)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRAW (xmm, imm8)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRAD (EVEX Versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSRAD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAD (ymm, ymm, xmm/m128) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPSRAD (ymm, imm8) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0
/// VPSRAD (xmm, xmm, xmm/m128) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRAD (xmm, imm8) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRAD (xmm, xmm, xmm/m128)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRAD (xmm, imm8)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRAQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSRAQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// TMP_DEST[VL-1:0] := ARITHMETIC_RIGHT_SHIFT_QWORDS(SRC1[VL-1:0], SRC2, VL)
/// FOR j := 0 TO 7
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn psrad() -> &'static [IrStatement] {
    let assignment = assign(b::sar(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSRAW (With 64-bit Operand)
///     IF (COUNT > 15)
///         THEN COUNT := 16;
///     FI;
///     DEST[15:0] := SignExtend(DEST[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd and 3rd words *)
///     DEST[63:48] := SignExtend(DEST[63:48] >> COUNT);
/// PSRAD (with 64-bit operand)
///     IF (COUNT > 31)
///         THEN COUNT := 32;
///     FI;
///     DEST[31:0] := SignExtend(DEST[31:0] >> COUNT);
///     DEST[63:32] := SignExtend(DEST[63:32] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := SignBit
/// ELSE
///     DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
/// FI;
/// ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := SignBit
/// ELSE
///     DEST[63:0] := SignExtend(SRC[63:0] >> COUNT);
/// FI;
/// ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
///     THENCOUNT := 16;
/// FI;
/// DEST[15:0] := SignExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
/// DEST[255:240] := SignExtend(SRC[255:240] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
///     THENCOUNT := 32;
/// FI;
/// DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[255:224] := SignExtend(SRC[255:224] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_QWORDS(SRC, COUNT_SRC, VL) ; VL: 128b, 256b or 512b
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
///     THENCOUNT := 64;
/// FI;
/// DEST[63:0] := SignExtend(SRC[63:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[VL-1:VL-64] := SignExtend(SRC[VL-1:VL-64] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
///     THENCOUNT := 16;
/// FI;
/// DEST[15:0] := SignExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[127:112] := SignExtend(SRC[127:112] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
///     THENCOUNT := 32;
/// FI;
/// DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
/// DEST[127:96] := SignExtend(SRC[127:96] >> COUNT);
/// VPSRAW (EVEX versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAW (ymm, ymm, xmm/m128) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPSRAW (ymm, imm8) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0
/// VPSRAW (xmm, xmm, xmm/m128) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRAW (xmm, imm8) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRAW (xmm, xmm, xmm/m128)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRAW (xmm, imm8)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRAD (EVEX Versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSRAD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAD (ymm, ymm, xmm/m128) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPSRAD (ymm, imm8) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0
/// VPSRAD (xmm, xmm, xmm/m128) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRAD (xmm, imm8) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRAD (xmm, xmm, xmm/m128)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRAD (xmm, imm8)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRAQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSRAQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// TMP_DEST[VL-1:0] := ARITHMETIC_RIGHT_SHIFT_QWORDS(SRC1[VL-1:0], SRC2, VL)
/// FOR j := 0 TO 7
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn psraq() -> &'static [IrStatement] {
    let sar_1 = b::sar(o1(), o2());
    let sar_1_flags = calc_flags_automatically(sar_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let sar_2 = b::sar(o1(), c(1));
    let sar_2_flags = calc_flags_automatically(sar_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let cond = condition(is_o2_exists(), [sar_1_flags, assign(sar_1, o1(), o1_size())], [sar_2_flags, assign(sar_2, o1(), o1_size())]);
    extend_undefined_flags(&[cond], &[&of, &af, &cf])
}

/// # Pseudocode
/// ```text
/// PSRAW (With 64-bit Operand)
///     IF (COUNT > 15)
///         THEN COUNT := 16;
///     FI;
///     DEST[15:0] := SignExtend(DEST[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd and 3rd words *)
///     DEST[63:48] := SignExtend(DEST[63:48] >> COUNT);
/// PSRAD (with 64-bit operand)
///     IF (COUNT > 31)
///         THEN COUNT := 32;
///     FI;
///     DEST[31:0] := SignExtend(DEST[31:0] >> COUNT);
///     DEST[63:32] := SignExtend(DEST[63:32] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := SignBit
/// ELSE
///     DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
/// FI;
/// ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := SignBit
/// ELSE
///     DEST[63:0] := SignExtend(SRC[63:0] >> COUNT);
/// FI;
/// ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
///     THENCOUNT := 16;
/// FI;
/// DEST[15:0] := SignExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
/// DEST[255:240] := SignExtend(SRC[255:240] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
///     THENCOUNT := 32;
/// FI;
/// DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[255:224] := SignExtend(SRC[255:224] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_QWORDS(SRC, COUNT_SRC, VL) ; VL: 128b, 256b or 512b
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
///     THENCOUNT := 64;
/// FI;
/// DEST[63:0] := SignExtend(SRC[63:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[VL-1:VL-64] := SignExtend(SRC[VL-1:VL-64] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
///     THENCOUNT := 16;
/// FI;
/// DEST[15:0] := SignExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
/// DEST[127:112] := SignExtend(SRC[127:112] >> COUNT);
/// ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
///     THENCOUNT := 32;
/// FI;
/// DEST[31:0] := SignExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
/// DEST[127:96] := SignExtend(SRC[127:96] >> COUNT);
/// VPSRAW (EVEX versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAW (ymm, ymm, xmm/m128) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPSRAW (ymm, imm8) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_WORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0
/// VPSRAW (xmm, xmm, xmm/m128) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRAW (xmm, imm8) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRAW (xmm, xmm, xmm/m128)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRAW (xmm, imm8)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRAD (EVEX Versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := ARITHMETIC_RIGHT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSRAD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRAD (ymm, ymm, xmm/m128) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPSRAD (ymm, imm8) - VEX
/// DEST[255:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0
/// VPSRAD (xmm, xmm, xmm/m128) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRAD (xmm, imm8) - VEX
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRAD (xmm, xmm, xmm/m128)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRAD (xmm, imm8)
/// DEST[127:0] := ARITHMETIC_RIGHT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRAQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := ARITHMETIC_RIGHT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSRAQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// TMP_DEST[VL-1:0] := ARITHMETIC_RIGHT_SHIFT_QWORDS(SRC1[VL-1:0], SRC2, VL)
/// FOR j := 0 TO 7
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_DEST[i+63:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn psraw() -> &'static [IrStatement] {
    let assignment = assign(b::sar(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSRLW (With 64-bit Operand)
///     IF (COUNT > 15)
///     THEN
///         DEST[64:0] := 0000000000000000H
///     ELSE
///         DEST[15:0] := ZeroExtend(DEST[15:0] >> COUNT);
///         (* Repeat shift operation for 2nd and 3rd words *)
///         DEST[63:48] := ZeroExtend(DEST[63:48] >> COUNT);
///     FI;
/// PSRLD (With 64-bit Operand)
///     IF (COUNT > 31)
///     THEN
///         DEST[64:0] := 0000000000000000H
///     ELSE
///         DEST[31:0] := ZeroExtend(DEST[31:0] >> COUNT);
///         DEST[63:32] := ZeroExtend(DEST[63:32] >> COUNT);
///     FI;
/// PSRLQ (With 64-bit Operand)
///     IF (COUNT > 63)
///     THEN
///         DEST[64:0] := 0000000000000000H
///         DEST := ZeroExtend(DEST >> COUNT);
///     FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
///     DEST[255:240] := ZeroExtend(SRC[255:240] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[127:112] := ZeroExtend(SRC[127:112] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[255:224] := ZeroExtend(SRC[255:224] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[127:96] := ZeroExtend(SRC[127:96] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] >> COUNT);
///     DEST[191:128] := ZeroExtend(SRC[191:128] >> COUNT);
///     DEST[255:192] := ZeroExtend(SRC[255:192] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] >> COUNT);
/// FI;
/// VPSRLW (EVEX Versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLW (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLW (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLW (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLW (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLW (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLW (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRLD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLD (EVEX Versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LOGICAL_RIGHT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LOGICAL_RIGHT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSRLD (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLD (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLD (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLD (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLD (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLD (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRLQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LOGICAL_RIGHT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LOGICAL_RIGHT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSRLQ (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLQ (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLQ (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLQ (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLQ (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLQ (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psrld() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VPSRLDQ (EVEX.512 Encoded Version)
/// TEMP := COUNT
/// IF (TEMP > 15) THEN TEMP := 16; FI
/// DEST[127:0] := SRC[127:0] >> (TEMP * 8)
/// DEST[255:128] := SRC[255:128] >> (TEMP * 8)
/// DEST[383:256] := SRC[383:256] >> (TEMP * 8)
/// DEST[511:384] := SRC[511:384] >> (TEMP * 8)
/// DEST[MAXVL-1:512] := 0;
/// VPSRLDQ (VEX.256 and EVEX.256 Encoded Version)
/// TEMP := COUNT
/// IF (TEMP > 15) THEN TEMP := 16; FI
/// DEST[127:0] := SRC[127:0] >> (TEMP * 8)
/// DEST[255:128] := SRC[255:128] >> (TEMP * 8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLDQ (VEX.128 and EVEX.128 Encoded Version)
/// TEMP := COUNT
/// IF (TEMP > 15) THEN TEMP := 16; FI
/// DEST := SRC >> (TEMP * 8)
/// DEST[MAXVL-1:128] := 0;
/// PSRLDQ (128-bit Legacy SSE Version)
/// TEMP := COUNT
/// IF (TEMP > 15) THEN TEMP := 16; FI
/// DEST := DEST >> (TEMP * 8)
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psrldq() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSRLW (With 64-bit Operand)
///     IF (COUNT > 15)
///     THEN
///         DEST[64:0] := 0000000000000000H
///     ELSE
///         DEST[15:0] := ZeroExtend(DEST[15:0] >> COUNT);
///         (* Repeat shift operation for 2nd and 3rd words *)
///         DEST[63:48] := ZeroExtend(DEST[63:48] >> COUNT);
///     FI;
/// PSRLD (With 64-bit Operand)
///     IF (COUNT > 31)
///     THEN
///         DEST[64:0] := 0000000000000000H
///     ELSE
///         DEST[31:0] := ZeroExtend(DEST[31:0] >> COUNT);
///         DEST[63:32] := ZeroExtend(DEST[63:32] >> COUNT);
///     FI;
/// PSRLQ (With 64-bit Operand)
///     IF (COUNT > 63)
///     THEN
///         DEST[64:0] := 0000000000000000H
///         DEST := ZeroExtend(DEST >> COUNT);
///     FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
///     DEST[255:240] := ZeroExtend(SRC[255:240] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[127:112] := ZeroExtend(SRC[127:112] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[255:224] := ZeroExtend(SRC[255:224] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[127:96] := ZeroExtend(SRC[127:96] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] >> COUNT);
///     DEST[191:128] := ZeroExtend(SRC[191:128] >> COUNT);
///     DEST[255:192] := ZeroExtend(SRC[255:192] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] >> COUNT);
/// FI;
/// VPSRLW (EVEX Versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLW (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLW (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLW (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLW (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLW (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLW (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRLD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLD (EVEX Versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LOGICAL_RIGHT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LOGICAL_RIGHT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSRLD (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLD (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLD (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLD (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLD (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLD (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRLQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LOGICAL_RIGHT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LOGICAL_RIGHT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSRLQ (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLQ (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLQ (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLQ (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLQ (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLQ (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psrlq() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSRLW (With 64-bit Operand)
///     IF (COUNT > 15)
///     THEN
///         DEST[64:0] := 0000000000000000H
///     ELSE
///         DEST[15:0] := ZeroExtend(DEST[15:0] >> COUNT);
///         (* Repeat shift operation for 2nd and 3rd words *)
///         DEST[63:48] := ZeroExtend(DEST[63:48] >> COUNT);
///     FI;
/// PSRLD (With 64-bit Operand)
///     IF (COUNT > 31)
///     THEN
///         DEST[64:0] := 0000000000000000H
///     ELSE
///         DEST[31:0] := ZeroExtend(DEST[31:0] >> COUNT);
///         DEST[63:32] := ZeroExtend(DEST[63:32] >> COUNT);
///     FI;
/// PSRLQ (With 64-bit Operand)
///     IF (COUNT > 63)
///     THEN
///         DEST[64:0] := 0000000000000000H
///         DEST := ZeroExtend(DEST >> COUNT);
///     FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[31:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS1(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[63:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 15th words *)
///     DEST[255:240] := ZeroExtend(SRC[255:240] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_WORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 15)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[15:0] := ZeroExtend(SRC[15:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 7th words *)
///     DEST[127:112] := ZeroExtend(SRC[127:112] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 31)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[255:224] := ZeroExtend(SRC[255:224] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_DWORDS(SRC, COUNT_SRC)
/// IF (COUNT > 31)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[31:0] := ZeroExtend(SRC[31:0] >> COUNT);
///     (* Repeat shift operation for 2nd through 3rd words *)
///     DEST[127:96] := ZeroExtend(SRC[127:96] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[255:0] := 0
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] >> COUNT);
///     DEST[191:128] := ZeroExtend(SRC[191:128] >> COUNT);
///     DEST[255:192] := ZeroExtend(SRC[255:192] >> COUNT);
/// FI;
/// LOGICAL_RIGHT_SHIFT_QWORDS(SRC, COUNT_SRC)
/// COUNT := COUNT_SRC[63:0];
/// IF (COUNT > 63)
/// THEN
///     DEST[127:0] := 00000000000000000000000000000000H
/// ELSE
///     DEST[63:0] := ZeroExtend(SRC[63:0] >> COUNT);
///     DEST[127:64] := ZeroExtend(SRC[127:64] >> COUNT);
/// FI;
/// VPSRLW (EVEX Versions, xmm/m128)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], SRC2)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLW (EVEX Versions, imm8)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS_128b(SRC1[127:0], imm8)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[255:0], imm8)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1[511:256], imm8)
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLW (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLW (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_WORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLW (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLW (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLW (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLW (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_WORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRLD (EVEX Versions, xmm/m128)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLD (EVEX Versions, imm8)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+31:i] := LOGICAL_RIGHT_SHIFT_DWORDS1(SRC1[31:0], imm8)
///                 ELSE DEST[i+31:i] := LOGICAL_RIGHT_SHIFT_DWORDS1(SRC1[i+31:i], imm8)
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
/// VPSRLD (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLD (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_DWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLD (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLD (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLD (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLD (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_DWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSRLQ (EVEX Versions, xmm/m128)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
/// IF VL = 128
///     TMP_DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS_128b(SRC1[127:0], SRC2)
/// FI;
/// IF VL = 256
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
/// FI;
/// IF VL = 512
///     TMP_DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[255:0], SRC2)
///     TMP_DEST[511:256] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1[511:256], SRC2)
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VPSRLQ (EVEX Versions, imm8)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC1 *is memory*)
///                 THEN DEST[i+63:i] := LOGICAL_RIGHT_SHIFT_QWORDS1(SRC1[63:0], imm8)
///                 ELSE DEST[i+63:i] := LOGICAL_RIGHT_SHIFT_QWORDS1(SRC1[i+63:i], imm8)
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
/// VPSRLQ (ymm, ymm, xmm/m128) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLQ (ymm, imm8) - VEX.256 Encoding
/// DEST[255:0] := LOGICAL_RIGHT_SHIFT_QWORDS_256b(SRC1, imm8)
/// DEST[MAXVL-1:256] := 0;
/// VPSRLQ (xmm, xmm, xmm/m128) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPSRLQ (xmm, imm8) - VEX.128 Encoding
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(SRC1, imm8)
/// DEST[MAXVL-1:128] := 0
/// PSRLQ (xmm, xmm, xmm/m128)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// PSRLQ (xmm, imm8)
/// DEST[127:0] := LOGICAL_RIGHT_SHIFT_QWORDS(DEST, imm8)
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psrlw() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSUBB (With 64-bit Operands)
///     DEST[7:0] := DEST[7:0] - SRC[7:0];
///     (* Repeat subtract operation for 2nd through 7th byte *)
///     DEST[63:56] := DEST[63:56] - SRC[63:56];
/// PSUBW (With 64-bit Operands)
///     DEST[15:0] := DEST[15:0] - SRC[15:0];
///     (* Repeat subtract operation for 2nd and 3rd word *)
///     DEST[63:48] := DEST[63:48] - SRC[63:48];
/// PSUBD (With 64-bit Operands)
///     DEST[31:0] := DEST[31:0] - SRC[31:0];
///     DEST[63:32] := DEST[63:32] - SRC[63:32];
/// PSUBD (With 128-bit Operands)
///     DEST[31:0] := DEST[31:0] - SRC[31:0];
///     (* Repeat subtract operation for 2nd and 3rd doubleword *)
///     DEST[127:96] := DEST[127:96] - SRC[127:96];
/// VPSUBB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC1[i+7:i] - SRC2[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC1[i+15:i] - SRC2[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := SRC1[i+31:i] - SRC2[31:0]
///                 ELSE DEST[i+31:i] := SRC1[i+31:i] - SRC2[i+31:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBB (VEX.256 Encoded Version)
/// DEST[7:0] := SRC1[7:0]-SRC2[7:0]
/// DEST[15:8] := SRC1[15:8]-SRC2[15:8]
/// DEST[23:16] := SRC1[23:16]-SRC2[23:16]
/// DEST[31:24] := SRC1[31:24]-SRC2[31:24]
/// DEST[39:32] := SRC1[39:32]-SRC2[39:32]
/// DEST[47:40] := SRC1[47:40]-SRC2[47:40]
/// DEST[55:48] := SRC1[55:48]-SRC2[55:48]
/// DEST[63:56] := SRC1[63:56]-SRC2[63:56]
/// DEST[71:64] := SRC1[71:64]-SRC2[71:64]
/// DEST[79:72] := SRC1[79:72]-SRC2[79:72]
/// DEST[87:80] := SRC1[87:80]-SRC2[87:80]
/// DEST[95:88] := SRC1[95:88]-SRC2[95:88]
/// DEST[103:96] := SRC1[103:96]-SRC2[103:96]
/// DEST[111:104] := SRC1[111:104]-SRC2[111:104]
/// DEST[119:112] := SRC1[119:112]-SRC2[119:112]
/// DEST[127:120] := SRC1[127:120]-SRC2[127:120]
/// DEST[135:128] := SRC1[135:128]-SRC2[135:128]
/// DEST[151:144] := SRC1[151:144]-SRC2[151:144]
/// DEST[159:152] := SRC1[159:152]-SRC2[159:152]
/// DEST[167:160] := SRC1[167:160]-SRC2[167:160]
/// DEST[175:168] := SRC1[175:168]-SRC2[175:168]
/// DEST[183:176] := SRC1[183:176]-SRC2[183:176]
/// DEST[191:184] := SRC1[191:184]-SRC2[191:184]
/// DEST[199:192] := SRC1[199:192]-SRC2[199:192]
/// DEST[207:200] := SRC1[207:200]-SRC2[207:200]
/// DEST[215:208] := SRC1[215:208]-SRC2[215:208]
/// DEST[223:216] := SRC1[223:216]-SRC2[223:216]
/// DEST[231:224] := SRC1[231:224]-SRC2[231:224]
/// DEST[239:232] := SRC1[239:232]-SRC2[239:232]
/// DEST[247:240] := SRC1[247:240]-SRC2[247:240]
/// DEST[255:248] := SRC1[255:248]-SRC2[255:248]
/// DEST[MAXVL-1:256] := 0
/// VPSUBB (VEX.128 Encoded Version)
/// DEST[7:0] := SRC1[7:0]-SRC2[7:0]
/// DEST[15:8] := SRC1[15:8]-SRC2[15:8]
/// DEST[23:16] := SRC1[23:16]-SRC2[23:16]
/// DEST[31:24] := SRC1[31:24]-SRC2[31:24]
/// DEST[39:32] := SRC1[39:32]-SRC2[39:32]
/// DEST[47:40] := SRC1[47:40]-SRC2[47:40]
/// DEST[55:48] := SRC1[55:48]-SRC2[55:48]
/// DEST[63:56] := SRC1[63:56]-SRC2[63:56]
/// DEST[71:64] := SRC1[71:64]-SRC2[71:64]
/// DEST[79:72] := SRC1[79:72]-SRC2[79:72]
/// DEST[87:80] := SRC1[87:80]-SRC2[87:80]
/// DEST[95:88] := SRC1[95:88]-SRC2[95:88]
/// DEST[103:96] := SRC1[103:96]-SRC2[103:96]
/// DEST[111:104] := SRC1[111:104]-SRC2[111:104]
/// DEST[119:112] := SRC1[119:112]-SRC2[119:112]
/// DEST[127:120] := SRC1[127:120]-SRC2[127:120]
/// DEST[MAXVL-1:128] := 0
/// PSUBB (128-bit Legacy SSE Version)
/// DEST[7:0] := DEST[7:0]-SRC[7:0]
/// DEST[15:8] := DEST[15:8]-SRC[15:8]
/// DEST[23:16] := DEST[23:16]-SRC[23:16]
/// DEST[31:24] := DEST[31:24]-SRC[31:24]
/// DEST[39:32] := DEST[39:32]-SRC[39:32]
/// DEST[47:40] := DEST[47:40]-SRC[47:40]
/// DEST[55:48] := DEST[55:48]-SRC[55:48]
/// DEST[63:56] := DEST[63:56]-SRC[63:56]
/// DEST[71:64] := DEST[71:64]-SRC[71:64]
/// DEST[79:72] := DEST[79:72]-SRC[79:72]
/// DEST[87:80] := DEST[87:80]-SRC[87:80]
/// DEST[95:88] := DEST[95:88]-SRC[95:88]
/// DEST[103:96] := DEST[103:96]-SRC[103:96]
/// DEST[111:104] := DEST[111:104]-SRC[111:104]
/// DEST[119:112] := DEST[119:112]-SRC[119:112]
/// DEST[127:120] := DEST[127:120]-SRC[127:120]
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSUBW (VEX.256 Encoded Version)
/// DEST[15:0] := SRC1[15:0]-SRC2[15:0]
/// DEST[31:16] := SRC1[31:16]-SRC2[31:16]
/// DEST[47:32] := SRC1[47:32]-SRC2[47:32]
/// DEST[63:48] := SRC1[63:48]-SRC2[63:48]
/// DEST[79:64] := SRC1[79:64]-SRC2[79:64]
/// DEST[95:80] := SRC1[95:80]-SRC2[95:80]
/// DEST[111:96] := SRC1[111:96]-SRC2[111:96]
/// DEST[127:112] := SRC1[127:112]-SRC2[127:112]
/// DEST[143:128] := SRC1[143:128]-SRC2[143:128]
/// DEST[159:144] := SRC1[159:144]-SRC2[159:144]
/// DEST[175:160] := SRC1[175:160]-SRC2[175:160]
/// DEST[191:176] := SRC1[191:176]-SRC2[191:176]
/// DEST[207:192] := SRC1207:192]-SRC2[207:192]
/// DEST[223:208] := SRC1[223:208]-SRC2[223:208]
/// DEST[239:224] := SRC1[239:224]-SRC2[239:224]
/// DEST[255:240] := SRC1[255:240]-SRC2[255:240]
/// DEST[MAXVL-1:256] := 0
/// VPSUBW (VEX.128 Encoded Version)
/// DEST[15:0] := SRC1[15:0]-SRC2[15:0]
/// DEST[31:16] := SRC1[31:16]-SRC2[31:16]
/// DEST[47:32] := SRC1[47:32]-SRC2[47:32]
/// DEST[63:48] := SRC1[63:48]-SRC2[63:48]
/// DEST[79:64] := SRC1[79:64]-SRC2[79:64]
/// DEST[95:80] := SRC1[95:80]-SRC2[95:80]
/// DEST[111:96] := SRC1[111:96]-SRC2[111:96]
/// DEST[127:112] := SRC1[127:112]-SRC2[127:112]
/// DEST[MAXVL-1:128] := 0
/// PSUBW (128-bit Legacy SSE Version)
/// DEST[15:0] := DEST[15:0]-SRC[15:0]
/// DEST[31:16] := DEST[31:16]-SRC[31:16]
/// DEST[47:32] := DEST[47:32]-SRC[47:32]
/// DEST[63:48] := DEST[63:48]-SRC[63:48]
/// DEST[79:64] := DEST[79:64]-SRC[79:64]
/// DEST[95:80] := DEST[95:80]-SRC[95:80]
/// DEST[111:96] := DEST[111:96]-SRC[111:96]
/// DEST[127:112] := DEST[127:112]-SRC[127:112]
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSUBD (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0]-SRC2[31:0]
/// DEST[63:32] := SRC1[63:32]-SRC2[63:32]
/// DEST[95:64] := SRC1[95:64]-SRC2[95:64]
/// DEST[127:96] := SRC1[127:96]-SRC2[127:96]
/// DEST[159:128] := SRC1[159:128]-SRC2[159:128]
/// DEST[191:160] := SRC1[191:160]-SRC2[191:160]
/// DEST[223:192] := SRC1[223:192]-SRC2[223:192]
/// DEST[255:224] := SRC1[255:224]-SRC2[255:224]
/// DEST[MAXVL-1:256] := 0
/// VPSUBD (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0]-SRC2[31:0]
/// DEST[63:32] := SRC1[63:32]-SRC2[63:32]
/// DEST[95:64] := SRC1[95:64]-SRC2[95:64]
/// DEST[127:96] := SRC1[127:96]-SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// PSUBD (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0]-SRC[31:0]
/// DEST[63:32] := DEST[63:32]-SRC[63:32]
/// DEST[95:64] := DEST[95:64]-SRC[95:64]
/// DEST[127:96] := DEST[127:96]-SRC[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psubb() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSUBB (With 64-bit Operands)
///     DEST[7:0] := DEST[7:0] - SRC[7:0];
///     (* Repeat subtract operation for 2nd through 7th byte *)
///     DEST[63:56] := DEST[63:56] - SRC[63:56];
/// PSUBW (With 64-bit Operands)
///     DEST[15:0] := DEST[15:0] - SRC[15:0];
///     (* Repeat subtract operation for 2nd and 3rd word *)
///     DEST[63:48] := DEST[63:48] - SRC[63:48];
/// PSUBD (With 64-bit Operands)
///     DEST[31:0] := DEST[31:0] - SRC[31:0];
///     DEST[63:32] := DEST[63:32] - SRC[63:32];
/// PSUBD (With 128-bit Operands)
///     DEST[31:0] := DEST[31:0] - SRC[31:0];
///     (* Repeat subtract operation for 2nd and 3rd doubleword *)
///     DEST[127:96] := DEST[127:96] - SRC[127:96];
/// VPSUBB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC1[i+7:i] - SRC2[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC1[i+15:i] - SRC2[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := SRC1[i+31:i] - SRC2[31:0]
///                 ELSE DEST[i+31:i] := SRC1[i+31:i] - SRC2[i+31:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBB (VEX.256 Encoded Version)
/// DEST[7:0] := SRC1[7:0]-SRC2[7:0]
/// DEST[15:8] := SRC1[15:8]-SRC2[15:8]
/// DEST[23:16] := SRC1[23:16]-SRC2[23:16]
/// DEST[31:24] := SRC1[31:24]-SRC2[31:24]
/// DEST[39:32] := SRC1[39:32]-SRC2[39:32]
/// DEST[47:40] := SRC1[47:40]-SRC2[47:40]
/// DEST[55:48] := SRC1[55:48]-SRC2[55:48]
/// DEST[63:56] := SRC1[63:56]-SRC2[63:56]
/// DEST[71:64] := SRC1[71:64]-SRC2[71:64]
/// DEST[79:72] := SRC1[79:72]-SRC2[79:72]
/// DEST[87:80] := SRC1[87:80]-SRC2[87:80]
/// DEST[95:88] := SRC1[95:88]-SRC2[95:88]
/// DEST[103:96] := SRC1[103:96]-SRC2[103:96]
/// DEST[111:104] := SRC1[111:104]-SRC2[111:104]
/// DEST[119:112] := SRC1[119:112]-SRC2[119:112]
/// DEST[127:120] := SRC1[127:120]-SRC2[127:120]
/// DEST[135:128] := SRC1[135:128]-SRC2[135:128]
/// DEST[151:144] := SRC1[151:144]-SRC2[151:144]
/// DEST[159:152] := SRC1[159:152]-SRC2[159:152]
/// DEST[167:160] := SRC1[167:160]-SRC2[167:160]
/// DEST[175:168] := SRC1[175:168]-SRC2[175:168]
/// DEST[183:176] := SRC1[183:176]-SRC2[183:176]
/// DEST[191:184] := SRC1[191:184]-SRC2[191:184]
/// DEST[199:192] := SRC1[199:192]-SRC2[199:192]
/// DEST[207:200] := SRC1[207:200]-SRC2[207:200]
/// DEST[215:208] := SRC1[215:208]-SRC2[215:208]
/// DEST[223:216] := SRC1[223:216]-SRC2[223:216]
/// DEST[231:224] := SRC1[231:224]-SRC2[231:224]
/// DEST[239:232] := SRC1[239:232]-SRC2[239:232]
/// DEST[247:240] := SRC1[247:240]-SRC2[247:240]
/// DEST[255:248] := SRC1[255:248]-SRC2[255:248]
/// DEST[MAXVL-1:256] := 0
/// VPSUBB (VEX.128 Encoded Version)
/// DEST[7:0] := SRC1[7:0]-SRC2[7:0]
/// DEST[15:8] := SRC1[15:8]-SRC2[15:8]
/// DEST[23:16] := SRC1[23:16]-SRC2[23:16]
/// DEST[31:24] := SRC1[31:24]-SRC2[31:24]
/// DEST[39:32] := SRC1[39:32]-SRC2[39:32]
/// DEST[47:40] := SRC1[47:40]-SRC2[47:40]
/// DEST[55:48] := SRC1[55:48]-SRC2[55:48]
/// DEST[63:56] := SRC1[63:56]-SRC2[63:56]
/// DEST[71:64] := SRC1[71:64]-SRC2[71:64]
/// DEST[79:72] := SRC1[79:72]-SRC2[79:72]
/// DEST[87:80] := SRC1[87:80]-SRC2[87:80]
/// DEST[95:88] := SRC1[95:88]-SRC2[95:88]
/// DEST[103:96] := SRC1[103:96]-SRC2[103:96]
/// DEST[111:104] := SRC1[111:104]-SRC2[111:104]
/// DEST[119:112] := SRC1[119:112]-SRC2[119:112]
/// DEST[127:120] := SRC1[127:120]-SRC2[127:120]
/// DEST[MAXVL-1:128] := 0
/// PSUBB (128-bit Legacy SSE Version)
/// DEST[7:0] := DEST[7:0]-SRC[7:0]
/// DEST[15:8] := DEST[15:8]-SRC[15:8]
/// DEST[23:16] := DEST[23:16]-SRC[23:16]
/// DEST[31:24] := DEST[31:24]-SRC[31:24]
/// DEST[39:32] := DEST[39:32]-SRC[39:32]
/// DEST[47:40] := DEST[47:40]-SRC[47:40]
/// DEST[55:48] := DEST[55:48]-SRC[55:48]
/// DEST[63:56] := DEST[63:56]-SRC[63:56]
/// DEST[71:64] := DEST[71:64]-SRC[71:64]
/// DEST[79:72] := DEST[79:72]-SRC[79:72]
/// DEST[87:80] := DEST[87:80]-SRC[87:80]
/// DEST[95:88] := DEST[95:88]-SRC[95:88]
/// DEST[103:96] := DEST[103:96]-SRC[103:96]
/// DEST[111:104] := DEST[111:104]-SRC[111:104]
/// DEST[119:112] := DEST[119:112]-SRC[119:112]
/// DEST[127:120] := DEST[127:120]-SRC[127:120]
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSUBW (VEX.256 Encoded Version)
/// DEST[15:0] := SRC1[15:0]-SRC2[15:0]
/// DEST[31:16] := SRC1[31:16]-SRC2[31:16]
/// DEST[47:32] := SRC1[47:32]-SRC2[47:32]
/// DEST[63:48] := SRC1[63:48]-SRC2[63:48]
/// DEST[79:64] := SRC1[79:64]-SRC2[79:64]
/// DEST[95:80] := SRC1[95:80]-SRC2[95:80]
/// DEST[111:96] := SRC1[111:96]-SRC2[111:96]
/// DEST[127:112] := SRC1[127:112]-SRC2[127:112]
/// DEST[143:128] := SRC1[143:128]-SRC2[143:128]
/// DEST[159:144] := SRC1[159:144]-SRC2[159:144]
/// DEST[175:160] := SRC1[175:160]-SRC2[175:160]
/// DEST[191:176] := SRC1[191:176]-SRC2[191:176]
/// DEST[207:192] := SRC1207:192]-SRC2[207:192]
/// DEST[223:208] := SRC1[223:208]-SRC2[223:208]
/// DEST[239:224] := SRC1[239:224]-SRC2[239:224]
/// DEST[255:240] := SRC1[255:240]-SRC2[255:240]
/// DEST[MAXVL-1:256] := 0
/// VPSUBW (VEX.128 Encoded Version)
/// DEST[15:0] := SRC1[15:0]-SRC2[15:0]
/// DEST[31:16] := SRC1[31:16]-SRC2[31:16]
/// DEST[47:32] := SRC1[47:32]-SRC2[47:32]
/// DEST[63:48] := SRC1[63:48]-SRC2[63:48]
/// DEST[79:64] := SRC1[79:64]-SRC2[79:64]
/// DEST[95:80] := SRC1[95:80]-SRC2[95:80]
/// DEST[111:96] := SRC1[111:96]-SRC2[111:96]
/// DEST[127:112] := SRC1[127:112]-SRC2[127:112]
/// DEST[MAXVL-1:128] := 0
/// PSUBW (128-bit Legacy SSE Version)
/// DEST[15:0] := DEST[15:0]-SRC[15:0]
/// DEST[31:16] := DEST[31:16]-SRC[31:16]
/// DEST[47:32] := DEST[47:32]-SRC[47:32]
/// DEST[63:48] := DEST[63:48]-SRC[63:48]
/// DEST[79:64] := DEST[79:64]-SRC[79:64]
/// DEST[95:80] := DEST[95:80]-SRC[95:80]
/// DEST[111:96] := DEST[111:96]-SRC[111:96]
/// DEST[127:112] := DEST[127:112]-SRC[127:112]
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSUBD (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0]-SRC2[31:0]
/// DEST[63:32] := SRC1[63:32]-SRC2[63:32]
/// DEST[95:64] := SRC1[95:64]-SRC2[95:64]
/// DEST[127:96] := SRC1[127:96]-SRC2[127:96]
/// DEST[159:128] := SRC1[159:128]-SRC2[159:128]
/// DEST[191:160] := SRC1[191:160]-SRC2[191:160]
/// DEST[223:192] := SRC1[223:192]-SRC2[223:192]
/// DEST[255:224] := SRC1[255:224]-SRC2[255:224]
/// DEST[MAXVL-1:256] := 0
/// VPSUBD (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0]-SRC2[31:0]
/// DEST[63:32] := SRC1[63:32]-SRC2[63:32]
/// DEST[95:64] := SRC1[95:64]-SRC2[95:64]
/// DEST[127:96] := SRC1[127:96]-SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// PSUBD (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0]-SRC[31:0]
/// DEST[63:32] := DEST[63:32]-SRC[63:32]
/// DEST[95:64] := DEST[95:64]-SRC[95:64]
/// DEST[127:96] := DEST[127:96]-SRC[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psubd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSUBQ (With 64-Bit Operands)
///     DEST[63:0] := DEST[63:0] - SRC[63:0];
/// PSUBQ (With 128-Bit Operands)
///     DEST[63:0] := DEST[63:0] - SRC[63:0];
///     DEST[127:64] := DEST[127:64] - SRC[127:64];
/// VPSUBQ (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0]-SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]-SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// VPSUBQ (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0]-SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]-SRC2[127:64]
/// DEST[191:128] := SRC1[191:128]-SRC2[191:128]
/// DEST[255:192] := SRC1[255:192]-SRC2[255:192]
/// DEST[MAXVL-1:256] := 0
/// VPSUBQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+63:i] := SRC1[i+63:i] - SRC2[63:0]
///                 ELSE DEST[i+63:i] := SRC1[i+63:i] - SRC2[i+63:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn psubq() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSUBSB (With 64-bit Operands)
///     DEST[7:0] := SaturateToSignedByte (DEST[7:0] - SRC (7:0]);
///     (* Repeat subtract operation for 2nd through 7th bytes *)
///     DEST[63:56] := SaturateToSignedByte (DEST[63:56] - SRC[63:56] );
/// PSUBSW (With 64-bit Operands)
///     DEST[15:0] := SaturateToSignedWord (DEST[15:0] - SRC[15:0] );
///     (* Repeat subtract operation for 2nd and 7th words *)
///     DEST[63:48] := SaturateToSignedWord (DEST[63:48] - SRC[63:48] );
/// VPSUBSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8;
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SaturateToSignedByte (SRC1[i+7:i] - SRC2[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0;
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToSignedWord (SRC1[i+15:i] - SRC2[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0;
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSUBSB (VEX.256 Encoded Version)
/// DEST[7:0] := SaturateToSignedByte (SRC1[7:0] - SRC2[7:0]);
/// (* Repeat subtract operation for 2nd through 31th bytes *)
/// DEST[255:248] := SaturateToSignedByte (SRC1[255:248] - SRC2[255:248]);
/// DEST[MAXVL-1:256] := 0;
/// VPSUBSB (VEX.128 Encoded Version)
/// DEST[7:0] := SaturateToSignedByte (SRC1[7:0] - SRC2[7:0]);
/// (* Repeat subtract operation for 2nd through 14th bytes *)
/// DEST[127:120] := SaturateToSignedByte (SRC1[127:120] - SRC2[127:120]);
/// DEST[MAXVL-1:128] := 0;
/// PSUBSB (128-bit Legacy SSE Version)
/// DEST[7:0] := SaturateToSignedByte (DEST[7:0] - SRC[7:0]);
/// (* Repeat subtract operation for 2nd through 14th bytes *)
/// DEST[127:120] := SaturateToSignedByte (DEST[127:120] - SRC[127:120]);
/// VPSUBSW (VEX.256 Encoded Version)
/// DEST[15:0] := SaturateToSignedWord (SRC1[15:0] - SRC2[15:0]);
/// (* Repeat subtract operation for 2nd through 15th words *)
/// DEST[255:240] := SaturateToSignedWord (SRC1[255:240] - SRC2[255:240]);
/// DEST[MAXVL-1:256] := 0;
/// VPSUBSW (VEX.128 Encoded Version)
/// DEST[15:0] := SaturateToSignedWord (SRC1[15:0] - SRC2[15:0]);
/// (* Repeat subtract operation for 2nd through 7th words *)
/// DEST[127:112] := SaturateToSignedWord (SRC1[127:112] - SRC2[127:112]);
/// DEST[MAXVL-1:128] := 0;
/// PSUBSW (128-bit Legacy SSE Version)
/// DEST[15:0] := SaturateToSignedWord (DEST[15:0] - SRC[15:0]);
/// (* Repeat subtract operation for 2nd through 7th words *)
/// DEST[127:112] := SaturateToSignedWord (DEST[127:112] - SRC[127:112]);
/// DEST[MAXVL-1:128] (Unmodified);
/// ```
#[box_to_static_reference]
pub(super) fn psubsb() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSUBSB (With 64-bit Operands)
///     DEST[7:0] := SaturateToSignedByte (DEST[7:0] - SRC (7:0]);
///     (* Repeat subtract operation for 2nd through 7th bytes *)
///     DEST[63:56] := SaturateToSignedByte (DEST[63:56] - SRC[63:56] );
/// PSUBSW (With 64-bit Operands)
///     DEST[15:0] := SaturateToSignedWord (DEST[15:0] - SRC[15:0] );
///     (* Repeat subtract operation for 2nd and 7th words *)
///     DEST[63:48] := SaturateToSignedWord (DEST[63:48] - SRC[63:48] );
/// VPSUBSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8;
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SaturateToSignedByte (SRC1[i+7:i] - SRC2[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0;
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToSignedWord (SRC1[i+15:i] - SRC2[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0;
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSUBSB (VEX.256 Encoded Version)
/// DEST[7:0] := SaturateToSignedByte (SRC1[7:0] - SRC2[7:0]);
/// (* Repeat subtract operation for 2nd through 31th bytes *)
/// DEST[255:248] := SaturateToSignedByte (SRC1[255:248] - SRC2[255:248]);
/// DEST[MAXVL-1:256] := 0;
/// VPSUBSB (VEX.128 Encoded Version)
/// DEST[7:0] := SaturateToSignedByte (SRC1[7:0] - SRC2[7:0]);
/// (* Repeat subtract operation for 2nd through 14th bytes *)
/// DEST[127:120] := SaturateToSignedByte (SRC1[127:120] - SRC2[127:120]);
/// DEST[MAXVL-1:128] := 0;
/// PSUBSB (128-bit Legacy SSE Version)
/// DEST[7:0] := SaturateToSignedByte (DEST[7:0] - SRC[7:0]);
/// (* Repeat subtract operation for 2nd through 14th bytes *)
/// DEST[127:120] := SaturateToSignedByte (DEST[127:120] - SRC[127:120]);
/// VPSUBSW (VEX.256 Encoded Version)
/// DEST[15:0] := SaturateToSignedWord (SRC1[15:0] - SRC2[15:0]);
/// (* Repeat subtract operation for 2nd through 15th words *)
/// DEST[255:240] := SaturateToSignedWord (SRC1[255:240] - SRC2[255:240]);
/// DEST[MAXVL-1:256] := 0;
/// VPSUBSW (VEX.128 Encoded Version)
/// DEST[15:0] := SaturateToSignedWord (SRC1[15:0] - SRC2[15:0]);
/// (* Repeat subtract operation for 2nd through 7th words *)
/// DEST[127:112] := SaturateToSignedWord (SRC1[127:112] - SRC2[127:112]);
/// DEST[MAXVL-1:128] := 0;
/// PSUBSW (128-bit Legacy SSE Version)
/// DEST[15:0] := SaturateToSignedWord (DEST[15:0] - SRC[15:0]);
/// (* Repeat subtract operation for 2nd through 7th words *)
/// DEST[127:112] := SaturateToSignedWord (DEST[127:112] - SRC[127:112]);
/// DEST[MAXVL-1:128] (Unmodified);
/// ```
#[box_to_static_reference]
pub(super) fn psubsw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSUBUSB (With 64-bit Operands)
///     DEST[7:0] := SaturateToUnsignedByte (DEST[7:0] - SRC (7:0] );
///     (* Repeat add operation for 2nd through 7th bytes *)
///     DEST[63:56] := SaturateToUnsignedByte (DEST[63:56] - SRC[63:56];
/// PSUBUSW (With 64-bit Operands)
///     DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] - SRC[15:0] );
///     (* Repeat add operation for 2nd and 3rd words *)
/// VPSUBUSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8;
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SaturateToUnsignedByte (SRC1[i+7:i] - SRC2[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0;
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSUBUSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16;
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToUnsignedWord (SRC1[i+15:i] - SRC2[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0;
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSUBUSB (VEX.256 Encoded Version)
/// DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] - SRC2[7:0]);
/// (* Repeat subtract operation for 2nd through 31st bytes *)
/// DEST[255:148] := SaturateToUnsignedByte (SRC1[255:248] - SRC2[255:248]);
/// DEST[MAXVL-1:256] := 0;
/// VPSUBUSB (VEX.128 Encoded Version)
/// DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] - SRC2[7:0]);
/// (* Repeat subtract operation for 2nd through 14th bytes *)
/// DEST[127:120] := SaturateToUnsignedByte (SRC1[127:120] - SRC2[127:120]);
/// DEST[MAXVL-1:128] := 0
/// PSUBUSB (128-bit Legacy SSE Version)
/// DEST[7:0] := SaturateToUnsignedByte (DEST[7:0] - SRC[7:0]);
/// (* Repeat subtract operation for 2nd through 14th bytes *)
/// DEST[127:120] := SaturateToUnsignedByte (DEST[127:120] - SRC[127:120]);
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSUBUSW (VEX.256 Encoded Version)
/// DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] - SRC2[15:0]);
/// (* Repeat subtract operation for 2nd through 15th words *)
/// DEST[255:240] := SaturateToUnsignedWord (SRC1[255:240] - SRC2[255:240]);
/// DEST[MAXVL-1:256] := 0;
/// VPSUBUSW (VEX.128 Encoded Version)
/// DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] - SRC2[15:0]);
/// (* Repeat subtract operation for 2nd through 7th words *)
/// DEST[127:112] := SaturateToUnsignedWord (SRC1[127:112] - SRC2[127:112]);
/// DEST[MAXVL-1:128] := 0
/// PSUBUSW (128-bit Legacy SSE Version)
/// DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] - SRC[15:0]);
/// (* Repeat subtract operation for 2nd through 7th words *)
/// DEST[127:112] := SaturateToUnsignedWord (DEST[127:112] - SRC[127:112]);
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psubusb() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSUBUSB (With 64-bit Operands)
///     DEST[7:0] := SaturateToUnsignedByte (DEST[7:0] - SRC (7:0] );
///     (* Repeat add operation for 2nd through 7th bytes *)
///     DEST[63:56] := SaturateToUnsignedByte (DEST[63:56] - SRC[63:56];
/// PSUBUSW (With 64-bit Operands)
///     DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] - SRC[15:0] );
///     (* Repeat add operation for 2nd and 3rd words *)
/// VPSUBUSB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8;
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SaturateToUnsignedByte (SRC1[i+7:i] - SRC2[i+7:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0;
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSUBUSW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16;
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SaturateToUnsignedWord (SRC1[i+15:i] - SRC2[i+15:i])
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0;
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0;
/// VPSUBUSB (VEX.256 Encoded Version)
/// DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] - SRC2[7:0]);
/// (* Repeat subtract operation for 2nd through 31st bytes *)
/// DEST[255:148] := SaturateToUnsignedByte (SRC1[255:248] - SRC2[255:248]);
/// DEST[MAXVL-1:256] := 0;
/// VPSUBUSB (VEX.128 Encoded Version)
/// DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] - SRC2[7:0]);
/// (* Repeat subtract operation for 2nd through 14th bytes *)
/// DEST[127:120] := SaturateToUnsignedByte (SRC1[127:120] - SRC2[127:120]);
/// DEST[MAXVL-1:128] := 0
/// PSUBUSB (128-bit Legacy SSE Version)
/// DEST[7:0] := SaturateToUnsignedByte (DEST[7:0] - SRC[7:0]);
/// (* Repeat subtract operation for 2nd through 14th bytes *)
/// DEST[127:120] := SaturateToUnsignedByte (DEST[127:120] - SRC[127:120]);
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSUBUSW (VEX.256 Encoded Version)
/// DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] - SRC2[15:0]);
/// (* Repeat subtract operation for 2nd through 15th words *)
/// DEST[255:240] := SaturateToUnsignedWord (SRC1[255:240] - SRC2[255:240]);
/// DEST[MAXVL-1:256] := 0;
/// VPSUBUSW (VEX.128 Encoded Version)
/// DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] - SRC2[15:0]);
/// (* Repeat subtract operation for 2nd through 7th words *)
/// DEST[127:112] := SaturateToUnsignedWord (SRC1[127:112] - SRC2[127:112]);
/// DEST[MAXVL-1:128] := 0
/// PSUBUSW (128-bit Legacy SSE Version)
/// DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] - SRC[15:0]);
/// (* Repeat subtract operation for 2nd through 7th words *)
/// DEST[127:112] := SaturateToUnsignedWord (DEST[127:112] - SRC[127:112]);
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psubusw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PSUBB (With 64-bit Operands)
///     DEST[7:0] := DEST[7:0] - SRC[7:0];
///     (* Repeat subtract operation for 2nd through 7th byte *)
///     DEST[63:56] := DEST[63:56] - SRC[63:56];
/// PSUBW (With 64-bit Operands)
///     DEST[15:0] := DEST[15:0] - SRC[15:0];
///     (* Repeat subtract operation for 2nd and 3rd word *)
///     DEST[63:48] := DEST[63:48] - SRC[63:48];
/// PSUBD (With 64-bit Operands)
///     DEST[31:0] := DEST[31:0] - SRC[31:0];
///     DEST[63:32] := DEST[63:32] - SRC[63:32];
/// PSUBD (With 128-bit Operands)
///     DEST[31:0] := DEST[31:0] - SRC[31:0];
///     (* Repeat subtract operation for 2nd and 3rd doubleword *)
///     DEST[127:96] := DEST[127:96] - SRC[127:96];
/// VPSUBB (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := SRC1[i+7:i] - SRC2[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBW (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// FOR j := 0 TO KL-1
///     i := j * 16
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+15:i] := SRC1[i+15:i] - SRC2[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] = 0
///             FI
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN DEST[i+31:i] := SRC1[i+31:i] - SRC2[31:0]
///                 ELSE DEST[i+31:i] := SRC1[i+31:i] - SRC2[i+31:i]
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
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPSUBB (VEX.256 Encoded Version)
/// DEST[7:0] := SRC1[7:0]-SRC2[7:0]
/// DEST[15:8] := SRC1[15:8]-SRC2[15:8]
/// DEST[23:16] := SRC1[23:16]-SRC2[23:16]
/// DEST[31:24] := SRC1[31:24]-SRC2[31:24]
/// DEST[39:32] := SRC1[39:32]-SRC2[39:32]
/// DEST[47:40] := SRC1[47:40]-SRC2[47:40]
/// DEST[55:48] := SRC1[55:48]-SRC2[55:48]
/// DEST[63:56] := SRC1[63:56]-SRC2[63:56]
/// DEST[71:64] := SRC1[71:64]-SRC2[71:64]
/// DEST[79:72] := SRC1[79:72]-SRC2[79:72]
/// DEST[87:80] := SRC1[87:80]-SRC2[87:80]
/// DEST[95:88] := SRC1[95:88]-SRC2[95:88]
/// DEST[103:96] := SRC1[103:96]-SRC2[103:96]
/// DEST[111:104] := SRC1[111:104]-SRC2[111:104]
/// DEST[119:112] := SRC1[119:112]-SRC2[119:112]
/// DEST[127:120] := SRC1[127:120]-SRC2[127:120]
/// DEST[135:128] := SRC1[135:128]-SRC2[135:128]
/// DEST[151:144] := SRC1[151:144]-SRC2[151:144]
/// DEST[159:152] := SRC1[159:152]-SRC2[159:152]
/// DEST[167:160] := SRC1[167:160]-SRC2[167:160]
/// DEST[175:168] := SRC1[175:168]-SRC2[175:168]
/// DEST[183:176] := SRC1[183:176]-SRC2[183:176]
/// DEST[191:184] := SRC1[191:184]-SRC2[191:184]
/// DEST[199:192] := SRC1[199:192]-SRC2[199:192]
/// DEST[207:200] := SRC1[207:200]-SRC2[207:200]
/// DEST[215:208] := SRC1[215:208]-SRC2[215:208]
/// DEST[223:216] := SRC1[223:216]-SRC2[223:216]
/// DEST[231:224] := SRC1[231:224]-SRC2[231:224]
/// DEST[239:232] := SRC1[239:232]-SRC2[239:232]
/// DEST[247:240] := SRC1[247:240]-SRC2[247:240]
/// DEST[255:248] := SRC1[255:248]-SRC2[255:248]
/// DEST[MAXVL-1:256] := 0
/// VPSUBB (VEX.128 Encoded Version)
/// DEST[7:0] := SRC1[7:0]-SRC2[7:0]
/// DEST[15:8] := SRC1[15:8]-SRC2[15:8]
/// DEST[23:16] := SRC1[23:16]-SRC2[23:16]
/// DEST[31:24] := SRC1[31:24]-SRC2[31:24]
/// DEST[39:32] := SRC1[39:32]-SRC2[39:32]
/// DEST[47:40] := SRC1[47:40]-SRC2[47:40]
/// DEST[55:48] := SRC1[55:48]-SRC2[55:48]
/// DEST[63:56] := SRC1[63:56]-SRC2[63:56]
/// DEST[71:64] := SRC1[71:64]-SRC2[71:64]
/// DEST[79:72] := SRC1[79:72]-SRC2[79:72]
/// DEST[87:80] := SRC1[87:80]-SRC2[87:80]
/// DEST[95:88] := SRC1[95:88]-SRC2[95:88]
/// DEST[103:96] := SRC1[103:96]-SRC2[103:96]
/// DEST[111:104] := SRC1[111:104]-SRC2[111:104]
/// DEST[119:112] := SRC1[119:112]-SRC2[119:112]
/// DEST[127:120] := SRC1[127:120]-SRC2[127:120]
/// DEST[MAXVL-1:128] := 0
/// PSUBB (128-bit Legacy SSE Version)
/// DEST[7:0] := DEST[7:0]-SRC[7:0]
/// DEST[15:8] := DEST[15:8]-SRC[15:8]
/// DEST[23:16] := DEST[23:16]-SRC[23:16]
/// DEST[31:24] := DEST[31:24]-SRC[31:24]
/// DEST[39:32] := DEST[39:32]-SRC[39:32]
/// DEST[47:40] := DEST[47:40]-SRC[47:40]
/// DEST[55:48] := DEST[55:48]-SRC[55:48]
/// DEST[63:56] := DEST[63:56]-SRC[63:56]
/// DEST[71:64] := DEST[71:64]-SRC[71:64]
/// DEST[79:72] := DEST[79:72]-SRC[79:72]
/// DEST[87:80] := DEST[87:80]-SRC[87:80]
/// DEST[95:88] := DEST[95:88]-SRC[95:88]
/// DEST[103:96] := DEST[103:96]-SRC[103:96]
/// DEST[111:104] := DEST[111:104]-SRC[111:104]
/// DEST[119:112] := DEST[119:112]-SRC[119:112]
/// DEST[127:120] := DEST[127:120]-SRC[127:120]
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSUBW (VEX.256 Encoded Version)
/// DEST[15:0] := SRC1[15:0]-SRC2[15:0]
/// DEST[31:16] := SRC1[31:16]-SRC2[31:16]
/// DEST[47:32] := SRC1[47:32]-SRC2[47:32]
/// DEST[63:48] := SRC1[63:48]-SRC2[63:48]
/// DEST[79:64] := SRC1[79:64]-SRC2[79:64]
/// DEST[95:80] := SRC1[95:80]-SRC2[95:80]
/// DEST[111:96] := SRC1[111:96]-SRC2[111:96]
/// DEST[127:112] := SRC1[127:112]-SRC2[127:112]
/// DEST[143:128] := SRC1[143:128]-SRC2[143:128]
/// DEST[159:144] := SRC1[159:144]-SRC2[159:144]
/// DEST[175:160] := SRC1[175:160]-SRC2[175:160]
/// DEST[191:176] := SRC1[191:176]-SRC2[191:176]
/// DEST[207:192] := SRC1207:192]-SRC2[207:192]
/// DEST[223:208] := SRC1[223:208]-SRC2[223:208]
/// DEST[239:224] := SRC1[239:224]-SRC2[239:224]
/// DEST[255:240] := SRC1[255:240]-SRC2[255:240]
/// DEST[MAXVL-1:256] := 0
/// VPSUBW (VEX.128 Encoded Version)
/// DEST[15:0] := SRC1[15:0]-SRC2[15:0]
/// DEST[31:16] := SRC1[31:16]-SRC2[31:16]
/// DEST[47:32] := SRC1[47:32]-SRC2[47:32]
/// DEST[63:48] := SRC1[63:48]-SRC2[63:48]
/// DEST[79:64] := SRC1[79:64]-SRC2[79:64]
/// DEST[95:80] := SRC1[95:80]-SRC2[95:80]
/// DEST[111:96] := SRC1[111:96]-SRC2[111:96]
/// DEST[127:112] := SRC1[127:112]-SRC2[127:112]
/// DEST[MAXVL-1:128] := 0
/// PSUBW (128-bit Legacy SSE Version)
/// DEST[15:0] := DEST[15:0]-SRC[15:0]
/// DEST[31:16] := DEST[31:16]-SRC[31:16]
/// DEST[47:32] := DEST[47:32]-SRC[47:32]
/// DEST[63:48] := DEST[63:48]-SRC[63:48]
/// DEST[79:64] := DEST[79:64]-SRC[79:64]
/// DEST[95:80] := DEST[95:80]-SRC[95:80]
/// DEST[111:96] := DEST[111:96]-SRC[111:96]
/// DEST[127:112] := DEST[127:112]-SRC[127:112]
/// DEST[MAXVL-1:128] (Unmodified)
/// VPSUBD (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0]-SRC2[31:0]
/// DEST[63:32] := SRC1[63:32]-SRC2[63:32]
/// DEST[95:64] := SRC1[95:64]-SRC2[95:64]
/// DEST[127:96] := SRC1[127:96]-SRC2[127:96]
/// DEST[159:128] := SRC1[159:128]-SRC2[159:128]
/// DEST[191:160] := SRC1[191:160]-SRC2[191:160]
/// DEST[223:192] := SRC1[223:192]-SRC2[223:192]
/// DEST[255:224] := SRC1[255:224]-SRC2[255:224]
/// DEST[MAXVL-1:256] := 0
/// VPSUBD (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0]-SRC2[31:0]
/// DEST[63:32] := SRC1[63:32]-SRC2[63:32]
/// DEST[95:64] := SRC1[95:64]-SRC2[95:64]
/// DEST[127:96] := SRC1[127:96]-SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// PSUBD (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0]-SRC[31:0]
/// DEST[63:32] := DEST[63:32]-SRC[63:32]
/// DEST[95:64] := DEST[95:64]-SRC[95:64]
/// DEST[127:96] := DEST[127:96]-SRC[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn psubw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (V)PTEST (128-bit Version)
/// IF (SRC[127:0] BITWISE AND DEST[127:0] = 0)
///     THEN ZF := 1;
///     ELSE ZF := 0;
/// IF (SRC[127:0] BITWISE AND NOT DEST[127:0] = 0)
///     THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// VPTEST (VEX.256 Encoded Version)
/// IF (SRC[255:0] BITWISE AND DEST[255:0] = 0) THEN ZF := 1;
///     ELSE ZF := 0;
/// IF (SRC[255:0] BITWISE AND NOT DEST[255:0] = 0) THEN CF := 1;
///     ELSE CF := 0;
/// DEST (unmodified)
/// AF := OF := PF := SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn ptest() -> &'static [IrStatement] {
    let assignment = assign(b::and(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (IA32_RTIT_STATUS.TriggerEn & IA32_RTIT_STATUS.ContextEn & IA32_RTIT_STATUS.FilterEn & IA32_RTIT_CTL.PTWEn) = 1
///     PTW.PayloadBytes := Encoded payload size;
///     PTW.IP := IA32_RTIT_CTL.FUPonPTW
///     IF IA32_RTIT_CTL.FUPonPTW = 1
///         Insert FUP packet with IP of PTWRITE;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn ptwrite() -> &'static [IrStatement] {
    [exception("ptwrite")].into()
}

/// # Pseudocode
/// ```text
/// PUNPCKHBW Instruction With 64-bit Operands:
///     DEST[7:0] := DEST[39:32];
///     DEST[15:8] := SRC[39:32];
///     DEST[23:16] := DEST[47:40];
///     DEST[31:24] := SRC[47:40];
///     DEST[39:32] := DEST[55:48];
///     DEST[47:40] := SRC[55:48];
///     DEST[55:48] := DEST[63:56];
///     DEST[63:56] := SRC[63:56];
/// PUNPCKHW Instruction With 64-bit Operands:
///     DEST[15:0] := DEST[47:32];
///     DEST[31:16] := SRC[47:32];
///     DEST[47:32] := DEST[63:48];
///     DEST[63:48] := SRC[63:48];
/// PUNPCKHDQ Instruction With 64-bit Operands:
///     DEST[31:0] := DEST[63:32];
///     DEST[63:32] := SRC[63:32];
/// INTERLEAVE_HIGH_BYTES_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_BYTES_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_HIGH_BYTES_256b (SRC1, SRC2)
/// DEST[7:0] := SRC1[71:64]
/// DEST[15:8] := SRC2[71:64]
/// DEST[23:16] := SRC1[79:72]
/// DEST[31:24] := SRC2[79:72]
/// DEST[39:32] := SRC1[87:80]
/// DEST[47:40] := SRC2[87:80]
/// DEST[55:48] := SRC1[95:88]
/// DEST[63:56] := SRC2[95:88]
/// DEST[71:64] := SRC1[103:96]
/// DEST[79:72] := SRC2[103:96]
/// DEST[87:80] := SRC1[111:104]
/// DEST[95:88] := SRC2[111:104]
/// DEST[103:96] := SRC1[119:112]
/// DEST[111:104] := SRC2[119:112]
/// DEST[119:112] := SRC1[127:120]
/// DEST[127:120] := SRC2[127:120]
/// DEST[135:128] := SRC1[199:192]
/// DEST[143:136] := SRC2[199:192]
/// DEST[151:144] := SRC1[207:200]
/// DEST[167:160] := SRC1[215:208]
/// DEST[175:168] := SRC2[215:208]
/// DEST[183:176] := SRC1[223:216]
/// DEST[191:184] := SRC2[223:216]
/// DEST[199:192] := SRC1[231:224]
/// DEST[207:200] := SRC2[231:224]
/// DEST[215:208] := SRC1[239:232]
/// DEST[223:216] := SRC2[239:232]
/// DEST[231:224] := SRC1[247:240]
/// DEST[239:232] := SRC2[247:240]
/// DEST[247:240] := SRC1[255:248]
/// DEST[255:248] := SRC2[255:248]
/// INTERLEAVE_HIGH_BYTES (SRC1, SRC2)
/// DEST[7:0] := SRC1[71:64]
/// DEST[15:8] := SRC2[71:64]
/// DEST[23:16] := SRC1[79:72]
/// DEST[31:24] := SRC2[79:72]
/// DEST[39:32] := SRC1[87:80]
/// DEST[47:40] := SRC2[87:80]
/// DEST[55:48] := SRC1[95:88]
/// DEST[63:56] := SRC2[95:88]
/// DEST[71:64] := SRC1[103:96]
/// DEST[79:72] := SRC2[103:96]
/// DEST[87:80] := SRC1[111:104]
/// DEST[95:88] := SRC2[111:104]
/// DEST[103:96] := SRC1[119:112]
/// DEST[111:104] := SRC2[119:112]
/// DEST[119:112] := SRC1[127:120]
/// DEST[127:120] := SRC2[127:120]
/// INTERLEAVE_HIGH_WORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_WORDS_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_HIGH_WORDS_256b(SRC1, SRC2)
/// DEST[15:0] := SRC1[79:64]
/// DEST[31:16] := SRC2[79:64]
/// DEST[47:32] := SRC1[95:80]
/// DEST[63:48] := SRC2[95:80]
/// DEST[79:64] := SRC1[111:96]
/// DEST[95:80] := SRC2[111:96]
/// DEST[111:96] := SRC1[127:112]
/// DEST[127:112] := SRC2[127:112]
/// DEST[143:128] := SRC1[207:192]
/// DEST[159:144] := SRC2[207:192]
/// DEST[175:160] := SRC1[223:208]
/// DEST[191:176] := SRC2[223:208]
/// DEST[207:192] := SRC1[239:224]
/// DEST[223:208] := SRC2[239:224]
/// DEST[239:224] := SRC1[255:240]
/// DEST[255:240] := SRC2[255:240]
/// DEST[15:0] := SRC1[79:64]
/// DEST[31:16] := SRC2[79:64]
/// DEST[47:32] := SRC1[95:80]
/// DEST[63:48] := SRC2[95:80]
/// DEST[79:64] := SRC1[111:96]
/// DEST[95:80] := SRC2[111:96]
/// DEST[111:96] := SRC1[127:112]
/// DEST[127:112] := SRC2[127:112]
/// INTERLEAVE_HIGH_DWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_HIGH_DWORDS_256b(SRC1, SRC2)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// DEST[159:128] := SRC1[223:192]
/// DEST[191:160] := SRC2[223:192]
/// DEST[223:192] := SRC1[255:224]
/// DEST[255:224] := SRC2[255:224]
/// INTERLEAVE_HIGH_DWORDS(SRC1, SRC2)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// INTERLEAVE_HIGH_QWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_HIGH_QWORDS_256b(SRC1, SRC2)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// DEST[191:128] := SRC1[255:192]
/// DEST[255:192] := SRC2[255:192]
/// INTERLEAVE_HIGH_QWORDS(SRC1, SRC2)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// PUNPCKHBW (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_BYTES(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHBW (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_BYTES(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHBW (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1, SRC2)
/// VPUNPCKHBW (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHWD (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_WORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHWD (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHWD (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHWD (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHDQ (128-bit LegacySSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_DWORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHDQ (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHDQ (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHDQ (EVEX.512 Encoded Version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHQDQ (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKHQDQ (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKHQDQ (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHQDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn punpckhbw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PUNPCKHBW Instruction With 64-bit Operands:
///     DEST[7:0] := DEST[39:32];
///     DEST[15:8] := SRC[39:32];
///     DEST[23:16] := DEST[47:40];
///     DEST[31:24] := SRC[47:40];
///     DEST[39:32] := DEST[55:48];
///     DEST[47:40] := SRC[55:48];
///     DEST[55:48] := DEST[63:56];
///     DEST[63:56] := SRC[63:56];
/// PUNPCKHW Instruction With 64-bit Operands:
///     DEST[15:0] := DEST[47:32];
///     DEST[31:16] := SRC[47:32];
///     DEST[47:32] := DEST[63:48];
///     DEST[63:48] := SRC[63:48];
/// PUNPCKHDQ Instruction With 64-bit Operands:
///     DEST[31:0] := DEST[63:32];
///     DEST[63:32] := SRC[63:32];
/// INTERLEAVE_HIGH_BYTES_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_BYTES_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_HIGH_BYTES_256b (SRC1, SRC2)
/// DEST[7:0] := SRC1[71:64]
/// DEST[15:8] := SRC2[71:64]
/// DEST[23:16] := SRC1[79:72]
/// DEST[31:24] := SRC2[79:72]
/// DEST[39:32] := SRC1[87:80]
/// DEST[47:40] := SRC2[87:80]
/// DEST[55:48] := SRC1[95:88]
/// DEST[63:56] := SRC2[95:88]
/// DEST[71:64] := SRC1[103:96]
/// DEST[79:72] := SRC2[103:96]
/// DEST[87:80] := SRC1[111:104]
/// DEST[95:88] := SRC2[111:104]
/// DEST[103:96] := SRC1[119:112]
/// DEST[111:104] := SRC2[119:112]
/// DEST[119:112] := SRC1[127:120]
/// DEST[127:120] := SRC2[127:120]
/// DEST[135:128] := SRC1[199:192]
/// DEST[143:136] := SRC2[199:192]
/// DEST[151:144] := SRC1[207:200]
/// DEST[167:160] := SRC1[215:208]
/// DEST[175:168] := SRC2[215:208]
/// DEST[183:176] := SRC1[223:216]
/// DEST[191:184] := SRC2[223:216]
/// DEST[199:192] := SRC1[231:224]
/// DEST[207:200] := SRC2[231:224]
/// DEST[215:208] := SRC1[239:232]
/// DEST[223:216] := SRC2[239:232]
/// DEST[231:224] := SRC1[247:240]
/// DEST[239:232] := SRC2[247:240]
/// DEST[247:240] := SRC1[255:248]
/// DEST[255:248] := SRC2[255:248]
/// INTERLEAVE_HIGH_BYTES (SRC1, SRC2)
/// DEST[7:0] := SRC1[71:64]
/// DEST[15:8] := SRC2[71:64]
/// DEST[23:16] := SRC1[79:72]
/// DEST[31:24] := SRC2[79:72]
/// DEST[39:32] := SRC1[87:80]
/// DEST[47:40] := SRC2[87:80]
/// DEST[55:48] := SRC1[95:88]
/// DEST[63:56] := SRC2[95:88]
/// DEST[71:64] := SRC1[103:96]
/// DEST[79:72] := SRC2[103:96]
/// DEST[87:80] := SRC1[111:104]
/// DEST[95:88] := SRC2[111:104]
/// DEST[103:96] := SRC1[119:112]
/// DEST[111:104] := SRC2[119:112]
/// DEST[119:112] := SRC1[127:120]
/// DEST[127:120] := SRC2[127:120]
/// INTERLEAVE_HIGH_WORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_WORDS_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_HIGH_WORDS_256b(SRC1, SRC2)
/// DEST[15:0] := SRC1[79:64]
/// DEST[31:16] := SRC2[79:64]
/// DEST[47:32] := SRC1[95:80]
/// DEST[63:48] := SRC2[95:80]
/// DEST[79:64] := SRC1[111:96]
/// DEST[95:80] := SRC2[111:96]
/// DEST[111:96] := SRC1[127:112]
/// DEST[127:112] := SRC2[127:112]
/// DEST[143:128] := SRC1[207:192]
/// DEST[159:144] := SRC2[207:192]
/// DEST[175:160] := SRC1[223:208]
/// DEST[191:176] := SRC2[223:208]
/// DEST[207:192] := SRC1[239:224]
/// DEST[223:208] := SRC2[239:224]
/// DEST[239:224] := SRC1[255:240]
/// DEST[255:240] := SRC2[255:240]
/// DEST[15:0] := SRC1[79:64]
/// DEST[31:16] := SRC2[79:64]
/// DEST[47:32] := SRC1[95:80]
/// DEST[63:48] := SRC2[95:80]
/// DEST[79:64] := SRC1[111:96]
/// DEST[95:80] := SRC2[111:96]
/// DEST[111:96] := SRC1[127:112]
/// DEST[127:112] := SRC2[127:112]
/// INTERLEAVE_HIGH_DWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_HIGH_DWORDS_256b(SRC1, SRC2)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// DEST[159:128] := SRC1[223:192]
/// DEST[191:160] := SRC2[223:192]
/// DEST[223:192] := SRC1[255:224]
/// DEST[255:224] := SRC2[255:224]
/// INTERLEAVE_HIGH_DWORDS(SRC1, SRC2)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// INTERLEAVE_HIGH_QWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_HIGH_QWORDS_256b(SRC1, SRC2)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// DEST[191:128] := SRC1[255:192]
/// DEST[255:192] := SRC2[255:192]
/// INTERLEAVE_HIGH_QWORDS(SRC1, SRC2)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// PUNPCKHBW (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_BYTES(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHBW (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_BYTES(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHBW (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1, SRC2)
/// VPUNPCKHBW (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHWD (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_WORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHWD (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHWD (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHWD (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHDQ (128-bit LegacySSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_DWORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHDQ (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHDQ (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHDQ (EVEX.512 Encoded Version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHQDQ (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKHQDQ (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKHQDQ (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHQDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn punpckhdq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PUNPCKHBW Instruction With 64-bit Operands:
///     DEST[7:0] := DEST[39:32];
///     DEST[15:8] := SRC[39:32];
///     DEST[23:16] := DEST[47:40];
///     DEST[31:24] := SRC[47:40];
///     DEST[39:32] := DEST[55:48];
///     DEST[47:40] := SRC[55:48];
///     DEST[55:48] := DEST[63:56];
///     DEST[63:56] := SRC[63:56];
/// PUNPCKHW Instruction With 64-bit Operands:
///     DEST[15:0] := DEST[47:32];
///     DEST[31:16] := SRC[47:32];
///     DEST[47:32] := DEST[63:48];
///     DEST[63:48] := SRC[63:48];
/// PUNPCKHDQ Instruction With 64-bit Operands:
///     DEST[31:0] := DEST[63:32];
///     DEST[63:32] := SRC[63:32];
/// INTERLEAVE_HIGH_BYTES_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_BYTES_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_HIGH_BYTES_256b (SRC1, SRC2)
/// DEST[7:0] := SRC1[71:64]
/// DEST[15:8] := SRC2[71:64]
/// DEST[23:16] := SRC1[79:72]
/// DEST[31:24] := SRC2[79:72]
/// DEST[39:32] := SRC1[87:80]
/// DEST[47:40] := SRC2[87:80]
/// DEST[55:48] := SRC1[95:88]
/// DEST[63:56] := SRC2[95:88]
/// DEST[71:64] := SRC1[103:96]
/// DEST[79:72] := SRC2[103:96]
/// DEST[87:80] := SRC1[111:104]
/// DEST[95:88] := SRC2[111:104]
/// DEST[103:96] := SRC1[119:112]
/// DEST[111:104] := SRC2[119:112]
/// DEST[119:112] := SRC1[127:120]
/// DEST[127:120] := SRC2[127:120]
/// DEST[135:128] := SRC1[199:192]
/// DEST[143:136] := SRC2[199:192]
/// DEST[151:144] := SRC1[207:200]
/// DEST[167:160] := SRC1[215:208]
/// DEST[175:168] := SRC2[215:208]
/// DEST[183:176] := SRC1[223:216]
/// DEST[191:184] := SRC2[223:216]
/// DEST[199:192] := SRC1[231:224]
/// DEST[207:200] := SRC2[231:224]
/// DEST[215:208] := SRC1[239:232]
/// DEST[223:216] := SRC2[239:232]
/// DEST[231:224] := SRC1[247:240]
/// DEST[239:232] := SRC2[247:240]
/// DEST[247:240] := SRC1[255:248]
/// DEST[255:248] := SRC2[255:248]
/// INTERLEAVE_HIGH_BYTES (SRC1, SRC2)
/// DEST[7:0] := SRC1[71:64]
/// DEST[15:8] := SRC2[71:64]
/// DEST[23:16] := SRC1[79:72]
/// DEST[31:24] := SRC2[79:72]
/// DEST[39:32] := SRC1[87:80]
/// DEST[47:40] := SRC2[87:80]
/// DEST[55:48] := SRC1[95:88]
/// DEST[63:56] := SRC2[95:88]
/// DEST[71:64] := SRC1[103:96]
/// DEST[79:72] := SRC2[103:96]
/// DEST[87:80] := SRC1[111:104]
/// DEST[95:88] := SRC2[111:104]
/// DEST[103:96] := SRC1[119:112]
/// DEST[111:104] := SRC2[119:112]
/// DEST[119:112] := SRC1[127:120]
/// DEST[127:120] := SRC2[127:120]
/// INTERLEAVE_HIGH_WORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_WORDS_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_HIGH_WORDS_256b(SRC1, SRC2)
/// DEST[15:0] := SRC1[79:64]
/// DEST[31:16] := SRC2[79:64]
/// DEST[47:32] := SRC1[95:80]
/// DEST[63:48] := SRC2[95:80]
/// DEST[79:64] := SRC1[111:96]
/// DEST[95:80] := SRC2[111:96]
/// DEST[111:96] := SRC1[127:112]
/// DEST[127:112] := SRC2[127:112]
/// DEST[143:128] := SRC1[207:192]
/// DEST[159:144] := SRC2[207:192]
/// DEST[175:160] := SRC1[223:208]
/// DEST[191:176] := SRC2[223:208]
/// DEST[207:192] := SRC1[239:224]
/// DEST[223:208] := SRC2[239:224]
/// DEST[239:224] := SRC1[255:240]
/// DEST[255:240] := SRC2[255:240]
/// DEST[15:0] := SRC1[79:64]
/// DEST[31:16] := SRC2[79:64]
/// DEST[47:32] := SRC1[95:80]
/// DEST[63:48] := SRC2[95:80]
/// DEST[79:64] := SRC1[111:96]
/// DEST[95:80] := SRC2[111:96]
/// DEST[111:96] := SRC1[127:112]
/// DEST[127:112] := SRC2[127:112]
/// INTERLEAVE_HIGH_DWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_HIGH_DWORDS_256b(SRC1, SRC2)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// DEST[159:128] := SRC1[223:192]
/// DEST[191:160] := SRC2[223:192]
/// DEST[223:192] := SRC1[255:224]
/// DEST[255:224] := SRC2[255:224]
/// INTERLEAVE_HIGH_DWORDS(SRC1, SRC2)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// INTERLEAVE_HIGH_QWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_HIGH_QWORDS_256b(SRC1, SRC2)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// DEST[191:128] := SRC1[255:192]
/// DEST[255:192] := SRC2[255:192]
/// INTERLEAVE_HIGH_QWORDS(SRC1, SRC2)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// PUNPCKHBW (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_BYTES(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHBW (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_BYTES(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHBW (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1, SRC2)
/// VPUNPCKHBW (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHWD (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_WORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHWD (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHWD (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHWD (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHDQ (128-bit LegacySSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_DWORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHDQ (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHDQ (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHDQ (EVEX.512 Encoded Version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHQDQ (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKHQDQ (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKHQDQ (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHQDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn punpckhqdq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PUNPCKHBW Instruction With 64-bit Operands:
///     DEST[7:0] := DEST[39:32];
///     DEST[15:8] := SRC[39:32];
///     DEST[23:16] := DEST[47:40];
///     DEST[31:24] := SRC[47:40];
///     DEST[39:32] := DEST[55:48];
///     DEST[47:40] := SRC[55:48];
///     DEST[55:48] := DEST[63:56];
///     DEST[63:56] := SRC[63:56];
/// PUNPCKHW Instruction With 64-bit Operands:
///     DEST[15:0] := DEST[47:32];
///     DEST[31:16] := SRC[47:32];
///     DEST[47:32] := DEST[63:48];
///     DEST[63:48] := SRC[63:48];
/// PUNPCKHDQ Instruction With 64-bit Operands:
///     DEST[31:0] := DEST[63:32];
///     DEST[63:32] := SRC[63:32];
/// INTERLEAVE_HIGH_BYTES_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_BYTES_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_HIGH_BYTES_256b (SRC1, SRC2)
/// DEST[7:0] := SRC1[71:64]
/// DEST[15:8] := SRC2[71:64]
/// DEST[23:16] := SRC1[79:72]
/// DEST[31:24] := SRC2[79:72]
/// DEST[39:32] := SRC1[87:80]
/// DEST[47:40] := SRC2[87:80]
/// DEST[55:48] := SRC1[95:88]
/// DEST[63:56] := SRC2[95:88]
/// DEST[71:64] := SRC1[103:96]
/// DEST[79:72] := SRC2[103:96]
/// DEST[87:80] := SRC1[111:104]
/// DEST[95:88] := SRC2[111:104]
/// DEST[103:96] := SRC1[119:112]
/// DEST[111:104] := SRC2[119:112]
/// DEST[119:112] := SRC1[127:120]
/// DEST[127:120] := SRC2[127:120]
/// DEST[135:128] := SRC1[199:192]
/// DEST[143:136] := SRC2[199:192]
/// DEST[151:144] := SRC1[207:200]
/// DEST[167:160] := SRC1[215:208]
/// DEST[175:168] := SRC2[215:208]
/// DEST[183:176] := SRC1[223:216]
/// DEST[191:184] := SRC2[223:216]
/// DEST[199:192] := SRC1[231:224]
/// DEST[207:200] := SRC2[231:224]
/// DEST[215:208] := SRC1[239:232]
/// DEST[223:216] := SRC2[239:232]
/// DEST[231:224] := SRC1[247:240]
/// DEST[239:232] := SRC2[247:240]
/// DEST[247:240] := SRC1[255:248]
/// DEST[255:248] := SRC2[255:248]
/// INTERLEAVE_HIGH_BYTES (SRC1, SRC2)
/// DEST[7:0] := SRC1[71:64]
/// DEST[15:8] := SRC2[71:64]
/// DEST[23:16] := SRC1[79:72]
/// DEST[31:24] := SRC2[79:72]
/// DEST[39:32] := SRC1[87:80]
/// DEST[47:40] := SRC2[87:80]
/// DEST[55:48] := SRC1[95:88]
/// DEST[63:56] := SRC2[95:88]
/// DEST[71:64] := SRC1[103:96]
/// DEST[79:72] := SRC2[103:96]
/// DEST[87:80] := SRC1[111:104]
/// DEST[95:88] := SRC2[111:104]
/// DEST[103:96] := SRC1[119:112]
/// DEST[111:104] := SRC2[119:112]
/// DEST[119:112] := SRC1[127:120]
/// DEST[127:120] := SRC2[127:120]
/// INTERLEAVE_HIGH_WORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_WORDS_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_HIGH_WORDS_256b(SRC1, SRC2)
/// DEST[15:0] := SRC1[79:64]
/// DEST[31:16] := SRC2[79:64]
/// DEST[47:32] := SRC1[95:80]
/// DEST[63:48] := SRC2[95:80]
/// DEST[79:64] := SRC1[111:96]
/// DEST[95:80] := SRC2[111:96]
/// DEST[111:96] := SRC1[127:112]
/// DEST[127:112] := SRC2[127:112]
/// DEST[143:128] := SRC1[207:192]
/// DEST[159:144] := SRC2[207:192]
/// DEST[175:160] := SRC1[223:208]
/// DEST[191:176] := SRC2[223:208]
/// DEST[207:192] := SRC1[239:224]
/// DEST[223:208] := SRC2[239:224]
/// DEST[239:224] := SRC1[255:240]
/// DEST[255:240] := SRC2[255:240]
/// DEST[15:0] := SRC1[79:64]
/// DEST[31:16] := SRC2[79:64]
/// DEST[47:32] := SRC1[95:80]
/// DEST[63:48] := SRC2[95:80]
/// DEST[79:64] := SRC1[111:96]
/// DEST[95:80] := SRC2[111:96]
/// DEST[111:96] := SRC1[127:112]
/// DEST[127:112] := SRC2[127:112]
/// INTERLEAVE_HIGH_DWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_HIGH_DWORDS_256b(SRC1, SRC2)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// DEST[159:128] := SRC1[223:192]
/// DEST[191:160] := SRC2[223:192]
/// DEST[223:192] := SRC1[255:224]
/// DEST[255:224] := SRC2[255:224]
/// INTERLEAVE_HIGH_DWORDS(SRC1, SRC2)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// INTERLEAVE_HIGH_QWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_HIGH_QWORDS_256b(SRC1, SRC2)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// DEST[191:128] := SRC1[255:192]
/// DEST[255:192] := SRC2[255:192]
/// INTERLEAVE_HIGH_QWORDS(SRC1, SRC2)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// PUNPCKHBW (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_BYTES(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHBW (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_BYTES(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHBW (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1, SRC2)
/// VPUNPCKHBW (EVEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_BYTES_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHWD (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_WORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHWD (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHWD (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHWD (EVEX Encoded Versions)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_WORDS_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHDQ (128-bit LegacySSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_DWORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKHDQ (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKHDQ (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHDQ (EVEX.512 Encoded Version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_DWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+31:i] := 0
///             FI
///     FI;
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKHQDQ (128-bit Legacy SSE Version)
/// DEST[127:0] := INTERLEAVE_HIGH_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKHQDQ (VEX.128 Encoded Version)
/// DEST[127:0] := INTERLEAVE_HIGH_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKHQDQ (VEX.256 Encoded Version)
/// DEST[255:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKHQDQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_HIGH_QWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn punpckhwd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PUNPCKLBW Instruction With 64-bit Operands:
///     DEST[63:56] := SRC[31:24];
///     DEST[55:48] := DEST[31:24];
///     DEST[47:40] := SRC[23:16];
///     DEST[39:32] := DEST[23:16];
///     DEST[31:24] := SRC[15:8];
///     DEST[23:16] := DEST[15:8];
///     DEST[15:8] := SRC[7:0];
///     DEST[7:0] := DEST[7:0];
/// PUNPCKLWD Instruction With 64-bit Operands:
///     DEST[63:48] := SRC[31:16];
///     DEST[47:32] := DEST[31:16];
///     DEST[31:16] := SRC[15:0];
///     DEST[15:0] := DEST[15:0];
/// PUNPCKLDQ Instruction With 64-bit Operands:
///     DEST[63:32] := SRC[31:0];
///     DEST[31:0] := DEST[31:0];
/// INTERLEAVE_BYTES_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_BYTES_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_BYTES_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_BYTES_256b (SRC1, SRC2)
/// DEST[7:0] := SRC1[7:0]
/// DEST[15:8] := SRC2[7:0]
/// DEST[23:16] := SRC1[15:8]
/// DEST[31:24] := SRC2[15:8]
/// DEST[39:32] := SRC1[23:16]
/// DEST[47:40] := SRC2[23:16]
/// DEST[55:48] := SRC1[31:24]
/// DEST[63:56] := SRC2[31:24]
/// DEST[71:64] := SRC1[39:32]
/// DEST[79:72] := SRC2[39:32]
/// DEST[87:80] := SRC1[47:40]
/// DEST[95:88] := SRC2[47:40]
/// DEST[103:96] := SRC1[55:48]
/// DEST[111:104] := SRC2[55:48]
/// DEST[119:112] := SRC1[63:56]
/// DEST[127:120] := SRC2[63:56]
/// DEST[135:128] := SRC1[135:128]
/// DEST[143:136] := SRC2[135:128]
/// DEST[151:144] := SRC1[143:136]
/// DEST[159:152] := SRC2[143:136]
/// DEST[167:160] := SRC1[151:144]
/// DEST[175:168] := SRC2[151:144]
/// DEST[183:176] := SRC1[159:152]
/// DEST[191:184] := SRC2[159:152]
/// DEST[199:192] := SRC1[167:160]
/// DEST[207:200] := SRC2[167:160]
/// DEST[215:208] := SRC1[175:168]
/// DEST[223:216] := SRC2[175:168]
/// DEST[231:224] := SRC1[183:176]
/// DEST[239:232] := SRC2[183:176]
/// DEST[247:240] := SRC1[191:184]
/// DEST[255:248] := SRC2[191:184]
/// INTERLEAVE_BYTES (SRC1, SRC2)
/// DEST[7:0] := SRC1[7:0]
/// DEST[15:8] := SRC2[7:0]
/// DEST[23:16] := SRC1[15:8]
/// DEST[31:24] := SRC2[15:8]
/// DEST[39:32] := SRC1[23:16]
/// DEST[47:40] := SRC2[23:16]
/// DEST[55:48] := SRC1[31:24]
/// DEST[63:56] := SRC2[31:24]
/// DEST[71:64] := SRC1[39:32]
/// DEST[79:72] := SRC2[39:32]
/// DEST[87:80] := SRC1[47:40]
/// DEST[95:88] := SRC2[47:40]
/// DEST[103:96] := SRC1[55:48]
/// DEST[111:104] := SRC2[55:48]
/// DEST[119:112] := SRC1[63:56]
/// DEST[127:120] := SRC2[63:56]
/// INTERLEAVE_WORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_WORDS_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_WORDS_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_WORDS_256b(SRC1, SRC2)
/// DEST[15:0] := SRC1[15:0]
/// DEST[31:16] := SRC2[15:0]
/// DEST[47:32] := SRC1[31:16]
/// DEST[63:48] := SRC2[31:16]
/// DEST[79:64] := SRC1[47:32]
/// DEST[95:80] := SRC2[47:32]
/// DEST[111:96] := SRC1[63:48]
/// DEST[127:112] := SRC2[63:48]
/// DEST[143:128] := SRC1[143:128]
/// DEST[159:144] := SRC2[143:128]
/// DEST[175:160] := SRC1[159:144]
/// DEST[191:176] := SRC2[159:144]
/// DEST[207:192] := SRC1[175:160]
/// DEST[223:208] := SRC2[175:160]
/// DEST[239:224] := SRC1[191:176]
/// DEST[255:240] := SRC2[191:176]
/// INTERLEAVE_WORDS (SRC1, SRC2)
/// DEST[15:0] := SRC1[15:0]
/// DEST[47:32] := SRC1[31:16]
/// DEST[63:48] := SRC2[31:16]
/// DEST[79:64] := SRC1[47:32]
/// DEST[95:80] := SRC2[47:32]
/// DEST[111:96] := SRC1[63:48]
/// DEST[127:112] := SRC2[63:48]
/// INTERLEAVE_DWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_DWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_DWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_DWORDS_256b(SRC1, SRC2)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// DEST[159:128] := SRC1[159:128]
/// DEST[191:160] := SRC2[159:128]
/// DEST[223:192] := SRC1[191:160]
/// DEST[255:224] := SRC2[191:160]
/// INTERLEAVE_DWORDS(SRC1, SRC2)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// INTERLEAVE_QWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_QWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_QWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_QWORDS_256b(SRC1, SRC2)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[191:128] := SRC1[191:128]
/// DEST[255:192] := SRC2[191:128]
/// INTERLEAVE_QWORDS(SRC1, SRC2)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// PUNPCKLBW
/// DEST[127:0] := INTERLEAVE_BYTES(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKLBW (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_BYTES(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKLBW (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_BYTES_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLBW (EVEX.512 Encoded Instruction)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// DEST[511:0] := INTERLEAVE_BYTES_512b(SRC1, SRC2)
/// PUNPCKLWD
/// DEST[127:0] := INTERLEAVE_WORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKLWD (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKLWD (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLWD (EVEX.512 Encoded Instruction)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// DEST[511:0] := INTERLEAVE_WORDS_512b(SRC1, SRC2)
/// PUNPCKLDQ
/// DEST[127:0] := INTERLEAVE_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKLDQ (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKLDQ (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLDQ (EVEX Encoded Instructions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+31:i] := 0
///             FI
/// ENDFOR
/// DEST511:0] := INTERLEAVE_DWORDS_512b(SRC1, SRC2)
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKLQDQ
/// DEST[127:0] := INTERLEAVE_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKLQDQ (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKLQDQ (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLQDQ (EVEX Encoded Instructions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn punpcklbw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PUNPCKLBW Instruction With 64-bit Operands:
///     DEST[63:56] := SRC[31:24];
///     DEST[55:48] := DEST[31:24];
///     DEST[47:40] := SRC[23:16];
///     DEST[39:32] := DEST[23:16];
///     DEST[31:24] := SRC[15:8];
///     DEST[23:16] := DEST[15:8];
///     DEST[15:8] := SRC[7:0];
///     DEST[7:0] := DEST[7:0];
/// PUNPCKLWD Instruction With 64-bit Operands:
///     DEST[63:48] := SRC[31:16];
///     DEST[47:32] := DEST[31:16];
///     DEST[31:16] := SRC[15:0];
///     DEST[15:0] := DEST[15:0];
/// PUNPCKLDQ Instruction With 64-bit Operands:
///     DEST[63:32] := SRC[31:0];
///     DEST[31:0] := DEST[31:0];
/// INTERLEAVE_BYTES_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_BYTES_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_BYTES_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_BYTES_256b (SRC1, SRC2)
/// DEST[7:0] := SRC1[7:0]
/// DEST[15:8] := SRC2[7:0]
/// DEST[23:16] := SRC1[15:8]
/// DEST[31:24] := SRC2[15:8]
/// DEST[39:32] := SRC1[23:16]
/// DEST[47:40] := SRC2[23:16]
/// DEST[55:48] := SRC1[31:24]
/// DEST[63:56] := SRC2[31:24]
/// DEST[71:64] := SRC1[39:32]
/// DEST[79:72] := SRC2[39:32]
/// DEST[87:80] := SRC1[47:40]
/// DEST[95:88] := SRC2[47:40]
/// DEST[103:96] := SRC1[55:48]
/// DEST[111:104] := SRC2[55:48]
/// DEST[119:112] := SRC1[63:56]
/// DEST[127:120] := SRC2[63:56]
/// DEST[135:128] := SRC1[135:128]
/// DEST[143:136] := SRC2[135:128]
/// DEST[151:144] := SRC1[143:136]
/// DEST[159:152] := SRC2[143:136]
/// DEST[167:160] := SRC1[151:144]
/// DEST[175:168] := SRC2[151:144]
/// DEST[183:176] := SRC1[159:152]
/// DEST[191:184] := SRC2[159:152]
/// DEST[199:192] := SRC1[167:160]
/// DEST[207:200] := SRC2[167:160]
/// DEST[215:208] := SRC1[175:168]
/// DEST[223:216] := SRC2[175:168]
/// DEST[231:224] := SRC1[183:176]
/// DEST[239:232] := SRC2[183:176]
/// DEST[247:240] := SRC1[191:184]
/// DEST[255:248] := SRC2[191:184]
/// INTERLEAVE_BYTES (SRC1, SRC2)
/// DEST[7:0] := SRC1[7:0]
/// DEST[15:8] := SRC2[7:0]
/// DEST[23:16] := SRC1[15:8]
/// DEST[31:24] := SRC2[15:8]
/// DEST[39:32] := SRC1[23:16]
/// DEST[47:40] := SRC2[23:16]
/// DEST[55:48] := SRC1[31:24]
/// DEST[63:56] := SRC2[31:24]
/// DEST[71:64] := SRC1[39:32]
/// DEST[79:72] := SRC2[39:32]
/// DEST[87:80] := SRC1[47:40]
/// DEST[95:88] := SRC2[47:40]
/// DEST[103:96] := SRC1[55:48]
/// DEST[111:104] := SRC2[55:48]
/// DEST[119:112] := SRC1[63:56]
/// DEST[127:120] := SRC2[63:56]
/// INTERLEAVE_WORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_WORDS_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_WORDS_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_WORDS_256b(SRC1, SRC2)
/// DEST[15:0] := SRC1[15:0]
/// DEST[31:16] := SRC2[15:0]
/// DEST[47:32] := SRC1[31:16]
/// DEST[63:48] := SRC2[31:16]
/// DEST[79:64] := SRC1[47:32]
/// DEST[95:80] := SRC2[47:32]
/// DEST[111:96] := SRC1[63:48]
/// DEST[127:112] := SRC2[63:48]
/// DEST[143:128] := SRC1[143:128]
/// DEST[159:144] := SRC2[143:128]
/// DEST[175:160] := SRC1[159:144]
/// DEST[191:176] := SRC2[159:144]
/// DEST[207:192] := SRC1[175:160]
/// DEST[223:208] := SRC2[175:160]
/// DEST[239:224] := SRC1[191:176]
/// DEST[255:240] := SRC2[191:176]
/// INTERLEAVE_WORDS (SRC1, SRC2)
/// DEST[15:0] := SRC1[15:0]
/// DEST[47:32] := SRC1[31:16]
/// DEST[63:48] := SRC2[31:16]
/// DEST[79:64] := SRC1[47:32]
/// DEST[95:80] := SRC2[47:32]
/// DEST[111:96] := SRC1[63:48]
/// DEST[127:112] := SRC2[63:48]
/// INTERLEAVE_DWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_DWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_DWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_DWORDS_256b(SRC1, SRC2)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// DEST[159:128] := SRC1[159:128]
/// DEST[191:160] := SRC2[159:128]
/// DEST[223:192] := SRC1[191:160]
/// DEST[255:224] := SRC2[191:160]
/// INTERLEAVE_DWORDS(SRC1, SRC2)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// INTERLEAVE_QWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_QWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_QWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_QWORDS_256b(SRC1, SRC2)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[191:128] := SRC1[191:128]
/// DEST[255:192] := SRC2[191:128]
/// INTERLEAVE_QWORDS(SRC1, SRC2)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// PUNPCKLBW
/// DEST[127:0] := INTERLEAVE_BYTES(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKLBW (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_BYTES(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKLBW (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_BYTES_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLBW (EVEX.512 Encoded Instruction)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// DEST[511:0] := INTERLEAVE_BYTES_512b(SRC1, SRC2)
/// PUNPCKLWD
/// DEST[127:0] := INTERLEAVE_WORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKLWD (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKLWD (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLWD (EVEX.512 Encoded Instruction)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// DEST[511:0] := INTERLEAVE_WORDS_512b(SRC1, SRC2)
/// PUNPCKLDQ
/// DEST[127:0] := INTERLEAVE_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKLDQ (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKLDQ (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLDQ (EVEX Encoded Instructions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+31:i] := 0
///             FI
/// ENDFOR
/// DEST511:0] := INTERLEAVE_DWORDS_512b(SRC1, SRC2)
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKLQDQ
/// DEST[127:0] := INTERLEAVE_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKLQDQ (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKLQDQ (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLQDQ (EVEX Encoded Instructions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn punpckldq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PUNPCKLBW Instruction With 64-bit Operands:
///     DEST[63:56] := SRC[31:24];
///     DEST[55:48] := DEST[31:24];
///     DEST[47:40] := SRC[23:16];
///     DEST[39:32] := DEST[23:16];
///     DEST[31:24] := SRC[15:8];
///     DEST[23:16] := DEST[15:8];
///     DEST[15:8] := SRC[7:0];
///     DEST[7:0] := DEST[7:0];
/// PUNPCKLWD Instruction With 64-bit Operands:
///     DEST[63:48] := SRC[31:16];
///     DEST[47:32] := DEST[31:16];
///     DEST[31:16] := SRC[15:0];
///     DEST[15:0] := DEST[15:0];
/// PUNPCKLDQ Instruction With 64-bit Operands:
///     DEST[63:32] := SRC[31:0];
///     DEST[31:0] := DEST[31:0];
/// INTERLEAVE_BYTES_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_BYTES_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_BYTES_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_BYTES_256b (SRC1, SRC2)
/// DEST[7:0] := SRC1[7:0]
/// DEST[15:8] := SRC2[7:0]
/// DEST[23:16] := SRC1[15:8]
/// DEST[31:24] := SRC2[15:8]
/// DEST[39:32] := SRC1[23:16]
/// DEST[47:40] := SRC2[23:16]
/// DEST[55:48] := SRC1[31:24]
/// DEST[63:56] := SRC2[31:24]
/// DEST[71:64] := SRC1[39:32]
/// DEST[79:72] := SRC2[39:32]
/// DEST[87:80] := SRC1[47:40]
/// DEST[95:88] := SRC2[47:40]
/// DEST[103:96] := SRC1[55:48]
/// DEST[111:104] := SRC2[55:48]
/// DEST[119:112] := SRC1[63:56]
/// DEST[127:120] := SRC2[63:56]
/// DEST[135:128] := SRC1[135:128]
/// DEST[143:136] := SRC2[135:128]
/// DEST[151:144] := SRC1[143:136]
/// DEST[159:152] := SRC2[143:136]
/// DEST[167:160] := SRC1[151:144]
/// DEST[175:168] := SRC2[151:144]
/// DEST[183:176] := SRC1[159:152]
/// DEST[191:184] := SRC2[159:152]
/// DEST[199:192] := SRC1[167:160]
/// DEST[207:200] := SRC2[167:160]
/// DEST[215:208] := SRC1[175:168]
/// DEST[223:216] := SRC2[175:168]
/// DEST[231:224] := SRC1[183:176]
/// DEST[239:232] := SRC2[183:176]
/// DEST[247:240] := SRC1[191:184]
/// DEST[255:248] := SRC2[191:184]
/// INTERLEAVE_BYTES (SRC1, SRC2)
/// DEST[7:0] := SRC1[7:0]
/// DEST[15:8] := SRC2[7:0]
/// DEST[23:16] := SRC1[15:8]
/// DEST[31:24] := SRC2[15:8]
/// DEST[39:32] := SRC1[23:16]
/// DEST[47:40] := SRC2[23:16]
/// DEST[55:48] := SRC1[31:24]
/// DEST[63:56] := SRC2[31:24]
/// DEST[71:64] := SRC1[39:32]
/// DEST[79:72] := SRC2[39:32]
/// DEST[87:80] := SRC1[47:40]
/// DEST[95:88] := SRC2[47:40]
/// DEST[103:96] := SRC1[55:48]
/// DEST[111:104] := SRC2[55:48]
/// DEST[119:112] := SRC1[63:56]
/// DEST[127:120] := SRC2[63:56]
/// INTERLEAVE_WORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_WORDS_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_WORDS_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_WORDS_256b(SRC1, SRC2)
/// DEST[15:0] := SRC1[15:0]
/// DEST[31:16] := SRC2[15:0]
/// DEST[47:32] := SRC1[31:16]
/// DEST[63:48] := SRC2[31:16]
/// DEST[79:64] := SRC1[47:32]
/// DEST[95:80] := SRC2[47:32]
/// DEST[111:96] := SRC1[63:48]
/// DEST[127:112] := SRC2[63:48]
/// DEST[143:128] := SRC1[143:128]
/// DEST[159:144] := SRC2[143:128]
/// DEST[175:160] := SRC1[159:144]
/// DEST[191:176] := SRC2[159:144]
/// DEST[207:192] := SRC1[175:160]
/// DEST[223:208] := SRC2[175:160]
/// DEST[239:224] := SRC1[191:176]
/// DEST[255:240] := SRC2[191:176]
/// INTERLEAVE_WORDS (SRC1, SRC2)
/// DEST[15:0] := SRC1[15:0]
/// DEST[47:32] := SRC1[31:16]
/// DEST[63:48] := SRC2[31:16]
/// DEST[79:64] := SRC1[47:32]
/// DEST[95:80] := SRC2[47:32]
/// DEST[111:96] := SRC1[63:48]
/// DEST[127:112] := SRC2[63:48]
/// INTERLEAVE_DWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_DWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_DWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_DWORDS_256b(SRC1, SRC2)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// DEST[159:128] := SRC1[159:128]
/// DEST[191:160] := SRC2[159:128]
/// DEST[223:192] := SRC1[191:160]
/// DEST[255:224] := SRC2[191:160]
/// INTERLEAVE_DWORDS(SRC1, SRC2)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// INTERLEAVE_QWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_QWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_QWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_QWORDS_256b(SRC1, SRC2)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[191:128] := SRC1[191:128]
/// DEST[255:192] := SRC2[191:128]
/// INTERLEAVE_QWORDS(SRC1, SRC2)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// PUNPCKLBW
/// DEST[127:0] := INTERLEAVE_BYTES(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKLBW (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_BYTES(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKLBW (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_BYTES_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLBW (EVEX.512 Encoded Instruction)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// DEST[511:0] := INTERLEAVE_BYTES_512b(SRC1, SRC2)
/// PUNPCKLWD
/// DEST[127:0] := INTERLEAVE_WORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKLWD (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKLWD (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLWD (EVEX.512 Encoded Instruction)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// DEST[511:0] := INTERLEAVE_WORDS_512b(SRC1, SRC2)
/// PUNPCKLDQ
/// DEST[127:0] := INTERLEAVE_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKLDQ (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKLDQ (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLDQ (EVEX Encoded Instructions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+31:i] := 0
///             FI
/// ENDFOR
/// DEST511:0] := INTERLEAVE_DWORDS_512b(SRC1, SRC2)
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKLQDQ
/// DEST[127:0] := INTERLEAVE_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKLQDQ (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKLQDQ (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLQDQ (EVEX Encoded Instructions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn punpcklqdq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// PUNPCKLBW Instruction With 64-bit Operands:
///     DEST[63:56] := SRC[31:24];
///     DEST[55:48] := DEST[31:24];
///     DEST[47:40] := SRC[23:16];
///     DEST[39:32] := DEST[23:16];
///     DEST[31:24] := SRC[15:8];
///     DEST[23:16] := DEST[15:8];
///     DEST[15:8] := SRC[7:0];
///     DEST[7:0] := DEST[7:0];
/// PUNPCKLWD Instruction With 64-bit Operands:
///     DEST[63:48] := SRC[31:16];
///     DEST[47:32] := DEST[31:16];
///     DEST[31:16] := SRC[15:0];
///     DEST[15:0] := DEST[15:0];
/// PUNPCKLDQ Instruction With 64-bit Operands:
///     DEST[63:32] := SRC[31:0];
///     DEST[31:0] := DEST[31:0];
/// INTERLEAVE_BYTES_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_BYTES_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_BYTES_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_BYTES_256b (SRC1, SRC2)
/// DEST[7:0] := SRC1[7:0]
/// DEST[15:8] := SRC2[7:0]
/// DEST[23:16] := SRC1[15:8]
/// DEST[31:24] := SRC2[15:8]
/// DEST[39:32] := SRC1[23:16]
/// DEST[47:40] := SRC2[23:16]
/// DEST[55:48] := SRC1[31:24]
/// DEST[63:56] := SRC2[31:24]
/// DEST[71:64] := SRC1[39:32]
/// DEST[79:72] := SRC2[39:32]
/// DEST[87:80] := SRC1[47:40]
/// DEST[95:88] := SRC2[47:40]
/// DEST[103:96] := SRC1[55:48]
/// DEST[111:104] := SRC2[55:48]
/// DEST[119:112] := SRC1[63:56]
/// DEST[127:120] := SRC2[63:56]
/// DEST[135:128] := SRC1[135:128]
/// DEST[143:136] := SRC2[135:128]
/// DEST[151:144] := SRC1[143:136]
/// DEST[159:152] := SRC2[143:136]
/// DEST[167:160] := SRC1[151:144]
/// DEST[175:168] := SRC2[151:144]
/// DEST[183:176] := SRC1[159:152]
/// DEST[191:184] := SRC2[159:152]
/// DEST[199:192] := SRC1[167:160]
/// DEST[207:200] := SRC2[167:160]
/// DEST[215:208] := SRC1[175:168]
/// DEST[223:216] := SRC2[175:168]
/// DEST[231:224] := SRC1[183:176]
/// DEST[239:232] := SRC2[183:176]
/// DEST[247:240] := SRC1[191:184]
/// DEST[255:248] := SRC2[191:184]
/// INTERLEAVE_BYTES (SRC1, SRC2)
/// DEST[7:0] := SRC1[7:0]
/// DEST[15:8] := SRC2[7:0]
/// DEST[23:16] := SRC1[15:8]
/// DEST[31:24] := SRC2[15:8]
/// DEST[39:32] := SRC1[23:16]
/// DEST[47:40] := SRC2[23:16]
/// DEST[55:48] := SRC1[31:24]
/// DEST[63:56] := SRC2[31:24]
/// DEST[71:64] := SRC1[39:32]
/// DEST[79:72] := SRC2[39:32]
/// DEST[87:80] := SRC1[47:40]
/// DEST[95:88] := SRC2[47:40]
/// DEST[103:96] := SRC1[55:48]
/// DEST[111:104] := SRC2[55:48]
/// DEST[119:112] := SRC1[63:56]
/// DEST[127:120] := SRC2[63:56]
/// INTERLEAVE_WORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_WORDS_256b(SRC1[255:0], SRC[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_WORDS_256b(SRC1[511:256], SRC[511:256])
/// INTERLEAVE_WORDS_256b(SRC1, SRC2)
/// DEST[15:0] := SRC1[15:0]
/// DEST[31:16] := SRC2[15:0]
/// DEST[47:32] := SRC1[31:16]
/// DEST[63:48] := SRC2[31:16]
/// DEST[79:64] := SRC1[47:32]
/// DEST[95:80] := SRC2[47:32]
/// DEST[111:96] := SRC1[63:48]
/// DEST[127:112] := SRC2[63:48]
/// DEST[143:128] := SRC1[143:128]
/// DEST[159:144] := SRC2[143:128]
/// DEST[175:160] := SRC1[159:144]
/// DEST[191:176] := SRC2[159:144]
/// DEST[207:192] := SRC1[175:160]
/// DEST[223:208] := SRC2[175:160]
/// DEST[239:224] := SRC1[191:176]
/// DEST[255:240] := SRC2[191:176]
/// INTERLEAVE_WORDS (SRC1, SRC2)
/// DEST[15:0] := SRC1[15:0]
/// DEST[47:32] := SRC1[31:16]
/// DEST[63:48] := SRC2[31:16]
/// DEST[79:64] := SRC1[47:32]
/// DEST[95:80] := SRC2[47:32]
/// DEST[111:96] := SRC1[63:48]
/// DEST[127:112] := SRC2[63:48]
/// INTERLEAVE_DWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_DWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_DWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_DWORDS_256b(SRC1, SRC2)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// DEST[159:128] := SRC1[159:128]
/// DEST[191:160] := SRC2[159:128]
/// DEST[223:192] := SRC1[191:160]
/// DEST[255:224] := SRC2[191:160]
/// INTERLEAVE_DWORDS(SRC1, SRC2)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// INTERLEAVE_QWORDS_512b (SRC1, SRC2)
/// TMP_DEST[255:0] := INTERLEAVE_QWORDS_256b(SRC1[255:0], SRC2[255:0])
/// TMP_DEST[511:256] := INTERLEAVE_QWORDS_256b(SRC1[511:256], SRC2[511:256])
/// INTERLEAVE_QWORDS_256b(SRC1, SRC2)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[191:128] := SRC1[191:128]
/// DEST[255:192] := SRC2[191:128]
/// INTERLEAVE_QWORDS(SRC1, SRC2)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// PUNPCKLBW
/// DEST[127:0] := INTERLEAVE_BYTES(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKLBW (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_BYTES(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKLBW (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_BYTES_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLBW (EVEX.512 Encoded Instruction)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_BYTES_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 8
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+7:i] := TMP_DEST[i+7:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+7:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+7:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// DEST[511:0] := INTERLEAVE_BYTES_512b(SRC1, SRC2)
/// PUNPCKLWD
/// DEST[127:0] := INTERLEAVE_WORDS(DEST, SRC)
/// DEST[255:127] (Unmodified)
/// VPUNPCKLWD (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_WORDS(SRC1, SRC2)
/// DEST[MAXVL-1:127] := 0
/// VPUNPCKLWD (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_WORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLWD (EVEX.512 Encoded Instruction)
/// (KL, VL) = (8, 128), (16, 256), (32, 512)
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS_256b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_WORDS_512b(SRC1[VL-1:0], SRC2[VL-1:0])
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 16
///         THEN DEST[i+15:i] := TMP_DEST[i+15:i]
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+15:i] remains unchanged*
///                 ELSE *zeroing-masking*
///                             ; zeroing-masking
///                     DEST[i+15:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// DEST[511:0] := INTERLEAVE_WORDS_512b(SRC1, SRC2)
/// PUNPCKLDQ
/// DEST[127:0] := INTERLEAVE_DWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKLDQ (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_DWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKLDQ (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_DWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLDQ (EVEX Encoded Instructions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_DWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+31:i] := 0
///             FI
/// ENDFOR
/// DEST511:0] := INTERLEAVE_DWORDS_512b(SRC1, SRC2)
/// DEST[MAXVL-1:VL] := 0
/// PUNPCKLQDQ
/// DEST[127:0] := INTERLEAVE_QWORDS(DEST, SRC)
/// DEST[MAXVL-1:128] (Unmodified)
/// VPUNPCKLQDQ (VEX.128 Encoded Instruction)
/// DEST[127:0] := INTERLEAVE_QWORDS(SRC1, SRC2)
/// DEST[MAXVL-1:128] := 0
/// VPUNPCKLQDQ (VEX.256 Encoded Instruction)
/// DEST[255:0] := INTERLEAVE_QWORDS_256b(SRC1, SRC2)
/// DEST[MAXVL-1:256] := 0
/// VPUNPCKLQDQ (EVEX Encoded Instructions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1) AND (SRC2 *is memory*)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL = 128
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 256
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS_256b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
/// FI;
/// IF VL = 512
///     TMP_DEST[VL-1:0] := INTERLEAVE_QWORDS_512b(SRC1[VL-1:0], TMP_SRC2[VL-1:0])
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
///                     DEST[i+63:i] := 0
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn punpcklwd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (* See Description section for possible sign-extension or zero-extension of source operand and for *)
/// (* a case in which the size of the memory store may be smaller than the instruction's operand size *)
/// IF StackAddrSize = 64
///     THEN
///         IF OperandSize = 64
///             THEN
///                 RSP := RSP - 8;
///                 Memory[SS:RSP] := SRC;
///                     (* push quadword *)
///         ELSE IF OperandSize = 32
///             THEN
///                 RSP := RSP - 4;
///                 Memory[SS:RSP] := SRC;
///                     (* push dword *)
///             ELSE (* OperandSize = 16 *)
///                 RSP := RSP - 2;
///                 Memory[SS:RSP] := SRC;
///                     (* push word *)
///         FI;
/// ELSE IF StackAddrSize = 32
///     THEN
///         IF OperandSize = 64
///             THEN
///                 ESP := ESP - 8;
///                 Memory[SS:ESP] := SRC;
///                     (* push quadword *)
///         ELSE IF OperandSize = 32
///             THEN
///                 ESP := ESP - 4;
///                 Memory[SS:ESP] := SRC;
///                     (* push dword *)
///             ELSE (* OperandSize = 16 *)
///                 ESP := ESP - 2;
///         FI;
///     ELSE (* StackAddrSize = 16 *)
///         IF OperandSize = 32
///             THEN
///                 SP := SP - 4;
///                 Memory[SS:SP] := SRC;
///                     (* push dword *)
///             ELSE (* OperandSize = 16 *)
///                 SP := SP - 2;
///                 Memory[SS:SP] := SRC;
///                     (* push word *)
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn push() -> &'static [IrStatement] {
    let set_sp = assign(b::sub(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let push = assign(o1(), d(rsp.clone()), o1_size());
    [set_sp, push].into()
}

/// # Pseudocode
/// ```text
/// IF 64-bit Mode
///     THEN #UD
/// FI;
/// IF OperandSize = 32 (* PUSHAD instruction *)
///     THEN
///         Temp := (ESP);
///         Push(EAX);
///         Push(ECX);
///         Push(EDX);
///         Push(EBX);
///         Push(Temp);
///         Push(EBP);
///         Push(ESI);
///         Push(EDI);
///     ELSE (* OperandSize = 16, PUSHA instruction *)
///         Temp := (SP);
///         Push(AX);
///         Push(CX);
///         Push(DX);
///         Push(BX);
///         Push(BP);
///         Push(SI);
///         Push(DI);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn pusha() -> &'static [IrStatement] {
    let set_sp = assign(b::sub(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let push = assign(o1(), d(rsp.clone()), o1_size());
    [set_sp, push].into()
}

/// # Pseudocode
/// ```text
/// IF 64-bit Mode
///     THEN #UD
/// FI;
/// IF OperandSize = 32 (* PUSHAD instruction *)
///     THEN
///         Temp := (ESP);
///         Push(EAX);
///         Push(ECX);
///         Push(EDX);
///         Push(EBX);
///         Push(Temp);
///         Push(EBP);
///         Push(ESI);
///         Push(EDI);
///     ELSE (* OperandSize = 16, PUSHA instruction *)
///         Temp := (SP);
///         Push(AX);
///         Push(CX);
///         Push(DX);
///         Push(BX);
///         Push(BP);
///         Push(SI);
///         Push(DI);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn pushad() -> &'static [IrStatement] {
    let set_sp = assign(b::sub(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let push = assign(o1(), d(rsp.clone()), o1_size());
    [set_sp, push].into()
}

/// # Pseudocode
/// ```text
/// IF (PE = 0) or (PE = 1 and ((VM = 0) or (VM = 1 and IOPL = 3)))
/// (* Real-Address Mode, Protected mode, or Virtual-8086 mode with IOPL equal to 3 *)
///     THEN
///         IF OperandSize = 32
///             THEN
///                 push (EFLAGS AND 00FCFFFFH);
///                 (* VM and RF bits are cleared in image stored on the stack *)
///             ELSE
///         FI;
///     ELSE IF 64-bit MODE (* In 64-bit Mode *)
///         IF OperandSize = 64
///             THEN
///                 push (RFLAGS AND 00000000_00FCFFFFH);
///                 (* VM and RF bits are cleared in image stored on the stack; *)
///             ELSE
///                 push (EFLAGS); (* Lower 16 bits only *)
///         FI;
///     ELSE (* In Virtual-8086 Mode with IOPL less than 3 *)
///         IF (CR4.VME = 0) OR (OperandSize = 32)
///             THEN #GP(0); (* Trap to virtual-8086 monitor *)
///             ELSE
///                 tempFLAGS = EFLAGS[15:0];
///                 tempFLAGS[9] = tempFLAGS[19];(* VIF replaces IF *)
///                 tempFlags[13:12] = 3; (* IOPL is set to 3 in image stored on the stack *)
///                 push (tempFLAGS);
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn pushf() -> &'static [IrStatement] {
    let set_sp = assign(b::sub(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let push = assign(o1(), d(rsp.clone()), o1_size());
    [set_sp, push].into()
}

/// # Pseudocode
/// ```text
/// IF (PE = 0) or (PE = 1 and ((VM = 0) or (VM = 1 and IOPL = 3)))
/// (* Real-Address Mode, Protected mode, or Virtual-8086 mode with IOPL equal to 3 *)
///     THEN
///         IF OperandSize = 32
///             THEN
///                 push (EFLAGS AND 00FCFFFFH);
///                 (* VM and RF bits are cleared in image stored on the stack *)
///             ELSE
///         FI;
///     ELSE IF 64-bit MODE (* In 64-bit Mode *)
///         IF OperandSize = 64
///             THEN
///                 push (RFLAGS AND 00000000_00FCFFFFH);
///                 (* VM and RF bits are cleared in image stored on the stack; *)
///             ELSE
///                 push (EFLAGS); (* Lower 16 bits only *)
///         FI;
///     ELSE (* In Virtual-8086 Mode with IOPL less than 3 *)
///         IF (CR4.VME = 0) OR (OperandSize = 32)
///             THEN #GP(0); (* Trap to virtual-8086 monitor *)
///             ELSE
///                 tempFLAGS = EFLAGS[15:0];
///                 tempFLAGS[9] = tempFLAGS[19];(* VIF replaces IF *)
///                 tempFlags[13:12] = 3; (* IOPL is set to 3 in image stored on the stack *)
///                 push (tempFLAGS);
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn pushfd() -> &'static [IrStatement] {
    let set_sp = assign(b::sub(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let push = assign(o1(), d(rsp.clone()), o1_size());
    [set_sp, push].into()
}

/// # Pseudocode
/// ```text
/// IF (PE = 0) or (PE = 1 and ((VM = 0) or (VM = 1 and IOPL = 3)))
/// (* Real-Address Mode, Protected mode, or Virtual-8086 mode with IOPL equal to 3 *)
///     THEN
///         IF OperandSize = 32
///             THEN
///                 push (EFLAGS AND 00FCFFFFH);
///                 (* VM and RF bits are cleared in image stored on the stack *)
///             ELSE
///         FI;
///     ELSE IF 64-bit MODE (* In 64-bit Mode *)
///         IF OperandSize = 64
///             THEN
///                 push (RFLAGS AND 00000000_00FCFFFFH);
///                 (* VM and RF bits are cleared in image stored on the stack; *)
///             ELSE
///                 push (EFLAGS); (* Lower 16 bits only *)
///         FI;
///     ELSE (* In Virtual-8086 Mode with IOPL less than 3 *)
///         IF (CR4.VME = 0) OR (OperandSize = 32)
///             THEN #GP(0); (* Trap to virtual-8086 monitor *)
///             ELSE
///                 tempFLAGS = EFLAGS[15:0];
///                 tempFLAGS[9] = tempFLAGS[19];(* VIF replaces IF *)
///                 tempFlags[13:12] = 3; (* IOPL is set to 3 in image stored on the stack *)
///                 push (tempFLAGS);
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn pushfq() -> &'static [IrStatement] {
    let set_sp = assign(b::sub(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let push = assign(o1(), d(rsp.clone()), o1_size());
    [set_sp, push].into()
}

/// # Pseudocode
/// ```text
/// PXOR (64-bit Operand)
/// DEST := DEST XOR SRC
/// PXOR (128-bit Legacy SSE Version)
/// DEST := DEST XOR SRC
/// DEST[MAXVL-1:128] (Unmodified)
/// VPXOR (VEX.128 Encoded Version)
/// DEST := SRC1 XOR SRC2
/// DEST[MAXVL-1:128] := 0
/// VPXOR (VEX.256 Encoded Version)
/// DEST := SRC1 XOR SRC2
/// DEST[MAXVL-1:256] := 0
/// VPXORD (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+31:i] := SRC1[i+31:i] BITWISE XOR SRC2[31:0]
///                     ELSE DEST[i+31:i] := SRC1[i+31:i] BITWISE XOR SRC2[i+31:i]
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[31:0] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VPXORQ (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN DEST[i+63:i] := SRC1[i+63:i] BITWISE XOR SRC2[63:0]
///                     ELSE DEST[i+63:i] := SRC1[i+63:i] BITWISE XOR SRC2[i+63:i]
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[63:0] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn pxor() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}
