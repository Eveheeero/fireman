# FCHS

Change Sign

Complements the sign bit of ST(0).
This operation changes a positive value into a negative value of equal magni-tude or vice versa.
The following table shows the results obtained when changing the sign of various classes of numbers.Table 3-20.
 FCHS ResultsST(0) SRCST(0) DEST + --F+ F - 0+ 0+ 0- 0+ F- F -+ NaNNaN NOTES:*F means finite floating-point value.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.

## Operation

```C
SignBit(ST(0)) := NOT (SignBit(ST(0)));FPU Flags AffectedC1Set to 0.C0, C2, C3 Undefined.
```
