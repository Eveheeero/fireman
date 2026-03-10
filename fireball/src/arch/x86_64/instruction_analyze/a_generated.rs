use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///     THEN
///         #UD;
///     ELSE
///         IF ((AL AND 0FH) > 9) or (AF = 1)
///             THEN
///                 AX := AX + 106H;
///                 AF := 1;
///                 CF := 1;
///             ELSE
///                 AF := 0;
///                 CF := 0;
///         FI;
///         AL := AL AND 0FH;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn aaa() -> &'static [IrStatement] {
    let stmt_0 = assign(b::add(ax.clone(), c(0x106)), ax.clone(), size_relative(ax.clone()));
    let stmt_1 = assign(c(1), af.clone(), size_relative(af.clone()));
    let stmt_2 = assign(c(1), cf.clone(), size_relative(cf.clone()));
    let stmt_3 = assign(c(0), af.clone(), size_relative(af.clone()));
    let stmt_4 = assign(c(0), cf.clone(), size_relative(cf.clone()));
    [stmt_0, stmt_1, stmt_2, stmt_3, stmt_4].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///     THEN
///         #UD;
///     ELSE
///         tempAL := AL;
///         tempAH := AH;
///         AL := (tempAL + (tempAH * imm8)) AND FFH;
///         (* imm8 is set to 0AH for the AAD mnemonic.*)
///         AH := 0;
/// FI;
/// The immediate value (imm8) is taken from the second byte of the instruction.
/// ```
#[box_to_static_reference]
pub(super) fn aad() -> &'static [IrStatement] {
    let v_0 = al.clone();
    let v_1 = ah.clone();
    let stmt_2 = assign(b::and(b::add(v_0, b::mul(v_1, o2())), unknown_data()), al.clone(), size_relative(al.clone()));
    let stmt_3 = assign(c(0), ah.clone(), size_relative(ah.clone()));
    [stmt_2, stmt_3].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode
///     THEN
///         #UD;
///     ELSE
///         tempAL := AL;
///         AH := tempAL / imm8; (* imm8 is set to 0AH for the AAM mnemonic *)
///         AL := tempAL MOD imm8;
/// FI;
/// The immediate value (imm8) is taken from the second byte of the instruction.
/// ```
#[box_to_static_reference]
pub(super) fn aam() -> &'static [IrStatement] {
    let v_0 = al.clone();
    let stmt_1 = assign(b::unsigned_div(v_0, o2()), ah.clone(), size_relative(ah.clone()));
    let stmt_2 = assign(b::unsigned_rem(v_0, o2()), al.clone(), size_relative(al.clone()));
    [stmt_1, stmt_2].into()
}

/// # Pseudocode
/// ```text
/// IF 64-bit mode
///     THEN
///         #UD;
///     ELSE
///         IF ((AL AND 0FH) > 9) or (AF = 1)
///             THEN
///                 AX := AX - 6;
///                 AH := AH - 1;
///                 AF := 1;
///                 CF := 1;
///                 AL := AL AND 0FH;
///             ELSE
///                 CF := 0;
///                 AF := 0;
///                 AL := AL AND 0FH;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn aas() -> &'static [IrStatement] {
    let stmt_0 = assign(b::sub(ax.clone(), c(6)), ax.clone(), size_relative(ax.clone()));
    let stmt_1 = assign(b::sub(ah.clone(), c(1)), ah.clone(), size_relative(ah.clone()));
    let stmt_2 = assign(c(1), af.clone(), size_relative(af.clone()));
    let stmt_3 = assign(c(1), cf.clone(), size_relative(cf.clone()));
    let stmt_4 = assign(b::and(al.clone(), c(0x0F)), al.clone(), size_relative(al.clone()));
    let stmt_5 = assign(c(0), cf.clone(), size_relative(cf.clone()));
    let stmt_6 = assign(c(0), af.clone(), size_relative(af.clone()));
    let stmt_7 = assign(b::and(al.clone(), c(0x0F)), al.clone(), size_relative(al.clone()));
    [stmt_0, stmt_1, stmt_2, stmt_3, stmt_4, stmt_5, stmt_6, stmt_7].into()
}

