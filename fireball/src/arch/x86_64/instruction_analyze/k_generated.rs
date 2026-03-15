use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// KADDW
/// DEST[15:0] := SRC1[15:0] + SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KADDB
/// DEST[7:0] := SRC1[7:0] + SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KADDQ
/// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KADDD
/// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kaddb() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KADDW
/// DEST[15:0] := SRC1[15:0] + SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KADDB
/// DEST[7:0] := SRC1[7:0] + SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KADDQ
/// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KADDD
/// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kaddd() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KADDW
/// DEST[15:0] := SRC1[15:0] + SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KADDB
/// DEST[7:0] := SRC1[7:0] + SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KADDQ
/// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KADDD
/// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kaddq() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KADDW
/// DEST[15:0] := SRC1[15:0] + SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KADDB
/// DEST[7:0] := SRC1[7:0] + SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KADDQ
/// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KADDD
/// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kaddw() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KANDW
/// DEST[15:0] := SRC1[15:0] BITWISE AND SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KANDB
/// DEST[7:0] := SRC1[7:0] BITWISE AND SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KANDQ
/// DEST[63:0] := SRC1[63:0] BITWISE AND SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KANDD
/// DEST[31:0] := SRC1[31:0] BITWISE AND SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kandb() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KANDW
/// DEST[15:0] := SRC1[15:0] BITWISE AND SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KANDB
/// DEST[7:0] := SRC1[7:0] BITWISE AND SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KANDQ
/// DEST[63:0] := SRC1[63:0] BITWISE AND SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KANDD
/// DEST[31:0] := SRC1[31:0] BITWISE AND SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kandd() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KANDNW
/// DEST[15:0] := (BITWISE NOT SRC1[15:0]) BITWISE AND SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KANDNB
/// DEST[7:0] := (BITWISE NOT SRC1[7:0]) BITWISE AND SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KANDNQ
/// DEST[63:0] := (BITWISE NOT SRC1[63:0]) BITWISE AND SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KANDND
/// DEST[31:0] := (BITWISE NOT SRC1[31:0]) BITWISE AND SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kandnb() -> &'static [IrStatement] {
    let assignment = assign(b::and(u::not(o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KANDNW
/// DEST[15:0] := (BITWISE NOT SRC1[15:0]) BITWISE AND SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KANDNB
/// DEST[7:0] := (BITWISE NOT SRC1[7:0]) BITWISE AND SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KANDNQ
/// DEST[63:0] := (BITWISE NOT SRC1[63:0]) BITWISE AND SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KANDND
/// DEST[31:0] := (BITWISE NOT SRC1[31:0]) BITWISE AND SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kandnd() -> &'static [IrStatement] {
    let assignment = assign(b::and(u::not(o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KANDNW
/// DEST[15:0] := (BITWISE NOT SRC1[15:0]) BITWISE AND SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KANDNB
/// DEST[7:0] := (BITWISE NOT SRC1[7:0]) BITWISE AND SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KANDNQ
/// DEST[63:0] := (BITWISE NOT SRC1[63:0]) BITWISE AND SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KANDND
/// DEST[31:0] := (BITWISE NOT SRC1[31:0]) BITWISE AND SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kandnq() -> &'static [IrStatement] {
    let assignment = assign(b::and(u::not(o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KANDNW
/// DEST[15:0] := (BITWISE NOT SRC1[15:0]) BITWISE AND SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KANDNB
/// DEST[7:0] := (BITWISE NOT SRC1[7:0]) BITWISE AND SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KANDNQ
/// DEST[63:0] := (BITWISE NOT SRC1[63:0]) BITWISE AND SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KANDND
/// DEST[31:0] := (BITWISE NOT SRC1[31:0]) BITWISE AND SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kandnw() -> &'static [IrStatement] {
    let assignment = assign(b::and(u::not(o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KANDW
/// DEST[15:0] := SRC1[15:0] BITWISE AND SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KANDB
/// DEST[7:0] := SRC1[7:0] BITWISE AND SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KANDQ
/// DEST[63:0] := SRC1[63:0] BITWISE AND SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KANDD
/// DEST[31:0] := SRC1[31:0] BITWISE AND SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kandq() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KANDW
/// DEST[15:0] := SRC1[15:0] BITWISE AND SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KANDB
/// DEST[7:0] := SRC1[7:0] BITWISE AND SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KANDQ
/// DEST[63:0] := SRC1[63:0] BITWISE AND SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KANDD
/// DEST[31:0] := SRC1[31:0] BITWISE AND SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kandw() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KMOVW
/// IF *destination is a memory location*
///     DEST[15:0] := SRC[15:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[15:0])
/// KMOVB
/// IF *destination is a memory location*
///     DEST[7:0] := SRC[7:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[7:0])
/// KMOVQ
/// IF *destination is a memory location or a GPR*
///     DEST[63:0] := SRC[63:0]
/// IF *destination is a mask register*
///     DEST := ZeroExtension(SRC[63:0])
/// KMOVD
/// IF *destination is a memory location*
///     DEST[31:0] := SRC[31:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[31:0])
/// ```
#[box_to_static_reference]
pub(super) fn kmovb() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KMOVW
/// IF *destination is a memory location*
///     DEST[15:0] := SRC[15:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[15:0])
/// KMOVB
/// IF *destination is a memory location*
///     DEST[7:0] := SRC[7:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[7:0])
/// KMOVQ
/// IF *destination is a memory location or a GPR*
///     DEST[63:0] := SRC[63:0]
/// IF *destination is a mask register*
///     DEST := ZeroExtension(SRC[63:0])
/// KMOVD
/// IF *destination is a memory location*
///     DEST[31:0] := SRC[31:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[31:0])
/// ```
#[box_to_static_reference]
pub(super) fn kmovd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KMOVW
/// IF *destination is a memory location*
///     DEST[15:0] := SRC[15:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[15:0])
/// KMOVB
/// IF *destination is a memory location*
///     DEST[7:0] := SRC[7:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[7:0])
/// KMOVQ
/// IF *destination is a memory location or a GPR*
///     DEST[63:0] := SRC[63:0]
/// IF *destination is a mask register*
///     DEST := ZeroExtension(SRC[63:0])
/// KMOVD
/// IF *destination is a memory location*
///     DEST[31:0] := SRC[31:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[31:0])
/// ```
#[box_to_static_reference]
pub(super) fn kmovq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KMOVW
/// IF *destination is a memory location*
///     DEST[15:0] := SRC[15:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[15:0])
/// KMOVB
/// IF *destination is a memory location*
///     DEST[7:0] := SRC[7:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[7:0])
/// KMOVQ
/// IF *destination is a memory location or a GPR*
///     DEST[63:0] := SRC[63:0]
/// IF *destination is a mask register*
///     DEST := ZeroExtension(SRC[63:0])
/// KMOVD
/// IF *destination is a memory location*
///     DEST[31:0] := SRC[31:0]
/// IF *destination is a mask register or a GPR *
///     DEST := ZeroExtension(SRC[31:0])
/// ```
#[box_to_static_reference]
pub(super) fn kmovw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KNOTW
/// DEST[15:0] := BITWISE NOT SRC[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KNOTB
/// DEST[7:0] := BITWISE NOT SRC[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KNOTQ
/// DEST[63:0] := BITWISE NOT SRC[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KNOTD
/// DEST[31:0] := BITWISE NOT SRC[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn knotb() -> &'static [IrStatement] {
    let assignment = assign(u::not(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KNOTW
/// DEST[15:0] := BITWISE NOT SRC[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KNOTB
/// DEST[7:0] := BITWISE NOT SRC[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KNOTQ
/// DEST[63:0] := BITWISE NOT SRC[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KNOTD
/// DEST[31:0] := BITWISE NOT SRC[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn knotd() -> &'static [IrStatement] {
    let assignment = assign(u::not(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KNOTW
/// DEST[15:0] := BITWISE NOT SRC[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KNOTB
/// DEST[7:0] := BITWISE NOT SRC[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KNOTQ
/// DEST[63:0] := BITWISE NOT SRC[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KNOTD
/// DEST[31:0] := BITWISE NOT SRC[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn knotq() -> &'static [IrStatement] {
    let assignment = assign(u::not(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KNOTW
/// DEST[15:0] := BITWISE NOT SRC[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KNOTB
/// DEST[7:0] := BITWISE NOT SRC[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KNOTQ
/// DEST[63:0] := BITWISE NOT SRC[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KNOTD
/// DEST[31:0] := BITWISE NOT SRC[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn knotw() -> &'static [IrStatement] {
    let assignment = assign(u::not(o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KORW
/// DEST[15:0] := SRC1[15:0] BITWISE OR SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KORB
/// DEST[7:0] := SRC1[7:0] BITWISE OR SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KORQ
/// DEST[63:0] := SRC1[63:0] BITWISE OR SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KORD
/// DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn korb() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KORW
/// DEST[15:0] := SRC1[15:0] BITWISE OR SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KORB
/// DEST[7:0] := SRC1[7:0] BITWISE OR SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KORQ
/// DEST[63:0] := SRC1[63:0] BITWISE OR SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KORD
/// DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kord() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KORW
/// DEST[15:0] := SRC1[15:0] BITWISE OR SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KORB
/// DEST[7:0] := SRC1[7:0] BITWISE OR SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KORQ
/// DEST[63:0] := SRC1[63:0] BITWISE OR SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KORD
/// DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn korq() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KORTESTW
/// TMP[15:0] := DEST[15:0] BITWISE OR SRC[15:0]
/// IF(TMP[15:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[15:0]=FFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTB
/// TMP[7:0] := DEST[7:0] BITWISE OR SRC[7:0]
/// IF(TMP[7:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[7:0]==FFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTQ
/// TMP[63:0] := DEST[63:0] BITWISE OR SRC[63:0]
/// IF(TMP[63:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[63:0]==FFFFFFFF_FFFFFFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTD
/// TMP[31:0] := DEST[31:0] BITWISE OR SRC[31:0]
/// IF(TMP[31:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[31:0]=FFFFFFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kortestb() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KORTESTW
/// TMP[15:0] := DEST[15:0] BITWISE OR SRC[15:0]
/// IF(TMP[15:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[15:0]=FFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTB
/// TMP[7:0] := DEST[7:0] BITWISE OR SRC[7:0]
/// IF(TMP[7:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[7:0]==FFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTQ
/// TMP[63:0] := DEST[63:0] BITWISE OR SRC[63:0]
/// IF(TMP[63:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[63:0]==FFFFFFFF_FFFFFFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTD
/// TMP[31:0] := DEST[31:0] BITWISE OR SRC[31:0]
/// IF(TMP[31:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[31:0]=FFFFFFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kortestd() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KORTESTW
/// TMP[15:0] := DEST[15:0] BITWISE OR SRC[15:0]
/// IF(TMP[15:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[15:0]=FFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTB
/// TMP[7:0] := DEST[7:0] BITWISE OR SRC[7:0]
/// IF(TMP[7:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[7:0]==FFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTQ
/// TMP[63:0] := DEST[63:0] BITWISE OR SRC[63:0]
/// IF(TMP[63:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[63:0]==FFFFFFFF_FFFFFFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTD
/// TMP[31:0] := DEST[31:0] BITWISE OR SRC[31:0]
/// IF(TMP[31:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[31:0]=FFFFFFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kortestq() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KORTESTW
/// TMP[15:0] := DEST[15:0] BITWISE OR SRC[15:0]
/// IF(TMP[15:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[15:0]=FFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTB
/// TMP[7:0] := DEST[7:0] BITWISE OR SRC[7:0]
/// IF(TMP[7:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[7:0]==FFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTQ
/// TMP[63:0] := DEST[63:0] BITWISE OR SRC[63:0]
/// IF(TMP[63:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[63:0]==FFFFFFFF_FFFFFFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// KORTESTD
/// TMP[31:0] := DEST[31:0] BITWISE OR SRC[31:0]
/// IF(TMP[31:0]=0)
///     THEN ZF := 1
///     ELSE ZF := 0
/// FI;
/// IF(TMP[31:0]=FFFFFFFFh)
///     THEN CF := 1
///     ELSE CF := 0
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kortestw() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KORW
/// DEST[15:0] := SRC1[15:0] BITWISE OR SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KORB
/// DEST[7:0] := SRC1[7:0] BITWISE OR SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KORQ
/// DEST[63:0] := SRC1[63:0] BITWISE OR SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KORD
/// DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn korw() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KSHIFTLW
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=15
///     THEN DEST[15:0] := SRC1[15:0] << COUNT;
/// FI;
/// KSHIFTLB
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=7
///         THEN DEST[7::0=]
///             SRC1[7:0] << COUNT;
/// FI;
/// KSHIFTLQ
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=63
///         THEN DEST[63:0] := SRC1[63:0] << COUNT;
/// FI;
/// KSHIFTLD
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=31
///         THEN DEST[31:0] := SRC1[31:0] << COUNT;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kshiftlb() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KSHIFTLW
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=15
///     THEN DEST[15:0] := SRC1[15:0] << COUNT;
/// FI;
/// KSHIFTLB
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=7
///         THEN DEST[7::0=]
///             SRC1[7:0] << COUNT;
/// FI;
/// KSHIFTLQ
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=63
///         THEN DEST[63:0] := SRC1[63:0] << COUNT;
/// FI;
/// KSHIFTLD
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=31
///         THEN DEST[31:0] := SRC1[31:0] << COUNT;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kshiftld() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KSHIFTLW
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=15
///     THEN DEST[15:0] := SRC1[15:0] << COUNT;
/// FI;
/// KSHIFTLB
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=7
///         THEN DEST[7::0=]
///             SRC1[7:0] << COUNT;
/// FI;
/// KSHIFTLQ
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=63
///         THEN DEST[63:0] := SRC1[63:0] << COUNT;
/// FI;
/// KSHIFTLD
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=31
///         THEN DEST[31:0] := SRC1[31:0] << COUNT;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kshiftlq() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KSHIFTLW
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=15
///     THEN DEST[15:0] := SRC1[15:0] << COUNT;
/// FI;
/// KSHIFTLB
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=7
///         THEN DEST[7::0=]
///             SRC1[7:0] << COUNT;
/// FI;
/// KSHIFTLQ
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=63
///         THEN DEST[63:0] := SRC1[63:0] << COUNT;
/// FI;
/// KSHIFTLD
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=31
///         THEN DEST[31:0] := SRC1[31:0] << COUNT;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kshiftlw() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KSHIFTRW
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=15
///     THEN DEST[15:0] := SRC1[15:0] >> COUNT;
/// FI;
/// KSHIFTRB
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=7
///         THEN DEST[7:0] := SRC1[7:0] >> COUNT;
/// FI;
/// KSHIFTRQ
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=63
///         THEN DEST[63:0] := SRC1[63:0] >> COUNT;
/// FI;
/// KSHIFTRD
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=31
///         THEN DEST[31::0=]  SRC1[31:0] >> COUNT;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kshiftrb() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KSHIFTRW
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=15
///     THEN DEST[15:0] := SRC1[15:0] >> COUNT;
/// FI;
/// KSHIFTRB
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=7
///         THEN DEST[7:0] := SRC1[7:0] >> COUNT;
/// FI;
/// KSHIFTRQ
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=63
///         THEN DEST[63:0] := SRC1[63:0] >> COUNT;
/// FI;
/// KSHIFTRD
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=31
///         THEN DEST[31::0=]  SRC1[31:0] >> COUNT;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kshiftrd() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KSHIFTRW
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=15
///     THEN DEST[15:0] := SRC1[15:0] >> COUNT;
/// FI;
/// KSHIFTRB
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=7
///         THEN DEST[7:0] := SRC1[7:0] >> COUNT;
/// FI;
/// KSHIFTRQ
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=63
///         THEN DEST[63:0] := SRC1[63:0] >> COUNT;
/// FI;
/// KSHIFTRD
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=31
///         THEN DEST[31::0=]  SRC1[31:0] >> COUNT;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kshiftrq() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KSHIFTRW
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=15
///     THEN DEST[15:0] := SRC1[15:0] >> COUNT;
/// FI;
/// KSHIFTRB
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=7
///         THEN DEST[7:0] := SRC1[7:0] >> COUNT;
/// FI;
/// KSHIFTRQ
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=63
///         THEN DEST[63:0] := SRC1[63:0] >> COUNT;
/// FI;
/// KSHIFTRD
/// COUNT := imm8[7:0]
/// DEST[MAX_KL-1:0] := 0
/// IF COUNT <=31
///         THEN DEST[31::0=]  SRC1[31:0] >> COUNT;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn kshiftrw() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KTESTW
/// TEMP[15:0] := SRC2[15:0] AND SRC1[15:0]
/// IF (TEMP[15:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[15:0] := SRC2[15:0] AND NOT SRC1[15:0]
/// IF (TEMP[15:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTB
/// TEMP[7:0] := SRC2[7:0] AND SRC1[7:0]
/// IF (TEMP[7:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[7:0] := SRC2[7:0] AND NOT SRC1[7:0]
/// IF (TEMP[7:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTQ
/// TEMP[63:0] := SRC2[63:0] AND SRC1[63:0]
/// IF (TEMP[63:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[63:0] := SRC2[63:0] AND NOT SRC1[63:0]
/// IF (TEMP[63:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTD
/// TEMP[31:0] := SRC2[31:0] AND SRC1[31:0]
/// IF (TEMP[31:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[31:0] := SRC2[31:0] AND NOT SRC1[31:0]
/// IF (TEMP[31:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn ktestb() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// KTESTW
/// TEMP[15:0] := SRC2[15:0] AND SRC1[15:0]
/// IF (TEMP[15:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[15:0] := SRC2[15:0] AND NOT SRC1[15:0]
/// IF (TEMP[15:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTB
/// TEMP[7:0] := SRC2[7:0] AND SRC1[7:0]
/// IF (TEMP[7:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[7:0] := SRC2[7:0] AND NOT SRC1[7:0]
/// IF (TEMP[7:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTQ
/// TEMP[63:0] := SRC2[63:0] AND SRC1[63:0]
/// IF (TEMP[63:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[63:0] := SRC2[63:0] AND NOT SRC1[63:0]
/// IF (TEMP[63:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTD
/// TEMP[31:0] := SRC2[31:0] AND SRC1[31:0]
/// IF (TEMP[31:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[31:0] := SRC2[31:0] AND NOT SRC1[31:0]
/// IF (TEMP[31:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn ktestd() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// KTESTW
/// TEMP[15:0] := SRC2[15:0] AND SRC1[15:0]
/// IF (TEMP[15:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[15:0] := SRC2[15:0] AND NOT SRC1[15:0]
/// IF (TEMP[15:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTB
/// TEMP[7:0] := SRC2[7:0] AND SRC1[7:0]
/// IF (TEMP[7:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[7:0] := SRC2[7:0] AND NOT SRC1[7:0]
/// IF (TEMP[7:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTQ
/// TEMP[63:0] := SRC2[63:0] AND SRC1[63:0]
/// IF (TEMP[63:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[63:0] := SRC2[63:0] AND NOT SRC1[63:0]
/// IF (TEMP[63:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTD
/// TEMP[31:0] := SRC2[31:0] AND SRC1[31:0]
/// IF (TEMP[31:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[31:0] := SRC2[31:0] AND NOT SRC1[31:0]
/// IF (TEMP[31:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn ktestq() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// KTESTW
/// TEMP[15:0] := SRC2[15:0] AND SRC1[15:0]
/// IF (TEMP[15:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[15:0] := SRC2[15:0] AND NOT SRC1[15:0]
/// IF (TEMP[15:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTB
/// TEMP[7:0] := SRC2[7:0] AND SRC1[7:0]
/// IF (TEMP[7:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[7:0] := SRC2[7:0] AND NOT SRC1[7:0]
/// IF (TEMP[7:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTQ
/// TEMP[63:0] := SRC2[63:0] AND SRC1[63:0]
/// IF (TEMP[63:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[63:0] := SRC2[63:0] AND NOT SRC1[63:0]
/// IF (TEMP[63:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// KTESTD
/// TEMP[31:0] := SRC2[31:0] AND SRC1[31:0]
/// IF (TEMP[31:0] = = 0)
///     THEN ZF :=1;
///     ELSE ZF := 0;
/// FI;
/// TEMP[31:0] := SRC2[31:0] AND NOT SRC1[31:0]
/// IF (TEMP[31:0] = = 0)
///     THEN CF :=1;
///     ELSE CF := 0;
/// FI;
/// AF := OF := PF := SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn ktestw() -> &'static [IrStatement] {
    let and_val = b::and(o1(), o2());
    let calc_flags = calc_flags_automatically(and_val, o1_size(), &[&sf, &zf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// KUNPCKBW
/// DEST[7:0] := SRC2[7:0]
/// DEST[15:8] := SRC1[7:0]
/// DEST[MAX_KL-1:16] := 0
/// KUNPCKWD
/// DEST[15:0] := SRC2[15:0]
/// DEST[31:16] := SRC1[15:0]
/// DEST[MAX_KL-1:32] := 0
/// KUNPCKDQ
/// DEST[31:0] := SRC2[31:0]
/// DEST[63:32] := SRC1[31:0]
/// DEST[MAX_KL-1:64] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kunpckbw() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KUNPCKBW
/// DEST[7:0] := SRC2[7:0]
/// DEST[15:8] := SRC1[7:0]
/// DEST[MAX_KL-1:16] := 0
/// KUNPCKWD
/// DEST[15:0] := SRC2[15:0]
/// DEST[31:16] := SRC1[15:0]
/// DEST[MAX_KL-1:32] := 0
/// KUNPCKDQ
/// DEST[31:0] := SRC2[31:0]
/// DEST[63:32] := SRC1[31:0]
/// DEST[MAX_KL-1:64] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kunpckdq() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KUNPCKBW
/// DEST[7:0] := SRC2[7:0]
/// DEST[15:8] := SRC1[7:0]
/// DEST[MAX_KL-1:16] := 0
/// KUNPCKWD
/// DEST[15:0] := SRC2[15:0]
/// DEST[31:16] := SRC1[15:0]
/// DEST[MAX_KL-1:32] := 0
/// KUNPCKDQ
/// DEST[31:0] := SRC2[31:0]
/// DEST[63:32] := SRC1[31:0]
/// DEST[MAX_KL-1:64] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kunpckwd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KXNORW
/// DEST[15:0] := NOT (SRC1[15:0] BITWISE XOR SRC2[15:0])
/// DEST[MAX_KL-1:16] := 0
/// KXNORB
/// DEST[7:0] := NOT (SRC1[7:0] BITWISE XOR SRC2[7:0])
/// DEST[MAX_KL-1:8] := 0
/// KXNORQ
/// DEST[63:0] := NOT (SRC1[63:0] BITWISE XOR SRC2[63:0])
/// DEST[MAX_KL-1:64] := 0
/// KXNORD
/// DEST[31:0] := NOT (SRC1[31:0] BITWISE XOR SRC2[31:0])
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kxnorb() -> &'static [IrStatement] {
    let assignment = assign(u::not(b::xor(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KXNORW
/// DEST[15:0] := NOT (SRC1[15:0] BITWISE XOR SRC2[15:0])
/// DEST[MAX_KL-1:16] := 0
/// KXNORB
/// DEST[7:0] := NOT (SRC1[7:0] BITWISE XOR SRC2[7:0])
/// DEST[MAX_KL-1:8] := 0
/// KXNORQ
/// DEST[63:0] := NOT (SRC1[63:0] BITWISE XOR SRC2[63:0])
/// DEST[MAX_KL-1:64] := 0
/// KXNORD
/// DEST[31:0] := NOT (SRC1[31:0] BITWISE XOR SRC2[31:0])
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kxnord() -> &'static [IrStatement] {
    let assignment = assign(u::not(b::xor(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KXNORW
/// DEST[15:0] := NOT (SRC1[15:0] BITWISE XOR SRC2[15:0])
/// DEST[MAX_KL-1:16] := 0
/// KXNORB
/// DEST[7:0] := NOT (SRC1[7:0] BITWISE XOR SRC2[7:0])
/// DEST[MAX_KL-1:8] := 0
/// KXNORQ
/// DEST[63:0] := NOT (SRC1[63:0] BITWISE XOR SRC2[63:0])
/// DEST[MAX_KL-1:64] := 0
/// KXNORD
/// DEST[31:0] := NOT (SRC1[31:0] BITWISE XOR SRC2[31:0])
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kxnorq() -> &'static [IrStatement] {
    let assignment = assign(u::not(b::xor(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KXNORW
/// DEST[15:0] := NOT (SRC1[15:0] BITWISE XOR SRC2[15:0])
/// DEST[MAX_KL-1:16] := 0
/// KXNORB
/// DEST[7:0] := NOT (SRC1[7:0] BITWISE XOR SRC2[7:0])
/// DEST[MAX_KL-1:8] := 0
/// KXNORQ
/// DEST[63:0] := NOT (SRC1[63:0] BITWISE XOR SRC2[63:0])
/// DEST[MAX_KL-1:64] := 0
/// KXNORD
/// DEST[31:0] := NOT (SRC1[31:0] BITWISE XOR SRC2[31:0])
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kxnorw() -> &'static [IrStatement] {
    let assignment = assign(u::not(b::xor(o2(), o3())), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KXORW
/// DEST[15:0] := SRC1[15:0] BITWISE XOR SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KXORB
/// DEST[7:0] := SRC1[7:0] BITWISE XOR SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KXORQ
/// DEST[63:0] := SRC1[63:0] BITWISE XOR SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KXORD
/// DEST[31:0] := SRC1[31:0] BITWISE XOR SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kxorb() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KXORW
/// DEST[15:0] := SRC1[15:0] BITWISE XOR SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KXORB
/// DEST[7:0] := SRC1[7:0] BITWISE XOR SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KXORQ
/// DEST[63:0] := SRC1[63:0] BITWISE XOR SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KXORD
/// DEST[31:0] := SRC1[31:0] BITWISE XOR SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kxord() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KXORW
/// DEST[15:0] := SRC1[15:0] BITWISE XOR SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KXORB
/// DEST[7:0] := SRC1[7:0] BITWISE XOR SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KXORQ
/// DEST[63:0] := SRC1[63:0] BITWISE XOR SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KXORD
/// DEST[31:0] := SRC1[31:0] BITWISE XOR SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kxorq() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// KXORW
/// DEST[15:0] := SRC1[15:0] BITWISE XOR SRC2[15:0]
/// DEST[MAX_KL-1:16] := 0
/// KXORB
/// DEST[7:0] := SRC1[7:0] BITWISE XOR SRC2[7:0]
/// DEST[MAX_KL-1:8] := 0
/// KXORQ
/// DEST[63:0] := SRC1[63:0] BITWISE XOR SRC2[63:0]
/// DEST[MAX_KL-1:64] := 0
/// KXORD
/// DEST[31:0] := SRC1[31:0] BITWISE XOR SRC2[31:0]
/// DEST[MAX_KL-1:32] := 0
/// ```
#[box_to_static_reference]
pub(super) fn kxorw() -> &'static [IrStatement] {
    let assignment = assign(b::xor(o2(), o3()), o1(), o1_size());
    [assignment].into()
}
