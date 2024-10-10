# PMAXUD/PMAXUQ

Maximum of Packed Unsigned Integers

Performs a SIMD compare of the packed unsigned dword or qword integers in the second source operand and the first source operand and returns the maximum value for each pair of integers to the destination operand.
128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding destination register remain unchanged.VEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding destination register are zeroed.VEX.256 encoded version: The first source operand is a YMM register; The second source operand is a YMM register EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register; The second source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is conditionally updated based on writemask k1.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
PMAXUD (128-bit Legacy SSE Version)IF DEST[31:0] >SRC[31:0] THENDEST[31:0] := DEST[31:0];ELSEDEST[31:0] := SRC[31:0]; FI;(* Repeat operation for 2nd through 7th words in source and destination operands *)IF DEST[127:96] >SRC[127:96] THENDEST[127:96] := DEST[127:96];ELSEDEST[127:96] := SRC[127:96]; FI;DEST[MAXVL-1:128] (Unmodified)VPMAXUD (VEX.128 Encoded Version)IF SRC1[31:0] > SRC2[31:0] THENDEST[31:0] := SRC1[31:0];ELSEDEST[31:0] := SRC2[31:0]; FI;(* Repeat operation for 2nd through 3rd dwords in source and destination operands *)IF SRC1[127:96] > SRC2[127:96] THENDEST[127:96] := SRC1[127:96];ELSEDEST[127:96] := SRC2[127:96]; FI;DEST[MAXVL-1:128] := 0VPMAXUD (VEX.256 Encoded Version)IF SRC1[31:0] > SRC2[31:0] THENDEST[31:0] := SRC1[31:0];ELSEDEST[31:0] := SRC2[31:0]; FI;(* Repeat operation for 2nd through 7th dwords in source and destination operands *)IF SRC1[255:224] > SRC2[255:224] THENDEST[255:224] := SRC1[255:224];ELSEVPMAXUD (EVEX Encoded Versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN IF SRC1[i+31:i] > SRC2[31:0] THEN DEST[i+31:i] := SRC1[i+31:i];ELSE DEST[i+31:i] := SRC2[31:0]; FI;ELSE IF SRC1[i+31:i] > SRC2[i+31:i] THEN DEST[i+31:i] := SRC1[i+31:i];ELSE DEST[i+31:i] := SRC2[i+31:i]; FI;FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[i+31:i] := 0FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0VPMAXUQ (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN IF SRC1[i+63:i] > SRC2[63:0] THEN DEST[i+63:i] := SRC1[i+63:i];ELSE DEST[i+63:i] := SRC2[63:0]; FI;ELSE IF SRC1[i+31:i] > SRC2[i+31:i] THEN DEST[i+63:i] := SRC1[i+63:i];ELSE DEST[i+63:i] := SRC2[i+63:i]; FI;FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[i+63:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVPMAXUD __m512i _mm512_max_epu32( __m512i a, __m512i b);VPMAXUD __m512i _mm512_mask_max_epu32(__m512i s, __mmask16 k, __m512i a, __m512i b);VPMAXUD __m512i _mm512_maskz_max_epu32( __mmask16 k, __m512i a, __m512i b);VPMAXUQ __m512i _mm512_max_epu64( __m512i a, __m512i b);VPMAXUQ __m512i _mm512_mask_max_epu64(__m512i s, __mmask8 k, __m512i a, __m512i b);VPMAXUQ __m512i _mm512_maskz_max_epu64( __mmask8 k, __m512i a, __m512i b);VPMAXUD __m256i _mm256_mask_max_epu32(__m256i s, __mmask16 k, __m256i a, __m256i b);VPMAXUD __m256i _mm256_maskz_max_epu32( __mmask16 k, __m256i a, __m256i b);VPMAXUQ __m256i _mm256_mask_max_epu64(__m256i s, __mmask8 k, __m256i a, __m256i b);VPMAXUQ __m256i _mm256_maskz_max_epu64( __mmask8 k, __m256i a, __m256i b);VPMAXUD __m128i _mm_mask_max_epu32(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMAXUD __m128i _mm_maskz_max_epu32( __mmask8 k, __m128i a, __m128i b);VPMAXUQ __m128i _mm_mask_max_epu64(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMAXUQ __m128i _mm_maskz_max_epu64( __mmask8 k, __m128i a, __m128i b);(V)PMAXUD __m128i _mm_max_epu32 ( __m128i a, __m128i b);VPMAXUD __m256i _mm256_max_epu32 ( __m256i a, __m256i b);
```
