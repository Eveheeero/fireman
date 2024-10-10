# INCSSPD/INCSSPQ

Increment Shadow Stack Pointer

This instruction can be used to increment the current shadow stack pointer by the operand size of the instruction times the unsigned 8-bit value specified by bits 7:0 in the source operand.
The instruction performs a pop and discard of the first and last element on the shadow stack in the range specified by the unsigned 8-bit value in bits 7:0 of the source operand.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic Equivalent

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #UD - The INCSSP instruction is not recognized in real-address mode.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  > IF CPL = 3 and IA32_U_CET.SH_STK_EN = 0.
  > IF CPL < 3 and IA32_S_CET.SH_STK_EN = 0.
  - #PF(fault-code) - If a page fault occurs.
- Virtual-8086 Mode Exceptions
  - #UD - The INCSSP instruction is not
  > recognized in virtual-8086 mode.

## Operation

```C
IF CPL = 3IF (CR4.CET & IA32_U_CET.SH_STK_EN) = 0THEN #UD; FI;ELSEIF (CR4.CET & IA32_S_CET.SH_STK_EN) = 0THEN #UD; FI;FI;IF (operand size is 64-bit)THENRange := R64[7:0];shadow_stack_load 8 bytes from SSP;IF Range > 0              THEN shadow_stack_load 8 bytes from SSP + 8 * (Range - 1);FI;SSP := SSP + Range * 8;ELSERange := R32[7:0];shadow_stack_load 4 bytes from SSP;IF Range > 0              THEN shadow_stack_load 4 bytes from SSP + 4 * (Range - 1);FI;SSP := SSP + Range * 4;FI;
```
