# FBSTP

Store BCD Integer and Pop

Converts the value in the ST(0) register to an 18-digit packed BCD integer, stores the result in the destination operand, and pops the register stack.
If the source value is a non-integral value, it is rounded to an integer value, according to rounding mode specified by the RC field of the FPU control word.
To pop the register stack, the processor marks the ST(0) register as empty and increments the stack pointer (TOP) by 1.The destination operand specifies the address where the first byte destination value is to be stored.
The BCD value (including its sign bit) requires 10 bytes of space in memory.
The following table shows the results obtained when storing various classes of numbers in packed BCD format.Table 3-19.
 FBSTP ResultsST(0)DEST  *or Value Too Large for DEST Format-   F -1-D-**0-1 < F < - 0- 0+ 0+ 0    **<F<+1+ 0  F+1+ D *or Value Too Large for DEST Format+ *NaNNOTES:FMeans finite floating-point value.DMeans packed-BCD number.*Indicates floating-point invalid-operation (#IA) exception.**±0 or ±1, depending on the rounding mode.If the converted value is too large for the destination format, or if the source operand is an , SNaN, QNAN, or is in an unsupported format, an invalid-arithmetic-operand condition is signaled.
If the invalid-operation exception is not masked, an invalid-arithmetic-operand exception (#IA) is generated and no value is stored in the destination operand.
If the invalid-operation exception is masked, the packed BCD indefinite value is stored in memory.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Converted value that exceeds 18 BCD digits in length.
  > Source operand is an SNaN, QNaN, 
  > ±
  > , or in an unsupported format.
  - #P - Value cannot be represented exactly in destination format.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
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
  - #GP(0) - If a segment register is being loaded with
  > a segment selector that points to a non-writable 
  > segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
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
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.

## Operation

```C
DEST := BCD(ST(0));PopRegisterStack;FPU Flags AffectedC1Set to 0 if stack underflow occurred.
```
