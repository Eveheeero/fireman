use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF (MASK[7] = 1)
///     THEN DEST[DI/EDI] := SRC[7:0] ELSE (* Memory location unchanged *); FI;
/// IF (MASK[15] = 1)
///     THEN DEST[DI/EDI +1] := SRC[15:8] ELSE (* Memory location unchanged *); FI;
///     (* Repeat operation for 3rd through 14th bytes in source operand *)
/// IF (MASK[127] = 1)
///     THEN DEST[DI/EDI +15] := SRC[127:120] ELSE (* Memory location unchanged *); FI;
/// ```
#[box_to_static_reference]
pub(super) fn maskmovdqu() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (MASK[7] = 1)
///     THEN DEST[DI/EDI] := SRC[7:0] ELSE (* Memory location unchanged *); FI;
/// IF (MASK[15] = 1)
///     THEN DEST[DI/EDI +1] := SRC[15:8] ELSE (* Memory location unchanged *); FI;
///     (* Repeat operation for 3rd through 6th bytes in source operand *)
/// IF (MASK[63] = 1)
///     THEN DEST[DI/EDI +15] := SRC[63:56] ELSE (* Memory location unchanged *); FI;
/// ```
#[box_to_static_reference]
pub(super) fn maskmovq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MAX(SRC1, SRC2)
/// {
///     IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;
///         ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC1 > SRC2) THEN DEST := SRC1;
///         ELSE DEST := SRC2;
///     FI;
/// }
/// VMAXPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     DEST[i+63:i] := MAX(SRC1[i+63:i], SRC2[63:0])
///                 ELSE
///                     DEST[i+63:i] := MAX(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE  DEST[i+63:i] := 0
///                         ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMAXPD (VEX.256 Encoded Version)
/// DEST[63:0] := MAX(SRC1[63:0], SRC2[63:0])
/// DEST[127:64] := MAX(SRC1[127:64], SRC2[127:64])
/// DEST[191:128] := MAX(SRC1[191:128], SRC2[191:128])
/// DEST[255:192] := MAX(SRC1[255:192], SRC2[255:192])
/// DEST[MAXVL-1:256] := 0
/// VMAXPD (VEX.128 Encoded Version)
/// DEST[63:0] := MAX(SRC1[63:0], SRC2[63:0])
/// DEST[127:64] := MAX(SRC1[127:64], SRC2[127:64])
/// DEST[MAXVL-1:128] := 0
/// MAXPD (128-bit Legacy SSE Version)
/// DEST[63:0] := MAX(DEST[63:0], SRC[63:0])
/// DEST[127:64] := MAX(DEST[127:64], SRC[127:64])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn maxpd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MAX(SRC1, SRC2)
/// {
///     IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;
///         ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC1 > SRC2) THEN DEST := SRC1;
///         ELSE DEST := SRC2;
///     FI;
/// }
/// VMAXPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     DEST[i+31:i] := MAX(SRC1[i+31:i], SRC2[31:0])
///                 ELSE
///                     DEST[i+31:i] := MAX(SRC1[i+31:i], SRC2[i+31:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE  DEST[i+31:i] := 0
///                         ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMAXPS (VEX.256 Encoded Version)
/// DEST[31:0] := MAX(SRC1[31:0], SRC2[31:0])
/// DEST[63:32] := MAX(SRC1[63:32], SRC2[63:32])
/// DEST[95:64] := MAX(SRC1[95:64], SRC2[95:64])
/// DEST[127:96] := MAX(SRC1[127:96], SRC2[127:96])
/// DEST[159:128] := MAX(SRC1[159:128], SRC2[159:128])
/// DEST[191:160] := MAX(SRC1[191:160], SRC2[191:160])
/// DEST[223:192] := MAX(SRC1[223:192], SRC2[223:192])
/// DEST[255:224] := MAX(SRC1[255:224], SRC2[255:224])
/// DEST[MAXVL-1:256] := 0
/// VMAXPS (VEX.128 Encoded Version)
/// DEST[31:0] := MAX(SRC1[31:0], SRC2[31:0])
/// DEST[63:32] := MAX(SRC1[63:32], SRC2[63:32])
/// DEST[95:64] := MAX(SRC1[95:64], SRC2[95:64])
/// DEST[127:96] := MAX(SRC1[127:96], SRC2[127:96])
/// DEST[MAXVL-1:128] := 0
/// MAXPS (128-bit Legacy SSE Version)
/// DEST[31:0] := MAX(DEST[31:0], SRC[31:0])
/// DEST[63:32] := MAX(DEST[63:32], SRC[63:32])
/// DEST[95:64] := MAX(DEST[95:64], SRC[95:64])
/// DEST[127:96] := MAX(DEST[127:96], SRC[127:96])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn maxps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MAX(SRC1, SRC2)
/// {
///     IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;
///         ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC1 > SRC2) THEN DEST := SRC1;
///         ELSE DEST := SRC2;
///     FI;
/// }
/// VMAXSD (EVEX Encoded Version)
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := MAX(SRC1[63:0], SRC2[63:0])
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
/// VMAXSD (VEX.128 Encoded Version)
/// DEST[63:0] := MAX(SRC1[63:0], SRC2[63:0])
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// MAXSD (128-bit Legacy SSE Version)
/// DEST[63:0] := MAX(DEST[63:0], SRC[63:0])
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn maxsd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MAX(SRC1, SRC2)
/// {
///     IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;
///         ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC1 > SRC2) THEN DEST := SRC1;
///         ELSE DEST := SRC2;
///     FI;
/// }
/// VMAXSS (EVEX Encoded Version)
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := MAX(SRC1[31:0], SRC2[31:0])
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
/// VMAXSS (VEX.128 Encoded Version)
/// DEST[31:0] := MAX(SRC1[31:0], SRC2[31:0])
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// MAXSS (128-bit Legacy SSE Version)
/// DEST[31:0] := MAX(DEST[31:0], SRC[31:0])
/// DEST[MAXVL-1:32] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn maxss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o3(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Wait_On_Following_Loads_And_Stores_Until(preceding_loads_and_stores_globally_visible);
/// ```
#[box_to_static_reference]
pub(super) fn mfence() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// MIN(SRC1, SRC2)
/// {
///     IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;
///         ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC1 < SRC2) THEN DEST := SRC1;
///         ELSE DEST := SRC2;
///     FI;
/// }
/// VMINPD (EVEX Encoded Version)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     DEST[i+63:i] := MIN(SRC1[i+63:i], SRC2[63:0])
///                 ELSE
///                     DEST[i+63:i] := MIN(SRC1[i+63:i], SRC2[i+63:i])
///             FI;
///         ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+63:i] remains unchanged*
///                 ELSE  DEST[i+63:i] := 0
///                         ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMINPD (VEX.256 Encoded Version)
/// DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])
/// DEST[127:64] := MIN(SRC1[127:64], SRC2[127:64])
/// DEST[191:128] := MIN(SRC1[191:128], SRC2[191:128])
/// DEST[255:192] := MIN(SRC1[255:192], SRC2[255:192])
/// VMINPD (VEX.128 Encoded Version)
/// DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])
/// DEST[127:64] := MIN(SRC1[127:64], SRC2[127:64])
/// DEST[MAXVL-1:128] := 0
/// MINPD (128-bit Legacy SSE Version)
/// DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])
/// DEST[127:64] := MIN(SRC1[127:64], SRC2[127:64])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn minpd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MIN(SRC1, SRC2)
/// {
///     IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;
///         ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;
///         ELSE IF (SRC1 < SRC2) THEN DEST := SRC1;
///         ELSE DEST := SRC2;
///     FI;
/// }
/// VMINPS (EVEX Encoded Version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                 THEN
///                     DEST[i+31:i] := MIN(SRC1[i+31:i], SRC2[31:0])
///                 ELSE
///                     DEST[i+31:i] := MIN(SRC1[i+31:i], SRC2[i+31:i])
///             FI;
///             ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[i+31:i] remains unchanged*
///                 ELSE  DEST[i+31:i] := 0
///                         ; zeroing-masking
///             FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMINPS (VEX.256 Encoded Version)
/// DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])
/// DEST[63:32] := MIN(SRC1[63:32], SRC2[63:32])
/// DEST[95:64] := MIN(SRC1[95:64], SRC2[95:64])
/// DEST[127:96] := MIN(SRC1[127:96], SRC2[127:96])
/// DEST[159:128] := MIN(SRC1[159:128], SRC2[159:128])
/// DEST[191:160] := MIN(SRC1[191:160], SRC2[191:160])
/// DEST[223:192] := MIN(SRC1[223:192], SRC2[223:192])
/// DEST[255:224] := MIN(SRC1[255:224], SRC2[255:224])
/// VMINPS (VEX.128 Encoded Version)
/// DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])
/// DEST[63:32] := MIN(SRC1[63:32], SRC2[63:32])
/// DEST[95:64] := MIN(SRC1[95:64], SRC2[95:64])
/// DEST[127:96] := MIN(SRC1[127:96], SRC2[127:96])
/// DEST[MAXVL-1:128] := 0
/// MINPS (128-bit Legacy SSE Version)
/// DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])
/// DEST[63:32] := MIN(SRC1[63:32], SRC2[63:32])
/// DEST[95:64] := MIN(SRC1[95:64], SRC2[95:64])
/// DEST[127:96] := MIN(SRC1[127:96], SRC2[127:96])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn minps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MIN(SRC1, SRC2)
/// {
///     IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;
///             ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;
///             ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;
///             ELSE IF (SRC1 < SRC2) THEN DEST := SRC1;
///             ELSE DEST := SRC2;
///     FI;
/// }
/// MINSD (EVEX Encoded Version)
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])
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
/// MINSD (VEX.128 Encoded Version)
/// DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// MINSD (128-bit Legacy SSE Version)
/// DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn minsd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MIN(SRC1, SRC2)
/// {
///     IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;
///             ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;
///             ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;
///             ELSE IF (SRC1 < SRC2) THEN DEST := SRC1;
///             ELSE DEST := SRC2;
///     FI;
/// }
/// MINSS (EVEX Encoded Version)
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])
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
/// VMINSS (VEX.128 Encoded Version)
/// DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// MINSS (128-bit Legacy SSE Version)
/// DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn minss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MONITOR sets up an address range for the monitor hardware using the content of EAX (RAX in 64-bit mode) as an effective address
/// and puts the monitor hardware in armed state. Always use memory of the write-back caching type. A store to the specified address
/// range will trigger the monitor hardware. The content of ECX and EDX are used to communicate other information to the monitor
/// hardware.
/// ```
#[box_to_static_reference]
pub(super) fn monitor() -> &'static [IrStatement] {
    [exception("monitor")].into()
}

