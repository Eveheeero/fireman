# EMMS

Empty MMX Technology State

Sets the values of all the tags in the x87 FPU tag word to empty (all 1s).
This operation marks the x87 FPU data registers (which are aliased to the MMX technology registers) as available for use by x87 FPU floating-point instruc-®tions.
(See Figure 8-7 in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for the format of the x87 FPU tag word.) All other MMX instructions (other than the EMMS instruction) set all the tags in x87 FPU tag word to valid (all 0s).The EMMS instruction must be used to clear the MMX technology state at the end of all MMX technology procedures or subroutines and before calling other procedures or subroutines that may execute x87 floating-point instructions.
If a floating-point instruction loads one of the registers in the x87 FPU data register stack before the x87 FPU tag word has been reset by the EMMS instruction, an x87 floating-point register stack overflow can occur that will result in an x87 floating-point exception or incorrect result.EMMS operation is the same in non-64-bit modes and 64-bit mode.

## Flags affected

- None

## Exceptions

- Protected Mode Exceptions
  - #UD - If CR0.EM[bit 2] = 1.
  - #NM - If CR0.TS[bit 3] = 1.
  - #MF - If there is a pending FPU exception.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
x87FPUTagWord := FFFFH;Intel C/C++ Compiler Intrinsic Equivalentvoid _mm_empty()
```
