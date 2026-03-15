use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///         THEN
///             #UD;
///         ELSE
///             old_AL := AL;
///             old_CF := CF;
///             CF := 0;
///             IF (((AL AND 0FH) > 9) or AF = 1)
///                 THEN
///                     AL := AL + 6;
///                     CF := old_CF or (Carry from AL := AL + 6);
///                     AF := 1;
/// ELSE
/// 
/// 
///                     AF := 0;
///             FI;
///             IF ((old_AL > 99H) or (old_CF = 1))
///     THEN
///                     AL := AL + 60H;
/// 
///                     CF := 1;
///                 ELSE
///                     CF := 0;
///             FI;
///     FI;
///     Example
///     ADD AL, BL Before: AL=79H BL=35H EFLAGS(OSZAPC)=XXXXXX
///                     After: AL=AEH BL=35H EFLAGS(0SZAPC)=110000
///     DAA
///                     Before: AL=AEH BL=35H EFLAGS(OSZAPC)=110000
///                     After: AL=14H BL=35H EFLAGS(0SZAPC)=X00111
///     DAA
///                     Before: AL=2EH BL=35H EFLAGS(OSZAPC)=110000
///                     After: AL=34H BL=35H EFLAGS(0SZAPC)=X00101
/// ```
#[box_to_static_reference]
pub(super) fn daa() -> &'static [IrStatement] {
    [exception("daa")].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///     THEN
///         #UD;
///     ELSE
///         old_AL := AL;
///         old_CF := CF;
///         CF := 0;
///         IF (((AL AND 0FH) > 9) or AF = 1)
///         THEN
///                 AL := AL - 6;
/// 
///                 CF := old_CF or (Borrow from AL := AL - 6);
///                 AF := 1;
///             ELSE
///                 AF := 0;
///         FI;
///         IF ((old_AL > 99H) or (old_CF = 1))
///             THEN
///                 AL := AL - 60H;
///                 CF := 1;
///         FI;
/// FI;
/// Example
/// SUB AL, BL Before: AL = 35H, BL = 47H, EFLAGS(OSZAPC) = XXXXXX
///                 After: AL = EEH, BL = 47H, EFLAGS(0SZAPC) = 010111
/// DAA
///                 Before: AL = EEH, BL = 47H, EFLAGS(OSZAPC) = 010111
///                 After: AL = 88H, BL = 47H, EFLAGS(0SZAPC) = X10111
/// ```
#[box_to_static_reference]
pub(super) fn das() -> &'static [IrStatement] {
    let stmt_0 = assign(al.clone(), o1(), o1_size());
    let stmt_1 = assign(cf.clone(), o1(), o1_size());
    let stmt_2 = assign(c(0), cf.clone(), size_relative(cf.clone()));
    let stmt_3 = assign(b::sub(al.clone(), c(6)), al.clone(), size_relative(al.clone()));
    let stmt_4 = assign(b::or(unknown_data(), unknown_data()), cf.clone(), size_relative(cf.clone()));
    let stmt_5 = assign(c(1), af.clone(), size_relative(af.clone()));
    let stmt_6 = assign(c(0), af.clone(), size_relative(af.clone()));
    [stmt_0, stmt_1, stmt_2, stmt_3, stmt_4, stmt_5, stmt_6].into()
}

/// # Pseudocode
/// ```text
/// DEST := DEST - 1;
/// ```
#[box_to_static_reference]
pub(super) fn dec() -> &'static [IrStatement] {
    let sub = b::sub(o1(), c(1));
    let calc_flags = calc_flags_automatically(sub.clone(), o1_size(), &[&of, &sf, &zf, &af, &pf]);
    let assignment = assign(sub, o1(), o1_size());
    [calc_flags, assignment].into()
}