/// # Pseudocode
/// ```text
/// DEST := DEST + SRC + CF;
/// ```
#[box_to_static_reference]
pub(super) fn adc() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let add = b::add(add, u::zero_extend(cf.clone()));
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(cf.clone(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1, type2, type3].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize is 64-bit
///     THEN CF:DEST[63:0] := DEST[63:0] + SRC[63:0] + CF;
///     ELSE CF:DEST[31:0] := DEST[31:0] + SRC[31:0] + CF;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn adcx() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let add = b::add(add, u::zero_extend(cf.clone()));
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(cf.clone(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1, type2, type3].into()
}

/// # Pseudocode
/// ```text
/// DEST := DEST + SRC;
/// ```
#[box_to_static_reference]
pub(super) fn add() -> &'static [IrStatement] {
    let op = b::add(o1(), o2());
    let assignment = assign(op.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(op, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, assignment, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// VADDPD (EVEX Encoded Versions) When SRC2 Operand is a Vector Register
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
///         THEN DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
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
/// VADDPD (EVEX Encoded Versions) When SRC2 Operand is a Memory Source
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+63:i] + SRC2[63:0]
///                     ELSE
///                         DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]
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
/// VADDPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] + SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] + SRC2[191:128]
/// DEST[255:192] := SRC1[255:192] + SRC2[255:192]
/// DEST[MAXVL-1:256] := 0
/// .
/// VADDPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] + SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// ADDPD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] + SRC[63:0]
/// DEST[127:64] := DEST[127:64] + SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn addpd() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VADDPS (EVEX Encoded Versions) When SRC2 Operand is a Register
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
///         THEN DEST[i+31:i] := SRC1[i+31:i] + SRC2[i+31:i]
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
/// VADDPS (EVEX Encoded Versions) When SRC2 Operand is a Memory Source
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b = 1)
///                     THEN
///                         DEST[i+31:i] := SRC1[i+31:i] + SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC1[i+31:i] + SRC2[i+31:i]
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
/// DEST[MAXVL-1:VL] := 0
/// VADDPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] + SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] + SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] + SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] + SRC2[159:128]
/// DEST[191:160]:= SRC1[191:160] + SRC2[191:160]
/// DEST[223:192] := SRC1[223:192] + SRC2[223:192]
/// DEST[255:224] := SRC1[255:224] + SRC2[255:224].
/// DEST[MAXVL-1:256] := 0
/// VADDPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] + SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] + SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] + SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// ADDPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] + SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] + SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] + SRC2[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn addps() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VADDSD (EVEX Encoded Version)
/// IF (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := SRC1[63:0] + SRC2[63:0]
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
/// VADDSD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] + SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ADDSD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] + SRC[63:0]
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn addsd() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VADDSS (EVEX Encoded Versions)
/// IF (EVEX.b = 1) AND SRC2 *is a register*
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := SRC1[31:0] + SRC2[31:0]
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
/// VADDSS DEST, SRC1, SRC2 (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] + SRC2[31:0]
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ADDSS DEST, SRC (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0] + SRC[31:0]
/// DEST[MAXVL-1:32] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn addss() -> &'static [IrStatement] {
    let assignment = assign(b::add(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ADDSUBPD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] - SRC[63:0]
/// DEST[127:64] := DEST[127:64] + SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// VADDSUBPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] - SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] + SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// VADDSUBPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] - SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] + SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] - SRC2[191:128]
/// DEST[255:192] := SRC1[255:192] + SRC2[255:192]
/// ```
#[box_to_static_reference]
pub(super) fn addsubpd() -> &'static [IrStatement] {
    [exception("addsubpd")].into()
}

/// # Pseudocode
/// ```text
/// ADDSUBPS (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0] - SRC[31:0]
/// DEST[63:32] := DEST[63:32] + SRC[63:32]
/// DEST[95:64] := DEST[95:64] - SRC[95:64]
/// DEST[127:96] := DEST[127:96] + SRC[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// VADDSUBPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] - SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] + SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] - SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] + SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// VADDSUBPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] - SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] + SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] - SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] + SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] - SRC2[159:128]
/// DEST[191:160] := SRC1[191:160] + SRC2[191:160]
/// DEST[223:192] := SRC1[223:192] - SRC2[223:192]
/// DEST[255:224] := SRC1[255:224] + SRC2[255:224]
/// ```
#[box_to_static_reference]
pub(super) fn addsubps() -> &'static [IrStatement] {
    [exception("addsubps")].into()
}

