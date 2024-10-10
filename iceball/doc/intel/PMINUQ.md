# PMINUD/PMINUQ

Minimum of Packed Unsigned Integers

Performs a SIMD compare of the packed unsigned dword/qword integers in the second source operand and the first source operand and returns the minimum value for each pair of integers to the destination operand.
128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding destination register remain unchanged.VEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding destination register are zeroed.VEX.256 encoded version: The second source operand can be an YMM register or a 256-bit memory location.
The first source and destination operands are YMM registers.
Bits (MAXVL-1:256) of thEVEX encoded versions: The first source operand is a ZMM/YMM/XMM register; The second source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is conditionally updated based on writemask k1.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
PMINUD (128-bit Legacy SSE Version)PMINUD instruction for 128-bit operands:IF DEST[31:0] < SRC[31:0] THENDEST[31:0] := DEST[31:0];ELSEDEST[31:0] := SRC[31:0]; FI;(* Repeat operation for 2nd through 7th words in source and destination operands *)IF DEST[127:96] < SRC[127:96] THENDEST[127:96] := DEST[127:96];ELSEDEST[127:96] := SRC[127:96]; FI;DEST[MAXVL-1:128] (Unmodified)VPMINUD (VEX.128 Encoded Version)VPMINUD instruction for 128-bit operands:IF SRC1[31:0] < SRC2[31:0] THENDEST[31:0] := SRC1[31:0];ELSEDEST[31:0] := SRC2[31:0]; FI;(* Repeat operation for 2nd through 3rd dwords in source and destination operands *)IF SRC1[127:96] < SRC2[127:96] THENDEST[127:96] := SRC1[127:96];ELSEDEST[127:96] := SRC2[127:96]; FI;DEST[MAXVL-1:128] := 0VPMINUD (VEX.256 Encoded Version)VPMINUD instruction for 128-bit operands:IF SRC1[31:0] < SRC2[31:0] THENDEST[31:0] := SRC1[31:0];ELSEDEST[31:0] := SRC2[31:0]; FI;(* Repeat operation for 2nd through 7th dwords in source and destination operands *)IF SRC1[255:224] < SRC2[255:224] THENDEST[255:224] := SRC1[255:224];ELSEVPMINUD (EVEX Encoded Versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN IF SRC1[i+31:i] < SRC2[31:0] THEN DEST[i+31:i] := SRC1[i+31:i];ELSE DEST[i+31:i] := SRC2[31:0]; FI;ELSE IF SRC1[i+31:i] < SRC2[i+31:i] THEN DEST[i+31:i] := SRC1[i+31:i];ELSE DEST[i+31:i] := SRC2[i+31:i]; FI;FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0VPMINUQ (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN IF SRC1[i+63:i] < SRC2[63:0] THEN DEST[i+63:i] := SRC1[i+63:i];ELSE DEST[i+63:i] := SRC2[63:0]; FI;ELSE IF SRC1[i+63:i] < SRC2[i+63:i] THEN DEST[i+63:i] := SRC1[i+63:i];ELSE DEST[i+63:i] := SRC2[i+63:i]; FI;FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVPMINUD __m512i _mm512_min_epu32( __m512i a, __m512i b);VPMINUD __m512i _mm512_mask_min_epu32(__m512i s, __mmask16 k, __m512i a, __m512i b);VPMINUD __m512i _mm512_maskz_min_epu32( __mmask16 k, __m512i a, __m512i b);VPMINUQ __m512i _mm512_min_epu64( __m512i a, __m512i b);VPMINUQ __m512i _mm512_mask_min_epu64(__m512i s, __mmask8 k, __m512i a, __m512i b);VPMINUQ __m512i _mm512_maskz_min_epu64( __mmask8 k, __m512i a, __m512i b);VPMINUD __m256i _mm256_mask_min_epu32(__m256i s, __mmask16 k, __m256i a, __m256i b);VPMINUD __m256i _mm256_maskz_min_epu32( __mmask16 k, __m256i a, __m256i b);VPMINUQ __m256i _mm256_mask_min_epu64(__m256i s, __mmask8 k, __m256i a, __m256i b);VPMINUQ __m256i _mm256_maskz_min_epu64( __mmask8 k, __m256i a, __m256i b);VPMINUD __m128i _mm_mask_min_epu32(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMINUD __m128i _mm_maskz_min_epu32( __mmask8 k, __m128i a, __m128i b);VPMINUQ __m128i _mm_mask_min_epu64(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMINUQ __m128i _mm_maskz_min_epu64( __mmask8 k, __m128i a, __m128i b);(V)PMINUD __m128i _mm_min_epu32 ( __m128i a, __m128i b);VPMINUD __m256i _mm256_min_epu32 ( __m256i a, __m256i b);
```
