# VPTERNLOGD/VPTERNLOGQ

Bitwise Ternary Logic

VPTERNLOGD/Q takes three bit vectors of 512-bit length (in the first, second, and third operand) as input data to form a set of 512 indices, each index is comprised of one bit from each input vector.
The imm8 byte specifies a boolean logic table producing a binary value for each 3-bit index value.
The final 512-bit boolean result is written to the destination operand (the first operand) using the writemask k1 with the granularity of doubleword element or quadword element into the destination.The destination operand is a ZMM (EVEX.512)/YMM (EVEX.256)/XMM (EVEX.128) register.
The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasTable 5-12 shows two examples of Boolean functions specified by immediate values 0xE2 and 0xE4, with the look up result listed in the fourth column following the three columns containing all possible values of the 3-bit index.Table 5-12.
Examples of VPTERNLOGD/Q Imm8 Boolean Function and Input Index ValuesVPTERNLOGD reg1, reg2, src3, 0xE2Bit Result with VPTERNLOGD reg1, reg2, src3, 0xE4Bit Result with Imm8=0xE2Imm8=0xE4Bit(reg1)Bit(reg2)Bit(src3)Bit(reg1)Bit(reg2)Bit(src3)0000000000110010010001010110011010001000101110111101110111111111Specifying different values in imm8 will allow any arbitrary three-input Boolean functions to be implemented in software using VPTERNLOGD/Q.
Table 5-1 and Table 5-2 provide a mapping of all 256 possible imm8 values to various Boolean expressions.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPTERNLOGD (EVEX encoded versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN FOR k := 0 TO 31 IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ k ]]ELSE DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ i+k ]]FI;; table lookup of immediate bellow;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31+i:i] remains unchanged*ELSE ; zeroing-maskingDEST[31+i:i] := 0FI;FI;VPTERNLOGQ (EVEX encoded versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN FOR k := 0 TO 63 IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ k ]]ELSE DEST[j][k] := imm[(DEST[i+k] << 2) + (SRC1[ i+k ] << 1) + SRC2[ i+k ]]FI;; table lookup of immediate bellow;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63+i:i] remains unchanged*ELSE ; zeroing-maskingDEST[63+i:i] := 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentsVPTERNLOGD __m512i _mm512_ternarylogic_epi32(__m512i a, __m512i b, int imm);VPTERNLOGD __m512i _mm512_mask_ternarylogic_epi32(__m512i s, __mmask16 m, __m512i a, __m512i b, int imm);VPTERNLOGD __m512i _mm512_maskz_ternarylogic_epi32(__mmask m, __m512i a, __m512i b, int imm);VPTERNLOGD __m256i _mm256_ternarylogic_epi32(__m256i a, __m256i b, int imm);VPTERNLOGD __m256i _mm256_mask_ternarylogic_epi32(__m256i s, __mmask8 m, __m256i a, __m256i b, int imm);VPTERNLOGD __m256i _mm256_maskz_ternarylogic_epi32( __mmask8 m, __m256i a, __m256i b, int imm);VPTERNLOGD __m128i _mm_ternarylogic_epi32(__m128i a, __m128i b, int imm);VPTERNLOGD __m128i _mm_mask_ternarylogic_epi32(__m128i s, __mmask8 m, __m128i a, __m128i b, int imm);VPTERNLOGD __m128i _mm_maskz_ternarylogic_epi32( __mmask8 m, __m128i a, __m128i b, int imm);VPTERNLOGQ __m512i _mm512_ternarylogic_epi64(__m512i a, __m512i b, int imm);VPTERNLOGQ __m512i _mm512_mask_ternarylogic_epi64(__m512i s, __mmask8 m, __m512i a, __m512i b, int imm);VPTERNLOGQ __m512i _mm512_maskz_ternarylogic_epi64( __mmask8 m, __m512i a, __m512i b, int imm);VPTERNLOGQ __m256i _mm256_ternarylogic_epi64(__m256i a, __m256i b, int imm);VPTERNLOGQ __m256i _mm256_mask_ternarylogic_epi64(__m256i s, __mmask8 m, __m256i a, __m256i b, int imm);VPTERNLOGQ __m256i _mm256_maskz_ternarylogic_epi64( __mmask8 m, __m256i a, __m256i b, int imm);VPTERNLOGQ __m128i _mm_ternarylogic_epi64(__m128i a, __m128i b, int imm);VPTERNLOGQ __m128i _mm_mask_ternarylogic_epi64(__m128i s, __mmask8 m, __m128i a, __m128i b, int imm);VPTERNLOGQ __m128i _mm_maskz_ternarylogic_epi64( __mmask8 m, __m128i a, __m128i b, int imm);
```
