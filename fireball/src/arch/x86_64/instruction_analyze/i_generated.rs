use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF SRC = 0
///     THEN #DE; (* Divide error *)
/// FI;
/// IF OperandSize = 8 (* Word/byte operation *)
///     THEN
///         temp := AX / SRC; (* Signed division *)
///         IF (temp > 7FH) or (temp < 80H)
///         (* If a positive result is greater than 7FH or a negative result is less than 80H *)
///             THEN #DE; (* Divide error *)
///             ELSE
///                 AL := temp;
///                 AH := AX SignedModulus SRC;
///         FI;
///     ELSE IF OperandSize = 16 (* Doubleword/word operation *)
///         THEN
///             temp := DX:AX / SRC; (* Signed division *)
///             IF (temp > 7FFFH) or (temp < 8000H)
///             (* If a positive result is greater than 7FFFH
///             or a negative result is less than 8000H *)
///                 THEN
///                     #DE; (* Divide error *)
///                 ELSE
///                     AX := temp;
///                     DX := DX:AX SignedModulus SRC;
///             FI;
///         FI;
///     ELSE IF OperandSize = 32 (* Quadword/doubleword operation *)
///             temp := EDX:EAX / SRC; (* Signed division *)
///             IF (temp > 7FFFFFFFH) or (temp < 80000000H)
///             (* If a positive result is greater than 7FFFFFFFH
///             or a negative result is less than 80000000H *)
///                 THEN
///                     #DE; (* Divide error *)
///                 ELSE
///                     EAX := temp;
///                     EDX := EDXE:AX SignedModulus SRC;
///             FI;
///         FI;
///     ELSE IF OperandSize = 64 (* Doublequadword/quadword operation *)
///             temp := RDX:RAX / SRC; (* Signed division *)
///             IF (temp > 7FFFFFFFFFFFFFFFH) or (temp < 8000000000000000H)
///             (* If a positive result is greater than 7FFFFFFFFFFFFFFFH
///             or a negative result is less than 8000000000000000H *)
///                 THEN
///                     #DE; (* Divide error *)
///                 ELSE
///                     RAX := temp;
///                     RDX := RDE:RAX SignedModulus SRC;
///             FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn idiv() -> &'static [IrStatement] {
    let operand_bit_size = bit_size_of_o1();
    let idiv_8 = [assign(b::signed_div(u::sign_extend(ax.clone()), u::sign_extend(o1())), al.clone(), o1_size()), assign(b::signed_rem(u::sign_extend(ax.clone()), u::sign_extend(o1())), ah.clone(), o1_size())];
    let value = b::add(b::shl(sized(rdx.clone(), o1_size()), operand_bit_size.clone()), sized(rax.clone(), o1_size()));
    let idiv_etc = [assign(b::signed_div(u::sign_extend(value.clone()), u::sign_extend(o1())), rax.clone(), o1_size()), assign(b::signed_rem(u::sign_extend(value), u::sign_extend(o1())), rdx.clone(), o1_size())];
    let idiv = condition(b::equal(operand_bit_size, c(8), size_unlimited()), idiv_8, idiv_etc);
    extend_undefined_flags(&[idiv], &[&of, &sf, &zf, &af, &cf, &pf])
}

/// # Pseudocode
/// ```text
/// IF (NumberOfOperands = 1)
///     THEN IF (OperandSize = 8)
///         THEN
///             TMP_XP := AL * SRC (* Signed multiplication; TMP_XP is a signed integer at twice the width of the SRC *);
///             AX := TMP_XP[15:0];
///             IF SignExtend(TMP_XP[7:0]) = TMP_XP
///                 THEN CF := 0; OF := 0;
///                 ELSE CF := 1; OF := 1; FI;
///         ELSE IF OperandSize = 16
///             THEN
///                 TMP_XP := AX * SRC (* Signed multiplication; TMP_XP is a signed integer at twice the width of the SRC *)
///                 DX:AX := TMP_XP[31:0];
///                 IF SignExtend(TMP_XP[15:0]) = TMP_XP
///                     THEN CF := 0; OF := 0;
///                     ELSE CF := 1; OF := 1; FI;
///             ELSE IF OperandSize = 32
///                 THEN
///                     TMP_XP := EAX * SRC (* Signed multiplication; TMP_XP is a signed integer at twice the width of the SRC*)
///                     EDX:EAX := TMP_XP[63:0];
///                     IF SignExtend(TMP_XP[31:0]) = TMP_XP
///                         THEN CF := 0; OF := 0;
///                         ELSE CF := 1; OF := 1; FI;
///                 ELSE (* OperandSize = 64 *)
///                     TMP_XP := RAX * SRC (* Signed multiplication; TMP_XP is a signed integer at twice the width of the SRC *)
///                     EDX:EAX := TMP_XP[127:0];
///                     IF SignExtend(TMP_XP[63:0]) = TMP_XP
///                         THEN CF := 0; OF := 0;
///                         ELSE CF := 1; OF := 1; FI;
///                 FI;
///     ELSE IF (NumberOfOperands = 2)
///         THEN
///             TMP_XP := DEST * SRC (* Signed multiplication; TMP_XP is a signed integer at twice the width of the SRC *)
///             DEST := TruncateToOperandSize(TMP_XP);
///             IF SignExtend(DEST) ≠ TMP_XP
///                 THEN CF := 1; OF := 1;
///                 ELSE CF := 0; OF := 0; FI;
///         ELSE (* NumberOfOperands = 3 *)
///             TMP_XP := SRC1 * SRC2 (* Signed multiplication; TMP_XP is a signed integer at twice the width of the SRC1 *)
///             DEST := TruncateToOperandSize(TMP_XP);
///             IF SignExtend(DEST) ≠ TMP_XP
///                 THEN CF := 1; OF := 1;
///                 ELSE CF := 0; OF := 0; FI;
///     FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn imul() -> &'static [IrStatement] {
    // IMUL: handles 1, 2, and 3-operand forms
    let result = b::mul(o1(), o2());
    let assignment = assign(result.clone(), o1(), o1_size());
    let calc_flags = calc_flags_automatically(result, o1_size(), &[&of, &cf]);
    extend_undefined_flags(&[calc_flags, assignment], &[&sf, &zf, &af, &pf])
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE ( * I/O operation is allowed *)
///                 DEST := SRC; (* Read from selected I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode with CPL ≤ IOPL *)
///         DEST := SRC; (* Read from selected I/O port *)
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn in() -> &'static [IrStatement] {
    let cond = condition(b::equal(unknown_data(), c(1), o1_size()), [stmt_0], []);
    [cond].into()
}