/// # Pseudocode
/// ```text
/// IF OperandSize is 64-bit
///     THEN OF:DEST[63:0] := DEST[63:0] + SRC[63:0] + OF;
///     ELSE OF:DEST[31:0] := DEST[31:0] + SRC[31:0] + OF;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn adox() -> &'static [IrStatement] {
    let size = o1_size();
    let add = b::add(o1(), o2());
    let add = b::add(add, u::zero_extend(cf.clone()));
    let assignment = assign(add.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(add, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(cf.clone(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1, type2, type3].into()
}

/// # Pseudocode
/// ```text
/// AESDEC
/// STATE := SRC1;
/// RoundKey := SRC2;
/// STATE := InvShiftRows( STATE );
/// STATE := InvSubBytes( STATE );
/// STATE := InvMixColumns( STATE );
/// DEST[127:0] := STATE XOR RoundKey;
/// DEST[MAXVL-1:128] (Unmodified)
/// VAESDEC (128b and 256b VEX Encoded Versions)
/// (KL,VL) = (1,128), (2,256)
/// FOR i = 0 to KL-1:
///     STATE := SRC1.xmm[i]
///     RoundKey := SRC2.xmm[i]
///     STATE := InvShiftRows( STATE )
///     STATE := InvSubBytes( STATE )
///     STATE := InvMixColumns( STATE )
///     DEST.xmm[i] := STATE XOR RoundKey
/// DEST[MAXVL-1:VL] := 0
/// VAESDEC (EVEX Encoded Version)
/// (KL,VL) = (1,128), (2,256), (4,512)
/// FOR i = 0 to KL-1:
///     STATE := SRC1.xmm[i]
///     RoundKey := SRC2.xmm[i]
///     STATE := InvShiftRows( STATE )
///     STATE := InvSubBytes( STATE )
///     STATE := InvMixColumns( STATE )
///     DEST.xmm[i] := STATE XOR RoundKey
/// DEST[MAXVL-1:VL] :=0
/// ```
#[box_to_static_reference]
pub(super) fn aesdec() -> &'static [IrStatement] {
    [exception("aesdec")].into()
}

/// # Pseudocode
/// ```text
/// AESDEC128KL
/// Handle := UnalignedLoad of 384 bit (SRC); // Load is not guaranteed to be atomic.
/// Illegal Handle = (HandleReservedBitSet (Handle) ||
///                 (Handle[0] AND (CPL > 0)) ||
///                 Handle [2] ||
///                 HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES128);
/// IF (Illegal Handle) {
///     THEN RFLAGS.ZF := 1;
///     ELSE
///         (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate384 (Handle[383:0], IWKey);
///         IF (Authentic == 0)
///             THEN RFLAGS.ZF := 1;
///             ELSE
///                     DEST := AES128Decrypt (DEST, UnwrappedKey) ;
///                     RFLAGS.ZF := 0;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn aesdec128kl() -> &'static [IrStatement] {
    [exception("aesdec128kl")].into()
}

/// # Pseudocode
/// ```text
/// AESDEC256KL
/// Handle := UnalignedLoad of 512 bit (SRC); // Load is not guaranteed to be atomic.
/// Illegal Handle = (HandleReservedBitSet (Handle) ||
///                 (Handle[0] AND (CPL > 0)) ||
///                 Handle [2] ||
///                 HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES256);
/// IF (Illegal Handle)
///     THEN RFLAGS.ZF := 1;
///     ELSE
///         (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate512 (Handle[511:0], IWKey);
///         IF (Authentic == 0)
///             THEN RFLAGS.ZF := 1;
///             ELSE
///                     DEST := AES256Decrypt (DEST, UnwrappedKey) ;
///                     RFLAGS.ZF := 0;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn aesdec256kl() -> &'static [IrStatement] {
    [exception("aesdec256kl")].into()
}

