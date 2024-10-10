# PUSHF/PUSHFD/PUSHFQ

Push EFLAGS Register Onto the Stack

Decrements the stack pointer by 4 (if the current operand-size attribute is 32) and pushes the entire contents of the EFLAGS register onto the stack, or decrements the stack pointer by 2 (if the operand-size attribute is 16) and pushes the lower 16 bits of the EFLAGS register (that is, the FLAGS register) onto the stack.
These instructions reverse the operation of the POPF/POPFD instructions.
When copying the entire EFLAGS register to the stack, the VM and RF flags (bits 16 and 17) are not copied; instead, ® 64 and the values for these flags are cleared in the EFLAGS image stored on the stack.
See Chapter 3 of the IntelIA-32 Architectures Software Developer's Manual, Volume 1, for more information about the EFLAGS register.
The PUSHF (push flags) and PUSHFD (push flags double) mnemonics reference the same opcode.
The PUSHF instruction is intended for use when the operand-size attribute is 16 and the PUSHFD instruction for when the operand-size attribute is 32.
Some assemblers may force the operand size to 16 when PUSHF is used and to 32 when PUSHFD is used.
Others may treat these mnemonics as synonyms (PUSHF/PUSHFD) and use the current setting of the operand-size attribute to determine the size of values to be pushed from the stack, regardless of the mnemonic used.In 64-bit mode, the instruction's default operation is to decrement the stack pointer (RSP) by 8 and pushes RFLAGS on the stack.
16-bit operation is supported using the operand size override prefix 66H.
32-bit operand size cannot be encoded in this mode.
When copying RFLAGS to the stack, the VM and RF flags (bits 16 and 17) are not copied; instead, values for these flags are cleared in the RFLAGS image stored on the stack.When operating in virtual-8086 mode (EFLAGS.VM = 1) without the virtual-8086 mode extensions (CR4.VME = 0), the PUSHF/PUSHFD instructions can be used only if IOPL = 3; otherwise, a general-protection exception (#GP) occurs.
If the virtual-8086 mode extensions are enabled (CR4.VME = 1), PUSHF (but not PUSHFD) can be executed in virtual-8086 mode with IOPL < 3.(The protected-mode virtual-interrupt feature - enabled by setting CR4.PVI - affects the CLI and STI instructions in the same manner as the virtual-8086 mode extensions.
PUSHF, however, is not affected by CR4.PVI.)In the real-address mode, if the ESP or SP register is 1 when PUSHF/PUSHFD instruction executes: an #SS excep-tion is generated but not delivered (the stack error reported prevents #SS delivery).
Next, the processor generates ® 64 and a #DF exception and enters a shutdown state as described in the #DF discussion in Chapter 6 of the IntelIA-32 Architectures Software Developer's Manual, Volume 3A.

## Flags affected

- None.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - If the I/O privilege level is less than 3.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is
  > made while alignment checking is enabled.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #SS(0) - If the new value of the ESP register
  >  is outside the stack segment boundary. 
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is made
  > while CPL = 3 and alignment checking is enabled.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #SS(0) - If the stack address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is made
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.

## Operation

```C
 =  = = == IF (PE0) or (PE 1 and ((VM0) or (VM  1 and IOPL 3)))(* Real-Address Mode, Protected mode, or Virtual-8086 mode with IOPL equal to 3 *)THEN IF OperandSize =32THEN push (EFLAGS AND 00FCFFFFH);(* VM and RF bits are cleared in image stored on the stack *)ELSE FI;ELSE IF 64-bit MODE (* In 64-bit Mode *)IF OperandSize = 64THEN push (RFLAGS AND 00000000_00FCFFFFH);(* VM and RF bits are cleared in image stored on the stack; *)ELSE push (EFLAGS); (* Lower 16 bits only *)FI;ELSE (* In Virtual-8086 Mode with IOPL less than 3 *)IF (CR4.VME = 0) OR (OperandSize = 32)THEN #GP(0); (* Trap to virtual-8086 monitor *)ELSEtempFLAGS = EFLAGS[15:0];tempFLAGS[9] = tempFLAGS[19];(* VIF replaces IF *)tempFlags[13:12] = 3; (* IOPL is set to 3 in image stored on the stack *)push (tempFLAGS);FI;FI;
```
