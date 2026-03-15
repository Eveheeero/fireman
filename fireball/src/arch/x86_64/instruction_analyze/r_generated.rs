use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// (* RCL and RCR Instructions *)
/// SIZE := OperandSize;
/// CASE (determine count) OF
///     SIZE := 8:tempCOUNT := (COUNT AND 1FH) MOD 9;
///     SIZE := 16:tempCOUNT := (COUNT AND 1FH) MOD 17;
///     SIZE := 32:tempCOUNT := COUNT AND 1FH;
///     SIZE := 64:tempCOUNT := COUNT AND 3FH;
/// ESAC;
/// IF OperandSize = 64
///     THEN COUNTMASK = 3FH;
///     ELSE COUNTMASK = 1FH;
/// FI;
/// (* RCL Instruction Operation *)
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := MSB(DEST);
///         DEST := (DEST * 2) + CF;
///         CF := tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// (* RCR Instruction Operation *)
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// FI;
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := LSB(SRC);
///         DEST := (DEST / 2) + (CF * 2SIZE);
///         CF := tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// (* ROL Instruction Operation *)
/// tempCOUNT := (COUNT & COUNTMASK) MOD SIZE
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := MSB(DEST);
///         DEST := (DEST * 2) + tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK)  ≠0
///     THEN CF := LSB(DEST);
/// FI;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// FI;
/// (* ROR Instruction Operation *)
/// tempCOUNT := (COUNT & COUNTMASK) MOD SIZE
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := LSB(SRC);
///         DEST := (DEST / 2) + (tempCF * 2SIZE);
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK)  ≠0
///     THEN CF := MSB(DEST);
/// FI;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR MSB - 1(DEST);
///     ELSE OF is undefined;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rcl() -> &'static [IrStatement] {
    let op = b::or(b::shl(o1(), o2()), b::shr(o1(), b::sub(bit_size_of_o1(), o2())));
    let assignment = assign(op, o1(), o1_size());
    extend_undefined_flags(&[assignment], &[&of, &cf])
}

/// # Pseudocode
/// ```text
/// RCPPS (128-bit Legacy SSE Version)
/// DEST[31:0] := APPROXIMATE(1/SRC[31:0])
/// DEST[63:32] := APPROXIMATE(1/SRC[63:32])
/// DEST[95:64] := APPROXIMATE(1/SRC[95:64])
/// DEST[127:96] := APPROXIMATE(1/SRC[127:96])
/// DEST[MAXVL-1:128] (Unmodified)
/// VRCPPS (VEX.128 Encoded Version)
/// DEST[31:0] := APPROXIMATE(1/SRC[31:0])
/// DEST[63:32] := APPROXIMATE(1/SRC[63:32])
/// DEST[95:64] := APPROXIMATE(1/SRC[95:64])
/// DEST[127:96] := APPROXIMATE(1/SRC[127:96])
/// DEST[MAXVL-1:128] := 0
/// VRCPPS (VEX.256 Encoded Version)
/// DEST[31:0] := APPROXIMATE(1/SRC[31:0])
/// DEST[63:32] := APPROXIMATE(1/SRC[63:32])
/// DEST[95:64] := APPROXIMATE(1/SRC[95:64])
/// DEST[127:96] := APPROXIMATE(1/SRC[127:96])
/// DEST[159:128] := APPROXIMATE(1/SRC[159:128])
/// DEST[191:160] := APPROXIMATE(1/SRC[191:160])
/// DEST[223:192] := APPROXIMATE(1/SRC[223:192])
/// DEST[255:224] := APPROXIMATE(1/SRC[255:224])
/// ```
#[box_to_static_reference]
pub(super) fn rcpps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RCPSS (128-bit Legacy SSE Version)
/// DEST[31:0] := APPROXIMATE(1/SRC[31:0])
/// DEST[MAXVL-1:32] (Unmodified)
/// VRCPSS (VEX.128 Encoded Version)
/// DEST[31:0] := APPROXIMATE(1/SRC2[31:0])
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn rcpss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// (* RCL and RCR Instructions *)
/// SIZE := OperandSize;
/// CASE (determine count) OF
///     SIZE := 8:tempCOUNT := (COUNT AND 1FH) MOD 9;
///     SIZE := 16:tempCOUNT := (COUNT AND 1FH) MOD 17;
///     SIZE := 32:tempCOUNT := COUNT AND 1FH;
///     SIZE := 64:tempCOUNT := COUNT AND 3FH;
/// ESAC;
/// IF OperandSize = 64
///     THEN COUNTMASK = 3FH;
///     ELSE COUNTMASK = 1FH;
/// FI;
/// (* RCL Instruction Operation *)
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := MSB(DEST);
///         DEST := (DEST * 2) + CF;
///         CF := tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// (* RCR Instruction Operation *)
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// FI;
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := LSB(SRC);
///         DEST := (DEST / 2) + (CF * 2SIZE);
///         CF := tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// (* ROL Instruction Operation *)
/// tempCOUNT := (COUNT & COUNTMASK) MOD SIZE
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := MSB(DEST);
///         DEST := (DEST * 2) + tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK)  ≠0
///     THEN CF := LSB(DEST);
/// FI;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// FI;
/// (* ROR Instruction Operation *)
/// tempCOUNT := (COUNT & COUNTMASK) MOD SIZE
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := LSB(SRC);
///         DEST := (DEST / 2) + (tempCF * 2SIZE);
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK)  ≠0
///     THEN CF := MSB(DEST);
/// FI;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR MSB - 1(DEST);
///     ELSE OF is undefined;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rcr() -> &'static [IrStatement] {
    let op = b::or(b::shr(o1(), o2()), b::shl(o1(), b::sub(bit_size_of_o1(), o2())));
    let assignment = assign(op, o1(), o1_size());
    extend_undefined_flags(&[assignment], &[&of, &cf])
}

/// # Pseudocode
/// ```text
/// DEST := FS/GS segment base address;
/// ```
#[box_to_static_reference]
pub(super) fn rdfsbase() -> &'static [IrStatement] {
    let stmt_0 = assign(b::unsigned_div(unknown_data(), unknown_data()), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// DEST := FS/GS segment base address;
/// ```
#[box_to_static_reference]
pub(super) fn rdgsbase() -> &'static [IrStatement] {
    let stmt_0 = assign(b::unsigned_div(unknown_data(), unknown_data()), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// EDX:EAX := MSR[ECX];
/// ```
#[box_to_static_reference]
pub(super) fn rdmsr() -> &'static [IrStatement] {
    [exception("RDMSR")].into()
}

/// # Pseudocode
/// ```text
/// DEST := IA32_TSC_AUX
/// ```
#[box_to_static_reference]
pub(super) fn rdpid() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// IF (ECX = 0)
///     THEN
///         EAX := PKRU;
///         EDX := 0;
///     ELSE #GP(0);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rdpkru() -> &'static [IrStatement] {
    let cond = condition(b::equal(ecx.clone(), c(0), o1_size()), [stmt_0, stmt_1], []);
    [cond].into()
}

