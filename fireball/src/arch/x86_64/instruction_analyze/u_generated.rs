use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// (V)UCOMISD (All Versions)
/// RESULT := UnorderedCompare(DEST[63:0] <> SRC[63:0]) {
/// (* Set EFLAGS *) CASE (RESULT) OF
///     UNORDERED: ZF,PF,CF := 111;
///     GREATER_THAN: ZF,PF,CF := 000;
///     LESS_THAN: ZF,PF,CF := 001;
///     EQUAL: ZF,PF,CF := 100;
/// ESAC;
/// OF, AF, SF := 0; }
/// ```
#[box_to_static_reference]
pub(super) fn ucomisd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (V)UCOMISS (All Versions)
/// RESULT := UnorderedCompare(DEST[31:0] <> SRC[31:0]) {
/// (* Set EFLAGS *) CASE (RESULT) OF
///     UNORDERED: ZF,PF,CF := 111;
///     GREATER_THAN: ZF,PF,CF := 000;
///     LESS_THAN: ZF,PF,CF := 001;
///     EQUAL: ZF,PF,CF := 100;
/// ESAC;
/// OF, AF, SF := 0; }
/// ```
#[box_to_static_reference]
pub(super) fn ucomiss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o1(), o2()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// #UD (* Generates invalid opcode exception *);
/// ```
#[box_to_static_reference]
pub(super) fn ud() -> &'static [IrStatement] {
    [exception("ud")].into()
}

/// # Pseudocode
/// ```text
/// Pop tempRIP;
/// Pop tempRFLAGS;// see below for how this is used to load RFLAGS
/// Pop tempRSP;
/// IF tempRIP is not canonical in current paging mode
///     THEN #GP(0);
/// FI;
/// IF ShadowStackEnabled(CPL)
///     THEN
///         PopShadowStack SSRIP;
///         IF SSRIP ≠ tempRIP
///             THEN #CP (FAR-RET/IRET);
///         FI;
/// FI;
/// RIP := tempRIP;
/// // update in RFLAGS only CF, PF, AF, ZF, SF, TF, DF, OF, NT, RF, AC, and ID
/// RFLAGS := (RFLAGS & ~254DD5H) | (tempRFLAGS & 254DD5H);
/// RSP := tempRSP;
/// UIF := 1;
/// Clear any cache-line monitoring established by MONITOR or UMONITOR;
/// ```
#[box_to_static_reference]
pub(super) fn uiret() -> &'static [IrStatement] {
    [exception("uiret")].into()
}

/// # Pseudocode
/// ```text
/// UMONITOR sets up an address range for the monitor hardware using the content of source register as an effective
/// address and puts the monitor hardware in armed state. A store to the specified address range will trigger the
/// monitor hardware.
/// ```
#[box_to_static_reference]
pub(super) fn umonitor() -> &'static [IrStatement] {
    [exception("umonitor")].into()
}

/// # Pseudocode
/// ```text
/// os_deadline := TSC+(IA32_UMWAIT_CONTROL[31:2]<<2)
/// instr_deadline := UINT64(EDX:EAX)
/// IF os_deadline < instr_deadline:
///     deadline := os_deadline
///     using_os_deadline := 1
/// ELSE:
///     deadline := instr_deadline
///     using_os_deadline := 0
/// WHILE monitor hardware armed AND TSC < deadline:
///     implementation_dependent_optimized_state(Source register, deadline, IA32_UMWAIT_CONTROL[0] )
/// IF using_os_deadline AND TSC > deadline:
///     RFLAGS.CF := 1
/// ELSE:
///     RFLAGS.CF := 0
/// RFLAGS.AF,PF,SF,ZF,OF := 0
/// ```
#[box_to_static_reference]
pub(super) fn umwait() -> &'static [IrStatement] {
    [exception("umwait")].into()
}