/// # Pseudocode
/// ```text
/// AESDECLAST
/// STATE := SRC1;
/// RoundKey := SRC2;
/// STATE := InvShiftRows( STATE );
/// STATE := InvSubBytes( STATE );
/// DEST[127:0] := STATE XOR RoundKey;
/// DEST[MAXVL-1:128] (Unmodified)
/// VAESDECLAST (128b and 256b VEX Encoded Versions)
/// (KL,VL) = (1,128), (2,256)
/// FOR i = 0 to KL-1:
///     STATE := SRC1.xmm[i]
///     RoundKey := SRC2.xmm[i]
///     STATE := InvShiftRows( STATE )
///     STATE := InvSubBytes( STATE )
///     DEST.xmm[i] := STATE XOR RoundKey
/// DEST[MAXVL-1:VL] := 0
/// VAESDECLAST (EVEX Encoded Version)
/// (KL,VL) = (1,128), (2,256), (4,512)
/// FOR i = 0 to KL-1:
///     STATE := SRC1.xmm[i]
///     RoundKey := SRC2.xmm[i]
///     STATE := InvShiftRows( STATE )
///     STATE := InvSubBytes( STATE )
///     DEST.xmm[i] := STATE XOR RoundKey
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn aesdeclast() -> &'static [IrStatement] {
    [exception("aesdeclast")].into()
}

/// # Pseudocode
/// ```text
/// AESDECWIDE128KL
/// Handle := UnalignedLoad of 384 bit (SRC);// Load is not guaranteed to be atomic.
/// Illegal Handle = (HandleReservedBitSet (Handle) ||
///                 (Handle[0] AND (CPL > 0)) ||
///                 Handle [2] ||
///                 HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES128);
/// IF (Illegal Handle)
///     THEN RFLAGS.ZF := 1;
///     ELSE
///         (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate384 (Handle[383:0], IWKey);
///         IF Authentic == 0 {
///             THEN RFLAGS.ZF := 1;
///             ELSE
///                     XMM0 := AES128Decrypt (XMM0, UnwrappedKey) ;
///                     XMM1 := AES128Decrypt (XMM1, UnwrappedKey) ;
///                     XMM2 := AES128Decrypt (XMM2, UnwrappedKey) ;
///                     XMM3 := AES128Decrypt (XMM3, UnwrappedKey) ;
///                     XMM4 := AES128Decrypt (XMM4, UnwrappedKey) ;
///                     XMM5 := AES128Decrypt (XMM5, UnwrappedKey) ;
///                     XMM6 := AES128Decrypt (XMM6, UnwrappedKey) ;
///                     XMM7 := AES128Decrypt (XMM7, UnwrappedKey) ;
///                     RFLAGS.ZF := 0;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn aesdecwide128kl() -> &'static [IrStatement] {
    [exception("aesdecwide128kl")].into()
}

/// # Pseudocode
/// ```text
/// AESDECWIDE256KL
/// Handle := UnalignedLoad of 512 bit (SRC); // Load is not guaranteed to be atomic.
/// Illegal Handle = (HandleReservedBitSet (Handle) ||
///                 (Handle[0] AND (CPL > 0)) ||
///                 Handle [2] ||
///                 HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES256);
/// IF (Illegal Handle) {
///     THEN RFLAGS.ZF := 1;
///     ELSE
///         (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate512 (Handle[511:0], IWKey);
///         IF (Authentic == 0)
///             THEN RFLAGS.ZF := 1;
///             ELSE
///                 XMM0 := AES256Decrypt (XMM0, UnwrappedKey) ;
///                 XMM1 := AES256Decrypt (XMM1, UnwrappedKey) ;
///                 XMM2 := AES256Decrypt (XMM2, UnwrappedKey) ;
///                 XMM3 := AES256Decrypt (XMM3, UnwrappedKey) ;
///                 XMM4 := AES256Decrypt (XMM4, UnwrappedKey) ;
///                 XMM5 := AES256Decrypt (XMM5, UnwrappedKey) ;
///                 XMM6 := AES256Decrypt (XMM6, UnwrappedKey) ;
///                 XMM7 := AES256Decrypt (XMM7, UnwrappedKey) ;
///                 RFLAGS.ZF := 0;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn aesdecwide256kl() -> &'static [IrStatement] {
    [exception("aesdecwide256kl")].into()
}