/// # Pseudocode
/// ```text
/// MSCB = Most Significant Counter Bit (* Model-specific *)
/// IF (((CR4.PCE = 1) or (CPL = 0) or (CR0.PE = 0)) and (ECX indicates a supported counter))
///     THEN
///         EAX := counter[31:0];
///         EDX := ZeroExtend(counter[MSCB:32]);
///     ELSE (* ECX is not valid or CR4.PCE is 0 and CPL is 1, 2, or 3 and CR0.PE is 1 *)
///         #GP(0);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rdpmc() -> &'static [IrStatement] {
    [exception("RDPMC")].into()
}

/// # Pseudocode
/// ```text
/// IF HW_RND_GEN.ready = 1
///     THEN
///         CASE of
///             operand size is 64: DEST[63:0] := HW_RND_GEN.data;
///             operand size is 32: DEST[31:0] := HW_RND_GEN.data;
///             operand size is 16: DEST[15:0] := HW_RND_GEN.data;
///         ESAC
///         CF := 1;
///     ELSE
///         CASE of
///             operand size is 64: DEST[63:0] := 0;
///             operand size is 32: DEST[31:0] := 0;
///             operand size is 16: DEST[15:0] := 0;
///         ESAC
///         CF := 0;
/// FI
/// OF, SF, ZF, AF, PF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn rdrand() -> &'static [IrStatement] {
    [exception("rdrand")].into()
}

/// # Pseudocode
/// ```text
/// IF HW_NRND_GEN.ready = 1
///     THEN
///         CASE of
///             operand size is 64: DEST[63:0] := HW_NRND_GEN.data;
///             operand size is 32: DEST[31:0] := HW_NRND_GEN.data;
///             operand size is 16: DEST[15:0] := HW_NRND_GEN.data;
///         ESAC;
///         CF := 1;
///     ELSE
///         CASE of
///             operand size is 64: DEST[63:0] := 0;
///             operand size is 32: DEST[31:0] := 0;
///             operand size is 16: DEST[15:0] := 0;
///         ESAC;
///         CF := 0;
/// FI;
/// OF, SF, ZF, AF, PF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn rdseed() -> &'static [IrStatement] {
    [exception("rdseed")].into()
}

/// # Pseudocode
/// ```text
/// IF CPL = 3
///     IF CR4.CET & IA32_U_CET.SH_STK_EN
///         IF (operand size is 64 bit)
///             THEN
///                 Dest := SSP;
///             ELSE
///                 Dest := SSP[31:0];
///         FI;
///     FI;
/// ELSE
///     IF CR4.CET & IA32_S_CET.SH_STK_EN
///         IF (operand size is 64 bit)
///             THEN
///                 Dest := SSP;
///             ELSE
///                 Dest := SSP[31:0];
///         FI;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rdsspd() -> &'static [IrStatement] {
    [exception("rdsspd")].into()
}

/// # Pseudocode
/// ```text
/// IF CPL = 3
///     IF CR4.CET & IA32_U_CET.SH_STK_EN
///         IF (operand size is 64 bit)
///             THEN
///                 Dest := SSP;
///             ELSE
///                 Dest := SSP[31:0];
///         FI;
///     FI;
/// ELSE
///     IF CR4.CET & IA32_S_CET.SH_STK_EN
///         IF (operand size is 64 bit)
///             THEN
///                 Dest := SSP;
///             ELSE
///                 Dest := SSP[31:0];
///         FI;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rdsspq() -> &'static [IrStatement] {
    [exception("rdsspq")].into()
}

/// # Pseudocode
/// ```text
/// IF (CR4.TSD = 0) or (CPL = 0) or (CR0.PE = 0)
///     THEN EDX:EAX := TimeStampCounter;
///     ELSE (* CR4.TSD = 1 and (CPL = 1, 2, or 3) and CR0.PE = 1 *)
///         #GP(0);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rdtsc() -> &'static [IrStatement] {
    [exception("RDTSC")].into()
}

/// # Pseudocode
/// ```text
/// IF (CR4.TSD = 0) or (CPL = 0) or (CR0.PE = 0)
///     THEN
///         EDX:EAX := TimeStampCounter;
///         ECX := IA32_TSC_AUX[31:0];
///     ELSE (* CR4.TSD = 1 and (CPL = 1, 2, or 3) and CR0.PE = 1 *)
///         #GP(0);
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rdtscp() -> &'static [IrStatement] {
    [exception("RDTSCP")].into()
}

