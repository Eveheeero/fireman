# FINIT/FNINIT

Initialize Floating-Point Unit

Sets the FPU control, status, tag, instruction pointer, and data pointer registers to their default states.
The FPU control word is set to 037FH (round to nearest, all exceptions masked, 64-bit precision).
The status word is cleared (no exception flags set, TOP is set to 0).
The data registers in the register stack are left unchanged, but they are all tagged as empty (11B).
Both the instruction and data pointers are cleared.The FINIT instruction checks for and handles any pending unmasked floating-point exceptions before performing the initialization; the FNINIT instruction does not.The assembler issues two instructions for the FINIT instruction (an FWAIT instruction followed by an FNINIT instruction), and the processor executes each of these instructions in separately.
If an exception is generated for either of these instructions, the save EIP points to the instruction that caused the exception.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture CompatibilityWhen operating a Pentium or Intel486 processor in MS-DOS compatibility mode, it is possible (under unusual circumstances) for an FNINIT instruction to be interrupted prior to being executed to handle a pending FPU excep-® tion.
See the section titled "No-Wait FPU Instructions Can Get FPU Interrupt in Window" in Appendix D of the Intel64 and IA-32 Architectures Software Developer's Manual, Volume 1, for a description of these circumstances.
An TM X1000 FNINIT instruction cannot be interrupted in this way on later Intel processors, except for the Intel Quarkprocessor.In the Intel387 math coprocessor, the FINIT/FNINIT instruction does not clear the instruction and data pointers.This instruction affects only the x87 FPU.
It does not affect the XMM and MXCSR registers.

## Exceptions

- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
FPUControlWord := 037FH;FPUStatusWord := 0;FPUTagWord := FFFFH;FPUDataPointer := 0;FPUInstructionPointer := 0;FPULastInstructionOpcode := 0;FPU Flags AffectedC0, C1, C2, C3 set to 0.
```
