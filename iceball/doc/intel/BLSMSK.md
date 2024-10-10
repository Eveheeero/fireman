# BLSMSK

Get Mask Up to Lowest Set Bit

Sets all the lower bits of the destination operand to "1" up to and including lowest set bit (=1) in the source operand.
If source operand is zero, BLSMSK sets all bits of the destination operand to 1 and also sets CF to 1.This instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- SF is updated based on the result. CF is set if the source if zero. ZF and OF flags are cleared. AF and PF flag are undefined.Intel C/C++ Compiler Intrinsic EquivalentBLSMSK unsigned __int32 _blsmsk_u32(unsigned __int32 src);BLSMSK unsigned __int64 _blsmsk_u64(unsigned __int64 src);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
temp := (SRC-1) XOR (SRC) ;SF := temp[OperandSize -1];ZF := 0;IF SRC = 0CF := 1;ELSECF := 0;FIDEST := temp;
```