/// # Pseudocode
/// ```text
/// IF AddressSize = 16
/// THEN
/// Use CX for CountReg;
/// Implicit Source/Dest operand for memory use of SI/DI;
/// ELSE IF AddressSize = 64
/// THEN Use RCX for CountReg;
/// Implicit Source/Dest operand for memory use of RSI/RDI;
/// ELSE
/// Use ECX for CountReg;
/// Implicit Source/Dest operand for memory use of ESI/EDI;
/// FI;
/// WHILE CountReg ≠ 0
///     DO
///         Service pending interrupts (if any);
///         Execute associated string instruction;
///         CountReg := (CountReg - 1);
///         IF CountReg = 0
///             THEN exit WHILE loop; FI;
///         IF (Repeat prefix is REPZ or REPE) and (ZF = 0)
///         or (Repeat prefix is REPNZ or REPNE) and (ZF = 1)
///             THEN exit WHILE loop; FI;
///     OD;
/// ```
#[box_to_static_reference]
pub(super) fn rep() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF AddressSize = 16
/// THEN
/// Use CX for CountReg;
/// Implicit Source/Dest operand for memory use of SI/DI;
/// ELSE IF AddressSize = 64
/// THEN Use RCX for CountReg;
/// Implicit Source/Dest operand for memory use of RSI/RDI;
/// ELSE
/// Use ECX for CountReg;
/// Implicit Source/Dest operand for memory use of ESI/EDI;
/// FI;
/// WHILE CountReg ≠ 0
///     DO
///         Service pending interrupts (if any);
///         Execute associated string instruction;
///         CountReg := (CountReg - 1);
///         IF CountReg = 0
///             THEN exit WHILE loop; FI;
///         IF (Repeat prefix is REPZ or REPE) and (ZF = 0)
///         or (Repeat prefix is REPNZ or REPNE) and (ZF = 1)
///             THEN exit WHILE loop; FI;
///     OD;
/// ```
#[box_to_static_reference]
pub(super) fn repe() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF AddressSize = 16
/// THEN
/// Use CX for CountReg;
/// Implicit Source/Dest operand for memory use of SI/DI;
/// ELSE IF AddressSize = 64
/// THEN Use RCX for CountReg;
/// Implicit Source/Dest operand for memory use of RSI/RDI;
/// ELSE
/// Use ECX for CountReg;
/// Implicit Source/Dest operand for memory use of ESI/EDI;
/// FI;
/// WHILE CountReg ≠ 0
///     DO
///         Service pending interrupts (if any);
///         Execute associated string instruction;
///         CountReg := (CountReg - 1);
///         IF CountReg = 0
///             THEN exit WHILE loop; FI;
///         IF (Repeat prefix is REPZ or REPE) and (ZF = 0)
///         or (Repeat prefix is REPNZ or REPNE) and (ZF = 1)
///             THEN exit WHILE loop; FI;
///     OD;
/// ```
#[box_to_static_reference]
pub(super) fn repne() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF AddressSize = 16
/// THEN
/// Use CX for CountReg;
/// Implicit Source/Dest operand for memory use of SI/DI;
/// ELSE IF AddressSize = 64
/// THEN Use RCX for CountReg;
/// Implicit Source/Dest operand for memory use of RSI/RDI;
/// ELSE
/// Use ECX for CountReg;
/// Implicit Source/Dest operand for memory use of ESI/EDI;
/// FI;
/// WHILE CountReg ≠ 0
///     DO
///         Service pending interrupts (if any);
///         Execute associated string instruction;
///         CountReg := (CountReg - 1);
///         IF CountReg = 0
///             THEN exit WHILE loop; FI;
///         IF (Repeat prefix is REPZ or REPE) and (ZF = 0)
///         or (Repeat prefix is REPNZ or REPNE) and (ZF = 1)
///             THEN exit WHILE loop; FI;
///     OD;
/// ```
#[box_to_static_reference]
pub(super) fn repnz() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF AddressSize = 16
/// THEN
/// Use CX for CountReg;
/// Implicit Source/Dest operand for memory use of SI/DI;
/// ELSE IF AddressSize = 64
/// THEN Use RCX for CountReg;
/// Implicit Source/Dest operand for memory use of RSI/RDI;
/// ELSE
/// Use ECX for CountReg;
/// Implicit Source/Dest operand for memory use of ESI/EDI;
/// FI;
/// WHILE CountReg ≠ 0
///     DO
///         Service pending interrupts (if any);
///         Execute associated string instruction;
///         CountReg := (CountReg - 1);
///         IF CountReg = 0
///             THEN exit WHILE loop; FI;
///         IF (Repeat prefix is REPZ or REPE) and (ZF = 0)
///         or (Repeat prefix is REPNZ or REPNE) and (ZF = 1)
///             THEN exit WHILE loop; FI;
///     OD;
/// ```
#[box_to_static_reference]
pub(super) fn repz() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// (* Near return *)
/// IF instruction = near return
///     THEN;
///             IF OperandSize = 32
///                 THEN
///                     IF top 4 bytes of stack not within stack limits
///                         THEN #SS(0); FI;
///                     EIP := Pop();
///                     IF ShadowStackEnabled(CPL)
///                         tempSsEIP = ShadowStackPop4B();
///                         IF EIP != TempSsEIP
///                             THEN #CP(NEAR_RET); FI;
///                     FI;
///                 ELSE
///                     IF OperandSize = 64
///                         THEN
///                             IF top 8 bytes of stack not within stack limits
///                                 THEN #SS(0); FI;
///                             RIP := Pop();
///                             IF ShadowStackEnabled(CPL)
///                                 tempSsEIP = ShadowStackPop8B();
///                                 IF RIP != tempSsEIP
///                                     THEN #CP(NEAR_RET); FI;
///                             FI;
///                         ELSE (* OperandSize = 16 *)
///                             IF top 2 bytes of stack not within stack limits
///                                 THEN #SS(0); FI;
///                             tempEIP := Pop();
///                             tempEIP := tempEIP AND 0000FFFFH;
///                             IF tempEIP not within code segment limits
///                                 THEN #GP(0); FI;
///                             EIP := tempEIP;
///                             IF ShadowStackEnabled(CPL)
///                                 tempSsEip = ShadowStackPop4B();
///                                 IF EIP != tempSsEIP
///                                     THEN #CP(NEAR_RET); FI;
///                             FI;
///                     FI;
///             FI;
///     IF instruction has immediate operand
///             THEN (* Release parameters from stack *)
///                 IF StackAddressSize = 32
///                     THEN
///                         ESP := ESP + SRC;
///                     ELSE
///                         IF StackAddressSize = 64
///                             THEN
///                                 RSP := RSP + SRC;
///                             ELSE (* StackAddressSize = 16 *)
///                                 SP := SP + SRC;
///                         FI;
///                 FI;
///     FI;
/// FI;
/// (* Real-address mode or virtual-8086 mode *)
/// IF ((PE = 0) or (PE = 1 AND VM = 1)) and instruction = far return
///     THEN
///             IF OperandSize = 32
///                 THEN
///                     IF top 8 bytes of stack not within stack limits
///                         THEN #SS(0); FI;
///                     EIP := Pop();
///                     CS := Pop(); (* 32-bit pop, high-order 16 bits discarded *)
///                 ELSE (* OperandSize = 16 *)
///                     IF top 4 bytes of stack not within stack limits
///                         THEN #SS(0); FI;
///                     tempEIP := Pop();
///                     tempEIP := tempEIP AND 0000FFFFH;
///                     IF tempEIP not within code segment limits
///                         THEN #GP(0); FI;
///                     EIP := tempEIP;
///                     CS := Pop(); (* 16-bit pop *)
///             FI;
///     IF instruction has immediate operand
///             THEN (* Release parameters from stack *)
///                 SP := SP + (SRC AND FFFFH);
///     FI;
/// FI;
/// (* Protected mode, not virtual-8086 mode *)
/// IF (PE = 1 and VM = 0 and IA32_EFER.LMA = 0) and instruction = far return
///     THEN
///             IF OperandSize = 32
///                 THEN
///                     IF second doubleword on stack is not within stack limits
///                         THEN #SS(0); FI;
///                 ELSE (* OperandSize = 16 *)
///                     IF second word on stack is not within stack limits
///                         THEN #SS(0); FI;
///             FI;
///     IF return code segment selector is NULL
///             THEN #GP(0); FI;
///     IF return code segment selector addresses descriptor beyond descriptor table limit
///             THEN #GP(selector); FI;
///     Obtain descriptor to which return code segment selector points from descriptor table;
///     IF return code segment descriptor is not a code segment
///             THEN #GP(selector); FI;
///     IF return code segment selector RPL < CPL
///             THEN #GP(selector); FI;
///     IF return code segment descriptor is conforming
///     and return code segment DPL > return code segment selector RPL
///             THEN #GP(selector); FI;
///     IF return code segment descriptor is non-conforming and return code
///     segment DPL ≠ return code segment selector RPL
///             THEN #GP(selector); FI;
///     IF return code segment descriptor is not present
///             THEN #NP(selector); FI:
///     IF return code segment selector RPL > CPL
///             THEN GOTO RETURN-TO-OUTER-PRIVILEGE-LEVEL;
///             ELSE GOTO RETURN-TO-SAME-PRIVILEGE-LEVEL;
///     FI;
/// FI;
/// RETURN-TO-SAME-PRIVILEGE-LEVEL:
///     IF the return instruction pointer is not within the return code segment limit
///             THEN #GP(0); FI;
///     IF OperandSize = 32
///             THEN
///                 EIP := Pop();
///                 CS := Pop(); (* 32-bit pop, high-order 16 bits discarded *)
///             ELSE (* OperandSize = 16 *)
///                 EIP := Pop();
///                 EIP := EIP AND 0000FFFFH;
///                 CS := Pop(); (* 16-bit pop *)
///     FI;
///     IF instruction has immediate operand
///             THEN (* Release parameters from stack *)
///                 IF StackAddressSize = 32
///                     THEN
///                         ESP := ESP + SRC;
///                     ELSE (* StackAddressSize = 16 *)
///                         SP := SP + SRC;
///                 FI;
///     FI;
///     IF ShadowStackEnabled(CPL)
///             (* SSP must be 8 byte aligned *)
///             IF SSP AND 0x7 != 0
///                 THEN #CP(FAR-RET/IRET); FI;
///             tempSsCS = shadow_stack_load 8 bytes from SSP+16;
///             tempSsLIP = shadow_stack_load 8 bytes from SSP+8;
///             prevSSP = shadow_stack_load 8 bytes from SSP;
///             SSP = SSP + 24;
///             (* do a 64 bit-compare to check if any bits beyond bit 15 are set *)
///             tempCS = CS; (* zero pad to 64 bit *)
///             IF tempCS != tempSsCS
///                 THEN #CP(FAR-RET/IRET); FI;
///             (* do a 64 bit-compare; pad CSBASE+RIP with 0 for 32 bit LIP*)
///             IF CSBASE + RIP != tempSsLIP
///                 THEN #CP(FAR-RET/IRET); FI;
///             (* prevSSP must be 4 byte aligned *)
///             IF prevSSP AND 0x3 != 0
///                 THEN #CP(FAR-RET/IRET); FI;
///             (* In legacy mode SSP must be in low 4GB *)
///             IF prevSSP[63:32] != 0
///                 THEN #GP(0); FI;
///             SSP := prevSSP
///     FI;
/// RETURN-TO-OUTER-PRIVILEGE-LEVEL:
///     IF top (16 + SRC) bytes of stack are not within stack limits (OperandSize = 32)
///     or top (8 + SRC) bytes of stack are not within stack limits (OperandSize = 16)
///                 THEN #SS(0); FI;
///     Read return segment selector;
///     IF stack segment selector is NULL
///             THEN #GP(0); FI;
///     IF return stack segment selector index is not within its descriptor table limits
///             THEN #GP(selector); FI;
///     Read segment descriptor pointed to by return segment selector;
///     IF stack segment selector RPL ≠ RPL of the return code segment selector
///     or stack segment is not a writable data segment
///     or stack segment descriptor DPL ≠ RPL of the return code segment selector
///                 THEN #GP(selector); FI;
///     IF stack segment not present
///             THEN #SS(StackSegmentSelector); FI;
///     IF the return instruction pointer is not within the return code segment limit
///             THEN #GP(0); FI;
///     IF OperandSize = 32
///             THEN
///                 EIP := Pop();
///                 CS := Pop(); (* 32-bit pop, high-order 16 bits discarded; segment descriptor loaded *)
///                 CS(RPL) := ReturnCodeSegmentSelector(RPL);
///                 IF instruction has immediate operand
///                     THEN (* Release parameters from called procedure's stack *)
///                         IF StackAddressSize = 32
///                             THEN
///                                 ESP := ESP + SRC;
///                             ELSE (* StackAddressSize = 16 *)
///                                 SP := SP + SRC;
///                         FI;
///                 FI;
///                 tempESP := Pop();
///                 tempSS := Pop(); (* 32-bit pop, high-order 16 bits discarded; seg. descriptor loaded *)
///             ELSE (* OperandSize = 16 *)
///                 EIP := Pop();
///                 EIP := EIP AND 0000FFFFH;
///                 CS := Pop(); (* 16-bit pop; segment descriptor loaded *)
///                 CS(RPL) := ReturnCodeSegmentSelector(RPL);
///                 IF instruction has immediate operand
///                     THEN (* Release parameters from called procedure's stack *)
///                         IF StackAddressSize = 32
///                             THEN
///                                 ESP := ESP + SRC;
///                             ELSE (* StackAddressSize = 16 *)
///                                 SP := SP + SRC;
///                         FI;
///                 FI;
///                 tempESP := Pop();
///                 tempSS := Pop(); (* 16-bit pop; segment descriptor loaded *)
///             FI;
///     IF ShadowStackEnabled(CPL)
///             (* check if 8 byte aligned *)
///             IF SSP AND 0x7 != 0
///                 THEN #CP(FAR-RET/IRET); FI;
///             IF ReturnCodeSegmentSelector(RPL) !=3
///                 THEN
///                     tempSsCS = shadow_stack_load 8 bytes from SSP+16;
///                     tempSsLIP = shadow_stack_load 8 bytes from SSP+8;
///                     tempSSP = shadow_stack_load 8 bytes from SSP;
///                     SSP = SSP + 24;
///                     (* Do 64 bit compare to detect bits beyond 15 being set *)
///                     tempCS = CS; (* zero extended to 64 bit *)
///                     IF tempCS != tempSsCS
///                         THEN #CP(FAR-RET/IRET); FI;
///                     (* Do 64 bit compare; pad CSBASE+RIP with 0 for 32 bit LA *)
///                     IF CSBASE + RIP != tempSsLIP
///                         THEN #CP(FAR-RET/IRET); FI;
///                     (* check if 4 byte aligned *)
///                     IF tempSSP AND 0x3 != 0
///                         THEN #CP(FAR-RET/IRET); FI;
///             FI;
///     FI;
///             tempOldCPL = CPL;
///             CPL := ReturnCodeSegmentSelector(RPL);
///             ESP := tempESP;
///             SS := tempSS;
///             tempOldSSP = SSP;
///             IF ShadowStackEnabled(CPL)
///                 IF CPL = 3
///                     THEN tempSSP := IA32_PL3_SSP; FI;
///                 IF tempSSP[63:32] != 0
///                     THEN #GP(0); FI;
///                 SSP := tempSSP
///             FI;
///             (* Now past all faulting points; safe to free the token. The token free is done using the old SSP
///             * and using a supervisor override as old CPL was a supervisor privilege level *)
///             IF ShadowStackEnabled(tempOldCPL)
///                 expected_token_value = tempOldSSP | BUSY_BIT (* busy bit - bit position 0 - must be set *)
///                 new_token_value = tempOldSSP                      (* clear the busy bit *)
///                 shadow_stack_lock_cmpxchg8b(tempOldSSP, new_token_value, expected_token_value)
///             FI;
///     FI;
///     FOR each SegReg in (ES, FS, GS, and DS)
///             DO
///                 tempDesc := descriptor cache for SegReg (* hidden part of segment register *)
///                     THEN (* Segment register invalid *)
///                         SegmentSelector := 0; (*Segment selector becomes null*)
///                 FI;
///             OD;
///     IF instruction has immediate operand
///             THEN (* Release parameters from calling procedure's stack *)
///                 IF StackAddressSize = 32
///                     THEN
///                         ESP := ESP + SRC;
///                     ELSE (* StackAddressSize = 16 *)
///                         SP := SP + SRC;
///                 FI;
///     FI;
/// (* IA-32e Mode *)
///     IF (PE = 1 and VM = 0 and IA32_EFER.LMA = 1) and instruction = far return
///             THEN
///                 IF OperandSize = 32
///                     THEN
///                         IF second doubleword on stack is not within stack limits
///                             THEN #SS(0); FI;
///                         IF first or second doubleword on stack is not in canonical space
///                             THEN #SS(0); FI;
///                     ELSE
///                         IF OperandSize = 16
///                             THEN
///                                 IF second word on stack is not within stack limits
///                                     THEN #SS(0); FI;
///                                 IF first or second word on stack is not in canonical space
///                                     THEN #SS(0); FI;
///                             ELSE (* OperandSize = 64 *)
///                                 IF first or second quadword on stack is not in canonical space
///                                     THEN #SS(0); FI;
///                         FI
///                 FI;
///             IF return code segment selector is NULL
///                 THEN GP(0); FI;
///             IF return code segment selector addresses descriptor beyond descriptor table limit
///                 THEN GP(selector); FI;
///             IF return code segment selector addresses descriptor in non-canonical space
///                 THEN GP(selector); FI;
///             Obtain descriptor to which return code segment selector points from descriptor table;
///             IF return code segment descriptor is not a code segment
///                 THEN #GP(selector); FI;
///             IF return code segment descriptor has L-bit = 1 and D-bit = 1
///                 THEN #GP(selector); FI;
///             IF return code segment selector RPL < CPL
///                 THEN #GP(selector); FI;
///             IF return code segment descriptor is conforming
///             and return code segment DPL > return code segment selector RPL
///                 THEN #GP(selector); FI;
///             IF return code segment descriptor is non-conforming
///             and return code segment DPL ≠ return code segment selector RPL
///                 THEN #GP(selector); FI;
///             IF return code segment descriptor is not present
///                 THEN #NP(selector); FI:
///             IF return code segment selector RPL > CPL
///                 THEN GOTO IA-32E-MODE-RETURN-TO-OUTER-PRIVILEGE-LEVEL;
///                 ELSE GOTO IA-32E-MODE-RETURN-TO-SAME-PRIVILEGE-LEVEL;
///             FI;
///     FI;
/// IA-32E-MODE-RETURN-TO-SAME-PRIVILEGE-LEVEL:
/// IF the return instruction pointer is not within the return code segment limit
///     THEN #GP(0); FI;
/// IF the return instruction pointer is not within canonical address space
///     THEN #GP(0); FI;
/// IF OperandSize = 32
///     THEN
///             EIP := Pop();
///             CS := Pop(); (* 32-bit pop, high-order 16 bits discarded *)
///     ELSE
///             IF OperandSize = 16
///                 THEN
///                     EIP := Pop();
///                     EIP := EIP AND 0000FFFFH;
///                     CS := Pop(); (* 16-bit pop *)
///                 ELSE (* OperandSize = 64 *)
///                     RIP := Pop();
///                     CS := Pop(); (* 64-bit pop, high-order 48 bits discarded *)
///             FI;
/// FI;
/// IF instruction has immediate operand
///     THEN (* Release parameters from stack *)
///             IF StackAddressSize = 32
///                 THEN
///                     ESP := ESP + SRC;
///                 ELSE
///                     IF StackAddressSize = 16
///                         THEN
///                             SP := SP + SRC;
///                         ELSE (* StackAddressSize = 64 *)
///                             RSP := RSP + SRC;
///                     FI;
///             FI;
/// FI;
/// IF ShadowStackEnabled(CPL)
///     IF SSP AND 0x7 != 0 (* check if aligned to 8 bytes *)
///             THEN #CP(FAR-RET/IRET); FI;
///     tempSsCS = shadow_stack_load 8 bytes from SSP+16;
///     tempSsLIP = shadow_stack_load 8 bytes from SSP+8;
///     tempSSP = shadow_stack_load 8 bytes from SSP;
///     SSP = SSP + 24;
///     tempCS = CS; (* zero padded to 64 bit *)
///     IF tempCS != tempSsCS (* 64 bit compare; CS zero padded to 64 bits *)
///             THEN #CP(FAR-RET/IRET); FI;
///             THEN #CP(FAR-RET/IRET); FI;
///     IF tempSSP AND 0x3 != 0 (* check if aligned to 4 bytes *)
///             THEN #CP(FAR-RET/IRET); FI;
///     IF (CS.L = 0 AND tempSSP[63:32] != 0) OR
///         (CS.L = 1 AND tempSSP is not canonical relative to the current paging mode)
///             THEN #GP(0); FI;
///     SSP := tempSSP
/// FI;
/// IA-32E-MODE-RETURN-TO-OUTER-PRIVILEGE-LEVEL:
/// IF top (16 + SRC) bytes of stack are not within stack limits (OperandSize = 32)
/// or top (8 + SRC) bytes of stack are not within stack limits (OperandSize = 16)
///     THEN #SS(0); FI;
/// IF top (16 + SRC) bytes of stack are not in canonical address space (OperandSize =32)
/// or top (8 + SRC) bytes of stack are not in canonical address space (OperandSize = 16)
/// or top (32 + SRC) bytes of stack are not in canonical address space (OperandSize = 64)
///     THEN #SS(0); FI;
/// Read return stack segment selector;
/// IF stack segment selector is NULL
///     THEN
///             IF new CS descriptor L-bit = 0
///                 THEN #GP(selector);
///             IF stack segment selector RPL = 3
///                 THEN #GP(selector);
/// FI;
/// IF return stack segment descriptor is not within descriptor table limits
///             THEN #GP(selector); FI;
/// IF return stack segment descriptor is in non-canonical address space
///             THEN #GP(selector); FI;
/// Read segment descriptor pointed to by return segment selector;
/// IF stack segment selector RPL ≠ RPL of the return code segment selector
/// or stack segment is not a writable data segment
/// or stack segment descriptor DPL ≠ RPL of the return code segment selector
///     THEN #GP(selector); FI;
/// IF stack segment not present
///     THEN #SS(StackSegmentSelector); FI;
/// IF the return instruction pointer is not within the return code segment limit
///     THEN #GP(0); FI:
/// IF the return instruction pointer is not within canonical address space
///     THEN #GP(0); FI;
/// IF OperandSize = 32
///     THEN
///             EIP := Pop();
///             CS := Pop(); (* 32-bit pop, high-order 16 bits discarded, segment descriptor loaded *)
///             CS(RPL) := ReturnCodeSegmentSelector(RPL);
///             IF instruction has immediate operand
///                 THEN (* Release parameters from called procedure's stack *)
///                     IF StackAddressSize = 32
///                         THEN
///                             ESP := ESP + SRC;
///                         ELSE
///                             IF StackAddressSize = 16
///                                 THEN
///                                     SP := SP + SRC;
///                                 ELSE (* StackAddressSize = 64 *)
///                                     RSP := RSP + SRC;
///                             FI;
///                     FI;
///             FI;
///             tempESP := Pop();
///             tempSS := Pop(); (* 32-bit pop, high-order 16 bits discarded, segment descriptor loaded *)
///     ELSE
///             IF OperandSize = 16
///                 THEN
///                     EIP := Pop();
///                     EIP := EIP AND 0000FFFFH;
///                     CS := Pop(); (* 16-bit pop; segment descriptor loaded *)
///                     CS(RPL) := ReturnCodeSegmentSelector(RPL);
///                     IF instruction has immediate operand
///                         THEN (* Release parameters from called procedure's stack *)
///                             IF StackAddressSize = 32
///                                 THEN
///                                     ESP := ESP + SRC;
///                                 ELSE
///                                     IF StackAddressSize = 16
///                                         THEN
///                                         SP := SP + SRC;
///                                         ELSE (* StackAddressSize = 64 *)
///                                         RSP := RSP + SRC;
///                                     FI;
///                             FI;
///                     FI;
///                     tempESP := Pop();
///                     tempSS := Pop(); (* 16-bit pop; segment descriptor loaded *)
///                 ELSE (* OperandSize = 64 *)
///                     RIP := Pop();
///                     CS := Pop(); (* 64-bit pop; high-order 48 bits discarded; seg. descriptor loaded *)
///                     CS(RPL) := ReturnCodeSegmentSelector(RPL);
///                     IF instruction has immediate operand
///                         THEN (* Release parameters from called procedure's stack *)
///                             RSP := RSP + SRC;
///                     FI;
///                     tempESP := Pop();
///                     tempSS := Pop(); (* 64-bit pop; high-order 48 bits discarded; seg. desc. loaded *)
///             FI;
/// FI;
/// IF ShadowStackEnabled(CPL)
///     (* check if 8 byte aligned *)
///     IF SSP AND 0x7 != 0
///             THEN #CP(FAR-RET/IRET); FI;
///     IF ReturnCodeSegmentSelector(RPL) !=3
///             THEN
///                 tempSsCS = shadow_stack_load 8 bytes from SSP+16;
///                 tempSsLIP = shadow_stack_load 8 bytes from SSP+8;
///                 tempSSP = shadow_stack_load 8 bytes from SSP;
///                 SSP = SSP + 24;
///                 tempCS = CS; (* zero padded to 64 bit *)
///                 IF tempCS != tempSsCS
///                     THEN #CP(FAR-RET/IRET); FI;
///                 (* Do 64 bit compare; pad CSBASE+RIP with 0 for 32 bit LIP *)
///                 IF CSBASE + RIP != tempSsLIP
///                     THEN #CP(FAR-RET/IRET); FI;
///                 (* check if 4 byte aligned *)
///                 IF tempSSP AND 0x3 != 0
///                     THEN #CP(FAR-RET/IRET); FI;
///     FI;
/// FI;
/// tempOldCPL = CPL;
/// CPL := ReturnCodeSegmentSelector(RPL);
/// ESP := tempESP;
/// SS := tempSS;
/// tempOldSSP = SSP;
/// IF ShadowStackEnabled(CPL)
///     IF CPL = 3
///             THEN tempSSP := IA32_PL3_SSP; FI;
///     IF (CS.L = 0 AND tempSSP[63:32] != 0) OR
///         (CS.L = 1 AND tempSSP is not canonical relative to the current paging mode)
///             THEN #GP(0); FI;
///     SSP := tempSSP
/// FI;
/// (* Now past all faulting points; safe to free the token. The token free is done using the old SSP
/// * and using a supervisor override as old CPL was a supervisor privilege level *)
/// IF ShadowStackEnabled(tempOldCPL)
///     expected_token_value = tempOldSSP | BUSY_BIT
///                                         (* busy bit - bit position 0 - must be set *)
///     new_token_value = tempOldSSP
///                                         (* clear the busy bit *)
///     shadow_stack_lock_cmpxchg8b(tempOldSSP, new_token_value, expected_token_value)
/// FI;
/// FOR each of segment register (ES, FS, GS, and DS)
///     DO
///             IF segment register points to data or non-conforming code segment
///             and CPL > segment descriptor DPL; (* DPL in hidden part of segment register *)
///                 THEN SegmentSelector := 0; (* SegmentSelector invalid *)
///             FI;
///     OD;
/// IF instruction has immediate operand
///     THEN (* Release parameters from calling procedure's stack *)
///             IF StackAddressSize = 32
///                 THEN
///                     ESP := ESP + SRC;
///                 ELSE
///                     IF StackAddressSize = 16
///                         THEN
///                             SP := SP + SRC;
///                         ELSE (* StackAddressSize = 64 *)
///                             RSP := RSP + SRC;
///                     FI;
///             FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn ret() -> &'static [IrStatement] {
    let jmp = jump(d(rsp.clone()));
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let operand_condition = condition(is_o1_exists(), [assign(b::add(rsp.clone(), u::zero_extend(o1())), rsp.clone(), size_architecture())], []);
    let halt = halt();
    [set_sp, operand_condition, jmp, halt].into()
}

