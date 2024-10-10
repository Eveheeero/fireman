# FABS

Absolute Value

Clears the sign bit of ST(0) to create the absolute value of the operand.
The following table shows the results obtained when creating the absolute value of various classes of numbers.Table 3-17.
 Results Obtained from FABS ST(0) SRCST(0) DEST +- -F+F  0+- 0 +00+  + F+ F++  NaNNaN NOTES:FMeans finite floating-point value.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
ST(0) := |ST(0)|;FPU Flags AffectedC1Set to 0.C0, C2, C3 Undefined.
```
