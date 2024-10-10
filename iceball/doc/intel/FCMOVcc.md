# FCMOVcc

Floating-Point Conditional Move

Tests the status flags in the EFLAGS register and moves the source operand (second operand) to the destination operand (first operand) if the given test condition is true.
The condition for each mnemonic os given in the Descrip-® 64 and IA-32 Architectures Software Developer's Manual, Volume tion column above and in Chapter 8 in the Intel1.
The source operand is always in the ST(i) register and the destination operand is always ST(0).The FCMOVcc instructions are useful for optimizing small IF constructions.
They also help eliminate branching over-head for IF operations and the possibility of branch mispredictions by the processor.
A processor may not support the FCMOVcc instructions.
Software can check if the FCMOVcc instructions are supported by checking the processor's feature information with the CPUID instruction (see "COMISS-Compare Scalar Ordered Single Precision Floating-Point Values and Set EFLAGS" in this chapter).
If both the CMOV and FPU feature bits are set, the FCMOVcc instructions are supported.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture CompatibilityThe FCMOVcc instructions were introduced to the IA-32 Architecture in the P6 family processors and are not avail-able in earlier IA-32 processors.

## Exceptions

- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  > Integer Flag
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF condition TRUETHEN ST(0) := ST(i);FI;FPU Flags AffectedC1Set to 0 if stack underflow occurred.C0, C2, C3 Undefined.
```
