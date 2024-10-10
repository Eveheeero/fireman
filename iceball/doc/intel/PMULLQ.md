# PMULLD/PMULLQ

Multiply Packed Integers and Store Low Result

Performs a SIMD signed multiply of the packed signed dword/qword integers from each element of the first source operand with the corresponding element in the second source operand.
The low 32/64 bits of each 64/128-bit intermediate results are stored to the destination operand.
128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding ZMM destina-tion register remain unchanged.VEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding ZMM register are zeroed.VEX.256 encoded version: The first source operand is a YMM register; The second source operand is a YMM register EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is conditionally updated based on writemask k1.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPMULLQ (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b == 1) AND (SRC2 *is memory*)THEN Temp[127:0] := SRC1[i+63:i] * SRC2[63:0]ELSE Temp[127:0] := SRC1[i+63:i] * SRC2[i+63:i]FI;DEST[i+63:i] := Temp[63:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VPMULLD (EVEX Encoded Versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN Temp[63:0] := SRC1[i+31:i] * SRC2[31:0]ELSE Temp[63:0] := SRC1[i+31:i] * SRC2[i+31:i]FI;DEST[i+31:i] := Temp[31:0]ELSE IF *merging-masking*; merging-masking*DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VPMULLD (VEX.256 Encoded Version)Temp0[63:0] := SRC1[31:0] * SRC2[31:0]Temp1[63:0] := SRC1[63:32] * SRC2[63:32]Temp2[63:0] := SRC1[95:64] * SRC2[95:64]Temp3[63:0] := SRC1[127:96] * SRC2[127:96]Temp4[63:0] := SRC1[159:128] * SRC2[159:128]Temp5[63:0] := SRC1[191:160] * SRC2[191:160]Temp6[63:0] := SRC1[223:192] * SRC2[223:192]Temp7[63:0] := SRC1[255:224] * SRC2[255:224]DEST[31:0] := Temp0[31:0]DEST[63:32] := Temp1[31:0]DEST[95:64] := Temp2[31:0]DEST[127:96] := Temp3[31:0]DEST[159:128] := Temp4[31:0]DEST[191:160] := Temp5[31:0]DEST[223:192] := Temp6[31:0]DEST[255:224] := Temp7[31:0]DEST[MAXVL-1:256] := 0VPMULLD (VEX.128 Encoded Version)Temp0[63:0] := SRC1[31:0] * SRC2[31:0]Temp1[63:0] := SRC1[63:32] * SRC2[63:32]Temp2[63:0] := SRC1[95:64] * SRC2[95:64]Temp3[63:0] := SRC1[127:96] * SRC2[127:96]DEST[31:0] := Temp0[31:0]DEST[63:32] := Temp1[31:0]DEST[95:64] := Temp2[31:0]DEST[127:96] := Temp3[31:0]DEST[MAXVL-1:128] := 0PMULLD (128-bit Legacy SSE Version)Temp0[63:0] := DEST[31:0] * SRC[31:0]Temp1[63:0] := DEST[63:32] * SRC[63:32]Temp2[63:0] := DEST[95:64] * SRC[95:64]Temp3[63:0] := DEST[127:96] * SRC[127:96]DEST[31:0] := Temp0[31:0]DEST[63:32] := Temp1[31:0]DEST[95:64] := Temp2[31:0]DEST[127:96] := Temp3[31:0]Intel C/C++ Compiler Intrinsic EquivalentVPMULLD __m512i _mm512_mullo_epi32(__m512i a, __m512i b);VPMULLD __m512i _mm512_mask_mullo_epi32(__m512i s, __mmask16 k, __m512i a, __m512i b);VPMULLD __m512i _mm512_maskz_mullo_epi32( __mmask16 k, __m512i a, __m512i b);VPMULLD __m256i _mm256_mask_mullo_epi32(__m256i s, __mmask8 k, __m256i a, __m256i b);VPMULLD __m256i _mm256_maskz_mullo_epi32( __mmask8 k, __m256i a, __m256i b);VPMULLD __m128i _mm_mask_mullo_epi32(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMULLD __m128i _mm_maskz_mullo_epi32( __mmask8 k, __m128i a, __m128i b);VPMULLD __m256i _mm256_mullo_epi32(__m256i a, __m256i b);PMULLD __m128i _mm_mullo_epi32(__m128i a, __m128i b);VPMULLQ __m512i _mm512_mullo_epi64(__m512i a, __m512i b);VPMULLQ __m512i _mm512_mask_mullo_epi64(__m512i s, __mmask8 k, __m512i a, __m512i b);VPMULLQ __m512i _mm512_maskz_mullo_epi64( __mmask8 k, __m512i a, __m512i b);VPMULLQ __m256i _mm256_mullo_epi64(__m256i a, __m256i b);VPMULLQ __m256i _mm256_mask_mullo_epi64(__m256i s, __mmask8 k, __m256i a, __m256i b);VPMULLQ __m256i _mm256_maskz_mullo_epi64( __mmask8 k, __m256i a, __m256i b);VPMULLQ __m128i _mm_mullo_epi64(__m128i a, __m128i b);VPMULLQ __m128i _mm_mask_mullo_epi64(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMULLQ __m128i _mm_maskz_mullo_epi64( __mmask8 k, __m128i a, __m128i b);
```
