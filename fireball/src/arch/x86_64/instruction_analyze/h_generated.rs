use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// HADDPD (128-bit Legacy SSE Version)
/// DEST[63:0] := SRC1[127:64] + SRC1[63:0]
/// DEST[127:64] := SRC2[127:64] + SRC2[63:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// VHADDPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[127:64] + SRC1[63:0]
/// DEST[127:64] := SRC2[127:64] + SRC2[63:0]
/// DEST[MAXVL-1:128] := 0
/// VHADDPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[127:64] + SRC1[63:0]
/// DEST[127:64] := SRC2[127:64] + SRC2[63:0]
/// DEST[191:128] := SRC1[255:192] + SRC1[191:128]
/// DEST[255:192] := SRC2[255:192] + SRC2[191:128]
/// ```
#[box_to_static_reference]
pub(super) fn haddpd() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// HADDPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[63:32] + SRC1[31:0]
/// DEST[63:32] := SRC1[127:96] + SRC1[95:64]
/// DEST[95:64] := SRC2[63:32] + SRC2[31:0]
/// DEST[127:96] := SRC2[127:96] + SRC2[95:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// VHADDPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[63:32] + SRC1[31:0]
/// DEST[63:32] := SRC1[127:96] + SRC1[95:64]
/// DEST[95:64] := SRC2[63:32] + SRC2[31:0]
/// DEST[127:96] := SRC2[127:96] + SRC2[95:64]
/// DEST[MAXVL-1:128] := 0
/// VHADDPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[63:32] + SRC1[31:0]
/// DEST[63:32] := SRC1[127:96] + SRC1[95:64]
/// DEST[95:64] := SRC2[63:32] + SRC2[31:0]
/// DEST[127:96] := SRC2[127:96] + SRC2[95:64]
/// DEST[159:128] := SRC1[191:160] + SRC1[159:128]
/// DEST[191:160] := SRC1[255:224] + SRC1[223:192]
/// DEST[223:192] := SRC2[191:160] + SRC2[159:128]
/// DEST[255:224] := SRC2[255:224] + SRC2[223:192]
/// ```
#[box_to_static_reference]
pub(super) fn haddps() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Enter Halt state;
/// ```
#[box_to_static_reference]
pub(super) fn hlt() -> &'static [IrStatement] {
    [halt()].into()
}

/// # Pseudocode
/// ```text
/// IF EAX = 0
/// THEN NOP
/// ELSE
/// FOREACH i such that EAX[i] = 1
/// Reset prediction history for feature i
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn hreset() -> &'static [IrStatement] {
    [exception("hreset")].into()
}

/// # Pseudocode
/// ```text
/// HSUBPD (128-bit Legacy SSE Version)
/// DEST[63:0] := SRC1[63:0] - SRC1[127:64]
/// DEST[127:64] := SRC2[63:0] - SRC2[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// VHSUBPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] - SRC1[127:64]
/// DEST[127:64] := SRC2[63:0] - SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// VHSUBPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] - SRC1[127:64]
/// DEST[127:64] := SRC2[63:0] - SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] - SRC1[255:192]
/// DEST[255:192] := SRC2[191:128] - SRC2[255:192]
/// ```
#[box_to_static_reference]
pub(super) fn hsubpd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// HSUBPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[31:0] - SRC1[63:32]
/// DEST[63:32] := SRC1[95:64] - SRC1[127:96]
/// DEST[95:64] := SRC2[31:0] - SRC2[63:32]
/// DEST[127:96] := SRC2[95:64] - SRC2[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// VHSUBPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] - SRC1[63:32]
/// DEST[63:32] := SRC1[95:64] - SRC1[127:96]
/// DEST[95:64] := SRC2[31:0] - SRC2[63:32]
/// DEST[127:96] := SRC2[95:64] - SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// VHSUBPS (VEX.256 Encoded Version)
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
pub(super) fn hsubps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}