/// # Pseudocode
/// ```text
/// DEST := DEST + 1;
/// ```
#[box_to_static_reference]
pub(super) fn inc() -> &'static [IrStatement] {
    let add = b::add(o1(), c(1));
    let calc_flags = calc_flags_automatically(add.clone(), o1_size(), &[&of, &sf, &zf, &af, &pf]);
    let assignment = assign(add, o1(), o1_size());
    [calc_flags, assignment].into()
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
/// IF (operand size is 64-bit)
///     THEN
///         Range := R64[7:0];
///         shadow_stack_load 8 bytes from SSP;
///         IF Range > 0
/// THEN shadow_stack_load 8 bytes from SSP + 8 * (Range - 1);
///         FI;
///         SSP := SSP + Range * 8;
///     ELSE
///         Range := R32[7:0];
///         shadow_stack_load 4 bytes from SSP;
///         IF Range > 0
/// THEN shadow_stack_load 4 bytes from SSP + 4 * (Range - 1);
///         FI;
///         SSP := SSP + Range * 4;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn incsspd() -> &'static [IrStatement] {
    [exception("incsspd")].into()
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
/// IF (operand size is 64-bit)
///     THEN
///         Range := R64[7:0];
///         shadow_stack_load 8 bytes from SSP;
///         IF Range > 0
/// THEN shadow_stack_load 8 bytes from SSP + 8 * (Range - 1);
///         FI;
///         SSP := SSP + Range * 8;
///     ELSE
///         Range := R32[7:0];
///         shadow_stack_load 4 bytes from SSP;
///         IF Range > 0
/// THEN shadow_stack_load 4 bytes from SSP + 4 * (Range - 1);
///         FI;
///         SSP := SSP + Range * 4;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn incsspq() -> &'static [IrStatement] {
    [exception("incsspq")].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE (* I/O operation is allowed *)
///                 DEST := SRC; (* Read from I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode with CPL IOPL *)
///         DEST := SRC; (* Read from I/O port *)
/// FI;
/// Non-64-bit Mode:
/// IF (Byte transfer)
///     THEN IF DF = 0
///         THEN (E)DI := (E)DI + 1;
///         ELSE (E)DI := (E)DI - 1; FI;
///     ELSE IF (Word transfer)
///         THEN IF DF = 0
///             THEN (E)DI := (E)DI + 2;
///             ELSE (E)DI := (E)DI - 2; FI;
///         ELSE (* Doubleword transfer *)
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 4;
///                 ELSE (E)DI := (E)DI - 4; FI;
///         FI;
/// FI;
/// FI64-bit Mode:
/// IF (Byte transfer)
///     THEN IF DF = 0
///         THEN (E|R)DI := (E|R)DI + 1;
///         ELSE (E|R)DI := (E|R)DI - 1; FI;
///     ELSE IF (Word transfer)
///         THEN IF DF = 0
///             THEN (E)DI := (E)DI + 2;
///             ELSE (E)DI := (E)DI - 2; FI;
///         ELSE (* Doubleword transfer *)
///             THEN IF DF = 0
///                 THEN (E|R)DI := (E|R)DI + 4;
///                 ELSE (E|R)DI := (E|R)DI - 4; FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn ins() -> &'static [IrStatement] {
    [exception("ins")].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE (* I/O operation is allowed *)
///                 DEST := SRC; (* Read from I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode with CPL IOPL *)
///         DEST := SRC; (* Read from I/O port *)
/// FI;
/// Non-64-bit Mode:
/// IF (Byte transfer)
///     THEN IF DF = 0
///         THEN (E)DI := (E)DI + 1;
///         ELSE (E)DI := (E)DI - 1; FI;
///     ELSE IF (Word transfer)
///         THEN IF DF = 0
///             THEN (E)DI := (E)DI + 2;
///             ELSE (E)DI := (E)DI - 2; FI;
///         ELSE (* Doubleword transfer *)
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 4;
///                 ELSE (E)DI := (E)DI - 4; FI;
///         FI;
/// FI;
/// FI64-bit Mode:
/// IF (Byte transfer)
///     THEN IF DF = 0
///         THEN (E|R)DI := (E|R)DI + 1;
///         ELSE (E|R)DI := (E|R)DI - 1; FI;
///     ELSE IF (Word transfer)
///         THEN IF DF = 0
///             THEN (E)DI := (E)DI + 2;
///             ELSE (E)DI := (E)DI - 2; FI;
///         ELSE (* Doubleword transfer *)
///             THEN IF DF = 0
///                 THEN (E|R)DI := (E|R)DI + 4;
///                 ELSE (E|R)DI := (E|R)DI - 4; FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn insb() -> &'static [IrStatement] {
    [exception("insb")].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE (* I/O operation is allowed *)
