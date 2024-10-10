# IRET/IRETD/IRETQ

Interrupt Return

Returns program control from an exception or interrupt handler to a program or procedure that was interrupted by an exception, an external interrupt, or a software-generated interrupt.
These instructions are also used to perform a return from a nested task.
(A nested task is created when a CALL instruction is used to initiate a task switch or when an interrupt or exception causes a task switch to an interrupt or exception handler.) See the section titled ® 64 and IA-32 Architectures Software Developer's Manual, Volume 3A."Task Linking" in Chapter 8 of the IntelIRET and IRETD are mnemonics for the same opcode.
The IRETD mnemonic (interrupt return double) is intended for use when returning from an interrupt when using the 32-bit operand size; however, most assemblers use the IRET mnemonic interchangeably for both operand sizes.In Real-Address Mode, the IRET instruction performs a far return to the interrupted program or procedure.
During this operation, the processor pops the return instruction pointer, return code segment selector, and EFLAGS image from the stack to the EIP, CS, and EFLAGS registers, respectively, and then resumes execution of the interrupted program or procedure.In Protected Mode, the action of the IRET instruction depends on the settings of the NT (nested task) and VM flags in the EFLAGS register and the VM flag in the EFLAGS image stored on the current stack.
Depending on the setting of these flags, the processor performs the following types of interrupt returns: - Return from virtual-8086 mode.
- Return to virtual-8086 mode.
- Intra-privilege level return.
- Inter-privilege level return.
- Return from nested task (task switch).If the NT flag (EFLAGS register) is cleared, the IRET instruction performs a far return from the interrupt procedure, without a task switch.
The code segment being returned to must be equally or less privileged than the interrupt handler routine (as indicated by the RPL field of the code segment selector popped from the stack).
As with a real-address mode interrupt return, the IRET instruction pops the return instruction pointer, return code segment selector, and EFLAGS image from the stack to the EIP, CS, and EFLAGS registers, respectively, and then resumes execution of the interrupted program or procedure.
If the return is to another privilege level, the IRET instruction also pops the stack pointer and SS from the stack, before resuming program execution.
If the return is to virtual-8086 mode, the processor also pops the data segment registers from the stack.If the NT flag is set, the IRET instruction performs a task switch (return) from a nested task (a task called with a CALL instruction, an interrupt, or an exception) back to the calling or interrupted task.
The updated state of the task executing the IRET instruction is saved in its TSS.
If the task is re-entered later, the code that follows the IRET instruction is executed.If the NT flag is set and the processor is in IA-32e mode, the IRET instruction causes a general protection excep-tion.® 64 and If nonmaskable interrupts (NMIs) are blocked (see Section 6.7.1, "Handling Multiple NMIs" in the IntelThis unblocking occurs even if the instruction causes a fault.
In such a case, NMIs are unmasked before the excep-tion handler is invoked.In 64-bit mode, the instruction's default operation size is 32 bits.
Use of the REX.W prefix promotes operation to 64 bits (IRETQ).
See the summary chart at the beginning of this section for encoding data and limits.
Refer to Chapter 6, "Procedure Calls, Interrupts, and Exceptions" and Chapter 17, "Control-flow Enforcement Tech-®nology (CET)" in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for CET details.® Instruction ordering.
IRET is a serializing instruction.
See Section 9.3 of the Intel64 and IA-32 Architectures Soft-ware Developer's Manual, Volume 3A.®See "Changes to Instruction Behavior in VMX Non-Root Operation" in Chapter 26 of the Intel 64 and IA-32 Archi-tectures Software Developer's Manual, Volume 3C, for more information about the behavior of this instruction in VMX non-root operation.

## Flags affected

- All the flags and fields in the EFLAGS register are potentially modified, depending on the mode of operation of the processor. If performing a return from a nested task to a previous task, the EFLAGS register will be modified according to the EFLAGS image stored in the previous task's TSS.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - If the return instruction pointer is not within the return code segment limit.
  > IF IOPL not equal to 3.
  - #PF(fault-code) - If a page fault occurs.
  - #SS(0) - If the top bytes of stack are not within stack limits.
- Compatibility Mode Exceptions
  - #GP(0) - If EFLAGS.NT[bit 14] = 1.
  > Other exceptions same as in Protected Mode.
- Real-Address Mode Exceptions
  - #GP - If the return instruction pointer is not within the return code segment limit.
  - #SS - If the top bytes of stack are not within stack limits.
- Protected Mode Exceptions
  - #GP(0) - If the return code or stack segment selector is NULL.
  > If the return instruction pointer is not within the return code segment limit.
  - #GP(selector) - If a segment selector index is outside its descriptor table limits.
  > If the return code segment selector RPL is less than the CPL.
  > If the DPL of a conforming-code segment is gr
  > eater than the return code segment selector 
  > RPL.
  > If the DPL for a nonconforming-code segment is
  >  not equal to the RPL of the code segment 
  > selector.
  > If the stack segment descriptor DPL is not eq
  > ual to the RPL of the return code segment 
  > selector.
  > If the stack segment is not a writable data segment.
  > If the stack segment selector RPL is not equal to
  >  the RPL of the return code segment selector.
  > If the segment descriptor for a code segment does not indicate it is a code segment.
  > If the segment selector for a TSS has its local/global bit set for local.
  > If a TSS segment descriptor specifies that the TSS is not busy.
  > If a TSS segment descriptor specifies that the TSS is not available.
  - #SS(0) - If the top bytes of stack are not within stack limits.
  > If the return stack segment is not present.
  - #NP (selector) - If the return code segment is not present.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference occurs
  > when the CPL is 3 and alignment checking is 
  > enabled.
  - #UD - If the LOCK prefix is used.
  > #CP (Far-RET/IRET)If the previous SS
  > P from shadow stack (when returning to CPL <3) or from IA32_PL3_SSP 
  > (returning to CPL 3) is not 4 byte aligned.
  > If returning to 32-bit or compatibility mode and the previous SSP from shadow stack (when 
  > returning to CPL <3) or fr
  > om IA32_PL3_SSP (returning to CPL 3) is beyond 4GB.
  > If return instruction pointer from stack and shadow stack do not match.
- 64-Bit Mode Exceptions
  - #GP(0) - If EFLAGS.NT[bit 14] = 1.
  > If the return code segment selector is NULL.
  > If the stack segment selector is NULL
  >  going back to compatibility mode.
  > If the stack segment selector is NULL going back to CPL3 64-bit mode.
  > If a NULL stack segment selector RPL is not equal to CPL going back to non-CPL3 64-bit mode.
  > If the return instruction pointer is no
  > t within the return code segment limit.
  > If the return instruction pointer is non-canonical.
  - #GP(Selector) - If a segment selector index is outside its descriptor table limits.
  > If a segment descriptor memory address is non-canonical.
  > If the segment descriptor for a code segment 
  > does not indicate it is a code segment.
  > If the proposed new code segment descriptor has both the D-bit and L-bit set.
  > If the DPL for a nonconforming-code segment is
  >  not equal to the RPL of the code segment 
  > selector.
  > If CPL is greater than the RPL 
  > of the code segment selector.
  > If the DPL of a conforming-code segment is gr
  > eater than the return code segment selector 
  > RPL.
  > If the stack segment is not a writable data segment.
  > If the stack segment descriptor DPL is not equal to the RPL of the return code segment 
  > selector.
  > If the stack segment selector RPL is not equal to
  >  the RPL of the return code segment selector.
  - #SS(0) - If an attempt to pop a value off the stack violates the SS limit.
  > If an attempt to pop a value off the stack causes a non-canonical address to be referenced.
  > If the return stack segment is not present.
  - #NP (selector) - If the return code segment is not present.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference occurs
  > when the CPL is 3 and alignment checking is 
  > enabled.
  - #UD - If the LOCK prefix is used.
  > #CP (Far-RET/IRET)If the previous SSP from shadow st
  > ack (when returning to CPL <3) or from IA32_PL3_SSP 
  > (returning to CPL 3) is not 4 byte aligned.
  > If returning to 32-bit or compatibility mode and the previous SSP from shadow stack (when 
  > returning to CPL <3) or from IA32_PL3_SS

## Operation

```C
IF PE = 0THEN GOTO REAL-ADDRESS-MODE;ELSIF (IA32_EFER.LMA = 0)THENIF (EFLAGS.VM = 1)THEN GOTO RETURN-FROM-VIRTUAL-8086-MODE;ELSE GOTO PROTECTED-MODE;FI;ELSE GOTO IA-32e-MODE;FI;REAL-ADDRESS-MODE;IF OperandSize = 32THENEIP := Pop();CS := Pop(); (* 32-bit pop, high-order 16 bits discarded *)tempEFLAGS := Pop();EFLAGS := (tempEFLAGS AND 257FD5H) OR (EFLAGS AND 1A0000H);= 16 *)ELSE (* OperandSize EIP := Pop(); (* 16-bit pop; clear upper 16 bits *)CS := Pop(); (* 16-bit pop *)EFLAGS[15:0] := Pop();FI;END;RETURN-FROM-VIRTUAL-8086-MODE: (* Processor is in virtual-8086 mode when IRET is executed and stays in virtual-8086 mode *)= = ==3 (* Virtual mode: PE 1, VM  1, IOPL  3 *)IF IOPL =THEN IF OperandSize  32THENEIP := Pop();CS := Pop(); (* 32-bit pop, high-order 16 bits discarded *)EFLAGS := Pop();(* VM, IOPL,VIP and VIF EFLAG bits not modified by pop *)IF EIP not within CS limitTHEN #GP(0); FI;= 16 *)ELSE (* OperandSize EIP := Pop(); (* 16-bit pop; clear upper 16 bits *)CS := Pop(); (* 16-bit pop *)IF EIP not within CS limitTHEN #GP(0); FI;FI;ELSE  == #GP(0); (* Trap to virtual-8086 monitor: PE 1, VM 1, IOPL < 3 *)FI;END;PROTECTED-MODE:=IF NT  1= ==THEN GOTO TASK-RETURN; (* PE 1, VM  0, NT  1 *)FI; = 32IF OperandSizeTHENEIP := Pop();CS := Pop(); (* 32-bit pop, high-order 16 bits discarded *)tempEFLAGS := Pop();= 16 *)ELSE (* OperandSize EIP := Pop(); (* 16-bit pop; clear upper bits *)CS := Pop(); (* 16-bit pop *)tempEFLAGS := Pop(); (* 16-bit pop; clear upper bits *)FI;= =1 and CPL  0IF tempEFLAGS(VM) THEN GOTO RETURN-TO-VIRTUAL-8086-MODE; ELSE GOTO PROTECTED-MODE-RETURN;FI;= =  =  1, VM0, NT1 *)TASK-RETURN: (* PE SWITCH-TASKS (without nesting) to TSS specified in link field of current TSS;Mark the task just abandoned as NOT BUSY;IF EIP is not within CS limitTHEN #GP(0); FI;END;RETURN-TO-VIRTUAL-8086-MODE: = =  1, CPL=0, VM1 in flag image *)(* Interrupted procedure was in virtual-8086 mode: PE (* If shadow stack or indirect branch tracking at CPL3 then #GP(0) *)IF CR4.CET AND (IA32_U_CET.ENDBR_EN OR IA32_U_CET.SHSTK_EN)THEN #GP(0); FI;shadowStackEnabled = ShadowStackEnabled(CPL)IF EIP not within CS limitTHEN #GP(0); FI;EFLAGS := tempEFLAGS;ESP := Pop();SS := Pop(); (* Pop 2 words; throw away high-order word *)ES := Pop(); (* Pop 2 words; throw away high-order word *)DS := Pop(); (* Pop 2 words; throw away high-order word *)FS := Pop(); (* Pop 2 words; throw away high-order word *)GS := Pop(); (* Pop 2 words; throw away high-order word *)IF shadowStackEnabled(* check if 8 byte aligned *)IF SSP AND 0x7 != 0CPL := 3;(* Resume execution in Virtual-8086 mode *)tempOldSSP = SSP;(* Now past all faulting points; safe to free the token. The token free is done using the old SSP * and using a supervisor override as old CPL was a supervisor privilege level *)IF shadowStackEnabledexpected_token_value = tempOldSSP | BUSY_BIT (* busy bit - bit position 0 - must be set *)new_token_value = tempOldSSP (* clear the busy bit *)shadow_stack_lock_cmpxchg8b(tempOldSSP, new_token_value, expected_token_value)FI;END;= 1 *)PROTECTED-MODE-RETURN: (* PE IF CS(RPL) > CPLTHEN GOTO RETURN-TO-OUTER-PRIVILEGE-LEVEL;ELSE GOTO RETURN-TO-SAME-PRIVILEGE-LEVEL; FI;END;RETURN-TO-OUTER-PRIVILEGE-LEVEL: =IF OperandSize 32THENtempESP := Pop();tempSS := Pop(); (* 32-bit pop, high-order 16 bits discarded *)= 16 ELSE IF OperandSize THENtempESP := Pop(); (* 16-bit pop; clear upper bits *)tempSS := Pop(); (* 16-bit pop *)= 64 *)ELSE (* OperandSize tempRSP := Pop();tempSS := Pop(); (* 64-bit pop, high-order 48 bits discarded *)FI; 64-Bit ModeIF new mode THENIF EIP is not within CS limitTHEN #GP(0); FI;ELSE (* new mode = 64-bit mode *)IF RIP is non-canonicalTHEN #GP(0); FI;FI;EFLAGS (CF, PF, AF, ZF, SF, TF, DF, OF, NT) := tempEFLAGS;= 32 or OperandSize = 64IF OperandSize THEN EFLAGS(RF, AC, ID) := tempEFLAGS; FI;IF CPL  IOPL THEN EFLAGS(IF) := tempEFLAGS; FI;= IF CPL 0THENEFLAGS(IOPL) := tempEFLAGS;= 32 or OperandSize = 64IF OperandSize THEN EFLAGS(VIF, VIP) := tempEFLAGS; FI;FI;IF ShadowStackEnabled(CPL)THEN #CP(FAR-RET/IRET); FI;IF CS(RPL) != 3THENtempSsCS = shadow_stack_load 8 bytes from SSP+16;tempSsLIP = shadow_stack_load 8 bytes from SSP+8;tempSSP = shadow_stack_load 8 bytes from SSP;SSP = SSP + 24;(* Do 64 bit compare to detect bits beyond 15 being set *)tempCS = CS; (* zero padded to 64 bit *)IF tempCS != tempSsCSTHEN #CP(FAR-RET/IRET); FI;(* Do 64 bit compare; pad CSBASE+RIP with 0 for 32 bit LIP *)IF CSBASE + RIP != tempSsEIPTHEN #CP(FAR-RET/IRET); FI;(* check if 4 byte aligned *)IF tempSSP AND 0x3 != 0THEN #CP(FAR-RET/IRET); FI;FI;FI;tempOldCPL = CPL;CPL := CS(RPL);IF OperandSize = 64THENRSP := tempRSP;SS := tempSS;ELSEESP := tempESP;SS := tempSS;FI;IF new mode != 64-Bit ModeTHENIF EIP is not within CS limitTHEN #GP(0); FI;ELSE (* new mode = 64-bit mode *)IF RIP is non-canonicalTHEN #GP(0); FI;FI;tempOldSSP = SSP;IF ShadowStackEnabled(CPL)IF CPL = 3THEN tempSSP := IA32_PL3_SSP; FI;IF ((IA32_EFER.LMA AND CS.L) = 0 AND tempSSP[63:32] != 0) OR((IA32_EFER.LMA AND CS.L) = 1 AND tempSSP is not canonical relative to the current paging mode)THEN #GP(0); FI;SSP := tempSSPFI;(* Now past all faulting points; safe to free the token. The token free is done using the old SSP * and using a supervisor override as old CPL was a supervisor privilege level *)IF ShadowStackEnabled(tempOldCPL)expected_token_value = tempOldSSP | BUSY_BIT (* busy bit - bit position 0 - must be set *)new_token_value = tempOldSSP (* clear the busy bit *)FOR each SegReg in (ES, FS, GS, and DS)DOtempDesc := descriptor cache for SegReg (* hidden part of segment register *)IF (SegmentSelector == NULL) OR (tempDesc(DPL) < CPL AND tempDesc(Type) is (data or non-conforming code)))THEN (* Segment register invalid *)SegmentSelector := 0; (*Segment selector becomes null*)FI;OD;END;= = 1, RPL CPL *)RETURN-TO-SAME-PRIVILEGE-LEVEL: (* PE  IF new mode 64-Bit ModeTHENIF EIP is not within CS limitTHEN #GP(0); FI;ELSE (* new mode = 64-bit mode *)IF RIP is non-canonicalTHEN #GP(0); FI;FI;EFLAGS (CF, PF, AF, ZF, SF, TF, DF, OF, NT) := tempEFLAGS;IF OperandSize = 32 or OperandSize = 64THEN EFLAGS(RF, AC, ID) := tempEFLAGS; FI;IF CPL  IOPLTHEN EFLAGS(IF) := tempEFLAGS; FI;IF CPL = 0   THEN  EFLAGS(IOPL) := tempEFLAGS; IF OperandSize = 32 or OperandSize = 64THEN EFLAGS(VIF, VIP) := tempEFLAGS; FI; FI;IF ShadowStackEnabled(CPL)IF SSP AND 0x7 != 0 (* check if aligned to 8 bytes *)THEN #CP(FAR-RET/IRET); FI;tempSsCS = shadow_stack_load 8 bytes from SSP+16;tempSsLIP = shadow_stack_load 8 bytes from SSP+8;tempSSP = shadow_stack_load 8 bytes from SSP;SSP = SSP + 24;tempCS = CS; (* zero padded to 64 bit *)IF tempCS != tempSsCS (* 64 bit compare; CS zero padded to 64 bits *)THEN #CP(FAR-RET/IRET); FI;IF CSBASE + RIP != tempSsLIP (* 64 bit compare; CSBASE+RIP zero padded to 64 bit for 32 bit LIP *)THEN #CP(FAR-RET/IRET); FI;IF tempSSP AND 0x3 != 0 (* check if aligned to 4 bytes *)THEN #CP(FAR-RET/IRET); FI;IF ((IA32_EFER.LMA AND CS.L) = 0 AND tempSSP[63:32] != 0) OR((IA32_EFER.LMA AND CS.L) = 1 AND tempSSP is not canonical relative to the current paging mode)THEN #GP(0); FI;FI;IF ShadowStackEnabled(CPL)IF IA32_EFER.LMA = 1(* In IA-32e-mode the IRET may be switching stacks if the interrupt/exception was delivered through an IDT with a non-zero IST *)(* In IA-32e mode for same CPL IRET there is always a stack switch. The below check verifies if the  tempSSP was not to same stack then there was a stack switch so do attempt to free the token *)IF tempSSP != SSPTHEN expected_token_value = SSP | BUSY_BIT(* busy bit - bit position 0 - must be set *)new_token_value = SSP(* clear the busy bit *)shadow_stack_lock_cmpxchg8b(SSP, new_token_value, expected_token_value)FI;FI;SSP := tempSSPFI;END;IA-32e-MODE:IF NT = 1THEN #GP(0); = 32ELSE IF OperandSizeTHENEIP := Pop();CS := Pop();tempEFLAGS := Pop();= 16 ELSE IF OperandSize THENEIP := Pop(); (* 16-bit pop; clear upper bits *)CS := Pop(); (* 16-bit pop *)tempEFLAGS := Pop(); (* 16-bit pop; clear upper bits *)FI;= 64 *)ELSE (* OperandSize THENRIP := Pop();CS := Pop(); (* 64-bit pop, high-order 48 bits discarded *)tempRFLAGS := Pop();FI;IF CS.RPL > CPLTHEN GOTO RETURN-TO-OUTER-PRIVILEGE-LEVEL;ELSEIF instruction began in 64-Bit ModeTHEN = 32IF OperandSizeTHENESP := Pop();SS := Pop(); (* 32-bit pop, high-order 16 bits discarded *)= 16 ELSE IF OperandSize THENESP := Pop(); (* 16-bit pop; clear upper bits *)SS := Pop(); (* 16-bit pop *)= 64 *)ELSE (* OperandSize RSP := Pop();SS := Pop(); (* 64-bit pop, high-order 48 bits discarded *)FI;FI;
```
