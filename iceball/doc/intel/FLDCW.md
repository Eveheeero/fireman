# FLDCW

Load x87 FPU Control Word

Loads the 16-bit source operand into the FPU control word.
The source operand is a memory location.
This instruc-tion is typically used to establish or change the FPU's mode of operation.If one or more exception flags are set in the FPU status word prior to loading a new FPU control word and the new control word unmasks one or more of those exceptions, a floating-point exception will be generated upon execution of the next floating-point instruction (except for the no-wait floating-point instructions, see the section titled "Soft-® 64 and IA-32 Architectures Software Developer's Manual, ware Exception Handling" in Chapter 8 of the IntelVolume 1).
To avoid raising exceptions when changing FPU operating modes, clear any pending exceptions (using the FCLEX or FNCLEX instruction) before loading the new control word.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Floating-Point Exceptions
  > None; however, this operation might unmask a pending exception in the FPU status word. That exception is then 
  > generated upon execution of the next 
  > "waiting" floating-point instruction.
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
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
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
FPUControlWord := SRC;FPU Flags AffectedC0, C1, C2, C3 undefined.
```
