# RSTORSSP

Restore Saved Shadow Stack Pointer

Restores SSP from the shadow-stack-restore token pointed to by m64.
If the SSP restore was successful then the instruction replaces the shadow-stack-restore token with a previous-ssp token.
The instruction sets the CF flag to indicate whether the SSP address recorded in the shadow-stack-restore token that was processed was 4 byte aligned, i.e., whether an alignment hole was created when the restore-shadow-stack token was pushed on this shadow stack.
Following RSTORSSP if a restore-shadow-stack token needs to be saved on the previous shadow stack, use the SAVEPREVSSP instruction.
If pushing a restore-shadow-stack token on the previous shadow stack is not required, the previous-ssp token can be popped using the INCSSPQ instruction.
If the CF flag was set to indicate presence of an alignment hole, an addi-tional INCSSPD instruction is needed to advance the SSP past the alignment hole.

## Flags affected

- CF is set to indicate if the shadow stack pointer in the restore token was 4 byte aligned, else it is cleared. ZF, PF, AF, OF, and SF are cleared.C/C++ Compiler Intrinsic EquivalentRSTORSSP void _rstorssp(void *);

## Exceptions

- Real-Address Mode Exceptions
  - #UD - The RSTORSSP instruction is no
  > t recognized in real-address mode. 
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > IF CPL = 3 and IA32_U_CET.SH_STK_EN = 0.
  > IF CPL < 3 and IA32_S_CET.SH_STK_EN = 0.
  - #GP(0) - If linear address of memory operand not 8 byte aligned.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If destination is located in a non-writeable segment.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #CP(rstorssp) - If L bit in token does not match (IA32_EFER.LMA & CS.L).
  > If address in token does not match linear address of memory operand.
  > If in 32-bit or compatibility mode and the address in token is not below 4G.
  - #PF(fault-code) - If a page fault occurs.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > If CPL = 3 and IA32_U_CET.SH_STK_EN = 0.
  > If CPL < 3 and IA32_S_CET.SH_STK_EN = 0.
  - #GP(0) - If linear address of memo
  > ry operand not 8 byte aligned.
  > If a memory address is in a non-canonical form.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #CP(rstorssp) - If L bit in token does not match (IA32_EFER.LMA & CS.L).
- Virtual-8086 Mode Exceptions
  - #UD - The RSTORSSP instruction is not recognized in virtual-8086 mode.

## Operation

```C
IF CPL = 3IF (CR4.CET & IA32_U_CET.SH_STK_EN) = 0THEN #UD; FI;ELSEIF (CR4.CET & IA32_S_CET.SH_STK_EN) = 0THEN #UD; FI;FI;SSP_LA = Linear_Address(mem operand)IF SSP_LA not aligned to 8 bytesTHEN #GP(0); FI;previous_ssp_token = SSP | (IA32_EFER.LMA AND CS.L) | 0x02Start Atomic Executionrestore_ssp_token = Locked shadow_stack_load 8 bytes from SSP_LAfault = 0IF ((restore_ssp_token & 0x03) != (IA32_EFER.LMA & CS.L))THEN fault = 1; FI;(* If L flag in token does not match IA32_EFER.LMA & CS.L or bit 1 is not 0 *)IF ((IA32_EFER.LMA AND CS.L) = 0 AND restore_ssp_token[63:32] != 0)THEN fault = 1; FI;(* If compatibility/legacy mode and SSP to be restored not below 4G *)TMP = restore_ssp_token & ~0x01TMP = (TMP - 8)THEN fault = 1; FI;(* If address in token does not match the requested top of stack *)TMP = (fault == 0) ? previous_ssp_token : restore_ssp_tokenshadow_stack_store 8 bytes of TMP to SSP_LA and release lockEnd Atomic ExecutionIF fault == 1    THEN #CP(RSTORSSP); FI;SSP = SSP_LA// Set the CF if the SSP in the restore token was 4 byte aligned, i.e., there is an alignment holeRFLAGS.CF = (restore_ssp_token & 0x04) ? 1 : 0;RFLAGS.ZF,PF,AF,OF,SF := 0;
```
