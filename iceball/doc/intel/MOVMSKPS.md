# MOVMSKPS

Extract Packed Single Precision Floating-Point Sign Mask

Extracts the sign bits from the packed single precision floating-point values in the source operand (second operand), formats them into a 4- or 8-bit mask, and stores the mask in the destination operand (first operand).
The source operand is an XMM or YMM register, and the destination operand is a general-purpose register.
The mask is stored in the 4 or 8 low-order bits of the destination operand.
The upper bits of the destination operand beyond the mask are filled with zeros.In 64-bit mode, the instruction can access additional registers (XMM8-XMM15, R8-R15) when used with a REX.R prefix.
The default operand size is 64-bit in 64-bit mode.128-bit versions: The source operand is a YMM register.
The destination operand is a general purpose register.
VEX.256 encoded version: The source operand is a YMM register.
The destination operand is a general purpose register.
Note: In VEX-encoded versions, VEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- Other Exceptions
  > See Table2-24, "Type 7 Class Exception Conditions," additionally:
  - #UD If - VEX.vvvv
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
DEST[0] := SRC[31]; DEST[1] := SRC[63]; DEST[2] := SRC[95]; DEST[3] := SRC[127]; IF DEST = r32THEN DEST[31:4] := ZeroExtend;ELSE DEST[63:4] := ZeroExtend;FI;(V)MOVMSKPS (128-bit version)DEST[0] := SRC[31]DEST[1] := SRC[63]DEST[2] := SRC[95]DEST[3] := SRC[127]IF DEST = r32THEN DEST[31:4] := 0;ELSE DEST[63:4] := 0;FIVMOVMSKPS (VEX.256 encoded version)DEST[0] := SRC[31]DEST[1] := SRC[63]DEST[2] := SRC[95]DEST[3] := SRC[127]DEST[4] := SRC[159]DEST[5] := SRC[191]DEST[6] := SRC[223]DEST[7] := SRC[255]IF DEST = r32THEN DEST[31:8] := 0;ELSE DEST[63:8] := 0;FIIntel C/C++ Compiler Intrinsic Equivalentint _mm_movemask_ps(__m128 a)int _mm256_movemask_ps(__m256 a)
```
