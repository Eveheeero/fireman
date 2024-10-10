# POR

Bitwise Logical OR

Performs a bitwise logical OR operation on the source operand (second operand) and the destination operand (first operand) and stores the result in the destination operand.
Each bit of the result is set to 1 if either or both of the corresponding bits of the first and second operands are 1; otherwise, it is set to 0.In 64-bit mode and not encoded with VEX/EVEX, using a REX Legacy SSE version: The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand is an MMX technology register.128-bit Legacy SSE version: The second source operand is an XMM register or a 128-bit memory location.
The first source and destination operands can be XMM registers.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: The second source operand is an XMM register or a 128-bit memory location.
The first source and destination operands can be XMM registers.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded version: The second source operand is an YMM register or a 256-bit memory location.
The first source and destination operands can be YMM registers.EVEX encoded version: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with write-mask k1 at 32/64-bit granularity.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
POR (64-bit Operand)DEST := DEST OR SRCPOR (128-bit Legacy SSE Version)DEST := DEST OR SRCDEST[MAXVL-1:128] (Unmodified)VPOR (VEX.128 Encoded Version)DEST := SRC1 OR SRC2DEST[MAXVL-1:128] := 0VPOR (VEX.256 Encoded Version)DEST := SRC1 OR SRC2DEST[MAXVL-1:256] := 0VPORD (EVEX Encoded Versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+31:i] := SRC1[i+31:i] BITWISE OR SRC2[31:0]ELSE DEST[i+31:i] := SRC1[i+31:i] BITWISE OR SRC2[i+31:i]FI;ELSE IF *merging-masking*; merging-masking*DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FI;FI;Intel C/C++ Compiler Intrinsic EquivalentVPORD __m512i _mm512_or_epi32(__m512i a, __m512i b);VPORD __m512i _mm512_mask_or_epi32(__m512i s, __mmask16 k, __m512i a, __m512i b);VPORD __m512i _mm512_maskz_or_epi32( __mmask16 k, __m512i a, __m512i b);VPORD __m256i _mm256_or_epi32(__m256i a, __m256i b);VPORD __m256i _mm256_mask_or_epi32(__m256i s, __mmask8 k, __m256i a, __m256i b,);VPORD __m256i _mm256_maskz_or_epi32( __mmask8 k, __m256i a, __m256i b);VPORD __m128i _mm_or_epi32(__m128i a, __m128i b);VPORD __m128i _mm_mask_or_epi32(__m128i s, __mmask8 k, __m128i a, __m128i b);VPORD __m128i _mm_maskz_or_epi32( __mmask8 k, __m128i a, __m128i b);VPORQ __m512i _mm512_or_epi64(__m512i a, __m512i b);VPORQ __m512i _mm512_mask_or_epi64(__m512i s, __mmask8 k, __m512i a, __m512i b);VPORQ __m512i _mm512_maskz_or_epi64(__mmask8 k, __m512i a, __m512i b);VPORQ __m256i _mm256_or_epi64(__m256i a, int imm);VPORQ __m256i _mm256_mask_or_epi64(__m256i s, __mmask8 k, __m256i a, __m256i b);VPORQ __m256i _mm256_maskz_or_epi64( __mmask8 k, __m256i a, __m256i b);VPORQ __m128i _mm_or_epi64(__m128i a, __m128i b);VPORQ __m128i _mm_mask_or_epi64(__m128i s, __mmask8 k, __m128i a, __m128i b);VPORQ __m128i _mm_maskz_or_epi64( __mmask8 k, __m128i a, __m128i b);POR __m64 _mm_or_si64(__m64 m1, __m64 m2)(V)POR __m128i _mm_or_si128(__m128i m1, __m128i m2)VPOR __m256i _mm256_or_si256 ( __m256i a, __m256i b)
```
