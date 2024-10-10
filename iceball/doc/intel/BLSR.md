# BLSR

Reset Lowest Set Bit

Copies all bits from the source operand to the destination operand and resets (=0) the bit position in the destina-tion operand that corresponds to the lowest set bit of the source operand.
If the source operand is zero BLSR sets CF.This instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- ZF and SF flags are updated based on the result. CF is set if the source is zero. OF flag is cleared. AF and PF flags are undefined.Intel C/C++ Compiler Intrinsic EquivalentBLSR unsigned __int32 _blsr_u32(unsigned __int32 src);BLSR unsigned __int64 _blsr_u64(unsigned __int64 src);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
temp := (SRC-1) bitwiseAND ( SRC );SF := temp[OperandSize -1];ZF := (temp = 0);IF SRC = 0CF := 1;ELSECF := 0;FIDEST := temp;
```