/// # Pseudocode
/// ```text
/// (* RCL and RCR Instructions *)
/// SIZE := OperandSize;
/// CASE (determine count) OF
///     SIZE := 8:tempCOUNT := (COUNT AND 1FH) MOD 9;
///     SIZE := 16:tempCOUNT := (COUNT AND 1FH) MOD 17;
///     SIZE := 32:tempCOUNT := COUNT AND 1FH;
///     SIZE := 64:tempCOUNT := COUNT AND 3FH;
/// ESAC;
/// IF OperandSize = 64
///     THEN COUNTMASK = 3FH;
///     ELSE COUNTMASK = 1FH;
/// FI;
/// (* RCL Instruction Operation *)
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := MSB(DEST);
///         DEST := (DEST * 2) + CF;
///         CF := tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// (* RCR Instruction Operation *)
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// FI;
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := LSB(SRC);
///         DEST := (DEST / 2) + (CF * 2SIZE);
///         CF := tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// (* ROL Instruction Operation *)
/// tempCOUNT := (COUNT & COUNTMASK) MOD SIZE
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := MSB(DEST);
///         DEST := (DEST * 2) + tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK)  ≠0
///     THEN CF := LSB(DEST);
/// FI;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// FI;
/// (* ROR Instruction Operation *)
/// tempCOUNT := (COUNT & COUNTMASK) MOD SIZE
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := LSB(SRC);
///         DEST := (DEST / 2) + (tempCF * 2SIZE);
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK)  ≠0
///     THEN CF := MSB(DEST);
/// FI;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR MSB - 1(DEST);
///     ELSE OF is undefined;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rol() -> &'static [IrStatement] {
    let op = b::or(b::shl(o1(), o2()), b::shr(o1(), b::sub(bit_size_of_o1(), o2())));
    let assignment = assign(op, o1(), o1_size());
    extend_undefined_flags(&[assignment], &[&of, &cf])
}

