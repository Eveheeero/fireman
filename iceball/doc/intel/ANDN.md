# ANDN

Logical AND NOT

Performs a bitwise logical AND of inverted second operand (the first source operand) with the third operand (the second source operand).
The result is stored in the first operand (destination operand).This instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- SF and ZF are updated based on result. OF and CF flags are cleared. AF and PF flags are undefined.Intel C/C++ Compiler Intrinsic EquivalentAuto-generated from high-level language.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
DEST := (NOT SRC1) bitwiseAND SRC2;SF := DEST[OperandSize -1];ZF := (DEST = 0);
```