///                 DEST := SRC; (* Read from I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode with CPL IOPL *)
///         DEST := SRC; (* Read from I/O port *)
/// FI;
/// Non-64-bit Mode:
/// IF (Byte transfer)
///     THEN IF DF = 0
///         THEN (E)DI := (E)DI + 1;
///         ELSE (E)DI := (E)DI - 1; FI;
///     ELSE IF (Word transfer)
///         THEN IF DF = 0
///             THEN (E)DI := (E)DI + 2;
///             ELSE (E)DI := (E)DI - 2; FI;
///         ELSE (* Doubleword transfer *)
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 4;
///                 ELSE (E)DI := (E)DI - 4; FI;
///         FI;
/// FI;
/// FI64-bit Mode:
/// IF (Byte transfer)
///     THEN IF DF = 0
///         THEN (E|R)DI := (E|R)DI + 1;
///         ELSE (E|R)DI := (E|R)DI - 1; FI;
///     ELSE IF (Word transfer)
///         THEN IF DF = 0
///             THEN (E)DI := (E)DI + 2;
///             ELSE (E)DI := (E)DI - 2; FI;
///         ELSE (* Doubleword transfer *)
///             THEN IF DF = 0
///                 THEN (E|R)DI := (E|R)DI + 4;
///                 ELSE (E|R)DI := (E|R)DI - 4; FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn insd() -> &'static [IrStatement] {
    [exception("insd")].into()
}

