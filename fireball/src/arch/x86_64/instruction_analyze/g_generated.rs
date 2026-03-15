use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// define affine_inverse_byte(tsrc2qw, src1byte, imm):
///     FOR i := 0 to 7:
///         * parity(x) = 1 if x has an odd number of 1s in it, and 0 otherwise.*
///         * inverse(x) is defined in the table above *
///         retbyte.bit[i] := parity(tsrc2qw.byte[7-i] AND inverse(src1byte)) XOR imm8.bit[i]
///     return retbyte
/// VGF2P8AFFINEINVQB dest, src1, src2, imm8 (EVEX Encoded Version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC2 is memory and EVEX.b==1:
///         tsrc2 := SRC2.qword[0]
///     ELSE:
///         tsrc2 := SRC2.qword[j]
/// FOR b := 0 to 7:
///     IF k1[j*8+b] OR *no writemask*:
///         FOR i := 0 to 7:
///             DEST.qword[j].byte[b] := affine_inverse_byte(tsrc2, SRC1.qword[j].byte[b], imm8)
///     ELSE IF *zeroing*:
///         DEST.qword[j].byte[b] := 0
///     *ELSE DEST.qword[j].byte[b] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VGF2P8AFFINEINVQB dest, src1, src2, imm8 (128b and 256b VEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256)
/// FOR j := 0 TO KL-1:
///     FOR b := 0 to 7:
///         DEST.qword[j].byte[b] := affine_inverse_byte(SRC2.qword[j], SRC1.qword[j].byte[b], imm8)
/// DEST[MAX_VL-1:VL] := 0
/// GF2P8AFFINEINVQB srcdest, src1, imm8 (128b SSE Encoded Version)
/// FOR j := 0 TO 1:
///     FOR b := 0 to 7:
///         SRCDEST.qword[j].byte[b] := affine_inverse_byte(SRC1.qword[j], SRCDEST.qword[j].byte[b], imm8)
/// ```
#[box_to_static_reference]
pub(super) fn gf2p8affineinvqb() -> &'static [IrStatement] {
    [exception("gf2p8affineinvqb")].into()
}

/// # Pseudocode
/// ```text
/// define parity(x):
///     t := 0
///                 // single bit
///     FOR i := 0 to 7:
///         t = t xor x.bit[i]
///     return t
/// define affine_byte(tsrc2qw, src1byte, imm):
///     FOR i := 0 to 7:
///         * parity(x) = 1 if x has an odd number of 1s in it, and 0 otherwise.*
///         retbyte.bit[i] := parity(tsrc2qw.byte[7-i] AND src1byte) XOR imm8.bit[i]
///     return retbyte
/// VGF2P8AFFINEQB dest, src1, src2, imm8 (EVEX Encoded Version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1:
///     IF SRC2 is memory and EVEX.b==1:
///         tsrc2 := SRC2.qword[0]
///     ELSE:
///         tsrc2 := SRC2.qword[j]
///     FOR b := 0 to 7:
///         IF k1[j*8+b] OR *no writemask*:
///             DEST.qword[j].byte[b] := affine_byte(tsrc2, SRC1.qword[j].byte[b], imm8)
///         ELSE IF *zeroing*:
///             DEST.qword[j].byte[b] := 0
///         *ELSE DEST.qword[j].byte[b] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VGF2P8AFFINEQB dest, src1, src2, imm8 (128b and 256b VEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256)
/// FOR j := 0 TO KL-1:
///     FOR b := 0 to 7:
///         DEST.qword[j].byte[b] := affine_byte(SRC2.qword[j], SRC1.qword[j].byte[b], imm8)
/// DEST[MAX_VL-1:VL] := 0
/// GF2P8AFFINEQB srcdest, src1, imm8 (128b SSE Encoded Version)
/// FOR j := 0 TO 1:
///     FOR b := 0 to 7:
///         SRCDEST.qword[j].byte[b] := affine_byte(SRC1.qword[j], SRCDEST.qword[j].byte[b], imm8)
/// ```
#[box_to_static_reference]
pub(super) fn gf2p8affineqb() -> &'static [IrStatement] {
    [exception("gf2p8affineqb")].into()
}

/// # Pseudocode
/// ```text
/// define gf2p8mul_byte(src1byte, src2byte):
///     tword := 0
///     FOR i := 0 to 7:
///         IF src2byte.bit[i]:
///             tword := tword XOR (src1byte<< i)
///         * carry out polynomial reduction by the characteristic polynomial p*
///     FOR i := 14 downto 8:
///         p := 0x11B << (i-8)
///                 *0x11B = 0000_0001_0001_1011 in binary*
///         IF tword.bit[i]:
///             tword := tword XOR p
/// return tword.byte[0]
/// VGF2P8MULB dest, src1, src2 (EVEX Encoded Version)
/// (KL, VL) = (16, 128), (32, 256), (64, 512)
/// FOR j := 0 TO KL-1:
///     IF k1[j] OR *no writemask*:
///         DEST.byte[j] := gf2p8mul_byte(SRC1.byte[j], SRC2.byte[j])
///     ELSE iF *zeroing*:
///         DEST.byte[j] := 0
///     * ELSE DEST.byte[j] remains unchanged*
/// DEST[MAX_VL-1:VL] := 0
/// VGF2P8MULB dest, src1, src2 (128b and 256b VEX Encoded Versions)
/// (KL, VL) = (16, 128), (32, 256)
/// FOR j := 0 TO KL-1:
///     DEST.byte[j] := gf2p8mul_byte(SRC1.byte[j], SRC2.byte[j])
/// DEST[MAX_VL-1:VL] := 0
/// GF2P8MULB srcdest, src1 (128b SSE Encoded Version)
/// FOR j := 0 TO 15:
///     SRCDEST.byte[j] :=gf2p8mul_byte(SRCDEST.byte[j], SRC1.byte[j])
/// ```
#[box_to_static_reference]
pub(super) fn gf2p8mulb() -> &'static [IrStatement] {
    [exception("gf2p8mulb")].into()
}