/// # Pseudocode
/// ```text
/// (* RCL and RCR Instructions *)
/// SIZE := OperandSize;
/// CASE (determine count) OF
///     SIZE := 8:tempCOUNT := (COUNT AND 1FH) MOD 9;
///     SIZE := 16:tempCOUNT := (COUNT AND 1FH) MOD 17;
///     SIZE := 32:tempCOUNT := COUNT AND 1FH;
///     SIZE := 64:tempCOUNT := COUNT AND 3FH;
/// ESAC;
/// IF OperandSize = 64
///     THEN COUNTMASK = 3FH;
///     ELSE COUNTMASK = 1FH;
/// FI;
/// (* RCL Instruction Operation *)
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := MSB(DEST);
///         DEST := (DEST * 2) + CF;
///         CF := tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// (* RCR Instruction Operation *)
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// FI;
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := LSB(SRC);
///         DEST := (DEST / 2) + (CF * 2SIZE);
///         CF := tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// (* ROL Instruction Operation *)
/// tempCOUNT := (COUNT & COUNTMASK) MOD SIZE
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := MSB(DEST);
///         DEST := (DEST * 2) + tempCF;
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK)  ≠0
///     THEN CF := LSB(DEST);
/// FI;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR CF;
///     ELSE OF is undefined;
/// FI;
/// (* ROR Instruction Operation *)
/// tempCOUNT := (COUNT & COUNTMASK) MOD SIZE
/// WHILE (tempCOUNT ≠ 0)
///     DO
///         tempCF := LSB(SRC);
///         DEST := (DEST / 2) + (tempCF * 2SIZE);
///         tempCOUNT := tempCOUNT - 1;
///     OD;
/// ELIHW;
/// IF (COUNT & COUNTMASK)  ≠0
///     THEN CF := MSB(DEST);
/// FI;
/// IF (COUNT & COUNTMASK) = 1
///     THEN OF := MSB(DEST) XOR MSB - 1(DEST);
///     ELSE OF is undefined;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn ror() -> &'static [IrStatement] {
    let op = b::or(b::shr(o1(), o2()), b::shl(o1(), b::sub(bit_size_of_o1(), o2())));
    let assignment = assign(op, o1(), o1_size());
    extend_undefined_flags(&[assignment], &[&of, &cf])
}

