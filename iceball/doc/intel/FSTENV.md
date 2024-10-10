# FSTENV/FNSTENV

Store x87 FPU Environment

Saves the current FPU operating environment at the memory location specified with the destination operand, and then masks all floating-point exceptions.
The FPU operating environment consists of the FPU control word, status ® 64 and word, tag word, instruction pointer, data pointer, and last opcode.
Figures 8-9 through 8-12 in the IntelIA-32 Architectures Software Developer's Manual, Volume 1, show the layout in memory of the stored environ-ment, depending on the operating mode of the processor (protected or real) and the current operand-size attribute (16-bit or 32-bit).
In virtual-8086 mode, the real mode layouts are used.The FSTENV instruction checks for and handles any pending unmasked floating-point exceptions before storing the FPU environment; the FNSTENV instruction does not.
The saved image reflects the state of the FPU after all floating-point instructions preceding the FSTENV/FNSTENV instruction in the instruction stream have been executed.These instructions are often used by exception handlers because they provide access to the FPU instruction and data pointers.
The environment is typically saved in the stack.
Masking all exceptions after saving the environment prevents floating-point exceptions from interrupting the exception handler.The assembler issues two instructions for the FSTENV instruction (an FWAIT instruction followed by an FNSTENV instruction), and the processor executes each of these instructions separately.
If an exception is generated for either of these instructions, the save EIP points to the instruction that caused the exception.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture CompatibilityWhen operating a Pentium or Intel486 processor in MS-DOS compatibility mode, it is possible (under unusual circumstances) for an FNSTENV instruction to be interrupted prior to being executed to handle a pending FPU exception.
See the section titled "No-Wait FPU Instructions Can Get FPU Interrupt in Window" in Appendix D of the ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for a description of these circum-Intelstances.
An FNSTENV instruction cannot be interrupted in this way on later Intel processors, except for the Intel TM X1000 processor.Quark

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the destination is located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
- Floating-Point Exceptions
  > None.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 

## Operation

```C
DEST[FPUControlWord] := FPUControlWord;DEST[FPUStatusWord] := FPUStatusWord;DEST[FPUTagWord] := FPUTagWord;DEST[FPUDataPointer] := FPUDataPointer;DEST[FPUInstructionPointer] := FPUInstructionPointer;DEST[FPULastInstructionOpcode] := FPULastInstructionOpcode;
```
