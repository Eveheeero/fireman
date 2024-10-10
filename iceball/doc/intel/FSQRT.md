# FSQRT

Square Root

Computes the square root of the source value in the ST(0) register and stores the result in ST(0).The following table shows the results obtained when taking the square root of various classes of numbers, assuming that neither overflow nor underflow occurs.Table 3-37.
 FSQRT ResultsSRC (ST(0))DEST (ST(0)) *-*- F00- - 00+ + FF+ +   ++NaNNaN NOTES:FMeans finite floating-point value.*Indicates floating-point invalid-arithmetic-operand (#IA) exception.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Source operand is an SNaN value or unsupported format.
  > Source operand is a negative value (except for 
  > -
  > 0).
  - #D - Source operand is a denormal value.
  - #P - Value cannot be represented
  > exactly in destination format.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
ST(0) := SquareRoot(ST(0));FPU Flags AffectedC1Set to 0 if stack underflow occurred.Set if result was rounded up; cleared otherwise.C0, C2, C3 Undefined.
```
