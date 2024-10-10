# LEAVE

High Level Procedure Exit

Releases the stack frame set up by an earlier ENTER instruction.
The LEAVE instruction copies the frame pointer (in the EBP register) into the stack pointer register (ESP), which releases the stack space allocated to the stack frame.
The old frame pointer (the frame pointer for the calling procedure that was saved by the ENTER instruction) is then popped from the stack into the EBP register, restoring the calling procedure's stack frame.
A RET instruction is commonly executed following a LEAVE instruction to return program control to the calling procedure.® 64 and IA-32 Architectures Soft-See "Procedure Calls for Block-Structured Languages" in Chapter 7 of the Intelware Developer's Manual, Volume 1, for detailed information on the use of the ENTER and LEAVE instructions.In 64-bit mode, the instruction's default operation size is 64 bits; 32-bit operation cannot be encoded.
See the summary chart at the beginning of this section for encoding data and limits.

## Exceptions

- Real-Address Mode Exceptions
  - #GP - If the EBP register points to a location outs
  > ide of the effective address space from 0 to FFFFH.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If the stack address is in a non-canonical form.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Virtual-8086 Mode Exceptions
  - #GP(0) - If the EBP register points to a location ou
  > tside of the effective address space from 0 to FFFFH.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #SS(0) - If the EBP register points to a location that is not within the limits of the current stack
  > segment.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
=IF StackAddressSize  32THENESP := EBP;=ELSE IF StackAddressSize  64THEN RSP := RBP; FI;=ELSE IF StackAddressSize  16THEN SP := BP; FI;FI;=IF OperandSize  32THEN EBP := Pop();= 64ELSE IF OperandSize THEN RBP := Pop(); FI;=ELSE IF OperandSize  16THEN BP := Pop(); FI;FI;
```
