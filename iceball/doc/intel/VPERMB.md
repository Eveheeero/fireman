# VPERMB

Permute Packed Bytes Elements

Copies bytes from the second source operand (the third operand) to the destination operand (the first operand) according to the byte indices in the first source operand (the second operand).
Note that this instruction permits a byte in the source operand to be copied to more than one location in the destination operand.
Only the low 6(EVEX.512)/5(EVEX.256)/4(EVEX.128) bits of each byte index is used to select the location of the source byte from the second source operand.The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM reg-ister, a 512/256/128-bit memory location.
The destination operand is a ZMM/YMM/XMM register updated at byte granularity by the writemask k1.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPERMB (EVEX encoded versions)(KL, VL) = (16, 128), (32, 256), (64, 512)IF VL = 128:n := 3;ELSE IF VL = 256:n := 4;ELSE IF VL = 512:n := 5;FI;FOR j := 0 TO KL-1:id := SRC1[j*8 + n : j*8] ; // location of the source byteIF k1[j] OR *no writemask* THENDEST[j*8 + 7: j*8] := SRC2[id*8 +7: id*8];ELSE IF zeroing-masking THENDEST[j*8 + 7: j*8] := 0;*ELSE DEST[j*8 + 7: j*8] remains unchanged*FIIntel C/C++ Compiler Intrinsic EquivalentVPERMB __m512i _mm512_permutexvar_epi8( __m512i idx, __m512i a);VPERMB __m512i _mm512_mask_permutexvar_epi8(__m512i s, __mmask64 k, __m512i idx, __m512i a);VPERMB __m512i _mm512_maskz_permutexvar_epi8( __mmask64 k, __m512i idx, __m512i a);VPERMB __m256i _mm256_permutexvar_epi8( __m256i idx, __m256i a);VPERMB __m256i _mm256_mask_permutexvar_epi8(__m256i s, __mmask32 k, __m256i idx, __m256i a);VPERMB __m256i _mm256_maskz_permutexvar_epi8( __mmask32 k, __m256i idx, __m256i a);VPERMB __m128i _mm_permutexvar_epi8( __m128i idx, __m128i a);VPERMB __m128i _mm_mask_permutexvar_epi8(__m128i s, __mmask16 k, __m128i idx, __m128i a);VPERMB __m128i _mm_maskz_permutexvar_epi8( __mmask16 k, __m128i idx, __m128i a);
```
