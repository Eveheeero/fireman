use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF IA-64 Mode
///     THEN
///         IF CPUID.80000001H.ECX[0] = 1;
///             THEN
///                 RFLAGS(SF:ZF:0:AF:0:PF:1:CF) := AH;
///             ELSE
///                 #UD;
///         FI
///     ELSE
///         EFLAGS(SF:ZF:0:AF:0:PF:1:CF) := AH;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn sahf() -> &'static [IrStatement] {
    let sf_a = assign(b::and(b::shr(ah.clone(), c(7)), c(1)), sf.clone(), size_relative(sf.clone()));
    let zf_a = assign(b::and(b::shr(ah.clone(), c(6)), c(1)), zf.clone(), size_relative(zf.clone()));
    let af_a = assign(b::and(b::shr(ah.clone(), c(4)), c(1)), af.clone(), size_relative(af.clone()));
    let pf_a = assign(b::and(b::shr(ah.clone(), c(2)), c(1)), pf.clone(), size_relative(pf.clone()));
    let cf_a = assign(b::and(ah.clone(), c(1)), cf.clone(), size_relative(cf.clone()));
    [sf_a, zf_a, af_a, pf_a, cf_a].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode and using REX.W
///     THEN
///         countMASK := 3FH;
///     ELSE
///         countMASK := 1FH;
/// FI
/// tempCOUNT := (COUNT AND countMASK);
/// tempDEST := DEST;
/// WHILE (tempCOUNT ≠ 0)
/// DO
///     IF instruction is SAL or SHL
///         THEN
///             CF := MSB(DEST);
///         ELSE (* Instruction is SAR or SHR *)
///             CF := LSB(DEST);
///     FI;
///     IF instruction is SAL or SHL
///         THEN
///             DEST := DEST * 2;
///         ELSE
///             IF instruction is SAR
///                 THEN
///                     DEST := DEST / 2; (* Signed divide, rounding toward negative infinity *)
///                 ELSE (* Instruction is SHR *)
///                     DEST := DEST / 2 ; (* Unsigned divide *)
///             FI;
///     FI;
///     tempCOUNT := tempCOUNT - 1;
/// OD;
/// (* Determine overflow for the various instructions *)
/// IF (COUNT and countMASK) = 1
///     THEN
///         IF instruction is SAL or SHL
///             THEN
///                 OF := MSB(DEST) XOR CF;
///             ELSE
///                 IF instruction is SAR
///                     THEN
///                         OF := 0;
///                     ELSE (* Instruction is SHR *)
///                         OF := MSB(tempDEST);
///                 FI;
///         FI;
///     ELSE IF (COUNT AND countMASK) = 0
///         THEN
///             All flags unchanged;
///         ELSE (* COUNT not 1 or 0 *)
///             OF := undefined;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn sal() -> &'static [IrStatement] {
    let shl_1 = b::shl(o1(), o2());
    let shl_1_flags = calc_flags_automatically(shl_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let shl_2 = b::shl(o1(), c(1));
    let shl_2_flags = calc_flags_automatically(shl_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let cond = condition(is_o2_exists(), [shl_1_flags, assign(shl_1, o1(), o1_size())], [shl_2_flags, assign(shl_2, o1(), o1_size())]);
    extend_undefined_flags(&[cond], &[&of, &af, &cf])
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode and using REX.W
///     THEN
///         countMASK := 3FH;
///     ELSE
///         countMASK := 1FH;
/// FI
/// tempCOUNT := (COUNT AND countMASK);
/// tempDEST := DEST;
/// WHILE (tempCOUNT ≠ 0)
/// DO
///     IF instruction is SAL or SHL
///         THEN
///             CF := MSB(DEST);
///         ELSE (* Instruction is SAR or SHR *)
///             CF := LSB(DEST);
///     FI;
///     IF instruction is SAL or SHL
///         THEN
///             DEST := DEST * 2;
///         ELSE
///             IF instruction is SAR
///                 THEN
///                     DEST := DEST / 2; (* Signed divide, rounding toward negative infinity *)
///                 ELSE (* Instruction is SHR *)
///                     DEST := DEST / 2 ; (* Unsigned divide *)
///             FI;
///     FI;
///     tempCOUNT := tempCOUNT - 1;
/// OD;
/// (* Determine overflow for the various instructions *)
/// IF (COUNT and countMASK) = 1
///     THEN
///         IF instruction is SAL or SHL
///             THEN
///                 OF := MSB(DEST) XOR CF;
///             ELSE
///                 IF instruction is SAR
///                     THEN
///                         OF := 0;
///                     ELSE (* Instruction is SHR *)
///                         OF := MSB(tempDEST);
///                 FI;
///         FI;
///     ELSE IF (COUNT AND countMASK) = 0
///         THEN
///             All flags unchanged;
///         ELSE (* COUNT not 1 or 0 *)
///             OF := undefined;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn sar() -> &'static [IrStatement] {
    let sar_1 = b::sar(o1(), o2());
    let sar_1_flags = calc_flags_automatically(sar_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let sar_2 = b::sar(o1(), c(1));
    let sar_2_flags = calc_flags_automatically(sar_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let cond = condition(is_o2_exists(), [sar_1_flags, assign(sar_1, o1(), o1_size())], [sar_2_flags, assign(sar_2, o1(), o1_size())]);
    extend_undefined_flags(&[cond], &[&of, &af, &cf])
}

/// # Pseudocode
/// ```text
/// TEMP := SRC1;
/// IF VEX.W1 and CS.L = 1
/// THEN
///     countMASK := 3FH;
/// ELSE
///     countMASK := 1FH;
/// FI
/// COUNT := (SRC2 AND countMASK)
/// DEST[OperandSize -1] = TEMP[OperandSize -1];
/// DO WHILE (COUNT  ≠0)
///     IF instruction is SHLX
///         THEN
///             DEST[] := DEST *2;
///         ELSE IF instruction is SHRX
///             THEN
///                 DEST[] := DEST /2; //unsigned divide
///         ELSE
///                     // SARX
///                 DEST[] := DEST /2; // signed divide, round toward negative infinity
///     FI;
///     COUNT := COUNT - 1;
/// OD
/// ```
#[box_to_static_reference]
pub(super) fn sarx() -> &'static [IrStatement] {
    let assignment = assign(b::sar(o2(), o3()), o1(), o1_size());
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
/// IF SSP not aligned to 8 bytes
///     THEN #GP(0); FI;
/// (* Pop the "previous-ssp" token from current shadow stack *)
/// previous_ssp_token = ShadowStackPop8B(SSP)
/// (* If the CF flag indicates there was a alignment hole on current shadow stack then pop that alignment hole *)
/// (* Note that the alignment hole must be zero and can be present only when in legacy/compatibility mode *)
/// IF RFLAGS.CF == 1 AND (IA32_EFER.LMA AND CS.L)
///     #GP(0)
/// FI;
/// IF RFLAGS.CF == 1
///     must_be_zero = ShadowStackPop4B(SSP)
///     IF must_be_zero != 0 THEN #GP(0)
/// FI;
/// (* Previous SSP token must have the bit 1 set *)
/// IF ((previous_ssp_token & 0x02) == 0)
///     THEN #GP(0); (* bit 1 was 0 *)
/// IF ((IA32_EFER.LMA AND CS.L) = 0 AND previous_ssp_token [63:32] != 0)
/// THEN #GP(0); FI; (* If compatibility/legacy mode and SSP not in 4G *)
/// (* Save Prev SSP from previous_ssp_token to the old shadow stack at next 8 byte aligned address *)
/// old_SSP = previous_ssp_token & ~0x03
/// temp := (old_SSP | (IA32_EFER.LMA & CS.L));
/// Shadow_stack_store 4 bytes of 0 to (old_SSP - 4)
/// Shadow_stack_store 8 bytes of temp to (old_SSP - 8)
/// ```
#[box_to_static_reference]
pub(super) fn saveprevssp() -> &'static [IrStatement] {
    [exception("saveprevssp")].into()
}

/// # Pseudocode
/// ```text
/// DEST := (DEST - (SRC + CF));
/// ```
#[box_to_static_reference]
pub(super) fn sbb() -> &'static [IrStatement] {
    let size = o1_size();
    let sub = b::sub(o1(), o2());
    let sub = b::sub(sub, u::zero_extend(cf.clone()));
    let assignment = assign(sub.clone(), o1(), &size);
    let calc_flags = calc_flags_automatically(sub, size, &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    let type3 = type_specified(cf.clone(), o1_size(), DataType::Int);
    [calc_flags, assignment, type1, type2, type3].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte comparison)
///     THEN
///         temp := AL - SRC;
///         SetStatusFlags(temp);
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1; FI;
///     ELSE IF (Word comparison)
///         THEN
///             temp := AX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (E)DI := (E)DI + 2;
///                 ELSE (E)DI := (E)DI - 2; FI;
///         FI;
///     ELSE IF (Doubleword comparison)
///         THEN
///             temp := EAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (E)DI := (E)DI + 4;
///                 ELSE (E)DI := (E)DI - 4; FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte comparison)
///     THEN
///         temp := AL - SRC;
///         SetStatusFlags(temp);
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1; FI;
///     ELSE IF (Word comparison)
///         THEN
///             temp := AX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 2;
///                 ELSE (R|E)DI := (R|E)DI - 2; FI;
///     ELSE IF (Doubleword comparison)
///         THEN
///             temp := EAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 4;
///                 ELSE (R|E)DI := (R|E)DI - 4; FI;
///         FI;
///     ELSE IF (Quadword comparison using REX.W )
///         THEN
///             temp := RAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 8;
///                 ELSE (R|E)DI := (R|E)DI - 8;
///             FI;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn scas() -> &'static [IrStatement] {
    let sub = b::sub(rax.clone(), d(rdi.clone()));
    let calc_flags = calc_flags_automatically(sub, size_architecture(), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte comparison)
///     THEN
///         temp := AL - SRC;
///         SetStatusFlags(temp);
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1; FI;
///     ELSE IF (Word comparison)
///         THEN
///             temp := AX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (E)DI := (E)DI + 2;
///                 ELSE (E)DI := (E)DI - 2; FI;
///         FI;
///     ELSE IF (Doubleword comparison)
///         THEN
///             temp := EAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (E)DI := (E)DI + 4;
///                 ELSE (E)DI := (E)DI - 4; FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte comparison)
///     THEN
///         temp := AL - SRC;
///         SetStatusFlags(temp);
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1; FI;
///     ELSE IF (Word comparison)
///         THEN
///             temp := AX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 2;
///                 ELSE (R|E)DI := (R|E)DI - 2; FI;
///     ELSE IF (Doubleword comparison)
///         THEN
///             temp := EAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 4;
///                 ELSE (R|E)DI := (R|E)DI - 4; FI;
///         FI;
///     ELSE IF (Quadword comparison using REX.W )
///         THEN
///             temp := RAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 8;
///                 ELSE (R|E)DI := (R|E)DI - 8;
///             FI;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn scasb() -> &'static [IrStatement] {
    let sub = b::sub(rax.clone(), d(rdi.clone()));
    let calc_flags = calc_flags_automatically(sub, size_result_byte(c(1)), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte comparison)
///     THEN
///         temp := AL - SRC;
///         SetStatusFlags(temp);
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1; FI;
///     ELSE IF (Word comparison)
///         THEN
///             temp := AX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (E)DI := (E)DI + 2;
///                 ELSE (E)DI := (E)DI - 2; FI;
///         FI;
///     ELSE IF (Doubleword comparison)
///         THEN
///             temp := EAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (E)DI := (E)DI + 4;
///                 ELSE (E)DI := (E)DI - 4; FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte comparison)
///     THEN
///         temp := AL - SRC;
///         SetStatusFlags(temp);
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1; FI;
///     ELSE IF (Word comparison)
///         THEN
///             temp := AX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 2;
///                 ELSE (R|E)DI := (R|E)DI - 2; FI;
///     ELSE IF (Doubleword comparison)
///         THEN
///             temp := EAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 4;
///                 ELSE (R|E)DI := (R|E)DI - 4; FI;
///         FI;
///     ELSE IF (Quadword comparison using REX.W )
///         THEN
///             temp := RAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 8;
///                 ELSE (R|E)DI := (R|E)DI - 8;
///             FI;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn scasd() -> &'static [IrStatement] {
    let sub = b::sub(rax.clone(), d(rdi.clone()));
    let calc_flags = calc_flags_automatically(sub, size_result_byte(c(4)), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte comparison)
///     THEN
///         temp := AL - SRC;
///         SetStatusFlags(temp);
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1; FI;
///     ELSE IF (Word comparison)
///         THEN
///             temp := AX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (E)DI := (E)DI + 2;
///                 ELSE (E)DI := (E)DI - 2; FI;
///         FI;
///     ELSE IF (Doubleword comparison)
///         THEN
///             temp := EAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (E)DI := (E)DI + 4;
///                 ELSE (E)DI := (E)DI - 4; FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte comparison)
///     THEN
///         temp := AL - SRC;
///         SetStatusFlags(temp);
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1; FI;
///     ELSE IF (Word comparison)
///         THEN
///             temp := AX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 2;
///                 ELSE (R|E)DI := (R|E)DI - 2; FI;
///     ELSE IF (Doubleword comparison)
///         THEN
///             temp := EAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 4;
///                 ELSE (R|E)DI := (R|E)DI - 4; FI;
///         FI;
///     ELSE IF (Quadword comparison using REX.W )
///         THEN
///             temp := RAX - SRC;
///             SetStatusFlags(temp);
///             IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 8;
///                 ELSE (R|E)DI := (R|E)DI - 8;
///             FI;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn scasw() -> &'static [IrStatement] {
    let sub = b::sub(rax.clone(), d(rdi.clone()));
    let calc_flags = calc_flags_automatically(sub, size_result_byte(c(2)), &[&of, &sf, &zf, &af, &cf, &pf]);
    [calc_flags].into()
}

/// # Pseudocode
/// ```text
/// IF reg > UITTSZ;
///     THEN #GP(0);
/// FI;
/// read tempUITTE from 16 bytes at UITTADDR+ (reg << 4);
/// IF tempUITTE.V = 0 or tempUITTE sets any reserved bit
///     THEN #GP(0);
/// FI;
/// read tempUPID from 16 bytes at tempUITTE.UPIDADDR;// under lock
/// IF tempUPID sets any reserved bits or bits that must be zero
///     THEN #GP(0); // release lock
/// FI;
/// tempUPID.PIR[tempUITTE.UV] := 1;
/// IF tempUPID.SN = tempUPID.ON = 0
///     THEN
///         tempUPID.ON := 1;
///         sendNotify := 1;
///     ELSE sendNotify := 0;
/// FI;
/// write tempUPID to 16 bytes at tempUITTE.UPIDADDR;// release lock
/// IF sendNotify = 1
///     THEN
///         IF local APIC is in x2APIC mode
///             THEN send ordinary IPI with vector tempUPID.NV
///                 to 32-bit physical APIC ID tempUPID.NDST;
///             ELSE send ordinary IPI with vector tempUPID.NV
///                 to 8-bit physical APIC ID tempUPID.NDST[15:8];
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn senduipi() -> &'static [IrStatement] {
    [exception("senduipi")].into()
}

