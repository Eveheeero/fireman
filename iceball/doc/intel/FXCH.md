# FXCH

Exchange Register Contents

Exchanges the contents of registers ST(0) and ST(i).
If no source operand is specified, the contents of ST(0) and ST(1) are exchanged.This instruction provides a simple means of moving values in the FPU register stack to the top of the stack [ST(0)], so that they can be operated on by those floating-point instructions that can only operate on values in ST(0).
For example, the following instruction sequence takes the square root of the third register from the top of the register stack:FXCH ST(3);FSQRT;FXCH ST(3);This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF (Number-of-operands) is 1THENtemp := ST(0);ST(0) := SRC;SRC := temp;ELSEtemp := ST(0);ST(0) := ST(1);ST(1) := temp;FI;FPU Flags AffectedC1Set to 0.C0, C2, C3 Undefined.
```
