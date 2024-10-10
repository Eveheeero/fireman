# FDECSTP

Decrement Stack-Top Pointer

Subtracts one from the TOP field of the FPU status word (decrements the top-of-stack pointer).
If the TOP field contains a 0, it is set to 7.
The effect of this instruction is to rotate the stack by one position.
The contents of the FPU data registers and tag register are not affected.
This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Floating-Point Exceptions
  > None.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
=IF TOP  0THEN TOP := 7;ELSE TOP := TOP - 1;FI;FPU Flags AffectedThe C1 flag is set to 0. The C0, C2, and C3 flags are undefined.
```