/// # Pseudocode
/// ```text
/// IF condition
///     THEN DEST := 1;
///     ELSE DEST := 0;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn serialize() -> &'static [IrStatement] {
    [exception("serialize")].into()
}

/// # Pseudocode
/// ```text
/// IF (CR4.CET = 0)
///     THEN #UD; FI;
/// IF (IA32_S_CET.SH_STK_EN = 0)
///     THEN #UD; FI;
/// IF CPL > 0
///     THEN GP(0); FI;
/// SSP_LA = IA32_PL0_SSP
/// If SSP_LA not aligned to 8 bytes
///     THEN #GP(0); FI;
/// expected_token_value = SSP_LA
///         (* busy bit must not be set *)
/// new_token_value          = SSP_LA | BUSY_BIT
///         (* set busy bit; bit position 0 *)
/// IF shadow_stack_lock_cmpxchg8B(SSP_LA, new_token_value, expected_token_value) != expected_token_value
///     THEN #CP(SETSSBSY); FI;
/// SSP = SSP_LA
/// ```
#[box_to_static_reference]
pub(super) fn setssbsy() -> &'static [IrStatement] {
    [exception("setssbsy")].into()
}

/// # Pseudocode
/// ```text
/// Wait_On_Following_Stores_Until(preceding_stores_globally_visible);
/// ```
#[box_to_static_reference]
pub(super) fn sfence() -> &'static [IrStatement] {
    [].into()
}

/// # Pseudocode
/// ```text
/// IF instruction is SGDT
///     IF OperandSize =16 or OperandSize = 32 (* Legacy or Compatibility Mode *)
///         THEN
///             DEST[0:15] := GDTR(Limit);
///             DEST[16:47] := GDTR(Base); (* Full 32-bit base address stored *)
///             FI;
///         ELSE (* 64-bit Mode *)
///             DEST[0:15] := GDTR(Limit);
///             DEST[16:79] := GDTR(Base); (* Full 64-bit base address stored *)
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn sgdt() -> &'static [IrStatement] {
    [exception("SGDT")].into()
}

/// # Pseudocode
/// ```text
/// SHA1MSG1
/// W0 := SRC1[127:96] ;
/// W1 := SRC1[95:64] ;
/// W2 := SRC1[63: 32] ;
/// W3 := SRC1[31: 0] ;
/// W4 := SRC2[127:96] ;
/// W5 := SRC2[95:64] ;
/// DEST[127:96] := W2 XOR W0;
/// DEST[95:64] := W3 XOR W1;
/// DEST[63:32] := W4 XOR W2;
/// DEST[31:0] := W5 XOR W3;
/// ```
#[box_to_static_reference]
pub(super) fn sha1msg1() -> &'static [IrStatement] {
    [exception("sha1msg1")].into()
}

