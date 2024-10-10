# PAND

Logical AND

Performs a bitwise logical AND operation on the first source operand and second source operand and stores the result in the destination operand.
Each bit of the result is set to 1 if the corresponding bits of the first and second operands are 1, otherwise it is set to 0.In 64-bit mode and not encoded with VEX/EVEX, using a REX prLegacy SSE instructions: The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand can be an MMX technology register.128-bit Legacy SSE version: The first source operand is an XMM register.
The second operand can be an XMM register or an 128-bit memory location.
The destination is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with write-mask k1 at 32/64-bit granularity.VEX.256 encoded versions: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded versions: The first source operand is an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.

## Flags affected

- None.

## Exceptions

- Other Exceptions
- Numeric Exceptions
  > None.

## Operation

```C
PAND (64-bit Operand)DEST := DEST AND SRCPAND (128-bit Legacy SSE Version)DEST := DEST AND SRCDEST[MAXVL-1:128] (Unmodified)VPAND (VEX.128 Encoded Version)DEST := SRC1 AND SRC2DEST[MAXVL-1:128] := 0VPAND (VEX.256 Encoded Instruction)DEST[255:0] := (SRC1[255:0] AND SRC2[255:0])DEST[MAXVL-1:256] := 0VPANDD (EVEX Encoded Versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+31:i] := SRC1[i+31:i] BITWISE AND SRC2[31:0]ELSE DEST[i+31:i] := SRC1[i+31:i] BITWISE AND SRC2[i+31:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VPANDQ (EVEX Encoded Versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+63:i] := SRC1[i+63:i] BITWISE AND SRC2[63:0]ELSE DEST[i+63:i] := SRC1[i+63:i] BITWISE AND SRC2[i+63:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentsVPANDD __m512i _mm512_and_epi32( __m512i a, __m512i b);VPANDD __m512i _mm512_mask_and_epi32(__m512i s, __mmask16 k, __m512i a, __m512i b);VPANDD __m512i _mm512_maskz_and_epi32( __mmask16 k, __m512i a, __m512i b);VPANDQ __m512i _mm512_and_epi64( __m512i a, __m512i b);VPANDQ __m512i _mm512_mask_and_epi64(__m512i s, __mmask8 k, __m512i a, __m512i b);VPANDQ __m512i _mm512_maskz_and_epi64( __mmask8 k, __m512i a, __m512i b);VPANDND __m256i _mm256_mask_and_epi32(__m256i s, __mmask8 k, __m256i a, __m256i b);VPANDND __m256i _mm256_maskz_and_epi32( __mmask8 k, __m256i a, __m256i b);VPANDND __m128i _mm_mask_and_epi32(__m128i s, __mmask8 k, __m128i a, __m128i b);VPANDND __m128i _mm_maskz_and_epi32( __mmask8 k, __m128i a, __m128i b);VPANDNQ __m256i _mm256_mask_and_epi64(__m256i s, __mmask8 k, __m256i a, __m256i b);VPANDNQ __m256i _mm256_maskz_and_epi64( __mmask8 k, __m256i a, __m256i b);VPANDNQ __m128i _mm_mask_and_epi64(__m128i s, __mmask8 k, __m128i a, __m128i b);VPANDNQ __m128i _mm_maskz_and_epi64( __mmask8 k, __m128i a, __m128i b);PAND __m64 _mm_and_si64 (__m64 m1, __m64 m2)(V)PAND __m128i _mm_and_si128 ( __m128i a, __m128i b)VPAND__m256i _mm256_and_si256 ( __m256i a, __m256i b)
```
