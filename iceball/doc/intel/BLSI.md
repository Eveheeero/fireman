# BLSI

Extract Lowest Set Isolated Bit

Extracts the lowest set bit from the source operand and set the corresponding bit in the destination register.
All other bits in the destination operand are zeroed.
If no bits are set in the source operand, BLSI sets all the bits in the destination to 0 and sets ZF and CF.This instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- ZF and SF are updated based on the result. CF is set if the source is not zero. OF flags are cleared. AF and PF flags are undefined.Intel C/C++ Compiler Intrinsic EquivalentBLSI unsigned __int32 _blsi_u32(unsigned __int32 src);BLSI unsigned __int64 _blsi_u64(unsigned __int64 src);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
temp := (-SRC) bitwiseAND (SRC); SF := temp[OperandSize -1];ZF := (temp = 0);IF SRC = 0CF := 0;ELSECF := 1;FIDEST := temp;
```
