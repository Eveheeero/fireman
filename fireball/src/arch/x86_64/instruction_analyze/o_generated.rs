use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// DEST := DEST OR SRC;
/// ```
#[box_to_static_reference]
pub(super) fn or() -> &'static [IrStatement] {
    let op = b::or(o1(), o2());
    let assignment = assign(op.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(op, o1_size(), &[&sf, &zf, &pf]);
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    let set_af = assign(undefined_data(), af.clone(), size_relative(af.clone()));
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, set_of, set_cf, set_af, assignment, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// VORPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                 THEN
///                     DEST[i+63:i] := SRC1[i+63:i] BITWISE OR SRC2[63:0]
///                 ELSE
///                     DEST[i+63:i] := SRC1[i+63:i] BITWISE OR SRC2[i+63:i]
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
/// VORPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] BITWISE OR SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] BITWISE OR SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] BITWISE OR SRC2[191:128]
/// DEST[255:192] := SRC1[255:192] BITWISE OR SRC2[255:192]
/// DEST[MAXVL-1:256] := 0
/// VORPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] BITWISE OR SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] BITWISE OR SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// ORPD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] BITWISE OR SRC[63:0]
/// DEST[127:64] := DEST[127:64] BITWISE OR SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn orpd() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VORPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///             IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                 THEN
///                     DEST[i+31:i] := SRC1[i+31:i] BITWISE OR SRC2[31:0]
///                 ELSE
///                     DEST[i+31:i] := SRC1[i+31:i] BITWISE OR SRC2[i+31:i]
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
/// VORPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] BITWISE OR SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] BITWISE OR SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] BITWISE OR SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] BITWISE OR SRC2[159:128]
/// DEST[191:160] := SRC1[191:160] BITWISE OR SRC2[191:160]
/// DEST[223:192] := SRC1[223:192] BITWISE OR SRC2[223:192]
/// DEST[255:224] := SRC1[255:224] BITWISE OR SRC2[255:224].
/// DEST[MAXVL-1:256] := 0
/// VORPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] BITWISE OR SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] BITWISE OR SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] BITWISE OR SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// ORPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] BITWISE OR SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] BITWISE OR SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] BITWISE OR SRC2[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn orps() -> &'static [IrStatement] {
    let assignment = assign(b::or(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE ( * I/O operation is allowed *)
///                 DEST := SRC; (* Writes to selected I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode with CPL ≤ IOPL *)
///         DEST := SRC; (* Writes to selected I/O port *)
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn out() -> &'static [IrStatement] {
    let cond = condition(b::equal(unknown_data(), c(1), o1_size()), [stmt_0], []);
    [cond].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE (* I/O operation is allowed *)
///                 DEST := SRC; (* Writes to I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode or 64-Bit Mode with CPL ≤ IOPL *)
///         DEST := SRC; (* Writes to I/O port *)
/// FI;
/// Byte transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 1;
///                         ELSE RSI := RSI or - 1;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN EE:S=SI  I
///                             + 1;
///                         ELSE ESI := ESI - 1;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 1;
///                 ELSE (E)SI := (E)SI - 1;
///             FI;
///     FI;
/// Word transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 2;
///                         ELSE RSI := RSI or - 2;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN ESI := ESI + 2;
///                         ELSE ESI := ESI - 2;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 2;
///                 ELSE (E)SI := (E)SI - 2;
///             FI;
///     FI;
/// Doubleword transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 4;
///                         ELSE RSI := RSI or - 4;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN ESI := ESI + 4;
///                         ELSE ESI := ESI - 4;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 4;
///                 ELSE (E)SI := (E)SI - 4;
///             FI;
///     FI;
/// ```
#[box_to_static_reference]
pub(super) fn outs() -> &'static [IrStatement] {
    [exception("outs")].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE (* I/O operation is allowed *)
///                 DEST := SRC; (* Writes to I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode or 64-Bit Mode with CPL ≤ IOPL *)
///         DEST := SRC; (* Writes to I/O port *)
/// FI;
/// Byte transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 1;
///                         ELSE RSI := RSI or - 1;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN EE:S=SI  I
///                             + 1;
///                         ELSE ESI := ESI - 1;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 1;
///                 ELSE (E)SI := (E)SI - 1;
///             FI;
///     FI;
/// Word transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 2;
///                         ELSE RSI := RSI or - 2;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN ESI := ESI + 2;
///                         ELSE ESI := ESI - 2;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 2;
///                 ELSE (E)SI := (E)SI - 2;
///             FI;
///     FI;
/// Doubleword transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 4;
///                         ELSE RSI := RSI or - 4;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN ESI := ESI + 4;
///                         ELSE ESI := ESI - 4;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 4;
///                 ELSE (E)SI := (E)SI - 4;
///             FI;
///     FI;
/// ```
#[box_to_static_reference]
pub(super) fn outsb() -> &'static [IrStatement] {
    [exception("outsb")].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE (* I/O operation is allowed *)
///                 DEST := SRC; (* Writes to I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode or 64-Bit Mode with CPL ≤ IOPL *)
///         DEST := SRC; (* Writes to I/O port *)
/// FI;
/// Byte transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 1;
///                         ELSE RSI := RSI or - 1;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN EE:S=SI  I
///                             + 1;
///                         ELSE ESI := ESI - 1;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 1;
///                 ELSE (E)SI := (E)SI - 1;
///             FI;
///     FI;
/// Word transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 2;
///                         ELSE RSI := RSI or - 2;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN ESI := ESI + 2;
///                         ELSE ESI := ESI - 2;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 2;
///                 ELSE (E)SI := (E)SI - 2;
///             FI;
///     FI;
/// Doubleword transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 4;
///                         ELSE RSI := RSI or - 4;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN ESI := ESI + 4;
///                         ELSE ESI := ESI - 4;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 4;
///                 ELSE (E)SI := (E)SI - 4;
///             FI;
///     FI;
/// ```
#[box_to_static_reference]
pub(super) fn outsd() -> &'static [IrStatement] {
    [exception("outsd")].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE (* I/O operation is allowed *)
///                 DEST := SRC; (* Writes to I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode or 64-Bit Mode with CPL ≤ IOPL *)
///         DEST := SRC; (* Writes to I/O port *)
/// FI;
/// Byte transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 1;
///                         ELSE RSI := RSI or - 1;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN EE:S=SI  I
///                             + 1;
///                         ELSE ESI := ESI - 1;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 1;
///                 ELSE (E)SI := (E)SI - 1;
///             FI;
///     FI;
/// Word transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 2;
///                         ELSE RSI := RSI or - 2;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN ESI := ESI + 2;
///                         ELSE ESI := ESI - 2;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 2;
///                 ELSE (E)SI := (E)SI - 2;
///             FI;
///     FI;
/// Doubleword transfer:
///     IF 64-bit mode
///         Then
///             IF 64-Bit Address Size
///                 THEN
///                     IF DF = 0
///                         THEN RSI := RSI RSI + 4;
///                         ELSE RSI := RSI or - 4;
///                     FI;
///                 ELSE (* 32-Bit Address Size *)
///                     IF DF = 0
///                         THEN ESI := ESI + 4;
///                         ELSE ESI := ESI - 4;
///                     FI;
///             FI;
///         ELSE
///             IF DF = 0
///                 THEN (E)SI := (E)SI + 4;
///                 ELSE (E)SI := (E)SI - 4;
///             FI;
///     FI;
/// ```
#[box_to_static_reference]
pub(super) fn outsw() -> &'static [IrStatement] {
    [exception("outsw")].into()
}