/// # Pseudocode
/// ```text
/// The following operational description applies not only to the INT n, INTO, INT3, or INT1 instructions, but also to
/// external interrupts, nonmaskable interrupts (NMIs), and exceptions. Some of these events push onto the stack an
/// error code.
/// The operational description specifies numerous checks whose failure may result in delivery of a nested exception.
/// In these cases, the original event is not delivered.
/// The operational description specifies the error code delivered by any nested exception. In some cases, the error
/// code is specified with a pseudofunction error_code(num,idt,ext), where idt and ext are bit values. The pseudofunc-
/// tion produces an error code as follows: (1)if idt is 0, the error code is (num& FCH)| ext; (2)if idt is 1, the error
/// code is (num<< 3)| 2| ext.
/// In many cases, the pseudofunction error_code is invoked with a pseudovariable EXT. The value of EXT depends on
/// the nature of the event whose delivery encountered a neste dexception: if that event is a software interrupn,t  (INT
/// INT3, or INTO), EXT is 0; otherwise (including INT1), EXT is 1.
/// IF PE = 0
///     THEN
///         GOTO REAL-ADDRESS-MODE;
///     ELSE (* PE = 1 *)
///         IF (EFLAGS.VM = 1 AND CR4.VME = 0 AND IOPL < 3 AND INT n)
///             THEN
///                 #GP(0); (* Bit 0 of error code is 0 because INTn *)
///             ELSE
///                 IF (EFLAGS.VM = 1 AND CR4.VME = 1 AND INT n)
///                     THEN
///                         Consult bit n of the software interrupt redirection bit map in the TSS;
///                         IF bit n is clear
///                             THEN (* redirect interrupt to 8086 program interrupt handler *)
///                                     Push EFLAGS[15:0];(* if IOPL < 3, save VIF in IF position and save IOPL position as 3 *)
///                                     Push CS;
///                                     Push IP;
///                                     IF IOPL = 3
///                                         THEN IF := 0; (* Clear interrupt flag *)
///                                         ELSE VIF := 0; (* Clear virtual interrupt flag *)
///                                     FI;
///                                     TF := 0; (* Clear trap flag *)
///                                     load CS and EIP (lower 16 bits only) from entry n in interrupt vector table referenced from TSS;
///                             ELSE
///                                     IF IOPL = 3
///                                         THEN GOTO PROTECTED-MODE;
///                                         ELSE #GP(0); (* Bit 0 of error code is 0 because INTn *)
///                                     FI;
///                         FI;
///                     ELSE (* Protected mode, IA-32e mode, or virtual-8086 mode interrupt *)
///                         IF (IA32_EFER.LMA = 0)
///                             THEN (* Protected mode, or virtual-8086 mode interrupt *)
///                                     GOTO PROTECTED-MODE;
///                             ELSE (* IA-32e mode interrupt *)
///                             GOTO IA-32e-MODE;
///                         FI;
///                 FI;
///         FI;
/// FI;
/// REAL-ADDRESS-MODE:
///     IF ((vector_number << 2) + 3) is not within IDT limit
///         THEN #GP; FI;
///     IF stack not large enough for a 6-byte return information
///         THEN #SS; FI;
///     Push (EFLAGS[15:0]);
///     IF := 0; (* Clear interrupt flag *)
///     TF := 0; (* Clear trap flag *)
///     AC := 0; (* Clear AC flag *)
///     Push(CS);
///     Push(IP);
///     (* No error codes are pushed in real-address mode*)
///     CS := IDT(Descriptor (vector_number << 2), selector));
///     EIP := IDT(Descriptor (vector_number << 2), offset)); (* 16 bit offset AND 0000FFFFH *)
/// END;
/// PROTECTED-MODE:
///     IF ((vector_number << 3) + 7) is not within IDT limits
///     or selected IDT descriptor is not an interrupt-, trap-, or task-gate type
///         THEN #GP(error_code(vector_number,1,EXT)); FI;
///         (* idt operand to error_code set because vector is used *)
///     IF software interrupt (* Generated by INT n, INT3, or INTO; does not apply to INT1 *)
///         THEN
///             IF gate DPL < CPL (* PE = 1, DPL < CPL, software interrupt *)
///                 THEN #GP(error_code(vector_number,1,0)); FI;
///                 (* idt operand to error_code set because vector is used *)
///                 (* ext operand to error_code is 0 because INTn, INT3, or INTO*)
///     FI;
///     IF gate not present
///         THEN #NP(error_code(vector_number,1,EXT)); FI;
///         (* idt operand to error_code set because vector is used *)
///     IF task gate (* Specified in the selected interrupt table descriptor *)
///         THEN GOTO TASK-GATE;
///         ELSE GOTO TRAP-OR-INTERRUPT-GATE; (* PE = 1, trap/interrupt gate *)
///     FI;
/// END;
/// IA-32e-MODE:
///     IF INTO and CS.L = 1 (64-bit mode)
///         THEN #UD;
///     FI;
///     IF ((vector_number << 4) + 15) is not in IDT limits
///     or selected IDT descriptor is not an interrupt-, or trap-gate type
///         THEN #GP(error_code(vector_number,1,EXT));
///         (* idt operand to error_code set because vector is used *)
///     FI;
///     IF software interrupt (* Generated by INT n, INT3, or INTO; does not apply to INT1 *)
///         THEN
///             IF gate DPL < CPL (* PE = 1, DPL < CPL, software interrupt *)
///                 THEN #GP(error_code(vector_number,1,0));
///                 (* idt operand to error_code set because vector is used *)
///             FI;
///     FI;
///     IF gate not present
///         THEN #NP(error_code(vector_number,1,EXT));
///         (* idt operand to error_code set because vector is used *)
///     FI;
///     GOTO TRAP-OR-INTERRUPT-GATE; (* Trap/interrupt gate *)
/// END;
/// TASK-GATE: (* PE = 1, task gate *)
///     Read TSS selector in task gate (IDT descriptor);
///         IF local/global bit is set to local or index not within GDT limits
///             THEN #GP(error_code(TSS selector,0,EXT)); FI;
///             (* idt operand to error_code is 0 because selector is used *)
///         Access TSS descriptor in GDT;
///         IF TSS descriptor specifies that the TSS is busy (low-order 5 bits set to 00001)
///             THEN #GP(error_code(TSS selector,0,EXT)); FI;
///             (* idt operand to error_code is 0 because selector is used *)
///         IF TSS not present
///             THEN #NP(error_code(TSS selector,0,EXT)); FI;
///             (* idt operand to error_code is 0 because selector is used *)
///     SWITCH-TASKS (with nesting) to TSS;
///     IF interrupt caused by fault with error code
///         THEN
///             IF stack limit does not allow push of error code
///                 THEN #SS(EXT); FI;
///             Push(error code);
///     FI;
///     IF EIP not within code segment limit
///         THEN #GP(EXT); FI;
/// END;
/// TRAP-OR-INTERRUPT-GATE:
///     Read new code-segment selector for trap or interrupt gate (IDT descriptor);
///     IF new code-segment selector is NULL
///         THEN #GP(EXT); FI; (* Error code contains NULL selector *)
///     IF new code-segment selector is not within its descriptor table limits
///         THEN #GP(error_code(new code-segment selector,0,EXT)); FI;
///         (* idt operand to error_code is 0 because selector is used *)
///     Read descriptor referenced by new code-segment selector;
///     IF descriptor does not indicate a code segment or new code-segment DPL > CPL
///         THEN #GP(error_code(new code-segment selector,0,EXT)); FI;
///         (* idt operand to error_code is 0 because selector is used *)
///     IF new code-segment descriptor is not present,
///         THEN #NP(error_code(new code-segment selector,0,EXT)); FI;
///         (* idt operand to error_code is 0 because selector is used *)
///     IF new code segment is non-conforming with DPL < CPL
///         THEN
///             IF VM = 0
///                 THEN
///                     GOTO INTER-PRIVILEGE-LEVEL-INTERRUPT;
///                     (* PE = 1, VM= 0, interrupt or trap gate, nonconforming code segment,
///                     DPL < CPL *)
///                 ELSE (* VM = 1 *)
///                     IF new code-segment DPL ≠ 0
///                         THEN #GP(error_code(new code-segment selector,0,EXT));
///                         (* idt operand to error_code is 0 because selector is used *)
///                     GOTO INTERRUPT-FROM-VIRTUAL-8086-MODE; FI;
///                     (* PE = 1, interrupt or trap gate, DPL < CPL, VM = 1 *)
///             FI;
///         ELSE (* PE = 1, interrupt or trap gate, DPL ≥ CPL *)
///             IF VM = 1
///                 THEN #GP(error_code(new code-segment selector,0,EXT));
///                 (* idt operand to error_code is 0 because selector is used *)
///             IF new code segment is conforming or new code-segment DPL = CPL
///                 THEN
///                     GOTO INTRA-PRIVILEGE-LEVEL-INTERRUPT;
///                 ELSE (* PE = 1, interrupt or trap gate, nonconforming code segment, DPL > CPL *)
///                     #GP(error_code(new code-segment selector,0,EXT));
///                     (* idt operand to error_code is 0 because selector is used *)
///             FI;
///     FI;
/// END;
/// INTER-PRIVILEGE-LEVEL-INTERRUPT:
///     (* PE = 1, interrupt or trap gate, non-conforming code segment, DPL < CPL *)
///     IF (IA32_EFER.LMA = 0) (* Not IA-32e mode *)
///         THEN
///         (* Identify stack-segment selector for new privilege level in current TSS *)
///             IF current TSS is 32-bit
///                 THEN
///                     TSSstackAddress := (new code-segment DPL << 3) + 4;
///                     IF (TSSstackAddress + 5) > current TSS limit
///                         THEN #TS(error_code(current TSS selector,0,EXT)); FI;
///                         (* idt operand to error_code is 0 because selector is used *)
///                     NewSS := 2 bytes loaded from (TSS base + TSSstackAddress + 4);
///                     NewESP := 4 bytes loaded from (TSS base + TSSstackAddress);
///                 ELSE(* current TSS is 16-bit *)
///                     TSSstackAddress := (new code-segment DPL << 2) + 2
///                     IF (TSSstackAddress + 3) > current TSS limit
///                         THEN #TS(error_code(current TSS selector,0,EXT)); FI;
///                         (* idt operand to error_code is 0 because selector is used *)
///                     NewSS := 2 bytes loaded from (TSS base + TSSstackAddress + 2);
///                     NewESP := 2 bytes loaded from (TSS base + TSSstackAddress);
///             FI;
///             IF NewSS is NULL
///                 THEN #TS(EXT); FI;
///             IF NewSS index is not within its descriptor-table limits
///             or NewSS RPL ≠ new code-segment DPL
///                 THEN #TS(error_code(NewSS,0,EXT)); FI;
///                 (* idt operand to error_code is 0 because selector is used *)
///             Read new stack-segment descriptor for NewSS in GDT or LDT;
///             IF new stack-segment DPL ≠ new code-segment DPL
///             or new stack-segment Type does not indicate writable data segment
///                 THEN #TS(error_code(NewSS,0,EXT)); FI;
///                 (* idt operand to error_code is 0 because selector is used *)
///             IF NewSS is not present
///                 THEN #SS(error_code(NewSS,0,EXT)); FI;
///                 (* idt operand to error_code is 0 because selector is used *)
///                 NewSSP := IA32_PLi_SSP (* where i = new code-segment DPL *)
///         ELSE (* IA-32e mode *)
///             IF IDT-gate IST = 0
///                 THEN TSSstackAddress := (new code-segment DPL << 3) + 4;
///                 ELSE TSSstackAddress := (IDT gate IST << 3) + 28;
///             FI;
///             IF (TSSstackAddress + 7) > current TSS limit
///                 THEN #TS(error_code(current TSS selector,0,EXT); FI;
///                 (* idt operand to error_code is 0 because selector is used *)
///             NewRSP := 8 bytes loaded from (current TSS base + TSSstackAddress);
///             NewSS := new code-segment DPL; (* NULL selector with RPL = new CPL *)
///             IF IDT-gate IST = 0
///                 THEN
///                     NewSSP := IA32_PLi_SSP (* where i = new code-segment DPL *)
///                 ELSE
///                     NewSSPAddress = IA32_INTERRUPT_SSP_TABLE_ADDR + (IDT-gate IST << 3)
///                     (* Check if shadow stacks are enabled at CPL 0 *)
///                     IF ShadowStackEnabled(CPL 0)
///                         THEN NewSSP := 8 bytes loaded from NewSSPAddress; FI;
///             FI;
///     FI;
///     IF IDT gate is 32-bit
///             THEN
///                 IF new stack does not have room for 24 bytes (error code pushed)
///                 or 20 bytes (no error code pushed)
///                     THEN #SS(error_code(NewSS,0,EXT)); FI;
///                     (* idt operand to error_code is 0 because selector is used *)
///             FI
///         ELSE
///             IF IDT gate is 16-bit
///                 THEN
///                     IF new stack does not have room for 12 bytes (error code pushed)
///                     or 10 bytes (no error code pushed);
///                         THEN #SS(error_code(NewSS,0,EXT)); FI;
///                         (* idt operand to error_code is 0 because selector is used *)
///             ELSE (* 64-bit IDT gate*)
///                 IF StackAddress is non-canonical
///                     THEN #SS(EXT); FI; (* Error code contains NULL selector *)
///         FI;
///     FI;
///     IF (IA32_EFER.LMA = 0) (* Not IA-32e mode *)
///         THEN
///             IF instruction pointer from IDT gate is not within new code-segment limits
///                 THEN #GP(EXT); FI; (* Error code contains NULL selector *)
///             ESP := NewESP;
///             SS := NewSS; (* Segment descriptor information also loaded *)
///         ELSE (* IA-32e mode *)
///             IF instruction pointer from IDT gate contains a non-canonical address
///                 THEN #GP(EXT); FI; (* Error code contains NULL selector *)
///             RSP := NewRSP & FFFFFFFFFFFFFFF0H;
///             SS := NewSS;
///     FI;
///     IF IDT gate is 32-bit
///         THEN
///             CS:EIP := Gate(CS:EIP); (* Segment descriptor information also loaded *)
///             IF IDT gate 16-bit
///                 THEN
///                     CS:IP := Gate(CS:IP);
///                     (* Segment descriptor information also loaded *)
///                 ELSE (* 64-bit IDT gate *)
///                     CS:RIP := Gate(CS:RIP);
///                     (* Segment descriptor information also loaded *)
///             FI;
///     FI;
///     IF IDT gate is 32-bit
///             THEN
///                 Push(far pointer to old stack);
///                 (* Old SS and ESP, 3 words padded to 4 *)
///                 Push(EFLAGS);
///                 Push(far pointer to return instruction);
///                 (* Old CS and EIP, 3 words padded to 4 *)
///                 Push(ErrorCode); (* If needed, 4 bytes *)
///             ELSE
///                 IF IDT gate 16-bit
///                     THEN
///                         Push(far pointer to old stack);
///                         (* Old SS and SP, 2 words *)
///                         Push(EFLAGS(15:0]);
///                         Push(far pointer to return instruction);
///                         (* Old CS and IP, 2 words *)
///                         Push(ErrorCode); (* If needed, 2 bytes *)
///                     ELSE (* 64-bit IDT gate *)
///                         Push(far pointer to old stack);
///                         (* Old SS and SP, each an 8-byte push *)
///                         Push(RFLAGS); (* 8-byte push *)
///                         Push(far pointer to return instruction);
///                         (* Old CS and RIP, each an 8-byte push *)
///                         Push(ErrorCode); (* If needed, 8-bytes *)
///             FI;
///     FI;
///     IF ShadowStackEnabled(CPL) AND CPL = 3
///         THEN
///             IF IA32_EFER.LMA = 0
///                 THEN IA32_PL3_SSP := SSP;
///                 ELSE (* adjust so bits 63:N get the value of bit N-1, where N is the CPU's maximum linear-address width *)
///                     IA32_PL3_SSP := LA_adjust(SSP);
///             FI;
///     FI;
///     CPL := new code-segment DPL;
///     CS(RPL) := CPL;
///     IF ShadowStackEnabled(CPL)
///         oldSSP := SSP
///         SSP := NewSSP
///         IF SSP & 0x07 != 0
///             THEN #GP(0); FI;
///         (* Token and CS:LIP:oldSSP pushed on shadow stack must be contained in a naturally aligned 32-byte region *)
///                                 - 24) & ~0x1F)
///         IF (SSP & ~0x1F) != ((SSP
///             #GP(0); FI;
///         IF ((IA32_EFER.LMA and CS.L) = 0 AND SSP[63:32] != 0)
///             THEN #GP(0); FI;
///         expected_token_value = SSP
///                                         (* busy bit - bit position 0 - must be clear *)
///         new_token_value = SSP | BUSY_BIT   (* Set the busy bit *)
///         IF shadow_stack_lock_cmpxchg8b(SSP, new_token_value, expected_token_value) != expected_token_value
///             THEN #GP(0); FI;
///         IF oldSS.DPL != 3
///             ShadowStackPush8B(oldCS); (* Padded with 48 high-order bits of 0 *)
///             ShadowStackPush8B(oldCSBASE + oldRIP); (* Padded with 32 high-order bits of 0 for 32 bit LIP*)
///             ShadowStackPush8B(oldSSP);
///         FI;
///     FI;
///     IF EndbranchEnabled (CPL)
///         IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH;
///         IA32_S_CET.SUPPRESS = 0
///     FI;
///     IF IDT gate is interrupt gate
///         THEN IF := 0 (* Interrupt flag set to 0, interrupts disabled *); FI;
///     TF := 0;
///     VM := 0;
///     RF := 0;
///     NT := 0;
/// END;
/// INTERRUPT-FROM-VIRTUAL-8086-MODE:
///     (* Identify stack-segment selector for privilege level 0 in current TSS *)
///     IF current TSS is 32-bit
///         THEN
///             IF TSS limit < 9
///                 THEN #TS(error_code(current TSS selector,0,EXT)); FI;
///                 (* idt operand to error_code is 0 because selector is used *)
///             NewSS := 2 bytes loaded from (current TSS base + 8);
///             NewESP := 4 bytes loaded from (current TSS base + 4);
///         ELSE (* current TSS is 16-bit *)
///             IF TSS limit < 5
///                 THEN #TS(error_code(current TSS selector,0,EXT)); FI;
///                 (* idt operand to error_code is 0 because selector is used *)
///             NewSS := 2 bytes loaded from (current TSS base + 4);
///             NewESP := 2 bytes loaded from (current TSS base + 2);
///     FI;
///     IF NewSS is NULL
///         THEN #TS(EXT); FI; (* Error code contains NULL selector *)
///     IF NewSS index is not within its descriptor table limits
///     or NewSS RPL ≠ 0
///         THEN #TS(error_code(NewSS,0,EXT)); FI;
///         (* idt operand to error_code is 0 because selector is used *)
///     Read new stack-segment descriptor for NewSS in GDT or LDT;
///     IF new stack-segment DPL ≠ 0 or stack segment does not indicate writable data segment
///         THEN #TS(error_code(NewSS,0,EXT)); FI;
///         (* idt operand to error_code is 0 because selector is used *)
///     IF new stack segment not present
///         THEN #SS(error_code(NewSS,0,EXT)); FI;
///         (* idt operand to error_code is 0 because selector is used *)
///     NewSSP := IA32_PL0_SSP (* the new code-segment DPL must be 0 *)
///     IF IDT gate is 32-bit
///         THEN
///             IF new stack does not have room for 40 bytes (error code pushed)
///             or 36 bytes (no error code pushed)
///                 THEN #SS(error_code(NewSS,0,EXT)); FI;
///                 (* idt operand to error_code is 0 because selector is used *)
///         ELSE (* IDT gate is 16-bit)
///             IF new stack does not have room for 20 bytes (error code pushed)
///             or 18 bytes (no error code pushed)
///                 THEN #SS(error_code(NewSS,0,EXT)); FI;
///                 (* idt operand to error_code is 0 because selector is used *)
///     FI;
///     IF instruction pointer from IDT gate is not within new code-segment limits
///         THEN #GP(EXT); FI; (* Error code contains NULL selector *)
///     tempEFLAGS := EFLAGS;
///     VM := 0;
///     TF := 0;
///     RF := 0;
///     NT := 0;
///     IF service through interrupt gate
///         THEN IF = 0; FI;
///     TempSS := SS;
///     TempESP := ESP;
///     SS := NewSS;
///     ESP := NewESP;
///     (* Following pushes are 16 bits for 16-bit IDT gates and 32 bits for 32-bit IDT gates;
///     Segment selector pushes in 32-bit mode are padded to two words *)
///     Push(GS);
///     Push(FS);
///     Push(DS);
///     Push(ES);
///     Push(TempSS);
///     Push(TempESP);
///     Push(TempEFlags);
///     Push(CS);
///     Push(EIP);
///     GS := 0; (* Segment registers made NULL, invalid for use in protected mode *)
///     FS := 0;
///     DS := 0;
///     ES := 0;
///     CS := Gate(CS); (* Segment descriptor information also loaded *)
///     CS(RPL) := 0;
///     CPL := 0;
///     IF IDT gate is 32-bit
///         THEN
///             EIP := Gate(instruction pointer);
///         ELSE (* IDT gate is 16-bit *)
///             EIP := Gate(instruction pointer) AND 0000FFFFH;
///     FI;
///     IF ShadowStackEnabled(0)
///         oldSSP := SSP
///         SSP := NewSSP
///         IF SSP & 0x07 != 0
///             THEN #GP(0); FI;
///         (* Token and CS:LIP:oldSSP pushed on shadow stack must be contained in a naturally aligned 32-byte region *)
///                                 - 24) & ~0x1F)
///             #GP(0); FI;
///     IF ((IA32_EFER.LMA and CS.L) = 0 AND SSP[63:32] != 0)
///         THEN #GP(0); FI;
///     expected_token_value = SSP (* busy bit - bit position 0 - must be clear *)
///     new_token_value = SSP | BUSY_BIT (* Set the busy bit *)
///     IF shadow_stack_lock_cmpxchg8b(SSP, new_token_value, expected_token_value) != expected_token_value
///         THEN #GP(0); FI;
///     FI;
///     IF EndbranchEnabled (CPL)
///         IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH;
///         IA32_S_CET.SUPPRESS = 0
///     FI;
/// (* Start execution of new routine in Protected Mode *)
/// END;
/// INTRA-PRIVILEGE-LEVEL-INTERRUPT:
///     NewSSP = SSP;
///     CHECK_SS_TOKEN = 0
///     (* PE = 1, DPL = CPL or conforming segment *)
///     IF IA32_EFER.LMA = 1 (* IA-32e mode *)
///         IF IDT-descriptor IST ≠ 0
///             THEN
///                 TSSstackAddress := (IDT-descriptor IST << 3) + 28;
///                 IF (TSSstackAddress + 7) > TSS limit
///                     THEN #TS(error_code(current TSS selector,0,EXT)); FI;
///                     (* idt operand to error_code is 0 because selector is used *)
///                 NewRSP := 8 bytes loaded from (current TSS base + TSSstackAddress);
///             ELSE NewRSP := RSP;
///         FI;
///         IF IDT-descriptor IST ≠ 0
///             IF ShadowStackEnabled(CPL)
///                 THEN
///                     NewSSPAddress = IA32_INTERRUPT_SSP_TABLE_ADDR + (IDT gate IST << 3)
///                     NewSSP := 8 bytes loaded from NewSSPAddress
///                     CHECK_SS_TOKEN = 1
///             FI;
///         FI;
///     FI;
///     IF 32-bit gate (* implies IA32_EFER.LMA = 0 *)
///         THEN
///             IF current stack does not have room for 16 bytes (error code pushed)
///             or 12 bytes (no error code pushed)
///                 THEN #SS(EXT); FI; (* Error code contains NULL selector *)
///         ELSE IF 16-bit gate (* implies IA32_EFER.LMA = 0 *)
///             IF current stack does not have room for 8 bytes (error code pushed)
///             or 6 bytes (no error code pushed)
///                 THEN #SS(EXT); FI; (* Error code contains NULL selector *)
///         ELSE (* IA32_EFER.LMA = 1, 64-bit gate*)
///                 IF NewRSP contains a non-canonical address
///                     THEN #SS(EXT); (* Error code contains NULL selector *)
///         FI;
///     FI;
///     IF (IA32_EFER.LMA = 0) (* Not IA-32e mode *)
///             IF instruction pointer from IDT gate is not within new code-segment limit
///                 THEN #GP(EXT); FI; (* Error code contains NULL selector *)
///         ELSE
///             IF instruction pointer from IDT gate contains a non-canonical address
///                 THEN #GP(EXT); FI; (* Error code contains NULL selector *)
///             RSP := NewRSP & FFFFFFFFFFFFFFF0H;
///     FI;
///     IF IDT gate is 32-bit (* implies IA32_EFER.LMA = 0 *)
///         THEN
///             Push (EFLAGS);
///             Push (far pointer to return instruction); (* 3 words padded to 4 *)
///             CS:EIP := Gate(CS:EIP); (* Segment descriptor information also loaded *)
///             Push (ErrorCode); (* If any *)
///         ELSE
///             IF IDT gate is 16-bit (* implies IA32_EFER.LMA = 0 *)
///                 THEN
///                     Push (FLAGS);
///                     Push (far pointer to return location); (* 2 words *)
///                     CS:IP := Gate(CS:IP);
///                     (* Segment descriptor information also loaded *)
///                     Push (ErrorCode); (* If any *)
///                 ELSE (* IA32_EFER.LMA = 1, 64-bit gate*)
///                     Push(far pointer to old stack);
///                     (* Old SS and SP, each an 8-byte push *)
///                     Push(RFLAGS); (* 8-byte push *)
///                     Push(far pointer to return instruction);
///                     (* Old CS and RIP, each an 8-byte push *)
///                     Push(ErrorCode); (* If needed, 8 bytes *)
///                     CS:RIP := GATE(CS:RIP);
///                     (* Segment descriptor information also loaded *)
///             FI;
///     FI;
///     CS(RPL) := CPL;
///     IF ShadowStackEnabled(CPL)
///         IF CHECK_SS_TOKEN == 1
///             THEN
///                 IF NewSSP & 0x07 != 0
///                     THEN #GP(0); FI;
///         (* Token and CS:LIP:oldSSP pushed on shadow stack must be contained in a naturally aligned 32-byte region *)
///         IF (NewSSP & ~0x1F) != ((NewSSP - 24) & ~0x1F)
///             #GP(0); FI;
///                 IF ((IA32_EFER.LMA and CS.L) = 0 AND NewSSP[63:32] != 0)
/// 
///                     THEN #GP(0); FI;
///                 expected_token_value = NewSSP (* busy bit - bit position 0 - must be clear *)
///                 new_token_value = NewSSP | BUSY_BIT (* Set the busy bit *)
///                 IF shadow_stack_lock_cmpxchg8b(NewSSP, new_token_value, expected_token_value) != expected_token_value
///                     THEN #GP(0); FI;
///         FI;
///         (* Align to next 8 byte boundary *)
///         tempSSP = SSP;
///         Shadow_stack_store 4 bytes of 0 to (NewSSP - 4)
///         SSP = newSSP & 0xFFFFFFFFFFFFFFF8H;
///         (* push cs:lip:ssp on shadow stack *)
///         ShadowStackPush8B(oldCS); (* Padded with 48 high-order bits of 0 *)
///         ShadowStackPush8B(oldCSBASE + oldRIP); (* Padded with 32 high-order bits of 0 for 32 bit LIP*)
///         ShadowStackPush8B(tempSSP);
///     FI;
///     IF EndbranchEnabled (CPL)
///         IF CPL = 3
///             THEN
///                 IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_U_CET.SUPPRESS = 0
///             ELSE
///                 IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_S_CET.SUPPRESS = 0
///         FI;
///     FI;
///     IF IDT gate is interrupt gate
///         THEN IF := 0; FI; (* Interrupt flag set to 0; interrupts disabled *)
///     TF := 0;
///     NT := 0;
///     VM := 0;
///     RF := 0;
/// END;
/// ```
#[box_to_static_reference]
pub(super) fn insertps() -> &'static [IrStatement] {
    let assignment = assign(o2(), o1(), o1_size());
    [assignment].into()
}

