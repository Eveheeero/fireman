# PEXT

Parallel Bits Extract

PEXT uses a mask in the second source operand (the third operand) to transfer either contiguous or non-contig-uous bits in the first source operand (the second operand) to contiguous low order bit positions in the destination (the first operand).
For each bit set in the MASK, PEXT extracts the corresponding bits from the first source operand and writes them into contiguous lower bits of destination operand.
The remaining upper bits of destination are zeroed.SRC1SSSSSSSSSSSSS313029282776543210SRC20000101010100(mask)S000000000SSDESTS27528bit 0bit 31Figure 4-9.
 PEXT ExampleThis instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An 

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentPEXT unsigned __int32 _pext_u32(unsigned __int32 src, unsigned __int32 mask);PEXT unsigned __int64 _pext_u64(unsigned __int64 src, unsigned __int32 mask);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
TEMP := SRC1;MASK := SRC2;DEST := 0 ;m := 0, k := 0;DOWHILE m < OperandSizeIF MASK[ m] = 1 THENDEST[ k] := TEMP[ m];    k := k+ 1;FIm := m+ 1;OD
```