/// # Pseudocode
/// ```text
/// SHA1MSG2
/// W13 := SRC2[95:64] ;
/// W14 := SRC2[63: 32] ;
/// W15 := SRC2[31: 0] ;
/// W16 := (SRC1[127:96] XOR W13 ) ROL 1;
/// W17 := (SRC1[95:64] XOR W14) ROL 1;
/// W18 := (SRC1[63: 32] XOR W15) ROL 1;
/// W19 := (SRC1[31: 0] XOR W16) ROL 1;
/// DEST[127:96] := W16;
/// DEST[95:64] := W17;
/// DEST[63:32] := W18;
/// DEST[31:0] := W19;
/// ```
#[box_to_static_reference]
pub(super) fn sha1msg2() -> &'static [IrStatement] {
    [exception("sha1msg2")].into()
}

/// # Pseudocode
/// ```text
/// SHA1NEXTE
/// TMP := (SRC1[127:96] ROL 30);
/// DEST[127:96] := SRC2[127:96] + TMP;
/// DEST[95:64] := SRC2[95:64];
/// DEST[63:32] := SRC2[63:32];
/// DEST[31:0] := SRC2[31:0];
/// ```
#[box_to_static_reference]
pub(super) fn sha1nexte() -> &'static [IrStatement] {
    [exception("sha1nexte")].into()
}

/// # Pseudocode
/// ```text
/// SHA1RNDS4
/// The function f() and Constant K are dependent on the value of the immediate.
/// IF ( imm8[1:0] = 0 )
///     THEN f() := f0(), K := K;
/// ELSE IF ( imm8[1:0] = 1 )
///     THEN f() := f1(), K := K;
/// ELSE IF ( imm8[1:0] = 2 )
///     THEN f() := f2(), K := K;
/// ELSE IF ( imm8[1:0] = 3 )
///     THEN f() := f3(), K := K3;
/// FI;
/// A := SRC1[127:96];
/// B := SRC1[95:64];
/// C := SRC1[63:32];
/// D := SRC1[31:0];
/// WE := SRC2[127:96];
/// W := SRC2[95:64];
/// W := SRC2[63:32];
/// W := SRC2[31:0];
/// Round i = 0 operation:
/// A_1 := f (B, C, D) + (A ROL 5) +WE +K;
/// B_1 := A;
/// C_1 := B ROL 30;
/// D_1 := C;
/// E_1 := D;
/// FOR i = 1 to 3
///     A_(i +1) := f (B_i, C_i, D_i) + (A_i ROL 5) +W+ E_i +K;
///     B_(i +1) := A_i;
///     C_(i +1) := B_i ROL 30;
///     D_(i +1) := C_i;
///     E_(i +1) := D_i;
/// ENDFOR
/// DEST[127:96] := A_4;
/// DEST[95:64] := B_4;
/// DEST[63:32] := C_4;
/// DEST[31:0] := D_4;
/// ```
#[box_to_static_reference]
pub(super) fn sha1rnds4() -> &'static [IrStatement] {
    [exception("sha1rnds4")].into()
}

/// # Pseudocode
/// ```text
/// SHA256MSG1
/// W4 := SRC2[31: 0] ;
/// W3 := SRC1[127:96] ;
/// W2 := SRC1[95:64] ;
/// W1 := SRC1[63: 32] ;
/// W0 := SRC1[31: 0] ;
/// DEST[127:96] := W3 + σ( W4);
/// DEST[95:64] := W2 + σ( W3);
/// DEST[63:32] := W1 + σ( W2);
/// DEST[31:0] := W0 + σ( W1);
/// ```
#[box_to_static_reference]
pub(super) fn sha256msg1() -> &'static [IrStatement] {
    [exception("sha256msg1")].into()
}

/// # Pseudocode
/// ```text
/// SHA256MSG2
/// W14 := SRC2[95:64] ;
/// W15 := SRC2[127:96] ;
/// W16 := SRC1[31: 0] + σ( W14) ;
/// W17 := SRC1[63: 32] + σ( W15) ;
/// W18 := SRC1[95: 64] + σ( W16) ;
/// W19 := SRC1[127: 96] + σ( W17) ;
/// DEST[127:96] := W19 ;
/// DEST[95:64] := W18 ;
/// DEST[63:32] := W17 ;
/// DEST[31:0] := W16;
/// ```
#[box_to_static_reference]
pub(super) fn sha256msg2() -> &'static [IrStatement] {
    [exception("sha256msg2")].into()
}

/// # Pseudocode
/// ```text
/// SHA256RNDS2
/// A_0 := SRC2[127:96];
/// B_0 := SRC2[95:64];
/// C_0 := SRC1[127:96];
/// D_0 := SRC1[95:64];
/// E_0 := SRC2[63:32];
/// F_0 := SRC2[31:0];
/// G_0 := SRC1[63:32];
/// H_0 := SRC1[31:0];
/// WK := XMM0[31: 0];
/// WK := XMM0[63: 32];
/// FOR i = 0 to 1
///     A_(i +1) := Ch (E_i, F_i, G_i) +Σ( E_i) +WK+ H_i + Maj(A_i , B_i, C_i) +Σ( A_i);
///     B_(i +1) := A_i;
///     C_(i +1) := B_i ;
///     D_(i +1) := C_i;
///     E_(i +1) := Ch (E_i, F_i, G_i) +Σ( E_i) +WK+ H_i + D_i;
///     F_(i +1) := E_i ;
///     G_(i +1) := F_i;
///     H_(i +1) := G_i;
/// ENDFOR
/// DEST[127:96] := A_2;
/// DEST[95:64] := B_2;
/// DEST[63:32] := E_2;
/// DEST[31:0] := F_2;
/// ```
#[box_to_static_reference]
pub(super) fn sha256rnds2() -> &'static [IrStatement] {
    [exception("sha256rnds2")].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode and using REX.W
///     THEN
///         countMASK := 3FH;
///     ELSE
///         countMASK := 1FH;
/// FI
/// tempCOUNT := (COUNT AND countMASK);
/// tempDEST := DEST;
/// WHILE (tempCOUNT ≠ 0)
/// DO
///     IF instruction is SAL or SHL
///         THEN
///             CF := MSB(DEST);
///         ELSE (* Instruction is SAR or SHR *)
///             CF := LSB(DEST);
///     FI;
///     IF instruction is SAL or SHL
///         THEN
///             DEST := DEST * 2;
///         ELSE
///             IF instruction is SAR
///                 THEN
///                     DEST := DEST / 2; (* Signed divide, rounding toward negative infinity *)
///                 ELSE (* Instruction is SHR *)
///                     DEST := DEST / 2 ; (* Unsigned divide *)
///             FI;
///     FI;
///     tempCOUNT := tempCOUNT - 1;
/// OD;
/// (* Determine overflow for the various instructions *)
/// IF (COUNT and countMASK) = 1
///     THEN
///         IF instruction is SAL or SHL
///             THEN
///                 OF := MSB(DEST) XOR CF;
///             ELSE
///                 IF instruction is SAR
///                     THEN
///                         OF := 0;
///                     ELSE (* Instruction is SHR *)
///                         OF := MSB(tempDEST);
///                 FI;
///         FI;
///     ELSE IF (COUNT AND countMASK) = 0
///         THEN
///             All flags unchanged;
///         ELSE (* COUNT not 1 or 0 *)
///             OF := undefined;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn shl() -> &'static [IrStatement] {
    let shl_1 = b::shl(o1(), o2());
    let shl_1_flags = calc_flags_automatically(shl_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let shl_2 = b::shl(o1(), c(1));
    let shl_2_flags = calc_flags_automatically(shl_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let cond = condition(is_o2_exists(), [shl_1_flags, assign(shl_1, o1(), o1_size())], [shl_2_flags, assign(shl_2, o1(), o1_size())]);
    extend_undefined_flags(&[cond], &[&of, &af, &cf])
}

/// # Pseudocode
/// ```text
/// IF (In 64-Bit Mode and REX.W = 1)
///     THEN COUNT := COUNT MOD 64;
///     ELSE COUNT := COUNT MOD 32;
/// FI
/// SIZE := OperandSize;
/// IF COUNT = 0
///     THEN
///         No operation;
///     ELSE
///         IF COUNT > SIZE
///             THEN (* Bad parameters *)
///                 DEST is undefined;
///                 CF, OF, SF, ZF, AF, PF are undefined;
///             ELSE (* Perform the shift *)
///                 CF := BIT[DEST, SIZE - COUNT];
///                 (* Last bit shifted out on exit *)
///                 FOR i := SIZE - 1 DOWN TO COUNT
///                     DO
///                         Bit(DEST, i) := Bit(DEST, i - COUNT);
///                     OD;
///                 FOR i := COUNT - 1 DOWN TO 0
///                     DO
///                         BIT[DEST, i] := BIT[SRC, i - COUNT + SIZE];
///                     OD;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn shld() -> &'static [IrStatement] {
    let op = b::or(b::shl(o1(), o3()), b::shr(o2(), b::sub(bit_size_of_o1(), o3())));
    let assignment = assign(op, o1(), o1_size());
    extend_undefined_flags(&[assignment], &[&of, &sf, &zf, &af, &cf, &pf])
}