/// # Pseudocode
/// ```text
/// IF ((PE = 1) and ((CPL > IOPL) or (VM = 1)))
///     THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)
///         IF (Any I/O Permission Bit for I/O port being accessed = 1)
///             THEN (* I/O operation is not allowed *)
///                 #GP(0);
///             ELSE (* I/O operation is allowed *)
///                 DEST := SRC; (* Read from I/O port *)
///         FI;
///     ELSE (Real Mode or Protected Mode with CPL IOPL *)
///         DEST := SRC; (* Read from I/O port *)
/// FI;
/// Non-64-bit Mode:
/// IF (Byte transfer)
///     THEN IF DF = 0
///         THEN (E)DI := (E)DI + 1;
///         ELSE (E)DI := (E)DI - 1; FI;
///     ELSE IF (Word transfer)
///         THEN IF DF = 0
///             THEN (E)DI := (E)DI + 2;
///             ELSE (E)DI := (E)DI - 2; FI;
///         ELSE (* Doubleword transfer *)
///             THEN IF DF = 0
///                 THEN (E)DI := (E)DI + 4;
///                 ELSE (E)DI := (E)DI - 4; FI;
///         FI;
/// FI;
/// FI64-bit Mode:
/// IF (Byte transfer)
///     THEN IF DF = 0
///         THEN (E|R)DI := (E|R)DI + 1;
///         ELSE (E|R)DI := (E|R)DI - 1; FI;
///     ELSE IF (Word transfer)
///         THEN IF DF = 0
///             THEN (E)DI := (E)DI + 2;
///             ELSE (E)DI := (E)DI - 2; FI;
///         ELSE (* Doubleword transfer *)
///             THEN IF DF = 0
///                 THEN (E|R)DI := (E|R)DI + 4;
///                 ELSE (E|R)DI := (E|R)DI - 4; FI;
///         FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn insw() -> &'static [IrStatement] {
    [exception("insw")].into()
}

