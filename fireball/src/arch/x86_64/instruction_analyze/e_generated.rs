use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// x87FPUTagWord := FFFFH;
/// ```
#[box_to_static_reference]
pub(super) fn emms() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// ENCODEKEY128
/// #GP (0) if a reserved bit² in SRC[31:0] is set
/// InputKey[127:0] := XMM0;
/// KeyMetadata[2:0] = SRC[2:0];
/// KeyMetadata[23:3] = 0;
///     // Reserved for future usage
/// KeyMetadata[27:24] = 0;
///     // KeyType is AES-128 (value of 0)
/// KeyMetadata[127:28] = 0; // Reserved for future usage
/// // KeyMetadata is the AAD input and InputKey is the Plaintext input for WrapKey128
/// Handle[383:0] := WrapKey128(InputKey[127:0], KeyMetadata[127:0], IWKey.Integrity Key[127:0], IWKey.Encryption Key[255:0]);
/// DEST[0] := IWKey.NoBackup;
/// DEST[4:1] := IWKey.KeySource[3:0];
/// DEST[31:5] = 0;
/// XMM0 := Handle[127:0];  // AAD
/// XMM1 := Handle[255:128]; // Integrity Tag
/// XMM2 := Handle[383:256]; // CipherText
/// XMM4 := 0; // Reserved for future usage
/// XMM5 := 0; // Reserved for future usage
/// XMM6 := 0; // Reserved for future usage
/// RFLAGS.OF, SF, ZF, AF, PF, CF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn encodekey128() -> &'static [IrStatement] {
    [exception("encodekey128")].into()
}

/// # Pseudocode
/// ```text
/// ENCODEKEY256
/// #GP (0) if a reserved bit² in SRC[31:0] is set
/// InputKey[255:0] := XMM1:XMM0;
/// KeyMetadata[2:0] = SRC[2:0];
/// KeyMetadata[23:3] = 0; // Reserved for future usage
/// KeyMetadata[27:24] = 1; // KeyType is AES-256 (value of 1)
/// KeyMetadata[127:28] = 0; // Reserved for future usage
/// // KeyMetadata is the AAD input and InputKey is the Plaintext input for WrapKey256
/// Handle[511:0] := WrapKey256(InputKey[255:0], KeyMetadata[127:0], IWKey.Integrity Key[127:0], IWKey.Encryption Key[255:0]);
/// DEST[0] := IWKey.NoBackup;
/// DEST[4:1] := IWKey.KeySource[3:0];
/// DEST[31:5] = 0;
/// XMM0 := Handle[127:0];  // AAD
/// XMM1 := Handle[255:128]; // Integrity Tag
/// XMM2 := Handle[383:256]; // CipherText[127:0]
/// XMM3 := Handle[511:384]; // CipherText[255:128]
/// XMM4 := 0; // Reserved for future usage
/// XMM5 := 0; // Reserved for future usage
/// XMM6 := 0; // Reserved for future usage
/// RFLAGS.OF, SF, ZF, AF, PF, CF := 0;
/// 1.Further details on Key Locker and usage of this instruction can be found here:
///     https://software.intel.com/content/www/us/en/develop/download/intel-key-locker-specification.html.
/// 2.SRC[31:3] are currently reserved for future usages. SRC[2], which indicates a no-decrypt restriction, is reserved if
///     CPUID.19H:EAX[2] is 0. SRC[1], which indicates a no-encrypt restriction, is reserved if CPUID.19H:EAX[1] is 0. SRC[0], which indicates
///     a CPL0-only restriction, is reserved if CPUID.19H:EAX[0] is 0.
/// ```
#[box_to_static_reference]
pub(super) fn encodekey256() -> &'static [IrStatement] {
    [exception("encodekey256")].into()
}

/// # Pseudocode
/// ```text
/// IF EndbranchEnabled(CPL) & (IA32_EFER.LMA = 0 | (IA32_EFER.LMA=1 & CS.L = 0)
///     IF CPL = 3
///         THEN
///             IA32_U_CET.TRACKER = IDLE
///             IA32_U_CET.SUPPRESS = 0
///         ELSE
///             IA32_S_CET.TRACKER = IDLE
///             IA32_S_CET.SUPPRESS = 0
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn endbr32() -> &'static [IrStatement] {
    [exception("endbr32")].into()
}

/// # Pseudocode
/// ```text
/// IF EndbranchEnabled(CPL) & IA32_EFER.LMA = 1 & CS.L = 1
///     IF CPL = 3
///         THEN
///             IA32_U_CET.TRACKER = IDLE
///             IA32_U_CET.SUPPRESS = 0
///         ELSE
///             IA32_S_CET.TRACKER = IDLE
///             IA32_S_CET.SUPPRESS = 0
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn endbr64() -> &'static [IrStatement] {
    [exception("endbr64")].into()
}