/// # Pseudocode
/// ```text
/// TEMP := SRC1;
/// IF VEX.W1 and CS.L = 1
/// THEN
///     countMASK := 3FH;
/// ELSE
///     countMASK := 1FH;
/// FI
/// COUNT := (SRC2 AND countMASK)
/// DEST[OperandSize -1] = TEMP[OperandSize -1];
/// DO WHILE (COUNT  ≠0)
///     IF instruction is SHLX
///         THEN
///             DEST[] := DEST *2;
///         ELSE IF instruction is SHRX
///             THEN
///                 DEST[] := DEST /2; //unsigned divide
///         ELSE
///                     // SARX
///                 DEST[] := DEST /2; // signed divide, round toward negative infinity
///     FI;
///     COUNT := COUNT - 1;
/// OD
/// ```
#[box_to_static_reference]
pub(super) fn shlx() -> &'static [IrStatement] {
    let assignment = assign(b::shl(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF 64-Bit Mode and using REX.W
///     THEN
///         countMASK := 3FH;
///     ELSE
///         countMASK := 1FH;
/// FI
/// tempCOUNT := (COUNT AND countMASK);
/// tempDEST := DEST;
/// WHILE (tempCOUNT ≠ 0)
/// DO
///     IF instruction is SAL or SHL
///         THEN
///             CF := MSB(DEST);
///         ELSE (* Instruction is SAR or SHR *)
///             CF := LSB(DEST);
///     FI;
///     IF instruction is SAL or SHL
///         THEN
///             DEST := DEST * 2;
///         ELSE
///             IF instruction is SAR
///                 THEN
///                     DEST := DEST / 2; (* Signed divide, rounding toward negative infinity *)
///                 ELSE (* Instruction is SHR *)
///                     DEST := DEST / 2 ; (* Unsigned divide *)
///             FI;
///     FI;
///     tempCOUNT := tempCOUNT - 1;
/// OD;
/// (* Determine overflow for the various instructions *)
/// IF (COUNT and countMASK) = 1
///     THEN
///         IF instruction is SAL or SHL
///             THEN
///                 OF := MSB(DEST) XOR CF;
///             ELSE
///                 IF instruction is SAR
///                     THEN
///                         OF := 0;
///                     ELSE (* Instruction is SHR *)
///                         OF := MSB(tempDEST);
///                 FI;
///         FI;
///     ELSE IF (COUNT AND countMASK) = 0
///         THEN
///             All flags unchanged;
///         ELSE (* COUNT not 1 or 0 *)
///             OF := undefined;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn shr() -> &'static [IrStatement] {
    let shr_1 = b::shr(o1(), o2());
    let shr_1_flags = calc_flags_automatically(shr_1.clone(), o1_size(), &[&sf, &zf, &pf]);
    let shr_2 = b::shr(o1(), c(1));
    let shr_2_flags = calc_flags_automatically(shr_2.clone(), o1_size(), &[&sf, &zf, &pf]);
    let cond = condition(is_o2_exists(), [shr_1_flags, assign(shr_1, o1(), o1_size())], [shr_2_flags, assign(shr_2, o1(), o1_size())]);
    extend_undefined_flags(&[cond], &[&of, &af, &cf])
}

/// # Pseudocode
/// ```text
/// IF (In 64-Bit Mode and REX.W = 1)
///     THEN COUNT := COUNT MOD 64;
///     ELSE COUNT := COUNT MOD 32;
/// FI
/// SIZE := OperandSize;
/// IF COUNT = 0
///     THEN
///         No operation;
///     ELSE
///         IF COUNT > SIZE
///             THEN (* Bad parameters *)
///                 DEST is undefined;
///                 CF, OF, SF, ZF, AF, PF are undefined;
///             ELSE (* Perform the shift *)
///                 CF := BIT[DEST, COUNT - 1]; (* Last bit shifted out on exit *)
///                 FOR i := 0 TO SIZE - 1 - COUNT
///                     DO
///                         BIT[DEST, i] := BIT[DEST, i + COUNT];
///                     OD;
///                 FOR i := SIZE - COUNT TO SIZE - 1
///                     DO
///                         BIT[DEST,i] := BIT[SRC, i + COUNT - SIZE];
///                     OD;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn shrd() -> &'static [IrStatement] {
    let op = b::or(b::shr(o1(), o3()), b::shl(o2(), b::sub(bit_size_of_o1(), o3())));
    let assignment = assign(op, o1(), o1_size());
    extend_undefined_flags(&[assignment], &[&of, &sf, &zf, &af, &cf, &pf])
}