/// # Pseudocode
/// ```text
/// Flush(InternalCaches);
/// SignalFlush(ExternalCaches);
/// Continue (* Continue execution *)
/// ```
#[box_to_static_reference]
pub(super) fn invd() -> &'static [IrStatement] {
    [exception("INVD")].into()
}

/// # Pseudocode
/// ```text
/// Invalidate(RelevantTLBEntries);
/// Continue; (* Continue execution *)
/// ```
#[box_to_static_reference]
pub(super) fn invlpg() -> &'static [IrStatement] {
    [exception("INVLPG")].into()
}

/// # Pseudocode
/// ```text
/// INVPCID_TYPE := value of register operand;
///                 // must be in the range of 0-3
/// INVPCID_DESC := value of memory operand;
/// CASE INVPCID_TYPE OF
///     0:
///             // individual-address invalidation
///         PCID := INVPCID_DESC[11:0];
///         L_ADDR := INVPCID_DESC[127:64];
///         Invalidate mappings for L_ADDR associated with PCID except global translations;
///         BREAK;
///     1:
///             // single PCID invalidation
///         PCID := INVPCID_DESC[11:0];
///         Invalidate all mappings associated with PCID except global translations;
///         BREAK;
///     2:
///             // all PCID invalidation including global translations
///         Invalidate all mappings for all PCIDs, including global translations;
///         BREAK;
///     3:
///             // all PCID invalidation retaining global translations
///         Invalidate all mappings for all PCIDs except global translations;
///         BREAK;
/// ESAC;
/// ```
#[box_to_static_reference]
pub(super) fn invpcid() -> &'static [IrStatement] {
    [exception("invpcid")].into()
}