/// # Pseudocode
/// ```text
/// VUNPCKHPD (EVEX Encoded Versions When SRC2 is a Register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF VL >= 128
///     TMP_DEST[63:0] := SRC1[127:64]
///     TMP_DEST[127:64] := SRC2[127:64]
/// FI;
/// IF VL >= 256
///     TMP_DEST[191:128] := SRC1[255:192]
///     TMP_DEST[255:192] := SRC2[255:192]
/// FI;
/// IF VL >= 512
///     TMP_DEST[319:256] := SRC1[383:320]
///     TMP_DEST[383:320] := SRC2[383:320]
///     TMP_DEST[447:384] := SRC1[511:448]
///     TMP_DEST[511:448] := SRC2[511:448]
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
/// VUNPCKHPD (EVEX Encoded Version When SRC2 is Memory)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL >= 128
///     TMP_DEST[63:0] := SRC1[127:64]
///     TMP_DEST[127:64] := TMP_SRC2[127:64]
/// FI;
/// IF VL >= 256
///     TMP_DEST[191:128] := SRC1[255:192]
///     TMP_DEST[255:192] := TMP_SRC2[255:192]
/// FI;
/// IF VL >= 512
///     TMP_DEST[319:256] := SRC1[383:320]
///     TMP_DEST[383:320] := TMP_SRC2[383:320]
///     TMP_DEST[447:384] := SRC1[511:448]
///     TMP_DEST[511:448] := TMP_SRC2[511:448]
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
/// VUNPCKHPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// DEST[191:128] := SRC1[255:192]
/// DEST[255:192] := SRC2[255:192]
/// DEST[MAXVL-1:256] := 0
/// VUNPCKHPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// UNPCKHPD (128-bit Legacy SSE Version)
/// DEST[63:0] := SRC1[127:64]
/// DEST[127:64] := SRC2[127:64]
/// ```
#[box_to_static_reference]
pub(super) fn unpckhpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VUNPCKHPS (EVEX Encoded Version When SRC2 is a Register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL >= 128
///     TMP_DEST[31:0] := SRC1[95:64]
///     TMP_DEST[63:32] := SRC2[95:64]
///     TMP_DEST[95:64] := SRC1[127:96]
///     TMP_DEST[127:96] := SRC2[127:96]
/// FI;
/// IF VL >= 256
///     TMP_DEST[159:128] := SRC1[223:192]
///     TMP_DEST[191:160] := SRC2[223:192]
///     TMP_DEST[223:192] := SRC1[255:224]
///     TMP_DEST[255:224] := SRC2[255:224]
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := SRC1[351:320]
///     TMP_DEST[319:288] := SRC2[351:320]
///     TMP_DEST[351:320] := SRC1[383:352]
///     TMP_DEST[383:352] := SRC2[383:352]
///     TMP_DEST[415:384] := SRC1[479:448]
///     TMP_DEST[447:416] := SRC2[479:448]
///     TMP_DEST[479:448] := SRC1[511:480]
///     TMP_DEST[511:480] := SRC2[511:480]
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
/// VUNPCKHPS (EVEX Encoded Version When SRC2 is Memory)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL >= 128
///     TMP_DEST[31:0] := SRC1[95:64]
///     TMP_DEST[63:32] := TMP_SRC2[95:64]
///     TMP_DEST[95:64] := SRC1[127:96]
///     TMP_DEST[127:96] := TMP_SRC2[127:96]
/// FI;
/// IF VL >= 256
///     TMP_DEST[159:128] := SRC1[223:192]
///     TMP_DEST[191:160] := TMP_SRC2[223:192]
///     TMP_DEST[223:192] := SRC1[255:224]
///     TMP_DEST[255:224] := TMP_SRC2[255:224]
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := SRC1[351:320]
///     TMP_DEST[319:288] := TMP_SRC2[351:320]
///     TMP_DEST[351:320] := SRC1[383:352]
///     TMP_DEST[383:352] := TMP_SRC2[383:352]
///     TMP_DEST[415:384] := SRC1[479:448]
///     TMP_DEST[447:416] := TMP_SRC2[479:448]
///     TMP_DEST[479:448] := SRC1[511:480]
///     TMP_DEST[511:480] := TMP_SRC2[511:480]
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
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VUNPCKHPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// DEST[159:128] := SRC1[223:192]
/// DEST[191:160] := SRC2[223:192]
/// DEST[223:192] := SRC1[255:224]
/// DEST[255:224] := SRC2[255:224]
/// DEST[MAXVL-1:256] := 0
/// VUNPCKHPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// UNPCKHPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[95:64]
/// DEST[63:32] := SRC2[95:64]
/// DEST[95:64] := SRC1[127:96]
/// DEST[127:96] := SRC2[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn unpckhps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VUNPCKLPD (EVEX Encoded Versions When SRC2 is a Register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF VL >= 128
///     TMP_DEST[63:0] := SRC1[63:0]
///     TMP_DEST[127:64] := SRC2[63:0]
/// FI;
/// IF VL >= 256
///     TMP_DEST[191:128] := SRC1[191:128]
///     TMP_DEST[255:192] := SRC2[191:128]
/// FI;
/// IF VL >= 512
///     TMP_DEST[319:256] := SRC1[319:256]
///     TMP_DEST[383:320] := SRC2[319:256]
///     TMP_DEST[447:384] := SRC1[447:384]
///     TMP_DEST[511:448] := SRC2[447:384]
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
/// VUNPCKLPD (EVEX Encoded Version When SRC2 is Memory)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF VL >= 128
///     TMP_DEST[63:0] := SRC1[63:0]
///     TMP_DEST[127:64] := TMP_SRC2[63:0]
/// FI;
/// IF VL >= 256
///     TMP_DEST[191:128] := SRC1[191:128]
///     TMP_DEST[255:192] := TMP_SRC2[191:128]
/// FI;
/// IF VL >= 512
///     TMP_DEST[319:256] := SRC1[319:256]
///     TMP_DEST[383:320] := TMP_SRC2[319:256]
///     TMP_DEST[447:384] := SRC1[447:384]
///     TMP_DEST[511:448] := TMP_SRC2[447:384]
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
/// VUNPCKLPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[191:128] := SRC1[191:128]
/// DEST[255:192] := SRC2[191:128]
/// DEST[MAXVL-1:256] := 0
/// VUNPCKLPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[MAXVL-1:128] := 0
/// UNPCKLPD (128-bit Legacy SSE Version)
/// DEST[63:0] := SRC1[63:0]
/// DEST[127:64] := SRC2[63:0]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn unpcklpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VUNPCKLPS (EVEX Encoded Version When SRC2 is a ZMM Register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF VL >= 128
///     TMP_DEST[31:0] := SRC1[31:0]
///     TMP_DEST[63:32] := SRC2[31:0]
///     TMP_DEST[95:64] := SRC1[63:32]
///     TMP_DEST[127:96] := SRC2[63:32]
/// FI;
/// IF VL >= 256
///     TMP_DEST[159:128] := SRC1[159:128]
///     TMP_DEST[191:160] := SRC2[159:128]
///     TMP_DEST[223:192] := SRC1[191:160]
///     TMP_DEST[255:224] := SRC2[191:160]
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := SRC1[287:256]
///     TMP_DEST[319:288] := SRC2[287:256]
///     TMP_DEST[351:320] := SRC1[319:288]
///     TMP_DEST[383:352] := SRC2[319:288]
///     TMP_DEST[415:384] := SRC1[415:384]
///     TMP_DEST[447:416] := SRC2[415:384]
///     TMP_DEST[479:448] := SRC1[447:416]
///     TMP_DEST[511:480] := SRC2[447:416]
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
/// VUNPCKLPS (EVEX Encoded Version When SRC2 is Memory)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 31
///     IF (EVEX.b = 1)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// IF VL >= 128
/// TMP_DEST[31:0] := SRC1[31:0]
/// TMP_DEST[63:32] := TMP_SRC2[31:0]
/// TMP_DEST[95:64] := SRC1[63:32]
/// TMP_DEST[127:96] := TMP_SRC2[63:32]
/// FI;
/// IF VL >= 256
///     TMP_DEST[159:128] := SRC1[159:128]
///     TMP_DEST[191:160] := TMP_SRC2[159:128]
///     TMP_DEST[223:192] := SRC1[191:160]
///     TMP_DEST[255:224] := TMP_SRC2[191:160]
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := SRC1[287:256]
///     TMP_DEST[319:288] := TMP_SRC2[287:256]
///     TMP_DEST[351:320] := SRC1[319:288]
///     TMP_DEST[383:352] := TMP_SRC2[319:288]
///     TMP_DEST[415:384] := SRC1[415:384]
///     TMP_DEST[447:416] := TMP_SRC2[415:384]
///     TMP_DEST[479:448] := SRC1[447:416]
///     TMP_DEST[511:480] := TMP_SRC2[447:416]
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
/// UNPCKLPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// DEST[159:128] := SRC1[159:128]
/// DEST[191:160] := SRC2[159:128]
/// DEST[223:192] := SRC1[191:160]
/// DEST[255:224] := SRC2[191:160]
/// DEST[MAXVL-1:256] := 0
/// VUNPCKLPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// DEST[MAXVL-1:128] := 0
/// UNPCKLPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[31:0]
/// DEST[63:32] := SRC2[31:0]
/// DEST[95:64] := SRC1[63:32]
/// DEST[127:96] := SRC2[63:32]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn unpcklps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}
