# PSUBQ

Subtract Packed Quadword Integers

Subtracts the second operand (source operand) from the first operand (destination operand) and stores the result in the destination operand.
When packed quadword operands are used, a SIMD subtract is performed.
When a quadword result is too large to be represented in 64 bits (overflow), the result is wrapped around and the low 64 bits are written to the destination element (that is, the carry is ignored).Note that the (V)PSUBQ instruction can operate on either unsigned or signed (two's complement notation) inte-gers; however, it does not set bits in the EFLAGS register to indicate overflow and/or a carry.
To prevent undetected overflow conditions, software must control the ranges of the values upon which it operates.In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE version 64-bit operand: The source operand can be a quadword integer stored in an MMX technology register or a 64-bit memory location.
128-bit Legacy SSE version: The second source operand is an XMM register or a 128-bit memory location.
The first source operand and destination operands are XMM registerVEX.128 encoded version: The second source operand is an XMM register or a 128-bit memory location.
The first source operand and destination operands are XMM registers.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded versions: The second source operand is an YMM register or an 256-bit memory location.
The first source operand and destination operands are YMM registers.
Bits (MAXVL-1:256) of the corresponding ZMM register are zeroed.EVEX encoded VPSUBQ: The second source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory loca-tion or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The first source operand and destination operands are ZMM/YMM/XMM registers.
The destination is conditionally updated with writemask k1.

## Flags affected

- None.

## Exceptions

- Numeric Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."

## Operation

```C
PSUBQ (With 64-Bit Operands) := -DEST[63:0]DEST[63:0]  SRC[63:0];PSUBQ (With 128-Bit Operands):= -DEST[63:0] DEST[63:0]  SRC[63:0];:= -DEST[127:64] DEST[127:64]  SRC[127:64];VPSUBQ (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0]-SRC2[63:0]DEST[127:64] := SRC1[127:64]-SRC2[127:64]DEST[MAXVL-1:128] := 0VPSUBQ (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0]-SRC2[63:0]DEST[127:64] := SRC1[127:64]-SRC2[127:64]DEST[191:128] := SRC1[191:128]-SRC2[191:128]DEST[255:192] := SRC1[255:192]-SRC2[255:192]DEST[MAXVL-1:256] := 0VPSUBQ (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+63:i] := SRC1[i+63:i] - SRC2[63:0]ELSE DEST[i+63:i] := SRC1[i+63:i] - SRC2[i+63:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+63:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentsVPSUBQ __m512i _mm512_sub_epi64(__m512i a, __m512i b);VPSUBQ __m512i _mm512_mask_sub_epi64(__m512i s, __mmask8 k, __m512i a, __m512i b);VPSUBQ __m512i _mm512_maskz_sub_epi64( __mmask8 k, __m512i a, __m512i b);VPSUBQ __m256i _mm256_mask_sub_epi64(__m256i s, __mmask8 k, __m256i a, __m256i b);VPSUBQ __m256i _mm256_maskz_sub_epi64( __mmask8 k, __m256i a, __m256i b);VPSUBQ __m128i _mm_mask_sub_epi64(__m128i s, __mmask8 k, __m128i a, __m128i b);VPSUBQ __m128i _mm_maskz_sub_epi64( __mmask8 k, __m128i a, __m128i b);PSUBQ __m64 _mm_sub_si64(__m64 m1, __m64 m2)(V)PSUBQ __m128i _mm_sub_epi64(__m128i m1, __m128i m2)VPSUBQ __m256i _mm256_sub_epi64(__m256i m1, __m256i m2)
```
