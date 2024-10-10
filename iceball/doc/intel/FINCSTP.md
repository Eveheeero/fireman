# FINCSTP

Increment Stack-Top Pointer

Adds one to the TOP field of the FPU status word (increments the top-of-stack pointer).
If the TOP field contains a 7, it is set to 0.
The effect of this instruction is to rotate the stack by one position.
The contents of the FPU data registers and tag register are not affected.
This operation is not equivalent to popping the stack, because the tag for the previous top-of-stack register is not marked empty.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Floating-Point Exceptions
  > None.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.

## Operation

```C
 =IF TOP 7THEN TOP := 0;ELSE TOP := TOP + 1;FI;FPU Flags AffectedThe C1 flag is set to 0. The C0, C2, and C3 flags are undefined.
```