/// # Pseudocode
/// ```text
/// AESENC
/// STATE := SRC1;
/// RoundKey := SRC2;
/// STATE := ShiftRows( STATE );
/// STATE := SubBytes( STATE );
/// STATE := MixColumns( STATE );
/// DEST[127:0] := STATE XOR RoundKey;
/// DEST[MAXVL-1:128] (Unmodified)
/// VAESENC (128b and 256b VEX Encoded Versions)
/// (KL,VL) = (1,128), (2,256)
/// FOR I := 0 to KL-1:
///     STATE := SRC1.xmm[i]
///     RoundKey := SRC2.xmm[i]
///     STATE := ShiftRows( STATE )
///     STATE := SubBytes( STATE )
///     STATE := MixColumns( STATE )
///     DEST.xmm[i] := STATE XOR RoundKey
/// DEST[MAXVL-1:VL] := 0
/// VAESENC (EVEX Encoded Version)
/// (KL,VL) = (1,128), (2,256), (4,512)
/// FOR i := 0 to KL-1:
///     STATE := SRC1.xmm[i] // xmm[i] is the i'th xmm word in the SIMD register
///     RoundKey := SRC2.xmm[i]
///     STATE := ShiftRows( STATE )
///     STATE := SubBytes( STATE )
///     STATE := MixColumns( STATE )
///     DEST.xmm[i] := STATE XOR RoundKey
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn aesenc() -> &'static [IrStatement] {
    [exception("aesenc")].into()
}

/// # Pseudocode
/// ```text
/// AESENC128KL
/// Handle := UnalignedLoad of 384 bit (SRC); // Load is not guaranteed to be atomic.
/// Illegal Handle = (
///                 HandleReservedBitSet (Handle) ||
///                 (Handle[0] AND (CPL > 0)) ||
///                 Handle [1] ||
///                 HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES128
///                 );
/// IF (Illegal Handle) {
///     THEN RFLAGS.ZF := 1;
///     ELSE
///         (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate384 (Handle[383:0], IWKey);
///         IF (Authentic == 0)
///         THEN RFLAGS.ZF := 1;
///         ELSE
///             DEST := AES128Encrypt (DEST, UnwrappedKey) ;
///             RFLAGS.ZF := 0;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn aesenc128kl() -> &'static [IrStatement] {
    [exception("aesenc128kl")].into()
}

/// # Pseudocode
/// ```text
/// AESENC256KL
/// Handle := UnalignedLoad of 512 bit (SRC); // Load is not guaranteed to be atomic.
/// Illegal Handle = (
///                 HandleReservedBitSet (Handle) ||
///                 (Handle[0] AND (CPL > 0)) ||
///                 Handle [1] ||
///                 HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES256
///                 );
/// IF (Illegal Handle)
///     THEN RFLAGS.ZF := 1;
///     ELSE
///         (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate512 (Handle[511:0], IWKey);
///         IF (Authentic == 0)
///             THEN RFLAGS.ZF := 1;
///             ELSE
///                     DEST := AES256Encrypt (DEST, UnwrappedKey) ;
///                     RFLAGS.ZF := 0;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn aesenc256kl() -> &'static [IrStatement] {
    [exception("aesenc256kl")].into()
}

/// # Pseudocode
/// ```text
/// AESENCLAST
/// STATE := SRC1;
/// RoundKey := SRC2;
/// STATE := ShiftRows( STATE );
/// STATE := SubBytes( STATE );
/// DEST[127:0] := STATE XOR RoundKey;
/// DEST[MAXVL-1:128] (Unmodified)
/// VAESENCLAST (128b and 256b VEX Encoded Versions)
/// (KL, VL) = (1,128), (2,256)
/// FOR I=0 to KL-1:
///     STATE := SRC1.xmm[i]
///     RoundKey := SRC2.xmm[i]
///     STATE := ShiftRows( STATE )
///     STATE := SubBytes( STATE )
///     DEST.xmm[i] := STATE XOR RoundKey
/// DEST[MAXVL-1:VL] := 0
/// VAESENCLAST (EVEX Encoded Version)
/// (KL,VL) = (1,128), (2,256), (4,512)
/// FOR i = 0 to KL-1:
///     STATE := SRC1.xmm[i]
///     RoundKey := SRC2.xmm[i]
///     STATE := ShiftRows( STATE )
///     STATE := SubBytes( STATE )
///     DEST.xmm[i] := STATE XOR RoundKey
/// DEST[MAXVL-1:VL] := 0
/// ```
#[box_to_static_reference]
pub(super) fn aesenclast() -> &'static [IrStatement] {
    [exception("aesenclast")].into()
}