/// # Pseudocode
/// ```text
/// IF ((DE = 1) and (SRC or DEST = DR4 or DR5))
///     THEN
///         #UD;
///     ELSE
///         DEST := SRC;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn mov() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVAPD (EVEX Encoded Versions, Register-Copy Form)
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
/// VMOVAPD (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[i+63:i]
///         ELSE
///         ELSE *DEST[i+63:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVAPD (EVEX Encoded Versions, Load-Form)
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
/// VMOVAPD (VEX.256 Encoded Version, Load - and Register Copy)
/// DEST[255:0] := SRC[255:0]
/// DEST[MAXVL-1:256] := 0
/// VMOVAPD (VEX.256 Encoded Version, Store-Form)
/// DEST[255:0] := SRC[255:0]
/// VMOVAPD (VEX.128 Encoded Version, Load - and Register Copy)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] := 0
/// MOVAPD (128-bit Load- and Register-Copy- Form Legacy SSE Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// (V)MOVAPD (128-bit Store-Form Version)
/// DEST[127:0] := SRC[127:0]
/// ```
#[box_to_static_reference]
pub(super) fn movapd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVAPS (EVEX Encoded Versions, Register-Copy Form)
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
/// VMOVAPS (EVEX Encoded Versions, Store Form)
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
/// VMOVAPS (EVEX Encoded Versions, Load Form)
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
/// VMOVAPS (VEX.256 Encoded Version, Load - and Register Copy)
/// DEST[255:0] := SRC[255:0]
/// DEST[MAXVL-1:256] := 0
/// VMOVAPS (VEX.256 Encoded Version, Store-Form)
/// DEST[255:0] := SRC[255:0]
/// VMOVAPS (VEX.128 Encoded Version, Load - and Register Copy)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] := 0
/// MOVAPS (128-bit Load- and Register-Copy- Form Legacy SSE Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// (V)MOVAPS (128-bit Store-Form Version)
/// DEST[127:0] := SRC[127:0]
/// ```
#[box_to_static_reference]
pub(super) fn movaps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// TEMP := SRC
/// IF ( OperandSize = 16)
///     THEN
///         DEST[7:0] := TEMP[15:8];
///         DEST[15:8] := TEMP[7:0];
///     ELES IF( OperandSize = 32)
///         DEST[7:0] := TEMP[31:24];
///         DEST[15:8] := TEMP[23:16];
///         DEST[23:16] := TEMP[15:8];
///         DEST[31:23] := TEMP[7:0];
///     ELSE IF ( OperandSize = 64)
///         DEST[7:0] := TEMP[63:56];
///         DEST[15:8] := TEMP[55:48];
///         DEST[23:16] := TEMP[47:40];
///         DEST[31:24] := TEMP[39:32];
///         DEST[39:32] := TEMP[31:24];
///         DEST[47:40] := TEMP[23:16];
///         DEST[55:48] := TEMP[15:8];
///         DEST[63:56] := TEMP[7:0];
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn movbe() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MOVD (When Destination Operand is an MMX Technology Register)
///     DEST[31:0] := SRC;
///     DEST[63:32] := 00000000H;
/// MOVD (When Destination Operand is an XMM Register)
///     DEST[31:0] := SRC;
///     DEST[127:32] := 000000000000000000000000H;
///     DEST[MAXVL-1:128] (Unmodified)
/// MOVD (When Source Operand is an MMX Technology or XMM Register)
///     DEST := SRC[31:0];
/// VMOVD (VEX-Encoded Version when Destination is an XMM Register)
///     DEST[31:0] := SRC[31:0]
///     DEST[MAXVL-1:32] := 0
/// MOVQ (When Destination Operand is an XMM Register)
///     DEST[63:0] := SRC[63:0];
///     DEST[127:64] := 0000000000000000H;
/// MOVQ (When Destination Operand is r/m64)
///     DEST[63:0] := SRC[63:0];
/// MOVQ (When Source Operand is an XMM Register or r/m64)
///     DEST := SRC[63:0];
/// VMOVQ (VEX-Encoded Version When Destination is an XMM Register)
///     DEST[63:0] := SRC[63:0]
///     DEST[MAXVL-1:64] := 0
/// VMOVD (EVEX-Encoded Version When Destination is an XMM Register)
/// DEST[31:0] := SRC[31:0]
/// DEST[MAXVL-1:32] := 0
/// VMOVQ (EVEX-Encoded Version When Destination is an XMM Register)
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] := 0
/// ```
#[box_to_static_reference]
pub(super) fn movd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVDDUP (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// TMP_SRC[63:0] := SRC[63:0]
/// TMP_SRC[127:64] := SRC[63:0]
/// IF VL >= 256
///     TMP_SRC[191:128] := SRC[191:128]
///     TMP_SRC[255:192] := SRC[191:128]
/// FI;
/// IF VL >= 512
///     TMP_SRC[319:256] := SRC[319:256]
///     TMP_SRC[383:320] := SRC[319:256]
///     TMP_SRC[477:384] := SRC[477:384]
///     TMP_SRC[511:484] := SRC[477:384]
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := TMP_SRC[i+63:i]
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] := 0
///                             ; zeroing-masking
///                 FI
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VMOVDDUP (VEX.256 Encoded Version)
/// DEST[63:0] := SRC[63:0]
/// DEST[127:64] := SRC[63:0]
/// DEST[191:128] := SRC[191:128]
/// DEST[255:192] := SRC[191:128]
/// DEST[MAXVL-1:256] := 0
/// VMOVDDUP (VEX.128 Encoded Version)
/// DEST[63:0] := SRC[63:0]
/// DEST[127:64] := SRC[63:0]
/// DEST[MAXVL-1:128] := 0
/// MOVDDUP (128-bit Legacy SSE Version)
/// DEST[63:0] := SRC[63:0]
/// DEST[127:64] := SRC[63:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn movddup() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// ```
#[box_to_static_reference]
pub(super) fn movdir64b() -> &'static [IrStatement] {
    let stmt_0 = assign(o2(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// ```
#[box_to_static_reference]
pub(super) fn movdiri() -> &'static [IrStatement] {
    let stmt_0 = assign(o2(), o1(), o1_size());
    [stmt_0].into()
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
pub(super) fn movdq16() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC[63:0];
/// ```
#[box_to_static_reference]
pub(super) fn movdq2q() -> &'static [IrStatement] {
    let stmt_0 = assign(o2(), o1(), o1_size());
    [stmt_0].into()
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
pub(super) fn movdq32() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
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
pub(super) fn movdq64() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
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
pub(super) fn movdqa() -> &'static [IrStatement] {
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
pub(super) fn movdqu() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MOVHLPS (128-bit Two-Argument Form)
/// DEST[63:0] := SRC[127:64]
/// DEST[MAXVL-1:64] (Unmodified)
/// VMOVHLPS (128-bit Three-Argument Form - VEX & EVEX)
/// DEST[63:0] := SRC2[127:64]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn movhlps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MOVHPD (128-bit Legacy SSE Load)
/// DEST[63:0] (Unmodified)
/// DEST[127:64] := SRC[63:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// VMOVHPD (VEX.128 & EVEX Encoded Load)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[MAXVL-1:128] := 0
/// VMOVHPD (Store)
/// DEST[63:0] := SRC[127:64]
/// ```
#[box_to_static_reference]
pub(super) fn movhpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MOVHPS (128-bit Legacy SSE Load)
/// DEST[63:0] (Unmodified)
/// DEST[127:64] := SRC[63:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// VMOVHPS (VEX.128 and EVEX Encoded Load)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[MAXVL-1:128] := 0
/// VMOVHPS (Store)
/// DEST[63:0] := SRC[127:64]
/// ```
#[box_to_static_reference]
pub(super) fn movhps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MOVLHPS (128-bit Two-Argument Form)
/// DEST[63:0] (Unmodified)
/// DEST[127:64] := SRC[63:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// VMOVLHPS (128-bit Three-Argument Form - VEX & EVEX)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn movlhps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MOVLPD (128-bit Legacy SSE Load)
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] (Unmodified)
/// VMOVLPD (VEX.128 & EVEX Encoded Load)
/// DEST[63:0] := SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VMOVLPD (Store)
/// DEST[63:0] := SRC[63:0]
/// ```
#[box_to_static_reference]
pub(super) fn movlpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MOVLPS (128-bit Legacy SSE Load)
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] (Unmodified)
/// VMOVLPS (VEX.128 & EVEX Encoded Load)
/// DEST[63:0] := SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VMOVLPS (Store)
/// DEST[63:0] := SRC[63:0]
/// ```
#[box_to_static_reference]
pub(super) fn movlps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (V)MOVMSKPD (128-bit Versions)
/// DEST[0] := SRC[63]
/// DEST[1] := SRC[127]
/// IF DEST = r32
///     THEN DEST[31:2] := 0;
///     ELSE DEST[63:2] := 0;
/// FI
/// VMOVMSKPD (VEX.256 Encoded Version)
/// DEST[0] := SRC[63]
/// DEST[1] := SRC[127]
/// DEST[2] := SRC[191]
/// DEST[3] := SRC[255]
/// IF DEST = r32
///     THEN DEST[31:4] := 0;
///     ELSE DEST[63:4] := 0;
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn movmskpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[0] := SRC[31];
/// DEST[1] := SRC[63];
/// DEST[2] := SRC[95];
/// DEST[3] := SRC[127];
/// IF DEST = r32
///     THEN DEST[31:4] := ZeroExtend;
///     ELSE DEST[63:4] := ZeroExtend;
/// FI;
/// 1.ModRM.MOD = 011B required
/// (V)MOVMSKPS (128-bit version)
/// DEST[0] := SRC[31]
/// DEST[1] := SRC[63]
/// DEST[2] := SRC[95]
/// DEST[3] := SRC[127]
/// IF DEST = r32
///     THEN DEST[31:4] := 0;
///     ELSE DEST[63:4] := 0;
/// FI
/// VMOVMSKPS (VEX.256 encoded version)
/// DEST[0] := SRC[31]
/// DEST[1] := SRC[63]
/// DEST[2] := SRC[95]
/// DEST[3] := SRC[127]
/// DEST[4] := SRC[159]
/// DEST[5] := SRC[191]
/// DEST[6] := SRC[223]
/// DEST[7] := SRC[255]
/// IF DEST = r32
///     THEN DEST[31:8] := 0;
///     ELSE DEST[63:8] := 0;
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn movmskps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVNTDQ(EVEX Encoded Versions)
/// VL = 128, 256, 512
/// DEST[VL-1:0] := SRC[VL-1:0]
/// DEST[MAXVL-1:VL] := 0
/// 1.ModRM.MOD != 011B
/// MOVNTDQ (Legacy and VEX Versions)
/// DEST := SRC
/// ```
#[box_to_static_reference]
pub(super) fn movntdq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// MOVNTDQA (128bit- Legacy SSE Form)
/// DEST := SRC
/// DEST[MAXVL-1:128] (Unmodified)
/// VMOVNTDQA (VEX.128 and EVEX.128 Encoded Form)
/// DEST := SRC
/// DEST[MAXVL-1:128] := 0
/// VMOVNTDQA (VEX.256 and EVEX.256 Encoded Forms)
/// DEST[255:0] := SRC[255:0]
/// DEST[MAXVL-1:256] := 0
/// VMOVNTDQA (EVEX.512 Encoded Form)
/// DEST[511:0] := SRC[511:0]
/// DEST[MAXVL-1:512] := 0
/// ```
#[box_to_static_reference]
pub(super) fn movntdqa() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// ```
#[box_to_static_reference]
pub(super) fn movnti() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVNTPD (EVEX Encoded Versions)
/// VL = 128, 256, 512
/// DEST[VL-1:0] := SRC[VL-1:0]
/// DEST[MAXVL-1:VL] := 0
/// 1.ModRM.MOD != 011B
/// MOVNTPD (Legacy and VEX Versions)
/// DEST := SRC
/// ```
#[box_to_static_reference]
pub(super) fn movntpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVNTPS (EVEX Encoded Versions)
/// VL = 128, 256, 512
/// DEST[VL-1:0] := SRC[VL-1:0]
/// DEST[MAXVL-1:VL] := 0
/// 1.ModRM.MOD != 011B
/// MOVNTPS
/// DEST := SRC
/// ```
#[box_to_static_reference]
pub(super) fn movntps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// ```
#[box_to_static_reference]
pub(super) fn movntq() -> &'static [IrStatement] {
    let stmt_0 = assign(o2(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// MOVQ Instruction When Operating on MMX Technology Registers and Memory Locations
///     DEST := SRC;
/// MOVQ Instruction When Source and Destination Operands are XMM Registers
///     DEST[63:0] := SRC[63:0];
///     DEST[127:64] := 0000000000000000H;
/// MOVQ Instruction When Source Operand is XMM Register and Destination
/// operand is memory location:
///     DEST := SRC[63:0];
/// MOVQ Instruction When Source Operand is Memory Location and Destination
/// operand is XMM register:
///     DEST[63:0] := SRC;
///     DEST[127:64] := 0000000000000000H;
/// VMOVQ (VEX.128.F3.0F 7E) With XMM Register Source and Destination
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] := 0
/// VMOVQ (VEX.128.66.0F D6) With XMM Register Source and Destination
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] := 0
/// VMOVQ (7E - EVEX Encoded Version) With XMM Register Source and Destination
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] := 0
/// VMOVQ (D6 - EVEX Encoded Version) With XMM Register Source and Destination
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] := 0
/// VMOVQ (7E) With Memory Source
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] := 0
/// VMOVQ (7E - EVEX Encoded Version) With Memory Source
/// DEST[63:0] := SRC[63:0]
/// DEST[:MAXVL-1:64] := 0
/// VMOVQ (D6) With Memory DEST
/// DEST[63:0] := SRC2[63:0]
/// ```
#[box_to_static_reference]
pub(super) fn movq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST[63:0] := SRC[63:0];
/// DEST[127:64] := 00000000000000000H;
/// ```
#[box_to_static_reference]
pub(super) fn movq2dq() -> &'static [IrStatement] {
    let stmt_0 = assign(o2(), o1(), o1_size());
    let stmt_1 = assign(c(0x00000000000000000), o1(), o1_size());
    [stmt_0, stmt_1].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// Non-64-bit Mode:
