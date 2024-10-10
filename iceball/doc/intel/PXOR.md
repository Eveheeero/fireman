# PXOR

Logical Exclusive OR

Performs a bitwise logical exclusive-OR (XOR) operation on the source operand (second operand) and the destina-tion operand (first operand) and stores the result in the destination operand.
Each bit of the result is 1 if the corre-sponding bits of the two operands are different; each bit is 0 if the corresponding bits of the operands are the same.In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE instructions 64-bit operand: The source operand can be an MMX technology register or a 64-bit 128-bit Legacy SSE version: The second source operand is an XMM register or a 128-bit memory location.
The first source operand and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corresponding YMM desti-nation register remain unchanged.VEX.128 encoded version: The second source operand is an XMM register or a 128-bit memory location.
The first source operand and destination operands are XMM registers.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding register destination are zeroed.EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with write-mask k1.

## Flags affected

- None.

## Exceptions

- Numeric Exceptions
  > None.
- Other Exceptions

## Operation

```C
PXOR (64-bit Operand)DEST := DEST XOR SRCPXOR (128-bit Legacy SSE Version)DEST := DEST XOR SRCDEST[MAXVL-1:128] (Unmodified)VPXOR (VEX.128 Encoded Version)DEST := SRC1 XOR SRC2DEST[MAXVL-1:128] := 0VPXOR (VEX.256 Encoded Version)DEST := SRC1 XOR SRC2DEST[MAXVL-1:256] := 0VPXORD (EVEX Encoded Versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+31:i] := SRC1[i+31:i] BITWISE XOR SRC2[31:0]ELSE DEST[i+31:i] := SRC1[i+31:i] BITWISE XOR SRC2[i+31:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingDEST[31:0] := 0FI;FI;VPXORQ (EVEX Encoded Versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+63:i] := SRC1[i+63:i] BITWISE XOR SRC2[63:0]ELSE DEST[i+63:i] := SRC1[i+63:i] BITWISE XOR SRC2[i+63:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingDEST[63:0] := 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPXORD __m512i _mm512_xor_epi32(__m512i a, __m512i b)VPXORD __m512i _mm512_mask_xor_epi32(__m512i s, __mmask16 m, __m512i a, __m512i b)VPXORD __m512i _mm512_maskz_xor_epi32( __mmask16 m, __m512i a, __m512i b)VPXORD __m256i _mm256_xor_epi32(__m256i a, __m256i b)VPXORD __m256i _mm256_mask_xor_epi32(__m256i s, __mmask8 m, __m256i a, __m256i b)VPXORD __m256i _mm256_maskz_xor_epi32( __mmask8 m, __m256i a, __m256i b)VPXORD __m128i _mm_xor_epi32(__m128i a, __m128i b)VPXORD __m128i _mm_mask_xor_epi32(__m128i s, __mmask8 m, __m128i a, __m128i b)VPXORD __m128i _mm_maskz_xor_epi32( __mmask16 m, __m128i a, __m128i b)VPXORQ __m512i _mm512_xor_epi64( __m512i a, __m512i b);VPXORQ __m512i _mm512_mask_xor_epi64(__m512i s, __mmask8 m, __m512i a, __m512i b);VPXORQ __m512i _mm512_maskz_xor_epi64(__mmask8 m, __m512i a, __m512i b);VPXORQ __m256i _mm256_xor_epi64( __m256i a, __m256i b);VPXORQ __m256i _mm256_mask_xor_epi64(__m256i s, __mmask8 m, __m256i a, __m256i b);VPXORQ __m256i _mm256_maskz_xor_epi64(__mmask8 m, __m256i a, __m256i b);VPXORQ __m128i _mm_xor_epi64( __m128i a, __m128i b);VPXORQ __m128i _mm_mask_xor_epi64(__m128i s, __mmask8 m, __m128i a, __m128i b);VPXORQ __m128i _mm_maskz_xor_epi64(__mmask8 m, __m128i a, __m128i b);PXOR:__m64 _mm_xor_si64 (__m64 m1, __m64 m2)(V)PXOR:__m128i _mm_xor_si128 ( __m128i a, __m128i b)VPXOR:__m256i _mm256_xor_si256 ( __m256i a, __m256i b)
```