/// # Pseudocode
/// ```text
/// TEMP := SRC1;
/// IF VEX.W1 and CS.L = 1
/// THEN
///     countMASK := 3FH;
/// ELSE
///     countMASK := 1FH;
/// FI
/// COUNT := (SRC2 AND countMASK)
/// DEST[OperandSize -1] = TEMP[OperandSize -1];
/// DO WHILE (COUNT  ≠0)
///     IF instruction is SHLX
///         THEN
///             DEST[] := DEST *2;
///         ELSE IF instruction is SHRX
///             THEN
///                 DEST[] := DEST /2; //unsigned divide
///         ELSE
///                     // SARX
///                 DEST[] := DEST /2; // signed divide, round toward negative infinity
///     FI;
///     COUNT := COUNT - 1;
/// OD
/// ```
#[box_to_static_reference]
pub(super) fn shrx() -> &'static [IrStatement] {
    let assignment = assign(b::shr(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSHUFPD (EVEX Encoded Versions When SRC2 is a Vector Register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF IMM0[0] = 0
///     THEN TMP_DEST[63:0] := SRC1[63:0]
///     ELSE TMP_DEST[63:0] := SRC1[127:64] FI;
/// IF IMM0[1] = 0
///     THEN TMP_DEST[127:64] := SRC2[63:0]
///     ELSE TMP_DEST[127:64] := SRC2[127:64] FI;
/// IF VL >= 256
///     IF IMM0[2] = 0
///         THEN TMP_DEST[191:128] := SRC1[191:128]
///         ELSE TMP_DEST[191:128] := SRC1[255:192] FI;
///     IF IMM0[3] = 0
///         THEN TMP_DEST[255:192] := SRC2[191:128]
///         ELSE TMP_DEST[255:192] := SRC2[255:192] FI;
/// FI;
/// IF VL >= 512
///     IF IMM0[4] = 0
///         THEN TMP_DEST[319:256] := SRC1[319:256]
///         ELSE TMP_DEST[319:256] := SRC1[383:320] FI;
///     IF IMM0[5] = 0
///         THEN TMP_DEST[383:320] := SRC2[319:256]
///         ELSE TMP_DEST[383:320] := SRC2[383:320] FI;
///     IF IMM0[6] = 0
///         THEN TMP_DEST[447:384] := SRC1[447:384]
///         ELSE TMP_DEST[447:384] := SRC1[511:448] FI;
///     IF IMM0[7] = 0
///         THEN TMP_DEST[511:448] := SRC2[447:384]
///         ELSE TMP_DEST[511:448] := SRC2[511:448] FI;
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
/// VSHUFPD (EVEX Encoded Versions When SRC2 is Memory)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF (EVEX.b = 1)
///         THEN TMP_SRC2[i+63:i] := SRC2[63:0]
///         ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]
///     FI;
/// ENDFOR;
/// IF IMM0[0] = 0
///     THEN TMP_DEST[63:0] := SRC1[63:0]
///     ELSE TMP_DEST[63:0] := SRC1[127:64] FI;
/// IF IMM0[1] = 0
///     THEN TMP_DEST[127:64] := TMP_SRC2[63:0]
///     ELSE TMP_DEST[127:64] := TMP_SRC2[127:64] FI;
/// IF VL >= 256
///     IF IMM0[2] = 0
///         THEN TMP_DEST[191:128] := SRC1[191:128]
///         ELSE TMP_DEST[191:128] := SRC1[255:192] FI;
///     IF IMM0[3] = 0
///         THEN TMP_DEST[255:192] := TMP_SRC2[191:128]
///         ELSE TMP_DEST[255:192] := TMP_SRC2[255:192] FI;
/// FI;
/// IF VL >= 512
///     IF IMM0[4] = 0
///         THEN TMP_DEST[319:256] := SRC1[319:256]
///         ELSE TMP_DEST[319:256] := SRC1[383:320] FI;
///     IF IMM0[5] = 0
///         THEN TMP_DEST[383:320] := TMP_SRC2[319:256]
///         ELSE TMP_DEST[383:320] := TMP_SRC2[383:320] FI;
///     IF IMM0[6] = 0
///         THEN TMP_DEST[447:384] := SRC1[447:384]
///         ELSE TMP_DEST[447:384] := SRC1[511:448] FI;
///     IF IMM0[7] = 0
///         THEN TMP_DEST[511:448] := TMP_SRC2[447:384]
///         ELSE TMP_DEST[511:448] := TMP_SRC2[511:448] FI;
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
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
/// VSHUFPD (VEX.256 Encoded Version)
/// IF IMM0[0] = 0
///     THEN DEST[63:0] := SRC1[63:0]
///     ELSE DEST[63:0] := SRC1[127:64] FI;
/// IF IMM0[1] = 0
///     THEN DEST[127:64] := SRC2[63:0]
///     ELSE DEST[127:64] := SRC2[127:64] FI;
/// IF IMM0[2] = 0
///     THEN DEST[191:128] := SRC1[191:128]
///     ELSE DEST[191:128] := SRC1[255:192] FI;
/// IF IMM0[3] = 0
///     THEN DEST[255:192] := SRC2[191:128]
///     ELSE DEST[255:192] := SRC2[255:192] FI;
/// DEST[MAXVL-1:256] (Unmodified)
/// VSHUFPD (VEX.128 Encoded Version)
/// IF IMM0[0] = 0
///     THEN DEST[63:0] := SRC1[63:0]
///     ELSE DEST[63:0] := SRC1[127:64] FI;
/// IF IMM0[1] = 0
///     THEN DEST[127:64] := SRC2[63:0]
///     ELSE DEST[127:64] := SRC2[127:64] FI;
/// DEST[MAXVL-1:128] := 0
/// VSHUFPD (128-bit Legacy SSE Version)
/// IF IMM0[0] = 0
///     THEN DEST[63:0] := SRC1[63:0]
///     ELSE DEST[63:0] := SRC1[127:64] FI;
/// IF IMM0[1] = 0
///     THEN DEST[127:64] := SRC2[63:0]
///     ELSE DEST[127:64] := SRC2[127:64] FI;
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn shufpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// Select4(SRC, control) {
/// CASE (control[1:0]) OF
///     0: TM:PS=  RC[31:0];
///     1: TM:PS=  RC[63:32];
///     2: TM:PS=  RC[95:64];
///     3: TM:PS=  RC[127:96];
/// ESAC;
/// RETURN TMP
/// }
/// VPSHUFPS (EVEX Encoded Versions When SRC2 is a Vector Register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// TMP_DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);
/// TMP_DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);
/// TMP_DEST[95:64] := Select4(SRC2[127:0], imm8[5:4]);
/// TMP_DEST[127:96] := Select4(SRC2[127:0], imm8[7:6]);
/// IF VL >= 256
///     TMP_DEST[159:128] := Select4(SRC1[255:128], imm8[1:0]);
///     TMP_DEST[191:160] := Select4(SRC1[255:128], imm8[3:2]);
///     TMP_DEST[223:192] := Select4(SRC2[255:128], imm8[5:4]);
///     TMP_DEST[255:224] := Select4(SRC2[255:128], imm8[7:6]);
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := Select4(SRC1[383:256], imm8[1:0]);
///     TMP_DEST[319:288] := Select4(SRC1[383:256], imm8[3:2]);
///     TMP_DEST[351:320] := Select4(SRC2[383:256], imm8[5:4]);
///     TMP_DEST[383:352] := Select4(SRC2[383:256], imm8[7:6]);
///     TMP_DEST[415:384] := Select4(SRC1[511:384], imm8[1:0]);
///     TMP_DEST[447:416] := Select4(SRC1[511:384], imm8[3:2]);
///     TMP_DEST[479:448] := Select4(SRC2[511:384], imm8[5:4]);
///     TMP_DEST[511:480] := Select4(SRC2[511:384], imm8[7:6]);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
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
/// VPSHUFPS (EVEX Encoded Versions When SRC2 is Memory)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF (EVEX.b = 1)
///         THEN TMP_SRC2[i+31:i] := SRC2[31:0]
///         ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]
///     FI;
/// ENDFOR;
/// TMP_DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);
/// TMP_DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);
/// TMP_DEST[95:64] := Select4(TMP_SRC2[127:0], imm8[5:4]);
/// TMP_DEST[127:96] := Select4(TMP_SRC2[127:0], imm8[7:6]);
/// IF VL >= 256
///     TMP_DEST[159:128] := Select4(SRC1[255:128], imm8[1:0]);
///     TMP_DEST[191:160] := Select4(SRC1[255:128], imm8[3:2]);
///     TMP_DEST[223:192] := Select4(TMP_SRC2[255:128], imm8[5:4]);
///     TMP_DEST[255:224] := Select4(TMP_SRC2[255:128], imm8[7:6]);
/// FI;
/// IF VL >= 512
///     TMP_DEST[287:256] := Select4(SRC1[383:256], imm8[1:0]);
///     TMP_DEST[319:288] := Select4(SRC1[383:256], imm8[3:2]);
///     TMP_DEST[351:320] := Select4(TMP_SRC2[383:256], imm8[5:4]);
///     TMP_DEST[383:352] := Select4(TMP_SRC2[383:256], imm8[7:6]);
///     TMP_DEST[415:384] := Select4(SRC1[511:384], imm8[1:0]);
///     TMP_DEST[447:416] := Select4(SRC1[511:384], imm8[3:2]);
///     TMP_DEST[479:448] := Select4(TMP_SRC2[511:384], imm8[5:4]);
///     TMP_DEST[511:480] := Select4(TMP_SRC2[511:384], imm8[7:6]);
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
/// VSHUFPS (VEX.256 Encoded Version)
/// DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);
/// DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);
/// DEST[95:64] := Select4(SRC2[127:0], imm8[5:4]);
/// DEST[127:96] := Select4(SRC2[127:0], imm8[7:6]);
/// DEST[159:128] := Select4(SRC1[255:128], imm8[1:0]);
/// DEST[191:160] := Select4(SRC1[255:128], imm8[3:2]);
/// DEST[223:192] := Select4(SRC2[255:128], imm8[5:4]);
/// DEST[255:224] := Select4(SRC2[255:128], imm8[7:6]);
/// DEST[MAXVL-1:256] := 0
/// VSHUFPS (VEX.128 Encoded Version)
/// DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);
/// DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);
/// DEST[95:64] := Select4(SRC2[127:0], imm8[5:4]);
/// DEST[127:96] := Select4(SRC2[127:0], imm8[7:6]);
/// DEST[MAXVL-1:128] := 0
/// SHUFPS (128-bit Legacy SSE Version)
/// DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);
/// DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);
/// DEST[95:64] := Select4(SRC2[127:0], imm8[5:4]);
/// DEST[127:96] := Select4(SRC2[127:0], imm8[7:6]);
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn shufps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF instruction is SIDT
///     THEN
///         IF OperandSize =16 or OperandSize = 32 (* Legacy or Compatibility Mode *)
///             THEN
///                 DEST[0:15] := IDTR(Limit);
///                 DEST[16:47] := IDTR(Base); FI; (* Full 32-bit base address stored *)
///             ELSE (* 64-bit Mode *)
///                 DEST[0:15] := IDTR(Limit);
///                 DEST[16:79] := IDTR(Base); (* Full 64-bit base address stored *)
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn sidt() -> &'static [IrStatement] {
    [exception("SIDT")].into()
}

/// # Pseudocode
/// ```text
/// DEST := LDTR(SegmentSelector);
/// ```
#[box_to_static_reference]
pub(super) fn sldt() -> &'static [IrStatement] {
    [exception("SLDT")].into()
}

/// # Pseudocode
/// ```text
/// DEST := CR0[15:0];
/// (* Machine status word *)
/// ```
#[box_to_static_reference]
pub(super) fn smsw() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// VSQRTPD (EVEX Encoded Versions)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1) AND (SRC *is register*)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC *is memory*)
///                     THEN DEST[i+63:i] := SQRT(SRC[63:0])
///                     ELSE DEST[i+63:i] := SQRT(SRC[i+63:i])
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
/// VSQRTPD (VEX.256 Encoded Version)
/// DEST[63:0] := SQRT(SRC[63:0])
/// DEST[127:64] := SQRT(SRC[127:64])
/// DEST[191:128] := SQRT(SRC[191:128])
/// DEST[255:192] := SQRT(SRC[255:192])
/// DEST[MAXVL-1:256] := 0
/// .
/// VSQRTPD (VEX.128 Encoded Version)
/// DEST[63:0] := SQRT(SRC[63:0])
/// DEST[127:64] := SQRT(SRC[127:64])
/// DEST[MAXVL-1:128] := 0
/// SQRTPD (128-bit Legacy SSE Version)
/// DEST[63:0] := SQRT(SRC[63:0])
/// DEST[127:64] := SQRT(SRC[127:64])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn sqrtpd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSQRTPS (EVEX Encoded Versions)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1) AND (SRC *is register*)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1) AND (SRC *is memory*)
///                     THEN DEST[i+31:i] := SQRT(SRC[31:0])
///                     ELSE DEST[i+31:i] := SQRT(SRC[i+31:i])
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
/// VSQRTPS (VEX.256 Encoded Version)
/// DEST[31:0] := SQRT(SRC[31:0])
/// DEST[63:32] := SQRT(SRC[63:32])
/// DEST[95:64] := SQRT(SRC[95:64])
/// DEST[127:96] := SQRT(SRC[127:96])
/// DEST[159:128] := SQRT(SRC[159:128])
/// DEST[191:160] := SQRT(SRC[191:160])
/// DEST[223:192] := SQRT(SRC[223:192])
/// DEST[255:224] := SQRT(SRC[255:224])
/// VSQRTPS (VEX.128 Encoded Version)
/// DEST[31:0] := SQRT(SRC[31:0])
/// DEST[63:32] := SQRT(SRC[63:32])
/// DEST[95:64] := SQRT(SRC[95:64])
/// DEST[127:96] := SQRT(SRC[127:96])
/// DEST[MAXVL-1:128] := 0
/// SQRTPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SQRT(SRC[31:0])
/// DEST[63:32] := SQRT(SRC[63:32])
/// DEST[95:64] := SQRT(SRC[95:64])
/// DEST[127:96] := SQRT(SRC[127:96])
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn sqrtps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSQRTSD (EVEX Encoded Version)
/// IF (EVEX.b = 1) AND (SRC2 *is register*)
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := SQRT(SRC2[63:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     THEN DEST[63:0] := 0
///         FI;
/// FI;
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// VSQRTSD (VEX.128 Encoded Version)
/// DEST[63:0] := SQRT(SRC2[63:0])
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// SQRTSD (128-bit Legacy SSE Version)
/// DEST[63:0] := SQRT(SRC[63:0])
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn sqrtsd() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSQRTSS (EVEX Encoded Version)
/// IF (EVEX.b = 1) AND (SRC2 *is register*)
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := SQRT(SRC2[31:0])
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[31:0] := 0
/// FI;
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// VSQRTSS (VEX.128 Encoded Version)
/// DEST[31:0] := SQRT(SRC2[31:0])
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// SQRTSS (128-bit Legacy SSE Version)
/// DEST[31:0] := SQRT(SRC2[31:0])
/// DEST[MAXVL-1:32] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn sqrtss() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// EFLAGS.AC := 1;
/// ```
#[box_to_static_reference]
pub(super) fn stac() -> &'static [IrStatement] {
    [exception("stac")].into()
}

/// # Pseudocode
/// ```text
/// CF := 1;
/// ```
#[box_to_static_reference]
pub(super) fn stc() -> &'static [IrStatement] {
    let set = assign(c(1), cf.clone(), size_relative(cf.clone()));
    [set].into()
}

/// # Pseudocode
/// ```text
/// DF := 1;
/// ```
#[box_to_static_reference]
pub(super) fn std() -> &'static [IrStatement] {
    let set = assign(c(1), df.clone(), size_relative(df.clone()));
    [set].into()
}