/// # Pseudocode
/// ```text
/// AESENCWIDE128KL
/// Handle := UnalignedLoad of 384 bit (SRC); // Load is not guaranteed to be atomic.
/// Illegal Handle = (
///                 HandleReservedBitSet (Handle) ||
///                 (Handle[0] AND (CPL > 0)) ||
///                 Handle [1] ||
///                 HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES128
///                 );
/// IF (Illegal Handle)
///     THEN RFLAGS.ZF := 1;
///     ELSE
///         (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate384 (Handle[383:0], IWKey);
///         IF Authentic == 0
///             THEN RFLAGS.ZF := 1;
///             ELSE
///             XMM0 := AES128Encrypt (XMM0, UnwrappedKey) ;
///                     XMM1 := AES128Encrypt (XMM1, UnwrappedKey) ;
///                     XMM2 := AES128Encrypt (XMM2, UnwrappedKey) ;
///                     XMM3 := AES128Encrypt (XMM3, UnwrappedKey) ;
///                     XMM4 := AES128Encrypt (XMM4, UnwrappedKey) ;
///                     XMM5 := AES128Encrypt (XMM5, UnwrappedKey) ;
///                     XMM6 := AES128Encrypt (XMM6, UnwrappedKey) ;
///                     XMM7 := AES128Encrypt (XMM7, UnwrappedKey) ;
///                     RFLAGS.ZF := 0;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// 1.Further details on Key Locker and usage of this instruction can be found here:
///     https://software.intel.com/content/www/us/en/develop/download/intel-key-locker-specification.html.
/// ```
#[box_to_static_reference]
pub(super) fn aesencwide128kl() -> &'static [IrStatement] {
    [exception("aesencwide128kl")].into()
}

/// # Pseudocode
/// ```text
/// AESENCWIDE256KL
/// Handle := UnalignedLoad of 512 bit (SRC); // Load is not guaranteed to be atomic.
/// Illegal Handle = (
///                 HandleReservedBitSet (Handle) ||
///                 (Handle[0] AND (CPL > 0)) ||
///                 Handle [1] ||
///                 HandleKeyType (Handle) != HANDLE_KEY_TYPE_AES256
///                 );
/// IF (Illegal Handle)
///     THEN RFLAGS.ZF := 1;
///     ELSE
///         (UnwrappedKey, Authentic) := UnwrapKeyAndAuthenticate512 (Handle[511:0], IWKey);
///         IF (Authentic == 0)
///             THEN RFLAGS.ZF := 1;
///             ELSE
///                     XMM0 := AES256Encrypt (XMM0, UnwrappedKey) ;
///                     XMM1 := AES256Encrypt (XMM1, UnwrappedKey) ;
///                     XMM2 := AES256Encrypt (XMM2, UnwrappedKey) ;
///                     XMM3 := AES256Encrypt (XMM3, UnwrappedKey) ;
///                     XMM4 := AES256Encrypt (XMM4, UnwrappedKey) ;
///                     XMM5 := AES256Encrypt (XMM5, UnwrappedKey) ;
///                     XMM6 := AES256Encrypt (XMM6, UnwrappedKey) ;
///                     XMM7 := AES256Encrypt (XMM7, UnwrappedKey) ;
///                     RFLAGS.ZF := 0;
///         FI;
/// FI;
/// RFLAGS.OF, SF, AF, PF, CF := 0;
/// 1.Further details on Key Locker and usage of this instruction can be found here:
///     https://software.intel.com/content/www/us/en/develop/download/intel-key-locker-specification.html.
/// ```
#[box_to_static_reference]
pub(super) fn aesencwide256kl() -> &'static [IrStatement] {
    [exception("aesencwide256kl")].into()
}

/// # Pseudocode
/// ```text
/// AESIMC
/// DEST[127:0] := InvMixColumns( SRC );
/// DEST[MAXVL-1:128] (Unmodified)
/// VAESIMC
/// DEST[127:0] := InvMixColumns( SRC );
/// DEST[MAXVL-1:128] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn aesimc() -> &'static [IrStatement] {
    [exception("aesimc")].into()
}

