# FCLEX/FNCLEX

Clear Exceptions

ModeLeg Mode9B DB E2FCLEXValidValidClear floating-point exception flags after checking for pending unmasked floating-point exceptions.1DB E2FNCLEXValidValidClear floating-point exception flags without checking for pending unmasked floating-point exceptions.NOTES:1.
See IA-32 Architecture Compatibility section below.Clears the floating-point exception flags (PE, UE, OE, ZE, DE, and IE), the exception summary status flag (ES), the stack fault flag (SF), and the busy flag (B) in the FPU status word.
The FCLEX instruction checks for and handles any pending unmasked floating-point exceptions before clearing the exception flags; the FNCLEX instruction does not.The assembler issues two instructions for the FCLEX instruction (an FWAIT instruction followed by an FNCLEX instruction), and the processor executes each of these instructions separately.
If an exception is generated for either of these instructions, the save EIP points to the instruction that caused the exception.IA-32 Architecture CompatibilityWhen operating a Pentium or Intel486 processor in MS-DOS* compatibility mode, it is possible (under unusual circumstances) for an FNCLEX instruction to be interrupted prior to being executed to handle a pending FPU excep-® tion.
See the section titled "No-Wait FPU Instructions Can Get FPU Interrupt in Window" in Appendix D of the Intel64 and IA-32 Architectures Software Developer's Manual, Volume 1, for a description of these circumstances.
An TM X1000 FNCLEX instruction cannot be interrupted in this way on later Intel processors, except for the Intel Quarkprocessor.This instruction affects only the x87 FPU floating-point exception flags.
It does not affect the SIMD floating-point exception flags in the MXCSR register.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
- Floating-Point Exceptions
  > None.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
FPUStatusWord[0:7] := 0;FPUStatusWord[15] := 0;FPU Flags AffectedThe PE, UE, OE, ZE, DE, IE, ES, SF, and B flags in the FPU status word are cleared. The C0, C1, C2, and C3 flags are undefined.
```