/// # Pseudocode
/// ```text
/// IF CR0.PE = 0  (* Executing in real-address mode *)
///     THEN IF := 1; (* Set Interrupt Flag *)
///     ELSE
///         IF IOPL ≥ CPL(* CPL = 3 if EFLAGS.VM = 1 *)
///             THEN IF := 1; (* Set Interrupt Flag *)
///             ELSE
///                 IF VME mode OR PVI mode
///                     THEN
///                         IF EFLAGS.VIP = 0
///                             THEN VIF := 1; (* Set Virtual Interrupt Flag *)
///                             ELSE #GP(0);
///                         FI;
///                     ELSE #GP(0);
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn sti() -> &'static [IrStatement] {
    let set = assign(c(1), if_.clone(), size_relative(if_.clone()));
    [set].into()
}

/// # Pseudocode
/// ```text
/// m32 := MXCSR;
/// ```
#[box_to_static_reference]
pub(super) fn stmxcsr() -> &'static [IrStatement] {
    let stmt_0 = assign(unknown_data(), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 2;
///                     ELSE (E)DI := (E)DI - 2;
///                 FI;
///         FI;
///     ELSE IF (Doubleword store)
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 4;
///                     ELSE (E)DI := (E)DI - 4;
///                 FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 2;
///                     ELSE (R|E)DI := (R|E)DI - 2;
///                 FI;
///         FI;
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 4;
///                     ELSE (R|E)DI := (R|E)DI - 4;
///                 FI;
///         FI;
///     ELSE IF (Quadword store using REX.W )
///         THEN
///             DEST := RAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 8;
///                     ELSE (R|E)DI := (R|E)DI - 8;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn stos() -> &'static [IrStatement] {
    let stos = assign(rax.clone(), d(rdi.clone()), size_architecture());
    [stos].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 2;
///                     ELSE (E)DI := (E)DI - 2;
///                 FI;
///         FI;
///     ELSE IF (Doubleword store)
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 4;
///                     ELSE (E)DI := (E)DI - 4;
///                 FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 2;
///                     ELSE (R|E)DI := (R|E)DI - 2;
///                 FI;
///         FI;
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 4;
///                     ELSE (R|E)DI := (R|E)DI - 4;
///                 FI;
///         FI;
///     ELSE IF (Quadword store using REX.W )
///         THEN
///             DEST := RAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 8;
///                     ELSE (R|E)DI := (R|E)DI - 8;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn stosb() -> &'static [IrStatement] {
    let stos = assign(rax.clone(), d(rdi.clone()), size_result_byte(c(1)));
    [stos].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 2;
///                     ELSE (E)DI := (E)DI - 2;
///                 FI;
///         FI;
///     ELSE IF (Doubleword store)
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 4;
///                     ELSE (E)DI := (E)DI - 4;
///                 FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 2;
///                     ELSE (R|E)DI := (R|E)DI - 2;
///                 FI;
///         FI;
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 4;
///                     ELSE (R|E)DI := (R|E)DI - 4;
///                 FI;
///         FI;
///     ELSE IF (Quadword store using REX.W )
///         THEN
///             DEST := RAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 8;
///                     ELSE (R|E)DI := (R|E)DI - 8;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn stosd() -> &'static [IrStatement] {
    let stos = assign(rax.clone(), d(rdi.clone()), size_result_byte(c(4)));
    [stos].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 2;
///                     ELSE (E)DI := (E)DI - 2;
///                 FI;
///         FI;
///     ELSE IF (Doubleword store)
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 4;
///                     ELSE (E)DI := (E)DI - 4;
///                 FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 2;
///                     ELSE (R|E)DI := (R|E)DI - 2;
///                 FI;
///         FI;
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 4;
///                     ELSE (R|E)DI := (R|E)DI - 4;
///                 FI;
///         FI;
///     ELSE IF (Quadword store using REX.W )
///         THEN
///             DEST := RAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 8;
///                     ELSE (R|E)DI := (R|E)DI - 8;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn stosq() -> &'static [IrStatement] {
    let stos = assign(rax.clone(), d(rdi.clone()), size_result_byte(c(8)));
    [stos].into()
}

/// # Pseudocode
/// ```text
/// Non-64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 1;
///                 ELSE (E)DI := (E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 2;
///                     ELSE (E)DI := (E)DI - 2;
///                 FI;
///         FI;
///     ELSE IF (Doubleword store)
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (E)DI := (E)DI + 4;
///                     ELSE (E)DI := (E)DI - 4;
///                 FI;
///         FI;
/// FI;
/// 64-bit Mode:
/// IF (Byte store)
///     THEN
///         DEST := AL;
///             THEN IF DF = 0
///                 THEN (R|E)DI := (R|E)DI + 1;
///                 ELSE (R|E)DI := (R|E)DI - 1;
///             FI;
///     ELSE IF (Word store)
///         THEN
///             DEST := AX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 2;
///                     ELSE (R|E)DI := (R|E)DI - 2;
///                 FI;
///         FI;
///         THEN
///             DEST := EAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 4;
///                     ELSE (R|E)DI := (R|E)DI - 4;
///                 FI;
///         FI;
///     ELSE IF (Quadword store using REX.W )
///         THEN
///             DEST := RAX;
///                 THEN IF DF = 0
///                     THEN (R|E)DI := (R|E)DI + 8;
///                     ELSE (R|E)DI := (R|E)DI - 8;
///                 FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn stosw() -> &'static [IrStatement] {
    let stos = assign(rax.clone(), d(rdi.clone()), size_result_byte(c(2)));
    [stos].into()
}

/// # Pseudocode
/// ```text
/// DEST := TR(SegmentSelector);
/// ```
#[box_to_static_reference]
pub(super) fn str() -> &'static [IrStatement] {
    [exception("STR")].into()
}

/// # Pseudocode
/// ```text
/// STTILECFG mem
/// if TILES_CONFIGURED == 0:
///     //write 64 bytes of zeros at mem pointer
///     buf[0..63] := 0
///     write_memory(mem, 64, buf)
/// else:
///     buf.byte[0] := tilecfg.palette_id
///     buf.byte[1] := tilecfg.start_row
///     buf.byte[2..15] := 0
///     p := 16
///     for n in 0 ... palette_table[tilecfg.palette_id].max_names-1:
///         buf.word[p/2] := tilecfg.t[n].colsb
///         p := p + 2
///     if p < 47:
///         buf.byte[p..47] := 0
///     p := 48
///     for n in 0 ... palette_table[tilecfg.palette_id].max_names-1:
///         buf.byte[p++] := tilecfg.t[n].rows
///     if p < 63:
///         buf.byte[p..63] := 0
///     write_memory(mem, 64, buf)
/// ```
#[box_to_static_reference]
pub(super) fn sttilecfg() -> &'static [IrStatement] {
    [exception("sttilecfg")].into()
}