/// IF (Byte move)
///     THEN IF DF = 0
///         THEN
///             (E)SI := (E)SI + 1;
///             (E)DI := (E)DI + 1;
///         ELSE
///             (E)SI := (E)SI - 1;
///             (E)DI := (E)DI - 1;
///         FI;
///     ELSE IF (Word move)
///         THEN IF DF = 0
///             (E)SI := (E)SI + 2;
///             (E)DI := (E)DI + 2;
///             FI;
///         ELSE
///             (E)SI := (E)SI - 2;
///             (E)DI := (E)DI - 2;
///         FI;
///     ELSE IF (Doubleword move)
///         THEN IF DF = 0
///             (E)SI := (E)SI + 4;
///             (E)DI := (E)DI + 4;
///             FI;
///         ELSE
///             (E)SI := (E)SI - 4;
///             (E)DI := (E)DI - 4;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte move)
///     THEN IF DF = 0
///         THEN
///             (R|E)SI := (R|E)SI + 1;
///             (R|E)DI := (R|E)DI + 1;
///         ELSE
///             (R|E)SI := (R|E)SI - 1;
///             (R|E)DI := (R|E)DI - 1;
///         FI;
///     ELSE IF (Word move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 2;
///             (R|E)DI := (R|E)DI + 2;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 2;
///             (R|E)DI := (R|E)DI - 2;
///         FI;
///     ELSE IF (Doubleword move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 4;
///             (R|E)DI := (R|E)DI + 4;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 4;
///             (R|E)DI := (R|E)DI - 4;
///         FI;
///     ELSE IF (Quadword move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 8;
///             (R|E)DI := (R|E)DI + 8;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 8;
///             (R|E)DI := (R|E)DI - 8;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn movs() -> &'static [IrStatement] {
    let mov = assign(d(rsi.clone()), d(rdi.clone()), size_architecture());
    [mov].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// Non-64-bit Mode:
