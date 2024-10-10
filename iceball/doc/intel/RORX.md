# RORX

Rotate Right Logical Without Affecting Flags

Rotates the bits of second operand right by the count value specified in imm8 without affecting arithmetic flags.
The RORX instruction does not read or write the arithmetic flags.This instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentAuto-generated from high-level language.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
IF (OperandSize = 32)y := imm8 AND 1FH;DEST := (SRC >> y) | (SRC << (32-y));ELSEIF (OperandSize = 64 ) y := imm8 AND 3FH;DEST := (SRC >> y) | (SRC << (64-y));FI;
```