/// # Pseudocode
/// ```text
/// IF SRC = 0
///     THEN #DE; FI; (* Divide Error *)
/// IF OperandSize = 8 (* Word/Byte Operation *)
///     THEN
///         temp := AX / SRC;
///         IF temp > FFH
///             THEN #DE; (* Divide error *)
///             ELSE
///                 AL := temp;
///                 AH := AX MOD SRC;
///         FI;
///     ELSE IF OperandSize = 16 (* Doubleword/word operation *)
///         THEN
///             temp := DX:AX / SRC;
///             IF temp > FFFFH
///                 THEN #DE; (* Divide error *)
///             ELSE
///                 AX := temp;
///                 DX := DX:AX MOD SRC;
///             FI;
///         FI;
///     ELSE IF Operandsize = 32 (* Quadword/doubleword operation *)
///         THEN
///             temp := EDX:EAX / SRC;
///             IF temp > FFFFFFFFH
///                 THEN #DE; (* Divide error *)
///             ELSE
///                 EAX := temp;
///                 EDX := EDX:EAX MOD SRC;
///             FI;
///         FI;
///     ELSE IF 64-Bit Mode and Operandsize = 64 (* Doublequadword/quadword operation *)
///         THEN
///             temp := RDX:RAX / SRC;
///             IF temp > FFFFFFFFFFFFFFFFH
///                 THEN #DE; (* Divide error *)
///             ELSE
///                 RAX := temp;
///                 RDX := RDX:RAX MOD SRC;
///             FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn div() -> &'static [IrStatement] {
    let operand_bit_size = bit_size_of_o1();
    let div_8 = [assign(b::unsigned_div(ax.clone(), o1()), al.clone(), o1_size()), assign(b::unsigned_rem(ax.clone(), o1()), ah.clone(), o1_size())];
    let value = b::add(b::shl(sized(rdx.clone(), o1_size()), operand_bit_size.clone()), sized(rax.clone(), o1_size()));
    let div_etc = [assign(b::unsigned_div(value.clone(), o1()), rax.clone(), o1_size()), assign(b::unsigned_rem(value, o1()), rdx.clone(), o1_size())];
    let div = condition(b::equal(operand_bit_size, c(8), size_unlimited()), div_8, div_etc);
    extend_undefined_flags(&[div], &[&of, &sf, &zf, &af, &cf, &pf])
}