/// IF (Byte move)
///     THEN IF DF = 0
///         THEN
///             (E)SI := (E)SI + 1;
///             (E)DI := (E)DI + 1;
///         ELSE
///             (E)SI := (E)SI - 1;
///             (E)DI := (E)DI - 1;
///         FI;
///     ELSE IF (Word move)
///         THEN IF DF = 0
///             (E)SI := (E)SI + 2;
///             (E)DI := (E)DI + 2;
///             FI;
///         ELSE
///             (E)SI := (E)SI - 2;
///             (E)DI := (E)DI - 2;
///         FI;
///     ELSE IF (Doubleword move)
///         THEN IF DF = 0
///             (E)SI := (E)SI + 4;
///             (E)DI := (E)DI + 4;
///             FI;
///         ELSE
///             (E)SI := (E)SI - 4;
///             (E)DI := (E)DI - 4;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte move)
///     THEN IF DF = 0
///         THEN
///             (R|E)SI := (R|E)SI + 1;
///             (R|E)DI := (R|E)DI + 1;
///         ELSE
///             (R|E)SI := (R|E)SI - 1;
///             (R|E)DI := (R|E)DI - 1;
///         FI;
///     ELSE IF (Word move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 2;
///             (R|E)DI := (R|E)DI + 2;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 2;
///             (R|E)DI := (R|E)DI - 2;
///         FI;
///     ELSE IF (Doubleword move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 4;
///             (R|E)DI := (R|E)DI + 4;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 4;
///             (R|E)DI := (R|E)DI - 4;
///         FI;
///     ELSE IF (Quadword move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 8;
///             (R|E)DI := (R|E)DI + 8;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 8;
///             (R|E)DI := (R|E)DI - 8;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn movsb() -> &'static [IrStatement] {
    let mov = assign(d(rsi.clone()), d(rdi.clone()), size_result_byte(c(1)));
    [mov].into()
}

/// # Pseudocode
/// ```text
/// VMOVSD (EVEX.LLIG.F2.0F 10 /r: VMOVSD xmm1, m64 With Support for 32 Registers)
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := SRC[63:0]
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///             FI;
/// FI;
/// DEST[MAXVL-1:64] := 0
/// VMOVSD (EVEX.LLIG.F2.0F 11 /r: VMOVSD m64, xmm1 With Support for 32 Registers)
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := SRC[63:0]
///     ELSE*DEST[63:0] remains unchanged*
///                             ; merging-masking
/// FI;
/// VMOVSD (EVEX.LLIG.F2.0F 11 /r: VMOVSD xmm1, xmm2, xmm3)
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := SRC2[63:0]
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
/// MOVSD (128-bit Legacy SSE Version: MOVSD xmm1, xmm2)
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] (Unmodified)
/// VMOVSD (VEX.128.F2.0F 11 /r: VMOVSD xmm1, xmm2, xmm3)
/// DEST[63:0] := SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VMOVSD (VEX.128.F2.0F 10 /r: VMOVSD xmm1, xmm2, xmm3)
/// DEST[63:0] := SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VMOVSD (VEX.128.F2.0F 10 /r: VMOVSD xmm1, m64)
/// DEST[63:0] := SRC[63:0]
/// DEST[MAXVL-1:64] := 0
/// MOVSD/VMOVSD (128-bit Versions: MOVSD m64, xmm1 or VMOVSD m64, xmm1)
/// DEST[63:0] := SRC[63:0]
/// MOVSD (128-bit Legacy SSE Version: MOVSD xmm1, m64)
/// DEST[63:0] := SRC[63:0]
/// DEST[127:64] := 0
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn movsd() -> &'static [IrStatement] {
    let mov = assign(d(rsi.clone()), d(rdi.clone()), size_result_byte(c(4)));
    [mov].into()
}

