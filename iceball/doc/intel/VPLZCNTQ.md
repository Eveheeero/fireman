# VPLZCNTD/Q

Count the Number of Leading Zero Bits for Packed Dword, Packed Qword Values

Counts the number of leading most significant zero bits in each dword or qword element of the source operand (the second operand) and stores the results in the destination register (the first operand) according to the writemask.
If an element is zero, the result for that element is the operand size of the element.
EVEX.512 encoded version: The source operand is a ZMM register, a 512-bit memory location, or a 512-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a ZMM register, conditionally updated using writemask k1.
EVEX.256 encoded version: The source operand is a YMM register, a 256-bit memory location, or a 256-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a YMM register, conditionally updated using writemask k1.
EVEX.128 encoded version: The source operand is a XMM register, a 128-bit memory location, or a 128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a XMM register, conditionally updated 

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPLZCNTD(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j*32IF MaskBit(j) OR *no writemask*THEN    temp := 32   DEST[i+31:i] := 0   WHILE (temp > 0) AND (SRC[i+temp-1] = 0)   DOtemp := temp - 1DEST[i+31:i] := DEST[i+31:i] + 1   ODELSE   IF *merging-masking* THEN *DEST[i+31:i] remains unchanged*ELSE DEST[i+31:i] := 0   FIFIENDFORDEST[MAXVL-1:VL] := 0VPLZCNTQ(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j*64IF MaskBit(j) OR *no writemask*THEN   temp := 64   DEST[i+63:i] := 0   WHILE (temp > 0) AND (SRC[i+temp-1] = 0)  DOtemp := temp - 1DEST[i+63:i] := DEST[i+63:i] + 1   ODELSE   IF *merging-masking* THEN *DEST[i+63:i] remains unchanged*ELSE DEST[i+63:i] := 0   FIFIIntel C/C++ Compiler Intrinsic EquivalentVPLZCNTD __m512i _mm512_lzcnt_epi32(__m512i a);VPLZCNTD __m512i _mm512_mask_lzcnt_epi32(__m512i s, __mmask16 m, __m512i a);VPLZCNTD __m512i _mm512_maskz_lzcnt_epi32( __mmask16 m, __m512i a);VPLZCNTQ __m512i _mm512_lzcnt_epi64(__m512i a);VPLZCNTQ __m512i _mm512_mask_lzcnt_epi64(__m512i s, __mmask8 m, __m512i a);VPLZCNTQ __m512i _mm512_maskz_lzcnt_epi64(__mmask8 m, __m512i a);VPLZCNTD __m256i _mm256_lzcnt_epi32(__m256i a);VPLZCNTD __m256i _mm256_mask_lzcnt_epi32(__m256i s, __mmask8 m, __m256i a);VPLZCNTD __m256i _mm256_maskz_lzcnt_epi32( __mmask8 m, __m256i a);VPLZCNTQ __m256i _mm256_lzcnt_epi64(__m256i a);VPLZCNTQ __m256i _mm256_mask_lzcnt_epi64(__m256i s, __mmask8 m, __m256i a);VPLZCNTQ __m256i _mm256_maskz_lzcnt_epi64(__mmask8 m, __m256i a);VPLZCNTD __m128i _mm_lzcnt_epi32(__m128i a);VPLZCNTD __m128i _mm_mask_lzcnt_epi32(__m128i s, __mmask8 m, __m128i a);VPLZCNTD __m128i _mm_maskz_lzcnt_epi32( __mmask8 m, __m128i a);VPLZCNTQ __m128i _mm_lzcnt_epi64(__m128i a);VPLZCNTQ __m128i _mm_mask_lzcnt_epi64(__m128i s, __mmask8 m, __m128i a);VPLZCNTQ __m128i _mm_maskz_lzcnt_epi64(__mmask8 m, __m128i a);
```