/// # Pseudocode
/// ```text
/// IF condition
///     THEN
/// 
///         tempEIP := EIP + SignExtend(DEST);
/// 
///         IF OperandSize = 16
///             THEN tempEIP := tempEIP AND 0000FFFFH;
/// FI;
///     IF tempEIP is not within code segment limit
///         THEN #GP(0);
/// 
///         ELSE EIP := tempEIP
/// FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn iret() -> &'static [IrStatement] {
    let jmp = jump(d(rsp.clone()));
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let operand_condition = condition(is_o1_exists(), [assign(b::add(rsp.clone(), u::zero_extend(o1())), rsp.clone(), size_architecture())], []);
    let halt = halt();
    [set_sp, operand_condition, jmp, halt].into()
}

/// # Pseudocode
/// ```text
/// IF condition
///     THEN
/// 
///         tempEIP := EIP + SignExtend(DEST);
/// 
///         IF OperandSize = 16
///             THEN tempEIP := tempEIP AND 0000FFFFH;
/// FI;
///     IF tempEIP is not within code segment limit
///         THEN #GP(0);
/// 
///         ELSE EIP := tempEIP
/// FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn iretd() -> &'static [IrStatement] {
    let jmp = jump(d(rsp.clone()));
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let operand_condition = condition(is_o1_exists(), [assign(b::add(rsp.clone(), u::zero_extend(o1())), rsp.clone(), size_architecture())], []);
    let halt = halt();
    [set_sp, operand_condition, jmp, halt].into()
}

/// # Pseudocode
/// ```text
/// IF condition
///     THEN
/// 
///         tempEIP := EIP + SignExtend(DEST);
/// 
///         IF OperandSize = 16
///             THEN tempEIP := tempEIP AND 0000FFFFH;
/// FI;
///     IF tempEIP is not within code segment limit
///         THEN #GP(0);
/// 
///         ELSE EIP := tempEIP
/// FI;
/// FI;
/// ```
#[box_to_static_reference]
pub(super) fn iretq() -> &'static [IrStatement] {
    let jmp = jump(d(rsp.clone()));
    let set_sp = assign(b::add(rsp.clone(), architecture_byte_size()), rsp.clone(), size_architecture());
    let operand_condition = condition(is_o1_exists(), [assign(b::add(rsp.clone(), u::zero_extend(o1())), rsp.clone(), size_architecture())], []);
    let halt = halt();
    [set_sp, operand_condition, jmp, halt].into()
}
