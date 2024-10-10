# PSLLDQ

Shift Double Quadword Left Logical

Shifts the destination operand (first operand) to the left by the number of bytes specified in the count operand (second operand).
The empty low-order bytes are cleared (set to all 0s).
If the value specified by the count operand is greater than 15, the destination operand is set to all 0s.
The count operand is an 8-bit immediate.128-bit Legacy SSE version: The source and destination operands are the same.
Bits (MAXVL-1:128) of the corre-sponding YMM destination register remain unchanged.VEX.128 encoded version: The source and destination operands are XMM registers.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded version: The source operand is YMM register.
The destination operand is an YMM register.
Bits (MAXVL-1:256) of the corresponding ZMM register are zeroed.
The count operand applies to both the low and high 128-bit lanes.EVEX encoded versions: The source operand is a ZMM/YMM/XMM register or a 512/256/128-bit memory location.
The destination operand is a ZMM/YMM/XMM register.
The count operand applies to each 128-bit lanes.

## Flags affected

- None.

## Exceptions

- Numeric Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-24, "Type 7 Class Exception Conditions."

## Operation

```C
VPSLLDQ (EVEX.U1.512 Encoded Version)TEMP := COUNTIF (TEMP > 15) THEN TEMP := 16; FIDEST[127:0] := SRC[127:0] << (TEMP * 8)DEST[255:128] := SRC[255:128] << (TEMP * 8)DEST[383:256] := SRC[383:256] << (TEMP * 8)VPSLLDQ (VEX.256 and EVEX.256 Encoded Version)TEMP := COUNTIF (TEMP > 15) THEN TEMP := 16; FIDEST[127:0] := SRC[127:0] << (TEMP * 8)DEST[255:128] := SRC[255:128] << (TEMP * 8)DEST[MAXVL-1:256] := 0VPSLLDQ (VEX.128 and EVEX.128 Encoded Version)TEMP := COUNTIF (TEMP > 15) THEN TEMP := 16; FIDEST := SRC << (TEMP * 8)DEST[MAXVL-1:128] := 0PSLLDQ(128-bit Legacy SSE Version)TEMP := COUNTIF (TEMP > 15) THEN TEMP := 16; FIDEST := DEST << (TEMP * 8)DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic Equivalent(V)PSLLDQ __m128i _mm_slli_si128 ( __m128i a, int imm)VPSLLDQ __m256i _mm256_slli_si256 ( __m256i a, const int imm)VPSLLDQ __m512i _mm512_bslli_epi128 ( __m512i a, const int imm)
```