/// # Pseudocode
/// ```text
/// AESKEYGENASSIST
/// X3[31:0] := SRC [127: 96];
/// X2[31:0] := SRC [95: 64];
/// X1[31:0] := SRC [63: 32];
/// X0[31:0] := SRC [31: 0];
/// RCON[31:0] := ZeroExtend(imm8[7:0]);
/// DEST[31:0] := SubWord(X1);
/// DEST[63:32 ] := RotWord( SubWord(X1) ) XOR RCON;
/// DEST[95:64] := SubWord(X3);
/// DEST[127:96] := RotWord( SubWord(X3) ) XOR RCON;
/// DEST[MAXVL-1:128] (Unmodified)
/// VAESKEYGENASSIST
/// X3[31:0] := SRC [127: 96];
/// X2[31:0] := SRC [95: 64];
/// X1[31:0] := SRC [63: 32];
/// X0[31:0] := SRC [31: 0];
/// RCON[31:0] := ZeroExtend(imm8[7:0]);
/// DEST[31:0] := SubWord(X1);
/// DEST[63:32 ] := RotWord( SubWord(X1) ) XOR RCON;
/// DEST[95:64] := SubWord(X3);
/// DEST[127:96] := RotWord( SubWord(X3) ) XOR RCON;
/// DEST[MAXVL-1:128] := 0;
/// ```
#[box_to_static_reference]
pub(super) fn aeskeygenassist() -> &'static [IrStatement] {
    [exception("aeskeygenassist")].into()
}

/// # Pseudocode
/// ```text
/// DEST := DEST AND SRC;
/// ```
#[box_to_static_reference]
pub(super) fn and() -> &'static [IrStatement] {
    let op = b::and(o1(), o2());
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
/// DEST := (NOT SRC1) bitwiseAND SRC2;
/// SF := DEST[OperandSize -1];
/// ZF := (DEST = 0);
/// ```
#[box_to_static_reference]
pub(super) fn andn() -> &'static [IrStatement] {
    let op = b::and(u::not(o2()), o3());
    let assignment = assign(op.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(op, o1_size(), &[&sf, &zf, &pf]);
    let set_of = assign(c(0), of.clone(), size_relative(of.clone()));
    let set_cf = assign(c(0), cf.clone(), size_relative(cf.clone()));
    [calc_flags, set_of, set_cf, assignment].into()
}