/// # Pseudocode
/// ```text
/// IF (OperandSize = 32)
///     y := imm8 AND 1FH;
///     DEST := (SRC >> y) | (SRC << (32-y));
/// ELSEIF (OperandSize = 64 )
///     y := imm8 AND 3FH;
///     DEST := (SRC >> y) | (SRC << (64-y));
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn rorx() -> &'static [IrStatement] {
    let op = b::or(b::shr(o2(), o3()), b::shl(o2(), b::sub(bit_size_of_o2(), o3())));
    let assignment = assign(op, o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (imm[2] = '1)
///     THEN // rounding mode is determined by MXCSR.RC
///         DEST[63:0] := ConvertDPFPToInteger_M(SRC[63:0]);
///         DEST[127:64] := ConvertDPFPToInteger_M(SRC[127:64]);
///     ELSE// rounding mode is determined by IMM8.RC
///         DEST[63:0] := ConvertDPFPToInteger_Imm(SRC[63:0]);
///         DEST[127:64] := ConvertDPFPToInteger_Imm(SRC[127:64]);
/// FI
/// ROUNDPD (128-bit Legacy SSE Version)
/// DEST[63:0] := RoundToInteger(SRC[63:0]], ROUND_CONTROL)
/// DEST[127:64] := RoundToInteger(SRC[127:64]], ROUND_CONTROL)
/// DEST[MAXVL-1:128] (Unmodified)
/// VROUNDPD (VEX.128 Encoded Version)
/// DEST[63:0] := RoundToInteger(SRC[63:0]], ROUND_CONTROL)
/// DEST[127:64] := RoundToInteger(SRC[127:64]], ROUND_CONTROL)
/// DEST[MAXVL-1:128] := 0
/// VROUNDPD (VEX.256 Encoded Version)
/// DEST[63:0] := RoundToInteger(SRC[63:0], ROUND_CONTROL)
/// DEST[127:64] := RoundToInteger(SRC[127:64]], ROUND_CONTROL)
/// DEST[191:128] := RoundToInteger(SRC[191:128]], ROUND_CONTROL)
/// DEST[255:192] := RoundToInteger(SRC[255:192] ], ROUND_CONTROL)
/// ```
#[box_to_static_reference]
pub(super) fn roundpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (imm[2] = '1)
///     THEN // rounding mode is determined by MXCSR.RC
///         DEST[31:0] := ConvertSPFPToInteger_M(SRC[31:0]);
///         DEST[63:32] := ConvertSPFPToInteger_M(SRC[63:32]);
///         DEST[95:64] := ConvertSPFPToInteger_M(SRC[95:64]);
///         DEST[127:96] := ConvertSPFPToInteger_M(SRC[127:96]);
///         DEST[31:0] := ConvertSPFPToInteger_Imm(SRC[31:0]);
///         DEST[63:32] := ConvertSPFPToInteger_Imm(SRC[63:32]);
///         DEST[95:64] := ConvertSPFPToInteger_Imm(SRC[95:64]);
///         DEST[127:96] := ConvertSPFPToInteger_Imm(SRC[127:96]);
/// FI;
/// ROUNDPS(128-bit Legacy SSE Version)
/// DEST[31:0] := RoundToInteger(SRC[31:0], ROUND_CONTROL)
/// DEST[63:32] := RoundToInteger(SRC[63:32], ROUND_CONTROL)
/// DEST[95:64] := RoundToInteger(SRC[95:64]], ROUND_CONTROL)
/// DEST[127:96] := RoundToInteger(SRC[127:96]], ROUND_CONTROL)
/// DEST[MAXVL-1:128] (Unmodified)
/// VROUNDPS (VEX.128 Encoded Version)
/// DEST[31:0] := RoundToInteger(SRC[31:0], ROUND_CONTROL)
/// DEST[63:32] := RoundToInteger(SRC[63:32], ROUND_CONTROL)
/// DEST[95:64] := RoundToInteger(SRC[95:64]], ROUND_CONTROL)
/// DEST[127:96] := RoundToInteger(SRC[127:96]], ROUND_CONTROL)
/// DEST[MAXVL-1:128] := 0
/// VROUNDPS (VEX.256 Encoded Version)
/// DEST[31:0] := RoundToInteger(SRC[31:0], ROUND_CONTROL)
/// DEST[63:32] := RoundToInteger(SRC[63:32], ROUND_CONTROL)
/// DEST[95:64] := RoundToInteger(SRC[95:64]], ROUND_CONTROL)
/// DEST[127:96] := RoundToInteger(SRC[127:96]], ROUND_CONTROL)
/// DEST[159:128] := RoundToInteger(SRC[159:128]], ROUND_CONTROL)
/// DEST[191:160] := RoundToInteger(SRC[191:160]], ROUND_CONTROL)
/// DEST[223:192] := RoundToInteger(SRC[223:192] ], ROUND_CONTROL)
/// DEST[255:224] := RoundToInteger(SRC[255:224] ], ROUND_CONTROL)
/// ```
#[box_to_static_reference]
pub(super) fn roundps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (imm[2] = '1)
///     THEN // rounding mode is determined by MXCSR.RC
///         DEST[63:0] := ConvertDPFPToInteger_M(SRC[63:0]);
///     ELSE// rounding mode is determined by IMM8.RC
///         DEST[63:0] := ConvertDPFPToInteger_Imm(SRC[63:0]);
/// FI;
/// DEST[127:63] remains unchanged ;
/// ROUNDSD (128-bit Legacy SSE Version)
/// DEST[63:0] := RoundToInteger(SRC[63:0], ROUND_CONTROL)
/// DEST[MAXVL-1:64] (Unmodified)
/// VROUNDSD (VEX.128 Encoded Version)
/// DEST[63:0] := RoundToInteger(SRC2[63:0], ROUND_CONTROL)
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn roundsd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF (imm[2] = '1)
///     THEN // rounding mode is determined by MXCSR.RC
///         DEST[31:0] := ConvertSPFPToInteger_M(SRC[31:0]);
///     ELSE// rounding mode is determined by IMM8.RC
///         DEST[31:0] := ConvertSPFPToInteger_Imm(SRC[31:0]);
/// FI;
/// DEST[127:32] remains unchanged ;
/// ROUNDSS (128-bit Legacy SSE Version)
/// DEST[31:0] := RoundToInteger(SRC[31:0], ROUND_CONTROL)
/// DEST[MAXVL-1:32] (Unmodified)
/// VROUNDSS (VEX.128 Encoded Version)
/// DEST[31:0] := RoundToInteger(SRC2[31:0], ROUND_CONTROL)
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn roundss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// ReturnFromSMM;
/// IF (IA-32e mode supported) or (CPUID DisplayFamily_DisplayModel = 06H_0CH )
///     THEN
///         ProcessorState := Restore(SMMDump(IA-32e SMM STATE MAP));
///     Else
///         ProcessorState := Restore(SMMDump(Non-32-Bit-Mode SMM STATE MAP));
/// FI
/// ```
#[box_to_static_reference]
pub(super) fn rsm() -> &'static [IrStatement] {
    [exception("rsm")].into()
}

/// # Pseudocode
/// ```text
/// RSQRTPS (128-bit Legacy SSE Version)
/// DEST[31:0] := APPROXIMATE(1/SQRT(SRC[31:0]))
/// DEST[63:32] := APPROXIMATE(1/SQRT(SRC1[63:32]))
/// DEST[95:64] := APPROXIMATE(1/SQRT(SRC1[95:64]))
/// DEST[127:96] := APPROXIMATE(1/SQRT(SRC2[127:96]))
/// DEST[MAXVL-1:128] (Unmodified)
/// VRSQRTPS (VEX.128 Encoded Version)
/// DEST[31:0] := APPROXIMATE(1/SQRT(SRC[31:0]))
/// DEST[63:32] := APPROXIMATE(1/SQRT(SRC1[63:32]))
/// DEST[95:64] := APPROXIMATE(1/SQRT(SRC1[95:64]))
/// DEST[127:96] := APPROXIMATE(1/SQRT(SRC2[127:96]))
/// DEST[MAXVL-1:128] := 0
/// VRSQRTPS (VEX.256 Encoded Version)
/// DEST[31:0] := APPROXIMATE(1/SQRT(SRC[31:0]))
/// DEST[63:32] := APPROXIMATE(1/SQRT(SRC1[63:32]))
/// DEST[95:64] := APPROXIMATE(1/SQRT(SRC1[95:64]))
/// DEST[127:96] := APPROXIMATE(1/SQRT(SRC2[127:96]))
/// DEST[159:128] := APPROXIMATE(1/SQRT(SRC2[159:128]))
/// DEST[191:160] := APPROXIMATE(1/SQRT(SRC2[191:160]))
/// DEST[223:192] := APPROXIMATE(1/SQRT(SRC2[223:192]))
/// DEST[255:224] := APPROXIMATE(1/SQRT(SRC2[255:224]))
/// ```
#[box_to_static_reference]
pub(super) fn rsqrtps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// RSQRTSS (128-bit Legacy SSE Version)
/// DEST[31:0] := APPROXIMATE(1/SQRT(SRC2[31:0]))
/// DEST[MAXVL-1:32] (Unmodified)
/// VRSQRTSS (VEX.128 Encoded Version)
/// DEST[31:0] := APPROXIMATE(1/SQRT(SRC2[31:0]))
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// ```
#[box_to_static_reference]
pub(super) fn rsqrtss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF CPL = 3
///     IF (CR4.CET & IA32_U_CET.SH_STK_EN) = 0
///         THEN #UD; FI;
/// ELSE
///     IF (CR4.CET & IA32_S_CET.SH_STK_EN) = 0
///         THEN #UD; FI;
/// FI;
/// SSP_LA = Linear_Address(mem operand)
/// IF SSP_LA not aligned to 8 bytes
///     THEN #GP(0); FI;
/// previous_ssp_token = SSP | (IA32_EFER.LMA AND CS.L) | 0x02
/// Start Atomic Execution
/// restore_ssp_token = Locked shadow_stack_load 8 bytes from SSP_LA
/// fault = 0
/// IF ((restore_ssp_token & 0x03) != (IA32_EFER.LMA & CS.L))
///     THEN fault = 1; FI;
///             (* If L flag in token does not match IA32_EFER.LMA & CS.L or bit 1 is not 0 *)
/// IF ((IA32_EFER.LMA AND CS.L) = 0 AND restore_ssp_token[63:32] != 0)
///     THEN fault = 1; FI;
///             (* If compatibility/legacy mode and SSP to be restored not below 4G *)
/// TMP = restore_ssp_token & ~0x01
/// TMP = (TMP - 8)
/// TMP = TMP & ~0x07
///     THEN fault = 1; FI;
///             (* If address in token does not match the requested top of stack *)
/// TMP = (fault == 0) ? previous_ssp_token : restore_ssp_token
/// shadow_stack_store 8 bytes of TMP to SSP_LA and release lock
/// End Atomic Execution
/// IF fault == 1
/// THEN #CP(RSTORSSP); FI;
/// SSP = SSP_LA
/// // Set the CF if the SSP in the restore token was 4 byte aligned, i.e., there is an alignment hole
/// RFLAGS.CF = (restore_ssp_token & 0x04) ? 1 : 0;
/// RFLAGS.ZF,PF,AF,OF,SF := 0;
/// ```
#[box_to_static_reference]
pub(super) fn rstorssp() -> &'static [IrStatement] {
    [exception("rstorssp")].into()
}