/// # Pseudocode
/// ```text
/// VDIVPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);  ; refer to Table 15-4 in the Intel® 64 and IA-32 Architectures
/// Software Developer's Manual, Volume 1
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+63:i] / SRC2[63:0]
///                     ELSE
///                         DEST[i+63:i] := SRC1[i+63:i] / SRC2[i+63:i]
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
/// VDIVPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] / SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] / SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] / SRC2[191:128]
/// DEST[255:192] := SRC1[255:192] / SRC2[255:192]
/// DEST[MAXVL-1:256] := 0;
/// VDIVPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] / SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] / SRC2[127:64]
/// DEST[MAXVL-1:128] := 0;
/// DIVPD (128-bit Legacy SSE Version)
/// DEST[63:0] := SRC1[63:0] / SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] / SRC2[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn divpd() -> &'static [IrStatement] {
    let assignment = assign(b::unsigned_div(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VDIVPS (EVEX Encoded Versions)
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
///                         DEST[i+31:i] := SRC1[i+31:i] / SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC1[i+31:i] / SRC2[i+31:i]
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
/// VDIVPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] / SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] / SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] / SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] / SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] / SRC2[159:128]
/// DEST[191:160] := SRC1[191:160] / SRC2[191:160]
/// DEST[223:192] := SRC1[223:192] / SRC2[223:192]
/// DEST[255:224] := SRC1[255:224] / SRC2[255:224].
/// DEST[MAXVL-1:256] := 0;
/// VDIVPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] / SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] / SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] / SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] / SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// DIVPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[31:0] / SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] / SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] / SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] / SRC2[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn divps() -> &'static [IrStatement] {
    let assignment = assign(b::unsigned_div(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VDIVSD (EVEX Encoded Version)
/// IF (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := SRC1[63:0] / SRC2[63:0]
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
/// VDIVSD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] / SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// DIVSD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] / SRC[63:0]
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn divsd() -> &'static [IrStatement] {
    let assignment = assign(b::unsigned_div(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VDIVSS (EVEX Encoded Version)
/// IF (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := SRC1[31:0] / SRC2[31:0]
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
/// VDIVSS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] / SRC2[31:0]
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// DIVSS (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0] / SRC[31:0]
/// DEST[MAXVL-1:32] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn divss() -> &'static [IrStatement] {
    let assignment = assign(b::unsigned_div(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DP_primitive (SRC1, SRC2)
/// IF (imm8[4] = 1)
///     THEN Temp1[63:0] := DEST[63:0] * SRC[63:0]; // update SIMD exception flags
///     ELSE Temp1[63:0] := +0.0; FI;
/// IF (imm8[5] = 1)
///     THEN Temp1[127:64] := DEST[127:64] * SRC[127:64]; // update SIMD exception flags
///     ELSE Temp1[127:64] := +0.0; FI;
/// /* if unmasked exception reported, execute exception handler*/
/// Temp2[63:0] := Temp1[63:0] + Temp1[127:64]; // update SIMD exception flags
/// /* if unmasked exception reported, execute exception handler*/
/// IF (imm8[0] = 1)
///     THEN DEST[63:0] := Temp2[63:0];
///     ELSE DEST[63:0] := +0.0; FI;
/// IF (imm8[1] = 1)
///     THEN DEST[127:64] := Temp2[63:0];
///     ELSE DEST[127:64] := +0.0; FI;
/// DPPD (128-bit Legacy SSE Version)
/// DEST[127:0] := DP_Primitive(SRC1[127:0], SRC2[127:0]);
/// DEST[MAXVL-1:128] (Unmodified)
/// VDPPD (VEX.128 Encoded Version)
/// DEST[127:0] := DP_Primitive(SRC1[127:0], SRC2[127:0]);
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn dppd() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// DP_primitive (SRC1, SRC2)
/// IF (imm8[4] = 1)
///     THEN Temp1[31:0] := DEST[31:0] * SRC[31:0]; // update SIMD exception flags
///     ELSE Temp1[31:0] := +0.0; FI;
/// IF (imm8[5] = 1)
///     THEN Temp1[63:32] := DEST[63:32] * SRC[63:32]; // update SIMD exception flags
///     ELSE Temp1[63:32] := +0.0; FI;
/// IF (imm8[6] = 1)
///     THEN Temp1[95:64] := DEST[95:64] * SRC[95:64]; // update SIMD exception flags
///     ELSE Temp1[95:64] := +0.0; FI;
/// IF (imm8[7] = 1)
///     THEN Temp1[127:96] := DEST[127:96] * SRC[127:96]; // update SIMD exception flags
///     ELSE Temp1[127:96] := +0.0; FI;
/// Temp2[31:0] := Temp1[31:0] + Temp1[63:32]; // update SIMD exception flags
/// /* if unmasked exception reported, execute exception handler*/
/// Temp3[31:0] := Temp1[95:64] + Temp1[127:96]; // update SIMD exception flags
/// /* if unmasked exception reported, execute exception handler*/
/// Temp4[31:0] := Temp2[31:0] + Temp3[31:0]; // update SIMD exception flags
/// /* if unmasked exception reported, execute exception handler*/
/// IF (imm8[0] = 1)
///     THEN DEST[31:0] := Temp4[31:0];
///     ELSE DEST[31:0] := +0.0; FI;
/// IF (imm8[1] = 1)
///     THEN DEST[63:32] := Temp4[31:0];
///     ELSE DEST[63:32] := +0.0; FI;
/// IF (imm8[2] = 1)
///     THEN DEST[95:64] := Temp4[31:0];
///     ELSE DEST[95:64] := +0.0; FI;
/// IF (imm8[3] = 1)
///     THEN DEST[127:96] := Temp4[31:0];
///     ELSE DEST[127:96] := +0.0; FI;
/// DPPS (128-bit Legacy SSE Version)
/// DEST[127:0] := DP_Primitive(SRC1[127:0], SRC2[127:0]);
/// DEST[MAXVL-1:128] (Unmodified)
/// VDPPS (VEX.128 Encoded Version)
/// DEST[127:0] := DP_Primitive(SRC1[127:0], SRC2[127:0]);
/// DEST[MAXVL-1:128] := 0
/// VDPPS (VEX.256 Encoded Version)
/// DEST[127:0] := DP_Primitive(SRC1[127:0], SRC2[127:0]);
/// DEST[255:128] := DP_Primitive(SRC1[255:128], SRC2[255:128]);
/// ```
#[box_to_static_reference]
pub(super) fn dpps() -> &'static [IrStatement] {
    let assignment = assign(b::mul(o2(), o3()), o1(), o1_size());
    [assignment].into()
}