/// # Pseudocode
/// ```text
/// UIF := 1;
/// ```
#[box_to_static_reference]
pub(super) fn stui() -> &'static [IrStatement] {
    let stmt_0 = assign(c(1), o1(), o1_size());
    [stmt_0].into()
}

/// # Pseudocode
/// ```text
/// DEST := (DEST - SRC);
/// ```
#[box_to_static_reference]
pub(super) fn sub() -> &'static [IrStatement] {
    let op = b::sub(o1(), o2());
    let assignment = assign(op.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(op, o1_size(), &[&of, &sf, &zf, &af, &cf, &pf]);
    let type1 = type_specified(o1(), o1_size(), DataType::Int);
    let type2 = type_specified(o2(), o2_size(), DataType::Int);
    [calc_flags, assignment, type1, type2].into()
}

/// # Pseudocode
/// ```text
/// VSUBPD (EVEX Encoded Versions When SRC2 Operand is a Vector Register)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask*
///             THEN DEST[i+63:i] := SRC1[i+63:i] - SRC2[i+63:i]
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[63:0] := 0
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSUBPD (EVEX Encoded Versions When SRC2 Operand is a Memory Source)
/// (KL, VL) = (2, 128), (4, 256), (8, 512)
/// FOR j := 0 TO KL-1
///     i := j * 64
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1)
///                     THEN DEST[i+63:i] := SRC1[i+63:i] - SRC2[63:0];
///                     ELSE EST[i+63:i] := SRC1[i+63:i] - SRC2[i+63:i];
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[63:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[63:0] := 0
///             FI;
///     FI;
/// ENDFOR
/// DEST[MAXVL-1:VL] := 0
/// VSUBPD (VEX.256 Encoded Version)
/// DEST[63:0] := SRC1[63:0] - SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] - SRC2[127:64]
/// DEST[191:128] := SRC1[191:128] - SRC2[191:128]
/// DEST[255:192] := SRC1[255:192] - SRC2[255:192]
/// DEST[MAXVL-1:256] := 0
/// VSUBPD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] - SRC2[63:0]
/// DEST[127:64] := SRC1[127:64] - SRC2[127:64]
/// DEST[MAXVL-1:128] := 0
/// SUBPD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] - SRC[63:0]
/// DEST[127:64] := DEST[127:64] - SRC[127:64]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn subpd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSUBPS (EVEX Encoded Versions When SRC2 Operand is a Vector Register)
/// (KL, VL) = (4, 128), (8, 256), (16, 512)
/// IF (VL = 512) AND (EVEX.b = 1)
///     THEN
///         SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
/// FI;
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask*
///             THEN DEST[i+31:i] := SRC1[i+31:i] - SRC2[i+31:i]
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[31:0] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VSUBPS (EVEX Encoded Versions When SRC2 Operand is a Memory Source)
/// (KL, VL) = (4, 128), (8, 256),(16, 512)
/// FOR j := 0 TO KL-1
///     i := j * 32
///     IF k1[j] OR *no writemask* THEN
///                 IF (EVEX.b = 1)
///                     THEN DEST[i+31:i] := SRC1[i+31:i] - SRC2[31:0];
///                     ELSE DEST[i+31:i] := SRC1[i+31:i] - SRC2[i+31:i];
///                 FI;
///     ELSE
///             IF *merging-masking*
///                         ; merging-masking
///                 THEN *DEST[31:0] remains unchanged*
///         ELSE ; zeroing-masking
///                     DEST[31:0] := 0
///             FI;
///     FI;
/// ENDFOR;
/// DEST[MAXVL-1:VL] := 0
/// VSUBPS (VEX.256 Encoded Version)
/// DEST[31:0] := SRC1[31:0] - SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] - SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] - SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] - SRC2[127:96]
/// DEST[159:128] := SRC1[159:128] - SRC2[159:128]
/// DEST[191:160] := SRC1[191:160] - SRC2[191:160]
/// DEST[223:192] := SRC1[223:192] - SRC2[223:192]
/// DEST[255:224] := SRC1[255:224] - SRC2[255:224].
/// DEST[MAXVL-1:256] := 0
/// VSUBPS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] - SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] - SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] - SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] - SRC2[127:96]
/// DEST[MAXVL-1:128] := 0
/// SUBPS (128-bit Legacy SSE Version)
/// DEST[31:0] := SRC1[31:0] - SRC2[31:0]
/// DEST[63:32] := SRC1[63:32] - SRC2[63:32]
/// DEST[95:64] := SRC1[95:64] - SRC2[95:64]
/// DEST[127:96] := SRC1[127:96] - SRC2[127:96]
/// DEST[MAXVL-1:128] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn subps() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSUBSD (EVEX Encoded Version)
/// IF (SRC2 *is register*) AND (EVEX.b = 1)
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[63:0] := SRC1[63:0] - SRC2[63:0]
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
/// VSUBSD (VEX.128 Encoded Version)
/// DEST[63:0] := SRC1[63:0] - SRC2[63:0]
/// DEST[127:64] := SRC1[127:64]
/// DEST[MAXVL-1:128] := 0
/// SUBSD (128-bit Legacy SSE Version)
/// DEST[63:0] := DEST[63:0] - SRC[63:0]
/// DEST[MAXVL-1:64] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn subsd() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// VSUBSS (EVEX Encoded Version)
/// IF (SRC2 *is register*) AND (EVEX.b = 1)
///     THEN
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);
///     ELSE
///             SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);
/// FI;
/// IF k1[0] or *no writemask*
///     THEN DEST[31:0] := SRC1[31:0] - SRC2[31:0]
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
/// VSUBSS (VEX.128 Encoded Version)
/// DEST[31:0] := SRC1[31:0] - SRC2[31:0]
/// DEST[127:32] := SRC1[127:32]
/// DEST[MAXVL-1:128] := 0
/// SUBSS (128-bit Legacy SSE Version)
/// DEST[31:0] := DEST[31:0] - SRC[31:0]
/// DEST[MAXVL-1:32] (Unmodified)
/// ```
#[box_to_static_reference]
pub(super) fn subss() -> &'static [IrStatement] {
    let assignment = assign(b::sub(o2(), o3()), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF CS.L ≠ 1 (* Not in 64-Bit Mode *)
///     THEN
///         #UD; FI;
/// IF CPL ≠ 0
///     THEN #GP(0); FI;
/// tmp := GS.base;
/// GS.base := IA32_KERNEL_GS_BASE;
/// IA32_KERNEL_GS_BASE := tmp;
/// ```
#[box_to_static_reference]
pub(super) fn swapgs() -> &'static [IrStatement] {
    [exception("swapgs")].into()
}

/// # Pseudocode
/// ```text
/// IF (CS.L ≠ 1 ) or (IA32_EFER.LMA ≠ 1) or (IA32_EFER.SCE ≠ 1)
/// (* Not in 64-Bit Mode or SYSCALL/SYSRET not enabled in IA32_EFER *)
///     THEN #UD;
/// FI;
/// RCX := RIP;
///                 (* Will contain address of next instruction *)
/// RIP := IA32_LSTAR;
/// R11 := RFLAGS;
/// RFLAGS := RFLAGS AND NOT(IA32_FMASK);
/// CS.Selector := IA32_STAR[47:32] AND FFFCH(* Operating system provides CS; RPL forced to 0 *)
/// (* Set rest of CS to a fixed value *)
/// CS.Base := 0;
///                     (* Flat segment *)
/// CS.Limit := FFFFFH;
///                     (* With 4-KByte granularity, implies a 4-GByte limit *)
/// CS.Type := 11;
///                     (* Execute/read code, accessed *)
/// CS.S := 1;
/// CS.DPL := 0;
/// CS.P := 1;
/// CS.L := 1;
///                     (* Entry is to 64-bit mode *)
/// CS.D := 0;
///                     (* Required if CS.L = 1 *)
/// CS.G := 1;
///                     (* 4-KByte granularity *)
/// IF ShadowStackEnabled(CPL)
///     THEN (* adjust so bits 63:N get the value of bit N-1, where N is the CPU's maximum linear-address width *)
///         IA32_PL3_SSP := LA_adjust(SSP);
///             (* With shadow stacks enabled the system call is supported from Ring 3 to Ring 0 *)
///             (* OS supporting Ring 0 to Ring 0 system calls or Ring 1/2 to ring 0 system call *)
///             (* Must preserve the contents of IA32_PL3_SSP to avoid losing ring 3 state *)
/// FI;
/// CPL := 0;
/// IF ShadowStackEnabled(CPL)
///     SSP := 0;
/// FI;
/// IF EndbranchEnabled(CPL)
///     IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///     IA32_S_CET.SUPPRESS = 0
/// FI;
/// SS.Selector := IA32_STAR[47:32] + 8;
///                     (* SS just above CS *)
/// (* Set rest of SS to a fixed value *)
/// SS.Base := 0;
///                     (* Flat segment *)
/// SS.Limit := FFFFFH;
///                     (* With 4-KByte granularity, implies a 4-GByte limit *)
/// SS.Type := 3;
///                     (* Read/write data, accessed *)
/// SS.S := 1;
/// SS.DPL := 0;
/// SS.P := 1;
/// SS.B := 1;
///                     (* 32-bit stack segment *)
/// SS.G := 1;
///                     (* 4-KByte granularity *)
/// ```
#[box_to_static_reference]
pub(super) fn syscall() -> &'static [IrStatement] {
    [exception("SYSCALL")].into()
}