/// # Pseudocode
/// ```text
/// VMOVSHDUP (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// TMP_SRC[31:0] := SRC[63:32]
/// TMP_SRC[63:32] := SRC[63:32]
/// TMP_SRC[95:64] := SRC[127:96]
/// TMP_SRC[127:96] := SRC[127:96]
/// IF VL >= 256
///     TMP_SRC[159:128] := SRC[191:160]
///     TMP_SRC[191:160] := SRC[191:160]
///     TMP_SRC[223:192] := SRC[255:224]
///     TMP_SRC[255:224] := SRC[255:224]
/// FI;
/// IF VL >= 512
///     TMP_SRC[287:256] := SRC[319:288]
///     TMP_SRC[319:288] := SRC[319:288]
///     TMP_SRC[351:320] := SRC[383:352]
///     TMP_SRC[383:352] := SRC[383:352]
///     TMP_SRC[415:384] := SRC[447:416]
///     TMP_SRC[447:416] := SRC[447:416]
///     TMP_SRC[479:448] := SRC[511:480]
///     TMP_SRC[511:480] := SRC[511:480]
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_SRC[i+31:i]
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
/// VMOVSHDUP (VEX.256 Encoded Version)
/// DEST[31:0] := SRC[63:32]
/// DEST[63:32] := SRC[63:32]
/// DEST[95:64] := SRC[127:96]
/// DEST[127:96] := SRC[127:96]
/// DEST[159:128] := SRC[191:160]
/// DEST[191:160] := SRC[191:160]
/// DEST[223:192] := SRC[255:224]
/// DEST[255:224] := SRC[255:224]
/// DEST[MAXVL-1:256] := 0
/// VMOVSHDUP (VEX.128 Encoded Version)
/// DEST[31:0] := SRC[63:32]
/// DEST[63:32] := SRC[63:32]
/// DEST[95:64] := SRC[127:96]
/// DEST[127:96] := SRC[127:96]
/// MOVSHDUP (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC[63:32]
/// DEST[63:32] := SRC[63:32]
/// DEST[95:64] := SRC[127:96]
/// DEST[127:96] := SRC[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn movshdup() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVSLDUP (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// TMP_SRC[31:0] := SRC[31:0]
/// TMP_SRC[63:32] := SRC[31:0]
/// TMP_SRC[95:64] := SRC[95:64]
/// TMP_SRC[127:96] := SRC[95:64]
/// IF VL >= 256
///     TMP_SRC[159:128] := SRC[159:128]
///     TMP_SRC[191:160] := SRC[159:128]
///     TMP_SRC[223:192] := SRC[223:192]
///     TMP_SRC[255:224] := SRC[223:192]
/// FI;
/// IF VL >= 512
///     TMP_SRC[287:256] := SRC[287:256]
///     TMP_SRC[319:288] := SRC[287:256]
///     TMP_SRC[351:320] := SRC[351:320]
///     TMP_SRC[383:352] := SRC[351:320]
///     TMP_SRC[415:384] := SRC[415:384]
///     TMP_SRC[447:416] := SRC[415:384]
///     TMP_SRC[479:448] := SRC[479:448]
///     TMP_SRC[511:480] := SRC[479:448]
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := TMP_SRC[i+31:i]
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
/// VMOVSLDUP (VEX.256 Encoded Version)
/// DEST[31:0] := SRC[31:0]
/// DEST[63:32] := SRC[31:0]
/// DEST[95:64] := SRC[95:64]
/// DEST[127:96] := SRC[95:64]
/// DEST[159:128] := SRC[159:128]
/// DEST[191:160] := SRC[159:128]
/// DEST[223:192] := SRC[223:192]
/// DEST[255:224] := SRC[223:192]
/// DEST[MAXVL-1:256] := 0
/// VMOVSLDUP (VEX.128 Encoded Version)
/// DEST[31:0] := SRC[31:0]
/// DEST[63:32] := SRC[31:0]
/// DEST[95:64] := SRC[95:64]
/// DEST[127:96] := SRC[95:64]
/// MOVSLDUP (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC[31:0]
/// DEST[63:32] := SRC[31:0]
/// DEST[95:64] := SRC[95:64]
/// DEST[127:96] := SRC[95:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn movsldup() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// Non-64-bit Mode:
/// IF (Byte move)
///     THEN IF DF = 0
///         THEN
///             (E)SI := (E)SI + 1;
///             (E)DI := (E)DI + 1;
///         ELSE
///             (E)SI := (E)SI - 1;
///             (E)DI := (E)DI - 1;
///         FI;
///     ELSE IF (Word move)
///         THEN IF DF = 0
///             (E)SI := (E)SI + 2;
///             (E)DI := (E)DI + 2;
///             FI;
///         ELSE
///             (E)SI := (E)SI - 2;
///             (E)DI := (E)DI - 2;
///         FI;
///     ELSE IF (Doubleword move)
///         THEN IF DF = 0
///             (E)SI := (E)SI + 4;
///             (E)DI := (E)DI + 4;
///             FI;
///         ELSE
///             (E)SI := (E)SI - 4;
///             (E)DI := (E)DI - 4;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte move)
///     THEN IF DF = 0
///         THEN
///             (R|E)SI := (R|E)SI + 1;
///             (R|E)DI := (R|E)DI + 1;
///         ELSE
///             (R|E)SI := (R|E)SI - 1;
///             (R|E)DI := (R|E)DI - 1;
///         FI;
///     ELSE IF (Word move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 2;
///             (R|E)DI := (R|E)DI + 2;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 2;
///             (R|E)DI := (R|E)DI - 2;
///         FI;
///     ELSE IF (Doubleword move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 4;
///             (R|E)DI := (R|E)DI + 4;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 4;
///             (R|E)DI := (R|E)DI - 4;
///         FI;
///     ELSE IF (Quadword move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 8;
///             (R|E)DI := (R|E)DI + 8;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 8;
///             (R|E)DI := (R|E)DI - 8;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn movsq() -> &'static [IrStatement] {
    let mov = assign(d(rsi.clone()), d(rdi.clone()), size_result_byte(c(8)));
    [mov].into()
}

