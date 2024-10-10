# BEXTR

Bit Field Extract

Extracts contiguous bits from the first source operand (the second operand) using an index value and length value specified in the second source operand (the third operand).
Bit 7:0 of the second source operand specifies the starting bit position of bit extraction.
A START value exceeding the operand size will not extract any bits from the second source operand.
Bit 15:8 of the second source operand specifies the maximum number of bits (LENGTH) beginning at the START position to extract.
Only bit positions up to (OperandSize -1) of the first source operand are extracted.
The extracted bits are written to the destination register, starting from the least significant bit.
All higher order bits in the destination operand (starting at bit position LENGTH) are zeroed.
The destination register is cleared if no bits are extracted.This instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- ZF is updated based on the result. AF, SF, and PF are undefined. All other flags are cleared. Intel C/C++ Compiler Intrinsic EquivalentBEXTR unsigned __int32 _bextr_u32(unsigned __int32 src, unsigned __int32 start. unsigned __int32 len);BEXTR unsigned __int64 _bextr_u64(unsigned __int64 src, unsigned __int32 start. unsigned __int32 len);

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
START := SRC2[7:0];LEN := SRC2[15:8];TEMP := ZERO_EXTEND_TO_512 (SRC1 );DEST := ZERO_EXTEND(TEMP[START+LEN -1: START]);ZF := (DEST = 0);
```
