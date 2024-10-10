# BZHI

Zero High Bits Starting with Specified Bit Position

BZHI copies the bits of the first source operand (the second operand) into the destination operand (the first operand) and clears the higher bits in the destination according to the INDEX value specified by the second source operand (the third operand).
The INDEX is specified by bits 7:0 of the second source operand.
The INDEX value is saturated at the value of OperandSize -1.
CF is set, if the number contained in the 8 low bits of the third operand is greater than OperandSize -1.This instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- ZF and SF flags are updated based on the result. CF flag is set as specified in the Operation section. OF flag is cleared. AF and PF flags are undefined.Intel C/C++ Compiler Intrinsic EquivalentBZHI unsigned __int32 _bzhi_u32(unsigned __int32 src, unsigned __int32 index);BZHI unsigned __int64 _bzhi_u64(unsigned __int64 src, unsigned __int32 index);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
N := SRC2[7:0]DEST := SRC1IF (N < OperandSize)DEST[OperandSize-1:N] := 0FIIF (N > OperandSize - 1)CF := 1ELSECF := 0FI
```