/// # Pseudocode
/// ```text
/// VMOVSS (EVEX.LLIG.F3.0F.W0 11 /r When the Source Operand is Memory and the Destination is an XMM Register)
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := SRC[31:0]
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///             FI;
/// FI;
/// DEST[MAXVL-1:32] := 0
/// VMOVSS (EVEX.LLIG.F3.0F.W0 10 /r When the Source Operand is an XMM Register and the Destination is Memory)
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := SRC[31:0]
///     ELSE*DEST[31:0] remains unchanged*
///                             ; merging-masking
/// FI;
/// VMOVSS (EVEX.LLIG.F3.0F.W0 10/11 /r Where the Source and Destination are XMM Registers)
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := SRC2[31:0]
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
/// MOVSS (Legacy SSE Version When the Source and Destination Operands are Both XMM Registers)
/// DEST[31:0] := SRC[31:0]
/// DEST[MAXVL-1:32] (Unmodified)
/// VMOVSS (VEX.128.F3.0F 11 /r Where the Destination is an XMM Register)
/// DEST[31:0] := SRC2[31:0]
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// VMOVSS (VEX.128.F3.0F 10 /r Where the Source and Destination are XMM Registers)
/// DEST[31:0] := SRC2[31:0]
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// VMOVSS (VEX.128.F3.0F 10 /r When the Source Operand is Memory and the Destination is an XMM Register)
/// DEST[31:0] := SRC[31:0]
/// DEST[MAXVL-1:32] := 0
/// MOVSS/VMOVSS (When the Source Operand is an XMM Register and the Destination is Memory)
/// DEST[31:0] := SRC[31:0]
/// MOVSS (Legacy SSE Version when the Source Operand is Memory and the Destination is an XMM Register)
/// DEST[31:0] := SRC[31:0]
/// DEST[127:32] := 0
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn movss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// Non-64-bit Mode:
/// IF (Byte move)
///     THEN IF DF = 0
///         THEN
///             (E)SI := (E)SI + 1;
///             (E)DI := (E)DI + 1;
///         ELSE
///             (E)SI := (E)SI - 1;
///             (E)DI := (E)DI - 1;
///         FI;
///     ELSE IF (Word move)
///         THEN IF DF = 0
///             (E)SI := (E)SI + 2;
///             (E)DI := (E)DI + 2;
///             FI;
///         ELSE
///             (E)SI := (E)SI - 2;
///             (E)DI := (E)DI - 2;
///         FI;
///     ELSE IF (Doubleword move)
///         THEN IF DF = 0
///             (E)SI := (E)SI + 4;
///             (E)DI := (E)DI + 4;
///             FI;
///         ELSE
///             (E)SI := (E)SI - 4;
///             (E)DI := (E)DI - 4;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte move)
///     THEN IF DF = 0
///         THEN
///             (R|E)SI := (R|E)SI + 1;
///             (R|E)DI := (R|E)DI + 1;
///         ELSE
///             (R|E)SI := (R|E)SI - 1;
///             (R|E)DI := (R|E)DI - 1;
///         FI;
///     ELSE IF (Word move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 2;
///             (R|E)DI := (R|E)DI + 2;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 2;
///             (R|E)DI := (R|E)DI - 2;
///         FI;
///     ELSE IF (Doubleword move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 4;
///             (R|E)DI := (R|E)DI + 4;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 4;
///             (R|E)DI := (R|E)DI - 4;
///         FI;
///     ELSE IF (Quadword move)
///         THEN IF DF = 0
///             (R|E)SI := (R|E)SI + 8;
///             (R|E)DI := (R|E)DI + 8;
///             FI;
///         ELSE
///             (R|E)SI := (R|E)SI - 8;
///             (R|E)DI := (R|E)DI - 8;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn movsw() -> &'static [IrStatement] {
    let mov = assign(d(rsi.clone()), d(rdi.clone()), size_result_byte(c(2)));
    [mov].into()
}

