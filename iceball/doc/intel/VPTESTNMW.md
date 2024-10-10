# VPTESTNMB/W/D/Q

Logical NAND and Set

Performs a bitwise logical NAND operation on the byte/word/doubleword/quadword element of the first source operand (the second operand) with the corresponding element of the second source operand (the third operand) and stores the logical comparison result into each bit of the destination operand (the first operand) according to the writemask k1.
Each bit of the result is set to 1 if the bitwise AND of the corresponding elements of the first and second src operands is zero; otherwise it is set to 0.EVEX encoded VPTESTNMD/Q: The first source operand is a ZMM/YMM/XMM registers.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The destination is updated according to the writemask.EVEX encoded VPTESTNMB/W: The first source operand is a ZMM/YMM/XMM registers.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location.
The destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > VPTESTNMD/VPTESTNMQ: 
  > See Table2-49, "Type E4 Class Exception Conditions."
  > VPTESTNMB/VPTESTNMW: 

## Operation

```C
VPTESTNMB(KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j*8IF MaskBit(j) OR *no writemask*THEN    DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] == 0)? 1 : 0ELSE DEST[j] := 0; zeroing masking onlyFIENDFORDEST[MAX_KL-1:KL] := 0VPTESTNMW(KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j*16IF MaskBit(j) OR *no writemask*THEN    DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] == 0)? 1 : 0ELSE DEST[j] := 0; zeroing masking onlyFIENDFORDEST[MAX_KL-1:KL] := 0VPTESTNMD(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j*32IF MaskBit(j) OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)   THEN DEST[i+31:i] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] == 0)? 1 : 0ELSE DEST[j] := (SRC1[i+31:i] BITWELSE DEST[j] := 0; zeroing masking onlyFIENDFORDEST[MAX_KL-1:KL] := 0VPTESTNMQ(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j*64IF MaskBit(j) OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] == 0)? 1 : 0;ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] == 0)? 1 : 0;FI;ELSE DEST[j] := 0; zeroing masking onlyFIENDFORDEST[MAX_KL-1:KL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPTESTNMB __mmask64 _mm512_testn_epi8_mask( __m512i a, __m512i b);VPTESTNMB __mmask64 _mm512_mask_testn_epi8_mask(__mmask64, __m512i a, __m512i b);VPTESTNMB __mmask32 _mm256_testn_epi8_mask(__m256i a, __m256i b);VPTESTNMB __mmask32 _mm256_mask_testn_epi8_mask(__mmask32, __m256i a, __m256i b);VPTESTNMB __mmask16 _mm_testn_epi8_mask(__m128i a, __m128i b);VPTESTNMB __mmask16 _mm_mask_testn_epi8_mask(__mmask16, __m128i a, __m128i b);VPTESTNMW __mmask32 _mm512_testn_epi16_mask( __m512i a, __m512i b);VPTESTNMW __mmask32 _mm512_mask_testn_epi16_mask(__mmask32, __m512i a, __m512i b);VPTESTNMW __mmask16 _mm256_testn_epi16_mask(__m256i a, __m256i b);VPTESTNMW __mmask16 _mm256_mask_testn_epi16_mask(__mmask16, __m256i a, __m256i b);VPTESTNMW __mmask8 _mm_testn_epi16_mask(__m128i a, __m128i b);VPTESTNMW __mmask8 _mm_mask_testn_epi16_mask(__mmask8, __m128i a, __m128i b);VPTESTNMD __mmask16 _mm512_testn_epi32_mask( __m512i a, __m512i b);VPTESTNMD __mmask16 _mm512_mask_testn_epi32_mask(__mmask16, __m512i a, __m512i b);VPTESTNMD __mmask8 _mm256_testn_epi32_mask(__m256i a, __m256i b);VPTESTNMD __mmask8 _mm256_mask_testn_epi32_mask(__mmask8, __m256i a, __m256i b);VPTESTNMD __mmask8 _mm_testn_epi32_mask(__m128i a, __m128i b);VPTESTNMD __mmask8 _mm_mask_testn_epi32_mask(__mmask8, __m128i a, __m128i b);VPTESTNMQ __mmask8 _mm512_testn_epi64_mask(__m512i a, __m512i b);VPTESTNMQ __mmask8 _mm512_mask_testn_epi64_mask(__mmask8, __m512i a, __m512i b);VPTESTNMQ __mmask8 _mm256_testn_epi64_mask(__m256i a, __m256i b);VPTESTNMQ __mmask8 _mm256_mask_testn_epi64_mask(__mmask8, __m256i a, __m256i b);VPTESTNMQ __mmask8 _mm_testn_epi64_mask(__m128i a, __m128i b);VPTESTNMQ __mmask8 _mm_mask_testn_epi64_mask(__mmask8, __m128i a, __m128i b);
```
