use super::{super::static_register::*, shortcuts::*};
use std::ops::Deref;

/// # Pseudocode
/// ```text
/// IF near jump
///     IF 64-bit Mode
///     THEN
///             IF near relative jump
///             THEN
///                 tempRIP := RIP + DEST; (* RIP is instruction following JMP instruction*)
///             ELSE (* Near absolute jump *)
///                 tempRIP := DEST;
///             FI;
///         ELSE
///             IF near relative jump
///             THEN
///                 tempEIP := EIP + DEST; (* EIP is instruction following JMP instruction*)
///             ELSE (* Near absolute jump *)
///                 tempEIP := DEST;
///             FI;
///     FI;
///     IF (IA32_EFER.LMA = 0 or target mode = Compatibility mode)
///     and tempEIP outside code segment limit
///         THEN #GP(0); FI
///     IF 64-bit mode and tempRIP is not canonical
///         THEN #GP(0);
///     FI;
///     IF OperandSize = 32
///         THEN
///             EIP := tempEIP;
///         ELSE
///             IF OperandSize = 16
///                 THEN (* OperandSize = 16 *)
///                     EIP := tempEIP AND 0000FFFFH;
///                 ELSE (* OperandSize = 64)
///                 RIP := tempRIP;
///             FI;
///     FI;
///     IF (JMP near indirect, absolute indirect)
///         IF EndbranchEnabledAndNotSuppressed(CPL)
///             IF CPL = 3
///                 THEN
///                     IF ( no 3EH prefix OR IA32_U_CET.NO_TRACK_EN == 0 )
///                         THEN
///                             IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                     FI;
///                 ELSE
///                     IF ( no 3EH prefix OR IA32_S_CET.NO_TRACK_EN == 0 )
///                         THEN
///                             IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                     FI;
///             FI;
///         FI;
///     FI;
/// FI;
/// IF far jump and (PE = 0 or (PE = 1 AND VM = 1)) (* Real-address or virtual-8086 mode *)
///     THEN
///         tempEIP := DEST(Offset); (* DEST is ptr16:32 or [m16:32] *)
///         IF tempEIP is beyond code segment limit
///             THEN #GP(0); FI;
///         CS := DEST(segment selector); (* DEST is ptr16:32 or [m16:32] *)
///         IF OperandSize = 32
///             THEN
///                 EIP := tempEIP; (* DEST is ptr16:32 or [m16:32] *)
///             ELSE (* OperandSize = 16 *)
///                 EIP := tempEIP AND 0000FFFFH; (* Clear upper 16 bits *)
///         FI;
/// FI;
/// IF far jump and (PE = 1 and VM = 0)
/// (* IA-32e mode or protected mode, not virtual-8086 mode *)
///     THEN
///         IF effective address in the CS, DS, ES, FS, GS, or SS segment is illegal
///         or segment selector in target operand NULL
///                 THEN #GP(0); FI;
///         IF segment selector index not within descriptor table limits
///             THEN #GP(new selector); FI;
///         Read type and access rights of segment descriptor;
///         IF (IA32_EFER.LMA = 0)
///             THEN
///                 IF segment type is not a conforming or nonconforming code
///                 segment, call gate, task gate, or TSS
///                     THEN #GP(segment selector); FI;
///             ELSE
///                 IF segment type is not a conforming or nonconforming code segment
///                 call gate
///                     THEN #GP(segment selector); FI;
///         FI;
///         Depending on type and access rights:
///             GO TO CONFORMING-CODE-SEGMENT;
///             GO TO NONCONFORMING-CODE-SEGMENT;
///             GO TO CALL-GATE;
///             GO TO TASK-GATE;
///             GO TO TASK-STATE-SEGMENT;
///     ELSE
///         #GP(segment selector);
/// FI;
/// CONFORMING-CODE-SEGMENT:
///     IF L-Bit = 1 and D-BIT = 1 and IA32_EFER.LMA = 1
///         THEN GP(new code segment selector); FI;
///     IF DPL > CPL
///         THEN #GP(segment selector); FI;
///     IF segment not present
///         THEN #NP(segment selector); FI;
///     tempEIP := DEST(Offset);
///     IF OperandSize = 16
///         THEN tempEIP := tempEIP AND 0000FFFFH;
///     FI;
///     IF (IA32_EFER.LMA = 0 or target mode = Compatibility mode) and
///     tempEIP outside code segment limit
///         THEN #GP(0); FI
///     IF tempEIP is non-canonical
///         THEN #GP(0); FI;
///     IF ShadowStackEnabled(CPL)
///         IF (IA32_EFER.LMA and DEST(segment selector).L) = 0
///             (* If target is legacy or compatibility mode then the SSP must be in low 4GB *)
///             IF (SSP & 0xFFFFFFFF00000000 != 0)
///                 THEN #GP(0); FI;
///         FI;
///     FI;
///     CS := DEST[segment selector]; (* Segment descriptor information also loaded *)
///     CS(RPL) := CPL
///     EIP := tempEIP;
///     IF EndbranchEnabled(CPL)
///         IF CPL = 3
///             THEN
///                 IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_U_CET.SUPPRESS = 0
///             ELSE
///                 IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_S_CET.SUPPRESS = 0
///         FI;
///     FI;
/// END;
/// NONCONFORMING-CODE-SEGMENT:
///     IF L-Bit = 1 and D-BIT = 1 and IA32_EFER.LMA = 1
///         THEN GP(new code segment selector); FI;
///     IF (RPL > CPL) OR (DPL ≠ CPL)
///         THEN #GP(code segment selector); FI;
///     IF segment not present
///         THEN #NP(segment selector); FI;
///     tempEIP := DEST(Offset);
///     IF OperandSize = 16
///         THEN tempEIP := tempEIP AND 0000FFFFH; FI;
///     IF (IA32_EFER.LMA = 0 OR target mode = Compatibility mode)
///     and tempEIP outside code segment limit
///         THEN #GP(0); FI
///     IF tempEIP is non-canonical THEN #GP(0); FI;
///     IF ShadowStackEnabled(CPL)
///         IF (IA32_EFER.LMA and DEST(segment selector).L) = 0
///             (* If target is legacy or compatibility mode then the SSP must be in low 4GB *)
///             IF (SSP & 0xFFFFFFFF00000000 != 0)
///                 THEN #GP(0); FI;
///         FI;
///     FI;
///     CS := DEST[segment selector]; (* Segment descriptor information also loaded *)
///     CS(RPL) := CPL;
///     EIP := tempEIP;
///     IF EndbranchEnabled(CPL)
///         IF CPL = 3
///             THEN
///                 IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_U_CET.SUPPRESS = 0
///             ELSE
///                 IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH
///                 IA32_S_CET.SUPPRESS = 0
///         FI;
///     FI;
/// END;
/// CALL-GATE:
///     IF call gate DPL < CPL
///     or call gate DPL < call gate segment-selector RPL
///             THEN #GP(call gate selector); FI;
///     IF call gate not present
///         THEN #NP(call gate selector); FI;
///     IF call gate code-segment selector is NULL
///         THEN #GP(0); FI;
///     IF call gate code-segment selector index outside descriptor table limits
///         THEN #GP(code segment selector); FI;
///     Read code segment descriptor;
///     IF code-segment segment descriptor does not indicate a code segment
///     or code-segment segment descriptor is conforming and DPL > CPL
///     or code-segment segment descriptor is non-conforming and DPL ≠ CPL
///             THEN #GP(code segment selector); FI;
///     IF IA32_EFER.LMA = 1 and (code-segment descriptor is not a 64-bit code segment
///     or code-segment segment descriptor has both L-Bit and D-bit set)
///             THEN #GP(code segment selector); FI;
///     IF code segment is not present
///         THEN #NP(code-segment selector); FI;
///     tempEIP := DEST(Offset);
///     IF GateSize = 16
///         THEN tempEIP := tempEIP AND 0000FFFFH; FI;
///     IF (IA32_EFER.LMA = 0 OR target mode = Compatibility mode) AND tempEIP
///     outside code segment limit
///         THEN #GP(0); FI
///     CS := DEST[SegmentSelector]; (* Segment descriptor information also loaded *)
///     CS(RPL) := CPL;
///     EIP := tempEIP;
///     IF EndbranchEnabled(CPL)
///         IF CPL = 3
///             THEN
///                 IA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH;
///                 IA32_U_CET.SUPPRESS = 0
///             ELSE
///                 IA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH;
///                 IA32_S_CET.SUPPRESS = 0
///         FI;
///     FI;
/// END;
/// TASK-GATE:
///     IF task gate DPL < CPL
///     or task gate DPL < task gate segment-selector RPL
///         THEN #GP(task gate selector); FI;
///     IF task gate not present
///         THEN #NP(gate selector); FI;
///     Read the TSS segment selector in the task-gate descriptor;
///     IF TSS segment selector local/global bit is set to local
///     or index not within GDT limits
///     or descriptor is not a TSS segment
///     or TSS descriptor specifies that the TSS is busy
///         THEN #GP(TSS selector); FI;
///     IF TSS not present
///         THEN #NP(TSS selector); FI;
///     SWITCH-TASKS to TSS;
///     IF EIP not within code segment limit
///         THEN #GP(0); FI;
/// END;
/// TASK-STATE-SEGMENT:
///     IF TSS DPL < CPL
///     or TSS DPL < TSS segment-selector RPL
///     or TSS descriptor indicates TSS not available
///         THEN #GP(TSS selector); FI;
///     IF TSS is not present
///         THEN #NP(TSS selector); FI;
///     SWITCH-TASKS to TSS;
///     IF EIP not within code segment limit
///         THEN #GP(0); FI;
/// END;
/// ```
#[box_to_static_reference]
pub(super) fn jmp() -> &'static [IrStatement] {
    [jump(o1())].into()
}