/// # Pseudocode
/// ```text
/// DEST := SignExtend(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn movsx() -> &'static [IrStatement] {
    let assignment = assign(u::sign_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := SignExtend(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn movsxd() -> &'static [IrStatement] {
    let assignment = assign(u::sign_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVUPD (EVEX Encoded Versions, Register-Copy Form)
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
/// VMOVUPD (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+63:i] := SRC[i+63:i]
///         ELSE *DEST[i+63:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVUPD (EVEX Encoded Versions, Load-Form)
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
/// VMOVUPD (VEX.256 Encoded Version, Load - and Register Copy)
/// DEST[255:0] := SRC[255:0]
/// DEST[MAXVL-1:256] := 0
/// VMOVUPD (VEX.256 Encoded Version, Store-Form)
/// DEST[255:0] := SRC[255:0]
/// VMOVUPD (VEX.128 Encoded Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] := 0
/// MOVUPD (128-bit Load- and Register-Copy- Form Legacy SSE Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// (V)MOVUPD (128-bit Store-Form Version)
/// DEST[127:0] := SRC[127:0]
/// ```
#[box_to_static_reference]
pub(super) fn movupd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMOVUPS (EVEX Encoded Versions, Register-Copy Form)
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
/// VMOVUPS (EVEX Encoded Versions, Store-Form)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN DEST[i+31:i] := SRC[i+31:i]
///         ELSE *DEST[i+31:i] remains unchanged*
///                         ; merging-masking
///     FI;
/// ENDFOR;
/// VMOVUPS (EVEX Encoded Versions, Load-Form)
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
/// VMOVUPS (VEX.256 Encoded Version, Load - and Register Copy)
/// DEST[255:0] := SRC[255:0]
/// DEST[MAXVL-1:256] := 0
/// VMOVUPS (VEX.256 Encoded Version, Store-Form)
/// DEST[255:0] := SRC[255:0]
/// VMOVUPS (VEX.128 Encoded Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] := 0
/// MOVUPS (128-bit Load- and Register-Copy- Form Legacy SSE Version)
/// DEST[127:0] := SRC[127:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// (V)MOVUPS (128-bit Store-Form Version)
/// DEST[127:0] := SRC[127:0]
/// ```
#[box_to_static_reference]
pub(super) fn movups() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DEST := ZeroExtend(SRC);
/// ```
#[box_to_static_reference]
pub(super) fn movzx() -> &'static [IrStatement] {
    let assignment = assign(u::zero_extend(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMPSADBW (VEX.256 Encoded Version)
/// BLK2_OFFSET := imm8[1:0]*32
/// BLK1_OFFSET := imm8[2]*32
/// SRC1_BYTE0 := SRC1[BLK1_OFFSET+7:BLK1_OFFSET]
/// SRC1_BYTE1 := SRC1[BLK1_OFFSET+15:BLK1_OFFSET+8]
/// SRC1_BYTE2 := SRC1[BLK1_OFFSET+23:BLK1_OFFSET+16]
/// SRC1_BYTE3 := SRC1[BLK1_OFFSET+31:BLK1_OFFSET+24]
/// SRC1_BYTE4 := SRC1[BLK1_OFFSET+39:BLK1_OFFSET+32]
/// SRC1_BYTE6 := SRC1[BLK1_OFFSET+55:BLK1_OFFSET+48]
/// SRC1_BYTE7 := SRC1[BLK1_OFFSET+63:BLK1_OFFSET+56]
/// SRC1_BYTE8 := SRC1[BLK1_OFFSET+71:BLK1_OFFSET+64]
/// SRC1_BYTE9 := SRC1[BLK1_OFFSET+79:BLK1_OFFSET+72]
/// SRC1_BYTE10 := SRC1[BLK1_OFFSET+87:BLK1_OFFSET+80]
/// SRC2_BYTE0 := SRC2[BLK2_OFFSET+7:BLK2_OFFSET]
/// SRC2_BYTE1 := SRC2[BLK2_OFFSET+15:BLK2_OFFSET+8]
/// SRC2_BYTE2 := SRC2[BLK2_OFFSET+23:BLK2_OFFSET+16]
/// SRC2_BYTE3 := SRC2[BLK2_OFFSET+31:BLK2_OFFSET+24]
/// TEMP0 := ABS(SRC1_BYTE0 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE1 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE2 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE3 - SRC2_BYTE3)
/// DEST[15:0] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE1 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE2 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE3 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE4 - SRC2_BYTE3)
/// DEST[31:16] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE2 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE3 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE4 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE5 - SRC2_BYTE3)
/// DEST[47:32] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE3 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE4 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE5 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE6 - SRC2_BYTE3)
/// DEST[63:48] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE4 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE5 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE6 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE7 - SRC2_BYTE3)
/// DEST[79:64] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE5 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE6 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE7 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE8 - SRC2_BYTE3)
/// DEST[95:80] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE6 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE7 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE8 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE9 - SRC2_BYTE3)
/// DEST[111:96] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE7 - SRC2_BYTE0)
/// TEMP2 := ABS(SRC1_BYTE9 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE10 - SRC2_BYTE3)
/// DEST[127:112] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// BLK2_OFFSET := imm8[4:3]*32 + 128
/// BLK1_OFFSET := imm8[5]*32 + 128
/// SRC1_BYTE0 := SRC1[BLK1_OFFSET+7:BLK1_OFFSET]
/// SRC1_BYTE1 := SRC1[BLK1_OFFSET+15:BLK1_OFFSET+8]
/// SRC1_BYTE2 := SRC1[BLK1_OFFSET+23:BLK1_OFFSET+16]
/// SRC1_BYTE3 := SRC1[BLK1_OFFSET+31:BLK1_OFFSET+24]
/// SRC1_BYTE4 := SRC1[BLK1_OFFSET+39:BLK1_OFFSET+32]
/// SRC1_BYTE5 := SRC1[BLK1_OFFSET+47:BLK1_OFFSET+40]
/// SRC1_BYTE6 := SRC1[BLK1_OFFSET+55:BLK1_OFFSET+48]
/// SRC1_BYTE7 := SRC1[BLK1_OFFSET+63:BLK1_OFFSET+56]
/// SRC1_BYTE8 := SRC1[BLK1_OFFSET+71:BLK1_OFFSET+64]
/// SRC1_BYTE9 := SRC1[BLK1_OFFSET+79:BLK1_OFFSET+72]
/// SRC1_BYTE10 := SRC1[BLK1_OFFSET+87:BLK1_OFFSET+80]
/// SRC2_BYTE0 := SRC2[BLK2_OFFSET+7:BLK2_OFFSET]
/// SRC2_BYTE1 := SRC2[BLK2_OFFSET+15:BLK2_OFFSET+8]
/// SRC2_BYTE2 := SRC2[BLK2_OFFSET+23:BLK2_OFFSET+16]
/// SRC2_BYTE3 := SRC2[BLK2_OFFSET+31:BLK2_OFFSET+24]
/// TEMP0 := ABS(SRC1_BYTE0 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE1 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE2 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE3 - SRC2_BYTE3)
/// DEST[143:128] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE1 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE2 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE3 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE4 - SRC2_BYTE3)
/// DEST[159:144] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE2 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE3 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE4 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE5 - SRC2_BYTE3)
/// DEST[175:160] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE3 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE4 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE5 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE6 - SRC2_BYTE3)
/// DEST[191:176] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE4 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE5 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE6 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE7 - SRC2_BYTE3)
/// DEST[207:192] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP1 := ABS(SRC1_BYTE6 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE7 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE8 - SRC2_BYTE3)
/// DEST[223:208] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE6 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE7 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE8 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE9 - SRC2_BYTE3)
/// DEST[239:224] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE7 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE8 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE9 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE10 - SRC2_BYTE3)
/// DEST[255:240] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// VMPSADBW (VEX.128 Encoded Version)
/// BLK2_OFFSET := imm8[1:0]*32
/// BLK1_OFFSET := imm8[2]*32
/// SRC1_BYTE0 := SRC1[BLK1_OFFSET+7:BLK1_OFFSET]
/// SRC1_BYTE1 := SRC1[BLK1_OFFSET+15:BLK1_OFFSET+8]
/// SRC1_BYTE2 := SRC1[BLK1_OFFSET+23:BLK1_OFFSET+16]
/// SRC1_BYTE3 := SRC1[BLK1_OFFSET+31:BLK1_OFFSET+24]
/// SRC1_BYTE4 := SRC1[BLK1_OFFSET+39:BLK1_OFFSET+32]
/// SRC1_BYTE5 := SRC1[BLK1_OFFSET+47:BLK1_OFFSET+40]
/// SRC1_BYTE6 := SRC1[BLK1_OFFSET+55:BLK1_OFFSET+48]
/// SRC1_BYTE7 := SRC1[BLK1_OFFSET+63:BLK1_OFFSET+56]
/// SRC1_BYTE8 := SRC1[BLK1_OFFSET+71:BLK1_OFFSET+64]
/// SRC1_BYTE9 := SRC1[BLK1_OFFSET+79:BLK1_OFFSET+72]
/// SRC1_BYTE10 := SRC1[BLK1_OFFSET+87:BLK1_OFFSET+80]
/// SRC2_BYTE0 := SRC2[BLK2_OFFSET+7:BLK2_OFFSET]
/// SRC2_BYTE1 := SRC2[BLK2_OFFSET+15:BLK2_OFFSET+8]
/// SRC2_BYTE2 := SRC2[BLK2_OFFSET+23:BLK2_OFFSET+16]
/// SRC2_BYTE3 := SRC2[BLK2_OFFSET+31:BLK2_OFFSET+24]
/// TEMP0 := ABS(SRC1_BYTE0 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE1 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE2 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE3 - SRC2_BYTE3)
/// DEST[15:0] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE1 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE2 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE3 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE4 - SRC2_BYTE3)
/// DEST[31:16] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE2 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE3 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE4 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE5 - SRC2_BYTE3)
/// TEMP0 := ABS(SRC1_BYTE3 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE4 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE5 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE6 - SRC2_BYTE3)
/// DEST[63:48] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE4 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE5 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE6 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE7 - SRC2_BYTE3)
/// DEST[79:64] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE5 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE6 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE7 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE8 - SRC2_BYTE3)
/// DEST[95:80] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE6 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE7 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE8 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE9 - SRC2_BYTE3)
/// DEST[111:96] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS(SRC1_BYTE7 - SRC2_BYTE0)
/// TEMP1 := ABS(SRC1_BYTE8 - SRC2_BYTE1)
/// TEMP2 := ABS(SRC1_BYTE9 - SRC2_BYTE2)
/// TEMP3 := ABS(SRC1_BYTE10 - SRC2_BYTE3)
/// DEST[127:112] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// DEST[MAXVL-1:128] := 0
/// MPSADBW (128-bit Legacy SSE Version)
/// SRC_OFFSET := imm8[1:0]*32
/// DEST_OFFSET := imm8[2]*32
/// DEST_BYTE0 := DEST[DEST_OFFSET+7:DEST_OFFSET]
/// DEST_BYTE1 := DEST[DEST_OFFSET+15:DEST_OFFSET+8]
/// DEST_BYTE2 := DEST[DEST_OFFSET+23:DEST_OFFSET+16]
/// DEST_BYTE3 := DEST[DEST_OFFSET+31:DEST_OFFSET+24]
/// DEST_BYTE4 := DEST[DEST_OFFSET+39:DEST_OFFSET+32]
/// DEST_BYTE5 := DEST[DEST_OFFSET+47:DEST_OFFSET+40]
/// DEST_BYTE6 := DEST[DEST_OFFSET+55:DEST_OFFSET+48]
/// DEST_BYTE7 := DEST[DEST_OFFSET+63:DEST_OFFSET+56]
/// DEST_BYTE8 := DEST[DEST_OFFSET+71:DEST_OFFSET+64]
/// DEST_BYTE9 := DEST[DEST_OFFSET+79:DEST_OFFSET+72]
/// DEST_BYTE10 := DEST[DEST_OFFSET+87:DEST_OFFSET+80]
/// SRC_BYTE0 := SRC[SRC_OFFSET+7:SRC_OFFSET]
/// SRC_BYTE1 := SRC[SRC_OFFSET+15:SRC_OFFSET+8]
/// SRC_BYTE2 := SRC[SRC_OFFSET+23:SRC_OFFSET+16]
/// SRC_BYTE3 := SRC[SRC_OFFSET+31:SRC_OFFSET+24]
/// TEMP0 := ABS( DEST_BYTE0 - SRC_BYTE0)
/// TEMP1 := ABS( DEST_BYTE1 - SRC_BYTE1)
/// TEMP3 := ABS( DEST_BYTE3 - SRC_BYTE3)
/// DEST[15:0] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS( DEST_BYTE1 - SRC_BYTE0)
/// TEMP1 := ABS( DEST_BYTE2 - SRC_BYTE1)
/// TEMP2 := ABS( DEST_BYTE3 - SRC_BYTE2)
/// TEMP3 := ABS( DEST_BYTE4 - SRC_BYTE3)
/// DEST[31:16] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS( DEST_BYTE2 - SRC_BYTE0)
/// TEMP1 := ABS( DEST_BYTE3 - SRC_BYTE1)
/// TEMP2 := ABS( DEST_BYTE4 - SRC_BYTE2)
/// TEMP3 := ABS( DEST_BYTE5 - SRC_BYTE3)
/// DEST[47:32] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS( DEST_BYTE3 - SRC_BYTE0)
/// TEMP1 := ABS( DEST_BYTE4 - SRC_BYTE1)
/// TEMP2 := ABS( DEST_BYTE5 - SRC_BYTE2)
/// TEMP3 := ABS( DEST_BYTE6 - SRC_BYTE3)
/// DEST[63:48] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS( DEST_BYTE4 - SRC_BYTE0)
/// TEMP1 := ABS( DEST_BYTE5 - SRC_BYTE1)
/// TEMP2 := ABS( DEST_BYTE6 - SRC_BYTE2)
/// TEMP3 := ABS( DEST_BYTE7 - SRC_BYTE3)
/// DEST[79:64] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS( DEST_BYTE5 - SRC_BYTE0)
/// TEMP1 := ABS( DEST_BYTE6 - SRC_BYTE1)
/// TEMP2 := ABS( DEST_BYTE7 - SRC_BYTE2)
/// TEMP3 := ABS( DEST_BYTE8 - SRC_BYTE3)
/// DEST[95:80] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS( DEST_BYTE6 - SRC_BYTE0)
/// TEMP1 := ABS( DEST_BYTE7 - SRC_BYTE1)
/// TEMP2 := ABS( DEST_BYTE8 - SRC_BYTE2)
/// TEMP3 := ABS( DEST_BYTE9 - SRC_BYTE3)
/// DEST[111:96] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// TEMP0 := ABS( DEST_BYTE7 - SRC_BYTE0)
/// TEMP1 := ABS( DEST_BYTE8 - SRC_BYTE1)
/// TEMP2 := ABS( DEST_BYTE9 - SRC_BYTE2)
/// TEMP3 := ABS( DEST_BYTE10 - SRC_BYTE3)
/// DEST[127:112] := TEMP0 + TEMP1 + TEMP2 + TEMP3
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn mpsadbw() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (Byte operation)
///     THEN
///         AX := AL * SRC;
///     ELSE (* Word or doubleword operation *)
///         IF OperandSize = 16
///             THEN
///                 DX:AX := AX * SRC;
///             ELSE IF OperandSize = 32
///                 THEN EDX:EAX := EAX * SRC; FI;
///             ELSE (* OperandSize = 64 *)
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn mul() -> &'static [IrStatement] {
    let assertion = assertion(u::not(is_o2_exists()));
    let operand_bit_size = bit_size_of_o1();
    let value_8 = b::mul(sized(al.clone(), size_relative(al.clone())), o1());
    let mul_8 = [calc_flags_automatically(value_8.clone(), o1_size(), &[&of, &cf]), assign(value_8, ax.clone(), size_relative(ax.clone()))];
    let value = b::mul(sized(rax.clone(), o1_size()), o1());
    let mul_etc = [calc_flags_automatically(value.clone(), o1_size(), &[&of, &cf]), assign(value.clone(), rax.clone(), o1_size()), assign(b::shr(u::zero_extend(value), operand_bit_size.clone()), rdx.clone(), o1_size())];
    let mul = condition(b::equal(operand_bit_size, c(8), size_unlimited()), mul_8, mul_etc);
    extend_undefined_flags(&[assertion, mul], &[&sf, &zf, &af, &pf])
}

/// # Pseudocode
/// ```text
/// VMULPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+63:i] * SRC2[63:0]
///                     ELSE
///                         DEST[i+63:i] := SRC1[i+63:i] * SRC2[i+63:i]
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
/// VMULPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] * SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] * SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] * SRC2[191:128]
/// DEST[255:192] := SRC1[255:192] * SRC2[255:192]
/// DEST[MAXVL-1:256] := 0;
/// .
/// VMULPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] * SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] * SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// MULPD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] * SRC[63:0]
/// DEST[127:64] := DEST[127:64] * SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn mulpd() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMULPS (EVEX Encoded Version)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := SRC1[i+31:i] * SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC1[i+31:i] * SRC2[i+31:i]
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
/// VMULPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] * SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] * SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] * SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] * SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] * SRC2[159:128]
/// DEST[191:160] := SRC1[191:160] * SRC2[191:160]
/// DEST[223:192] := SRC1[223:192] * SRC2[223:192]
/// DEST[255:224] := SRC1[255:224] * SRC2[255:224].
/// DEST[MAXVL-1:256] := 0;
/// VMULPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] * SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] * SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] * SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] * SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// MULPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[31:0] * SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] * SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] * SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] * SRC2[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn mulps() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMULSD (EVEX Encoded Version)
/// IF (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := SRC1[63:0] * SRC2[63:0]
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///     FI;
/// ENDFOR
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VMULSD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] * SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// MULSD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] * SRC[63:0]
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn mulsd() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VMULSS (EVEX Encoded Version)
/// IF (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := SRC1[31:0] * SRC2[31:0]
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[31:0] := 0
///                 FI
///     FI;
/// ENDFOR
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// VMULSS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] * SRC2[31:0]
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// MULSS (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0] * SRC[31:0]
/// DEST[MAXVL-1:32] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn mulss() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// // DEST1: ModRM:reg
/// // DEST2: VEX.vvvv
/// IF (OperandSize = 32)
///     SRC1 := EDX;
///     DEST2 := (SRC1*SRC2)[31:0];
///     DEST1 := (SRC1*SRC2)[63:32];
/// ELSE IF (OperandSize = 64)
///     SRC1 := RDX;
///         DEST2 := (SRC1*SRC2)[63:0];
///         DEST1 := (SRC1*SRC2)[127:64];
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn mulx() -> &'static [IrStatement] {
    let product = b::mul(o2(), rdx.clone());
    let assignment = assign(product, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (* MWAIT takes the argument in EAX as a hint extension and is architected to take the argument in ECX as an instruction extension
/// MWAIT EAX, ECX *)
/// {
/// WHILE ( ("Monitor Hardware is in armed state")) {
///     implementation_dependent_optimized_state(EAX, ECX); }
/// Set the state of Monitor Hardware as triggered;
/// }
/// ```
#[box_to_static_reference]
pub(super) fn mwait() -> &'static [IrStatement] {
    [exception("mwait")].into()
}