/// # Pseudocode
/// ```text
/// VANDNPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///                 IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := (NOT(SRC1[i+63:i])) BITWISE AND SRC2[63:0]
///                     ELSE
///                         DEST[i+63:i] := (NOT(SRC1[i+63:i])) BITWISE AND SRC2[i+63:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] = 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VANDNPD (VEX.256 Encoded Version)
/// DEST[63:0] := (NOT(SRC1[63:0])) BITWISE AND SRC2[63:0]
/// DEST[127:64] := (NOT(SRC1[127:64])) BITWISE AND SRC2[127:64]
/// DEST[191:128] := (NOT(SRC1[191:128])) BITWISE AND SRC2[191:128]
/// DEST[255:192] := (NOT(SRC1[255:192])) BITWISE AND SRC2[255:192]
/// DEST[MAXVL-1:256] := 0
/// VANDNPD (VEX.128 Encoded Version)
/// DEST[63:0] := (NOT(SRC1[63:0])) BITWISE AND SRC2[63:0]
/// DEST[127:64] := (NOT(SRC1[127:64])) BITWISE AND SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// ANDNPD (128-bit Legacy SSE Version)
/// DEST[63:0] := (NOT(DEST[63:0])) BITWISE AND SRC[63:0]
/// DEST[127:64] := (NOT(DEST[127:64])) BITWISE AND SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn andnpd() -> &'static [IrStatement] {
    let assignment = assign(b::and(u::not(o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VANDNPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///                 IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+31:i] := (NOT(SRC1[i+31:i])) BITWISE AND SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := (NOT(SRC1[i+31:i])) BITWISE AND SRC2[i+31:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] = 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VANDNPS (VEX.256 Encoded Version)
/// DEST[31:0] := (NOT(SRC1[31:0])) BITWISE AND SRC2[31:0]
/// DEST[63:32] := (NOT(SRC1[63:32])) BITWISE AND SRC2[63:32]
/// DEST[95:64] := (NOT(SRC1[95:64])) BITWISE AND SRC2[95:64]
/// DEST[127:96] := (NOT(SRC1[127:96])) BITWISE AND SRC2[127:96]
/// DEST[159:128] := (NOT(SRC1[159:128])) BITWISE AND SRC2[159:128]
/// DEST[191:160] := (NOT(SRC1[191:160])) BITWISE AND SRC2[191:160]
/// DEST[223:192] := (NOT(SRC1[223:192])) BITWISE AND SRC2[223:192]
/// DEST[255:224] := (NOT(SRC1[255:224])) BITWISE AND SRC2[255:224].
/// DEST[MAXVL-1:256] := 0
/// VANDNPS (VEX.128 Encoded Version)
/// DEST[31:0] := (NOT(SRC1[31:0])) BITWISE AND SRC2[31:0]
/// DEST[63:32] := (NOT(SRC1[63:32])) BITWISE AND SRC2[63:32]
/// DEST[95:64] := (NOT(SRC1[95:64])) BITWISE AND SRC2[95:64]
/// DEST[127:96] := (NOT(SRC1[127:96])) BITWISE AND SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// ANDNPS (128-bit Legacy SSE Version)
/// DEST[31:0] := (NOT(DEST[31:0])) BITWISE AND SRC[31:0]
/// DEST[63:32] := (NOT(DEST[63:32])) BITWISE AND SRC[63:32]
/// DEST[95:64] := (NOT(DEST[95:64])) BITWISE AND SRC[95:64]
/// DEST[127:96] := (NOT(DEST[127:96])) BITWISE AND SRC[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn andnps() -> &'static [IrStatement] {
    let assignment = assign(b::and(u::not(o2()), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VANDPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///         THEN
///                 IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+63:i] BITWISE AND SRC2[63:0]
///                     ELSE
///                         DEST[i+63:i] := SRC1[i+63:i] BITWISE AND SRC2[i+63:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+63:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+63:i] = 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VANDPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] BITWISE AND SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] BITWISE AND SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] BITWISE AND SRC2[191:128]
/// DEST[255:192] := SRC1[255:192] BITWISE AND SRC2[255:192]
/// DEST[MAXVL-1:256] := 0
/// VANDPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] BITWISE AND SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] BITWISE AND SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// ANDPD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] BITWISE AND SRC[63:0]
/// DEST[127:64] := DEST[127:64] BITWISE AND SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn andpd() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VANDPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///                 IF (EVEX.b == 1) AND (SRC2 *is memory*)
///                     THEN
///                         DEST[i+63:i] := SRC1[i+31:i] BITWISE AND SRC2[31:0]
///                     ELSE
///                         DEST[i+31:i] := SRC1[i+31:i] BITWISE AND SRC2[i+31:i]
///                 FI;
///         ELSE
///                 IF *merging-masking*
///                             ; merging-masking
///                     THEN *DEST[i+31:i] remains unchanged*
///             ELSE ; zeroing-masking
///                         DEST[i+31:i] := 0
///                 FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0;
/// VANDPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] BITWISE AND SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] BITWISE AND SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] BITWISE AND SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] BITWISE AND SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] BITWISE AND SRC2[159:128]
/// DEST[191:160] := SRC1[191:160] BITWISE AND SRC2[191:160]
/// DEST[223:192] := SRC1[223:192] BITWISE AND SRC2[223:192]
/// DEST[255:224] := SRC1[255:224] BITWISE AND SRC2[255:224].
/// DEST[MAXVL-1:256] := 0;
/// VANDPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] BITWISE AND SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] BITWISE AND SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] BITWISE AND SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] BITWISE AND SRC2[127:96]
/// DEST[MAXVL-1:128] := 0;
/// ANDPS (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0] BITWISE AND SRC[31:0]
/// DEST[63:32] := DEST[63:32] BITWISE AND SRC[63:32]
/// DEST[95:64] := DEST[95:64] BITWISE AND SRC[95:64]
/// DEST[127:96] := DEST[127:96] BITWISE AND SRC[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn andps() -> &'static [IrStatement] {
    let assignment = assign(b::and(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF 64-BIT MODE
///     THEN
///         See MOVSXD;
///     ELSE
///         IF DEST[RPL] < SRC[RPL]
///             THEN
///                 ZF := 1;
///                 DEST[RPL] := SRC[RPL];
///             ELSE
///                 ZF := 0;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn arpl() -> &'static [IrStatement] {
    [exception("arpl")].into()
}