/// # Pseudocode
/// ```text
/// IF CR0.PE = 0 OR IA32_SYSENTER_CS[15:2] = 0 THEN #GP(0); FI;
/// RFLAGS.VM := 0;
///                     (* Ensures protected mode execution *)
/// RFLAGS.IF := 0;
///                     (* Mask interrupts *)
/// IF in IA-32e mode
///     THEN
///         RSP := IA32_SYSENTER_ESP;
///         RIP := IA32_SYSENTER_EIP;
/// ELSE
///         ESP := IA32_SYSENTER_ESP[31:0];
///         EIP := IA32_SYSENTER_EIP[31:0];
/// FI;
/// CS.Selector := IA32_SYSENTER_CS[15:0] AND FFFCH;
///                     (* Operating system provides CS; RPL forced to 0 *)
/// (* Set rest of CS to a fixed value *)
/// CS.Base := 0;
///                     (* Flat segment *)
/// CS.Limit := FFFFFH;
///                     (* With 4-KByte granularity, implies a 4-GByte limit *)
/// CS.Type := 11;
///                     (* Execute/read code, accessed *)
/// CS.S := 1;
/// CS.DPL := 0;
/// CS.P := 1;
/// IF in IA-32e mode
///     THEN
///         CS.L := 1;
///                     (* Entry is to 64-bit mode *)
///         CS.D := 0;
///                     (* Required if CS.L = 1 *)
///     ELSE
///         CS.L := 0;
///         CS.D := 1;
///                     (* 32-bit code segment*)
/// FI;
/// CS.G := 1;
///                     (* 4-KByte granularity *)
/// IF ShadowStackEnabled(CPL)
///     THEN
///         IF IA32_EFER.LMA = 0
///             THEN IA32_PL3_SSP := SSP;
///             ELSE (* adjust so bits 63:N get the value of bit N-1, where N is the CPU's maximum linear-address width *)
///                 IA32_PL3_SSP := LA_adjust(SSP);
///         FI;
/// FI;
/// CPL := 0;
/// IF ShadowStackEnabled(CPL)
///     SSP := 0;
/// FI;
/// IF EndbranchEnabled(CPL)
///     IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///     IA32_S_CET.SUPPRESS = 0
/// FI;
/// SS.Selector := CS.Selector + 8;
///                     (* SS just above CS *)
/// (* Set rest of SS to a fixed value *)
/// SS.Base := 0;
///                     (* Flat segment *)
/// SS.Limit := FFFFFH;
///                     (* With 4-KByte granularity, implies a 4-GByte limit *)
/// SS.Type := 3;
///                     (* Read/write data, accessed *)
/// SS.S := 1;
/// SS.DPL := 0;
/// SS.P := 1;
/// SS.B := 1;
///                     (* 32-bit stack segment*)
/// SS.G := 1;
///                     (* 4-KByte granularity *)
/// ```
#[box_to_static_reference]
pub(super) fn sysenter() -> &'static [IrStatement] {
    [exception("SYSENTER")].into()
}

/// # Pseudocode
/// ```text
/// IF IA32_SYSENTER_CS[15:2] = 0 OR CR0.PE = 0 OR CPL ≠ 0 THEN #GP(0); FI;
/// IF operand size is 64-bit
///     THEN(* Return to 64-bit mode *)
///         RSP := RCX;
///         RIP := RDX;
///     ELSE(* Return to protected mode or compatibility mode *)
///         RSP := ECX;
///         RIP := EDX;
/// FI;
/// IF operand size is 64-bit
///             (* Operating system provides CS; RPL forced to 3 *)
///     THEN CS.Selector := IA32_SYSENTER_CS[15:0] + 32;
///     ELSE CS.Selector := IA32_SYSENTER_CS[15:0] + 16;
/// FI;
/// CS.Selector := CS.Selector OR 3;
///             (* RPL forced to 3 *)
/// (* Set rest of CS to a fixed value *)
/// CS.Base := 0;
///             (* Flat segment *)
/// CS.Limit := FFFFFH;
///             (* With 4-KByte granularity, implies a 4-GByte limit *)
/// CS.Type := 11;
///             (* Execute/read code, accessed *)
/// CS.S := 1;
/// CS.DPL := 3;
/// CS.P := 1;
/// IF operand size is 64-bit
///     THEN(* return to 64-bit mode *)
///         CS.L := 1;
///             (* 64-bit code segment *)
///         CS.D := 0;
///             (* Required if CS.L = 1 *)
///     ELSE(* return to protected mode or compatibility mode *)
///         CS.L := 0;
///         CS.D := 1;
///             (* 32-bit code segment*)
/// FI;
/// CS.G := 1;
///             (* 4-KByte granularity *)
/// CPL := 3;
/// IF ShadowStackEnabled(CPL)
///     THEN SSP := IA32_PL3_SSP;
/// FI;
/// SS.Selector := CS.Selector + 8;
///             (* SS just above CS *)
/// (* Set rest of SS to a fixed value *)
/// SS.Base := 0;
///             (* Flat segment *)
/// SS.Limit := FFFFFH;
///             (* With 4-KByte granularity, implies a 4-GByte limit *)
/// SS.Type := 3;
///             (* Read/write data, accessed *)
/// SS.S := 1;
/// SS.DPL := 3;
/// SS.P := 1;
/// SS.G := 1;
///             (* 4-KByte granularity *)
/// ```
#[box_to_static_reference]
pub(super) fn sysexit() -> &'static [IrStatement] {
    [exception("SYSEXIT")].into()
}

/// # Pseudocode
/// ```text
/// IF (CS.L ≠ 1 ) or (IA32_EFER.LMA ≠ 1) or (IA32_EFER.SCE ≠ 1)
/// (* Not in 64-Bit Mode or SYSCALL/SYSRET not enabled in IA32_EFER *)
///     THEN #UD; FI;
/// IF (CPL ≠ 0) THEN #GP(0); FI;
/// IF (operand size is 64-bit)
///     THEN (* Return to 64-Bit Mode *)
///         IF (RCX is not canonical) THEN #GP(0);
///         RIP := RCX;
///     ELSE (* Return to Compatibility Mode *)
///         RIP := ECX;
/// FI;
/// RFLAGS := (R11 & 3C7FD7H) | 2;
///             (* Clear RF, VM, reserved bits; set bit 1 *)
/// IF (operand size is 64-bit)
///     THEN CS.Selector := IA32_STAR[63:48]+16;
///     ELSE CS.Selector := IA32_STAR[63:48];
/// FI;
/// CS.Selector := CS.Selector OR 3;
///             (* RPL forced to 3 *)
/// (* Set rest of CS to a fixed value *)
/// CS.Base := 0;
///             (* Flat segment *)
/// CS.Limit := FFFFFH;
///             (* With 4-KByte granularity, implies a 4-GByte limit *)
/// CS.Type := 11;
///             (* Execute/read code, accessed *)
/// CS.S := 1;
/// CS.DPL := 3;
/// CS.P := 1;
/// IF (operand size is 64-bit)
///     THEN (* Return to 64-Bit Mode *)
///         CS.L := 1;
///             (* 64-bit code segment *)
///         CS.D := 0;
///             (* Required if CS.L = 1 *)
///     ELSE (* Return to Compatibility Mode *)
///         CS.L := 0;
///             (* Compatibility mode *)
///         CS.D := 1;
///             (* 32-bit code segment *)
/// FI;
/// CS.G := 1;
///             (* 4-KByte granularity *)
/// CPL := 3;
/// IF ShadowStackEnabled(CPL)
///     SSP := IA32_PL3_SSP;
/// FI;
/// SS.Selector := (IA32_STAR[63:48]+8) OR 3;(* RPL forced to 3 *)
/// (* Set rest of SS to a fixed value *)
/// SS.Base := 0;
///             (* Flat segment *)
/// SS.Limit := FFFFFH;
///             (* With 4-KByte granularity, implies a 4-GByte limit *)
/// SS.Type := 3;
///             (* Read/write data, accessed *)
/// SS.S := 1;
/// SS.DPL := 3;
/// SS.P := 1;
/// SS.B := 1;
///             (* 32-bit stack segment*)
/// ```
#[box_to_static_reference]
pub(super) fn sysret() -> &'static [IrStatement] {
    [exception("SYSRET")].into()
}