/// # Pseudocode
/// ```text
/// IF IA32_PASID[31] = 0
///     THEN #GP;
/// ELSE
///     COMMAND := (SRC & ~FFFFFFFFH) | (IA32_PASID & FFFFFH);
///     DEST := COMMAND;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn enqcmd() -> &'static [IrStatement] {
    [exception("enqcmd")].into()
}

/// # Pseudocode
/// ```text
/// DEST := SRC;
/// ```
#[box_to_static_reference]
pub(super) fn enqcmds() -> &'static [IrStatement] {
    let stmt_0 = assign(o2(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// AllocSize := imm16;
/// NestingLevel := imm8 MOD 32;
/// IF (OperandSize = 64)
///     THEN
///         Push(RBP); (* RSP decrements by 8 *)
///         FrameTemp := RSP;
///     ELSE IF OperandSize = 32
///         THEN
///             Push(EBP); (* (E)SP decrements by 4 *)
///             FrameTemp := ESP; FI;
///     ELSE (* OperandSize = 16 *)
///             Push(BP); (* RSP or (E)SP decrements by 2 *)
///             FrameTemp := SP;
/// FI;
/// IF NestingLevel = 0
///     THEN GOTO CONTINUE;
/// FI;
/// IF (NestingLevel > 1)
///     THEN FOR i := 1 to (NestingLevel - 1)
///         DO
///             IF (OperandSize = 64)
///                 THEN
///                     RBP := RBP - 8;
///                     Push([RBP]); (* Quadword push *)
///                 ELSE IF OperandSize = 32
///                     THEN
///                         IF StackSize = 32
///                             EBP := EBP - 4;
///                             Push([EBP]); (* Doubleword push *)
///                         ELSE (* StackSize = 16 *)
///                             BP := BP - 4;
///                             Push([BP]); (* Doubleword push *)
///                         FI;
///                     FI;
///                 ELSE (* OperandSize = 16 *)
///                     IF StackSize = 64
///                         THEN
///                             RBP := RBP - 2;
///                             Push([RBP]); (* Word push *)
///                     ELSE IF StackSize = 32
///                         THEN
///                             EBP := EBP - 2;
///                             Push([EBP]); (* Word push *)
///                         ELSE (* StackSize = 16 *)
///                             BP := BP - 2;
///                             Push([BP]); (* Word push *)
///                     FI;
///                 FI;
///     OD;
/// FI;
/// IF (OperandSize = 64) (* nestinglevel 1 *)
///     THEN
///         Push(FrameTemp); (* Quadword push and RSP decrements by 8 *)
///     ELSE IF OperandSize = 32
///         THEN
///             Push(FrameTemp); FI; (* Doubleword push and (E)SP decrements by 4 *)
///     ELSE (* OperandSize = 16 *)
///             Push(FrameTemp); (* Word push and RSP|ESP|SP decrements by 2 *)
/// FI;
/// CONTINUE:
/// IF 64-Bit Mode (StackSize = 64)
///     THEN
///             RBP := FrameTemp;
///             RSP := RSP - AllocSize;
///     ELSE IF OperandSize = 32
///         THEN
///             EBP := FrameTemp;
///             ESP := ESP - AllocSize; FI;
///     ELSE (* OperandSize = 16 *)
///             BP := FrameTemp[15:1]; (* Bits 16 and above of applicable RBP/EBP are unmodified *)
///             SP := SP - AllocSize;
/// FI;
/// END;
/// ```
#[box_to_static_reference]
pub(super) fn enter() -> &'static [IrStatement] {
    let push_bp = assign(rbp.clone(), d(b::sub(rsp.clone(), architecture_byte_size())), size_architecture());
    let set_sp1 = assign(b::sub(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let set_bp = assign(rsp.clone(), rbp.clone(), size_architecture());
    let set_sp2 = assign(b::sub(rsp.clone(), u::zero_extend(o1())), rsp.clone(), size_architecture());
    [push_bp, set_sp1, set_bp, set_sp2].into()
}

/// # Pseudocode
/// ```text
/// VEXTRACTPS (EVEX and VEX.128 Encoded Version)
/// SRC_OFFSET := IMM8[1:0]
/// IF (64-Bit Mode and DEST is register)
///     DEST[31:0] := (SRC[127:0] >> (SRC_OFFSET*32)) AND 0FFFFFFFFh
///     DEST[63:32] := 0
/// ELSE
///     DEST[31:0] := (SRC[127:0] >> (SRC_OFFSET*32)) AND 0FFFFFFFFh
/// FI
/// EXTRACTPS (128-bit Legacy SSE Version)
/// SRC_OFFSET := IMM8[1:0]
/// IF (64-Bit Mode and DEST is register)
///     DEST[31:0] := (SRC[127:0] >> (SRC_OFFSET*32)) AND 0FFFFFFFFh
///     DEST[63:32] := 0
/// ELSE
///     DEST[31:0] := (SRC[127:0] >> (SRC_OFFSET*32)) AND 0FFFFFFFFh
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn extractps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}
