# PDEP

Parallel Bits Deposit

PDEP uses a mask in the second source operand (the third operand) to transfer/scatter contiguous low order bits in the first source operand (the second operand) into the destination (the first operand).
PDEP takes the low bits from the first source operand and deposit them in the destination operand at the corresponding bit locations that are set in the second source operand (mask).
All other bits (bits not set in mask) in destination are set to zero.SRC1SSSSSSSSSSSSS313029282776543210SRC20001010100100(mask)DEST000000000SSSS1032bit 0bit 31Figure 4-8.
 PDEP ExampleThis instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentPDEP unsigned __int32 _pdep_u32(unsigned __int32 src, unsigned __int32 mask);PDEP unsigned __int64 _pdep_u64(unsigned __int64 src, unsigned __int32 mask);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
TEMP := SRC1;MASK := SRC2;DEST := 0 ;m := 0, k := 0;DOWHILE m < OperandSizeIF MASK[ m] = 1 THENDEST[ m] := TEMP[ k];    k := k+ 1;FI
```
