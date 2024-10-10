# JMP

Jump

Transfers program control to a different point in the instruction stream without recording return information.
The destination (target) operand specifies the address of the instruction being jumped to.
This operand can be an immediate value, a general-purpose register, or a memory location.This instruction can be used to execute four different types of jumps: - Near jump-A jump to an instruction within the current code segment (the segment currently pointed to by the CS register), sometimes referred to as an intrasegment jump.
- Short jump-A near jump where the jump range is limited to -128 to +127 from the current EIP value.
- Far jump-A jump to an instruction located in a different segment than the current code segment but at the same privilege level, sometimes referred to as an intersegment jump.
- Task switch-A jump to an instruction located in a different task.
® A task switch can only be executed in protected mode (see Chapter 8, in the Intel64 and IA-32 Architectures Software Developer's Manual, Volume 3A, for information on performing task switches with the JMP instruction).Near and Short Jumps.
When executing a near jump, the processor jumps to the address (within the current code segment) that is specified with the target operand.
The target operand specifies either an absolute offset (that is value of the instruction pointer in the EIP register).
A near jump to a relative offset of 8-bits (rel8) is referred to as a short jump.
The CS register is not changed on near and short jumps.An absolute offset is specified indirectly in a general-purpose register or a memory location (r/m16 or r/m32).
The operand-size attribute determines the size of the target operand (16 or 32 bits).
Absolute offsets are loaded directly into the EIP register.
If the operand-size attribute is 16, the upper two bytes of the EIP register are cleared, resulting in a maximum instruction pointer size of 16 bits.A relative offset (rel8, rel16, or rel32) is generally specified as a label in assembly code, but at the machine code level, it is encoded as a signed 8-, 16-, or 32-bit immediate value.
This value is added to the value in the EIP register.
(Here, the EIP register contains the address of the instruction following the JMP instruction).
When using relative offsets, the opcode (for short vs.
near jumps) and the operand-size attribute (for near relative jumps) determines the size of the target operand (8, 16, or 32 bits).Far Jumps in Real-Address or Virtual-8086 Mode.
When executing a far jump in real-address or virtual-8086 mode, the processor jumps to the code segment and offset specified with the target operand.
Here the target operand specifies an absolute far address either directly with a pointer (ptr16:16 or ptr16:32) or indirectly with a memory location (m16:16 or m16:32).
With the pointer method, the segment and address of the called procedure is encoded in the instruction, using a 4-byte (16-bit operand size) or 6-byte (32-bit operand size) far address imme-diate.
With the indirect method, the target operand specifies a memory location that contains a 4-byte (16-bit operand size) or 6-byte (32-bit operand size) far address.
The far address is loaded directly into the CS and EIP registers.
If the operand-size attribute is 16, the upper two bytes of the EIP register are cleared.Far Jumps in Protected Mode.
When the processor is operating in protected mode, the JMP instruction can be used to perform the following three types of far jumps: - A far jump to a conforming or non-conforming code segment.
- A far jump through a call gate.
- A task switch.(The JMP instruction cannot be used to perform inter-privilege-level far jumps.)In protected mode, the processor always uses the segment selector part of the far address to access the corre-sponding descriptor in the GDT or LDT.
The descriptor type (code segment, call gate, task gate, or TSS) and access rights determine the type of jump to be performed.If the selected descriptor is for a code segment, a far jump to a code segment at the same privilege level is performed.
(If the selected code segment is at a different privilege level and the code segment is non-conforming, a general-protection exception is generated.) A far jump to the same privilege level in protected mode is very similar to one carried out in real-address or virtual-8086 mode.
The target operand specifies an absolute far address either directly with a pointer (ptr16:16 or ptr16:32) or indirectly with a memory location (m16:16 or m16:32).
The operand-size attribute determines the size of the offset (16 or 32 bits) in the far address.
The new code segment selector and its descriptor are loaded into CS register, and the offset from the instruction is loaded into the EIP register.
Note that a call gate (described in the next paragraph) can also be used to perform far call to a code segment at the same privilege level.
Using this mechanism provides an extra level of indirection and is the preferred method of making jumps between 16-bit and 32-bit code segments.When executing a far jump through a call gate, the segment selector specified by the target operand identifies the call gate.
(The offset part of the target operand is ignored.) The processor then jumps to the code segment speci-fied in the call gate descriptor and begins executing the instruction at the offset specified in the call gate.
No stack switch occurs.
Here again, the target operand can specify the far address of the call gate either directly with a pointer (ptr16:16 or ptr16:32) or indirectly with a memory location (m16:16 or m16:32).Executing a task switch with the JMP instruction is somewhat similar to executing a jump through a call gate.
Here the target operand specifies the segment selector of the task gate for the task being switched to (and the offset part of the target operand is ignored).
The task gate in turn points to the TSS for the task, which contains the segment selectors for the task's code and stack segments.
The TSS also contains the EIP value for the next instruc-tion that was to be executed before the task was suspended.
This instruction pointer value is loaded into the EIP register so that the task begins executing again at this next instruction.
The JMP instruction can also specify the segment selector of the TSS directly, which eliminates the indirection of the ® 64 and IA-32 Architectures Software Developer's Manual, Volume 3A, for Note that when you execute at task switch with a JMP instruction, the nested task flag (NT) is not set in the EFLAGS register and the new TSS's previous task link field is not loaded with the old task's TSS selector.
A return to the previous task can thus not be carried out by executing the IRET instruction.
Switching tasks with the JMP instruc-tion differs in this regard from the CALL instruction which does set the NT flag and save the previous task link infor-mation, allowing a return to the calling task with an IRET instruction.Refer to Chapter 6, "Procedure Calls, Interrupts, and Exceptions" and Chapter 17, "Control-flow Enforcement Tech-® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for CET details.nology (CET)" in the IntelIn 64-Bit Mode.
The instruction's operation size is fixed at 64 bits.
If a selector points to a gate, then RIP equals the 64-bit displacement taken from gate; else RIP equals the zero-extended offset from the far pointer referenced in the instruction.
See the summary chart at the beginning of this section for encoding data and limits.
Instruction ordering.
Instructions following a far jump may be fetched from memory before earlier instructions complete execution, but they will not execute (even speculatively) until all instructions prior to the far jump have completed execution (the later instructions may execute before data stored by the earlier instructions have become globally visible).Instructions sequentially following a near indirect JMP instruction (i.e., those not at the target) may be executed speculatively.
If software needs to prevent this (e.g., in order to prevent a speculative execution side channel), then an INT3 or LFENCE instruction opcode can be placed after the near indirect JMP in order to block speculative execution.

## Flags affected

- All flags are affected if a task switch occurs; no flags are affected if a task switch does not occur.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If offset in target operand, call gate, or TSS is beyond the code segment limits.
  > If the segment selector in the destination operand, call gate, task gate, or TSS is NULL.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  - #GP(selector) - If the segment selector index is outside descriptor table limits.
  > If the segment descriptor pointed to by the segment selector in the destination operand is not 
  > for a conforming-code segment, nonconforming-co
  > de segment, call gate, task gate, or task 
  > state segment.
  > If the DPL for a nonconforming-code
  >  segment is not equal to the CPL
  > (When not using a call gate.) If the RPL for the 
  > segment's segment selector is greater than the 
  > CPL.
  > If the DPL for a conforming-code segment is greater than the CPL.
  > If the DPL from a call-gate, task-gate, or TSS 
  > segment descriptor is less than the CPL or than 
  > the RPL of the call-gate, task-gate, or TSS's segment selector.
  > If the segment descriptor for selector in a call gate does not indicate it is a code segment.
  > If the segment descriptor for the segment selector 
  > in a task gate does not indicate an available 
  > TSS.
  > If the segment selector for a TSS has its local/global bit set for local.
  > If a TSS segment descriptor specifies that the TSS is busy or not available.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #NP (selector) - If the code segment being accessed is not present.
  > If call gate, task gate, or TSS not present.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3. (Only occu
  > rs when fetching target from memory.)
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If the target operand is beyond the code segment limits.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an un
  > aligned memory reference is made. (Only occurs 
  > when fetching target from memory.)
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same as 64-bit mode exceptions.
- 64-Bit Mode Exceptions
  - #GP(0) - If a memory address is non-canonical.
  > If target offset in destination operand is non-canonical.
  > If target offset in destination operand is beyond the new code segment limit.
  > If the segment selector in th
  > e destination operand is NULL.
  - #GP(selector) - If the code segment or 64-bit
  > call gate is outside descriptor table limits. 
  > If the code segment or 64-bit call gate overlaps non-canonical space. 
  > If the segment descriptor from a 64-bit call gate is in non-canonical space. 
  > If the segment descriptor pointed to by the segm
  > ent selector in the destination operand is not 
  > for a conforming-code segment, nonconforming-code segment, 64-bit call gate.
  > If the segment descriptor pointed to by the segment selector in the destination operand is a 
  > code segment, and has both the D-bit and the L-bit set.
  > If the DPL for a nonconforming-code segment is
  >  not equal to the CPL, or the RPL for the 
  > segment's segment selector is greater than the CPL.
  > If the DPL for a conforming-code segment is greater than the CPL.
  > If the DPL from a 64-bit call-gate is less than the CPL or than the RPL of the 64-bit call-gate.
  > If the upper type field of a 64-bit call gate is not 0x0.
  > If the segment selector from a 64-bit call gate is beyond the descriptor table limits.
  > If the code segment descriptor pointed to by the 
  > selector in the 64-bit gate doesn't have the L-
  > bit set and the D-bit clear.
  > If the segment descriptor for a segment selector fr
  > om the 64-bit call gate does not indicate it 
  > is a code segment. 
  > If the code segment is non-conforming and CPL 
  >  
  > DPL.
  > If the code segment is confirming and CPL < DPL.
  - #NP(selector) - If a code segment or 64-bit call gate is not present.
  - #UD - (64-bit mode only) If a far jump is direct to an absolute address in memory.
  > If the LOCK prefix is used.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an

## Operation

```C
IF near jumpIF 64-bit Mode THEN IF near relative jump THENtempRIP := RIP + DEST; (* RIP is instruction following JMP instruction*) ELSE (* Near absolute jump *)tempRIP := DEST;FI;ELSEIF near relative jump THENtempEIP := EIP + DEST; (* EIP is instruction following JMP instruction*) ELSE (* Near absolute jump *)tempEIP := DEST;FI;FI;== 0 or target mode  Compatibility mode) IF (IA32_EFER.LMA and tempEIP outside code segment limit THEN #GP(0); FIIF 64-bit mode and tempRIP is not canonicalTHEN #GP(0);FI;= 32IF OperandSize  THEN EIP := tempEIP;  ELSE =IF OperandSize  16=THEN (* OperandSize  16 *)EIP := tempEIP AND 0000FFFFH;= ELSE (* OperandSize FI; FI;IF (JMP near indirect, absolute indirect)IF EndbranchEnabledAndNotSuppressed(CPL)IF CPL = 3THENIF ( no 3EH prefix OR IA32_U_CET.NO_TRACK_EN == 0 )THENIA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCHFI;ELSEIF ( no 3EH prefix OR IA32_S_CET.NO_TRACK_EN == 0 )THENIA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCHFI;FI;FI;FI;FI;=== 0 or (PE  1 AND VM  1)) (* Real-address or virtual-8086 mode *)IF far jump and (PE  THEN tempEIP := DEST(Offset); (* DEST is ptr16:32 or [m16:32] *) IF tempEIP is beyond code segment limit THEN #GP(0); FI; CS := DEST(segment selector); (* DEST is ptr16:32 or [m16:32] *)= IF OperandSize  32 THENEIP := tempEIP; (* DEST is ptr16:32 or [m16:32] *)= ELSE (* OperandSize  16 *)EIP := tempEIP AND 0000FFFFH; (* Clear upper 16 bits *) FI;FI;== 1 and VM  0) IF far jump and (PE (* IA-32e mode or protected mode, not virtual-8086 mode *) THEN IF effective address in the CS, DS, ES, FS, GS, or SS segment is illegalor segment selector in target operand NULLTHEN #GP(0); FI; IF segment selector index not within descriptor table limitsTHEN #GP(new selector); FI;Read type and access rights of segment descriptor;= 0) IF (IA32_EFER.LMA THENIF segment type is not a conforming or nonconforming code segment, call gate, task gate, or TSS THEN #GP(segment selector); FI; ELSEIF segment type is not a conforming or nonconforming code segmentcall gateTHEN #GP(segment selector); FI; FI;Depending on type and access rights:GO TO CONFORMING-CODE-SEGMENT;GO TO CALL-GATE;GO TO TASK-GATE;GO TO TASK-STATE-SEGMENT; ELSE  #GP(segment selector);FI;CONFORMING-CODE-SEGMENT:=== 1 and D-BIT  1 and IA32_EFER.LMA  1IF L-Bit THEN GP(new code segment selector); FI; IF DPL > CPL THEN #GP(segment selector); FI; IF segment not presentTHEN #NP(segment selector); FI;tempEIP := DEST(Offset);= 16 IF OperandSize  THEN tempEIP := tempEIP AND 0000FFFFH; FI;== 0 or target mode  Compatibility mode) and IF (IA32_EFER.LMA tempEIP outside code segment limit THEN #GP(0); FIIF tempEIP is non-canonicalTHEN #GP(0); FI;IF ShadowStackEnabled(CPL)IF (IA32_EFER.LMA and DEST(segment selector).L) = 0(* If target is legacy or compatibility mode then the SSP must be in low 4GB *)IF (SSP & 0xFFFFFFFF00000000 != 0)THEN #GP(0); FI;FI;FI;CS := DEST[segment selector]; (* Segment descriptor information also loaded *)CS(RPL) := CPLEIP := tempEIP;IF EndbranchEnabled(CPL)IF CPL = 3THENIA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCHIA32_U_CET.SUPPRESS = 0ELSEIA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCHIA32_S_CET.SUPPRESS = 0FI;FI;END;NONCONFORMING-CODE-SEGMENT:=== 1 and D-BIT  1 and IA32_EFER.LMA  1IF L-Bit THEN GP(new code segment selector); FI; IF (RPL > CPL) OR (DPL CPL)THEN #GP(code segment selector); FI;IF segment not present THEN #NP(segment selector); FI;tempEIP := DEST(Offset);= 16 IF OperandSize  THEN tempEIP := tempEIP AND 0000FFFFH; FI;==IF (IA32_EFER.LMA and tempEIP outside code segment limit THEN #GP(0); FIIF tempEIP is non-canonical THEN #GP(0); FI;IF ShadowStackEnabled(CPL)IF (IA32_EFER.LMA and DEST(segment selector).L) = 0(* If target is legacy or compatibility mode then the SSP must be in low 4GB *)IF (SSP & 0xFFFFFFFF00000000 != 0)THEN #GP(0); FI;FI;FI;CS := DEST[segment selector]; (* Segment descriptor information also loaded *)CS(RPL) := CPL;EIP := tempEIP;IF EndbranchEnabled(CPL)IF CPL = 3THENIA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCHIA32_U_CET.SUPPRESS = 0ELSEIA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCHIA32_S_CET.SUPPRESS = 0FI;FI;END;CALL-GATE:IF call gate DPL < CPL or call gate DPL < call gate segment-selector RPL THEN #GP(call gate selector); FI;IF call gate not presentTHEN #NP(call gate selector); FI;IF call gate code-segment selector is NULLTHEN #GP(0); FI;IF call gate code-segment selector index outside descriptor table limitsTHEN #GP(code segment selector); FI;Read code segment descriptor;IF code-segment segment descriptor does not indicate a code segmentor code-segment segment descriptor is conforming and DPL > CPL  CPLor code-segment segment descriptor is non-conforming and DPLTHEN #GP(code segment selector); FI;=IF IA32_EFER.LMA  1 and (code-segment descriptor is not a 64-bit code segment or code-segment segment descriptor has both L-Bit and D-bit set)THEN #GP(code segment selector); FI;IF code segment is not presentTHEN #NP(code-segment selector); FI; tempEIP := DEST(Offset);= 16  IF GateSize  THEN tempEIP := tempEIP AND 0000FFFFH; FI;==IF (IA32_EFER.LMA  0 OR target mode  Compatibility mode) AND tempEIP outside code segment limit THEN #GP(0); FICS := DEST[SegmentSelector]; (* Segment descriptor information also loaded *)IF EndbranchEnabled(CPL)IF CPL = 3THENIA32_U_CET.TRACKER = WAIT_FOR_ENDBRANCH;IA32_U_CET.SUPPRESS = 0ELSEIA32_S_CET.TRACKER = WAIT_FOR_ENDBRANCH;IA32_S_CET.SUPPRESS = 0FI;FI;END;TASK-GATE:IF task gate DPL < CPL or task gate DPL < task gate segment-selector RPL THEN #GP(task gate selector); FI;IF task gate not present THEN #NP(gate selector); FI;Read the TSS segment selector in the task-gate descriptor;IF TSS segment selector local/global bit is set to localor index not within GDT limitsor descriptor is not a TSS segmentor TSS descriptor specifies that the TSS is busyTHEN #GP(TSS selector); FI; IF TSS not present THEN #NP(TSS selector); FI; SWITCH-TASKS to TSS; IF EIP not within code segment limit THEN #GP(0); FI;END;TASK-STATE-SEGMENT:IF TSS DPL < CPLor TSS DPL < TSS segment-selector RPLor TSS descriptor indicates TSS not availableTHEN #GP(TSS selector); FI;IF TSS is not presentTHEN #NP(TSS selector); FI;SWITCH-TASKS to TSS;IF EIP not within code segment limit THEN #GP(0); FI;END;
```
