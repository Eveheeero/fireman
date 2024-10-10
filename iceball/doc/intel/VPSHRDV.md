# VPSHRDV

Concatenate and Variable Shift Packed Data Right Logical



## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPSHRDVW DEST, SRC2, SRC3(KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1:IF MaskBit(j) OR *no writemask*:DEST.word[j] := concat(SRC2.word[j], DEST.word[j]) >> (SRC3.word[j] & 15)ELSE IF *zeroing*:DEST.word[j] := 0*ELSE DEST.word[j] remains unchanged*DEST[MAX_VL-1:VL] := 0VPSHRDVD DEST, SRC2, SRC3(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1:IF SRC3 is broadcast memop:tsrc3 := SRC3.dword[0]ELSE:tsrc3 := SRC3.dword[j]IF MaskBit(j) OR *no writemask*:DEST.dword[j] := concat(SRC2.dword[j], DEST.dword[j]) >> (tsrc3 & 31)ELSE IF *zeroing*:DEST.dword[j] := 0*ELSE DEST.dword[j] remains unchanged*DEST[MAX_VL-1:VL] := 0VPSHRDVQ DEST, SRC2, SRC3(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1:IF SRC3 is broadcast memop:tsrc3 := SRC3.qword[0]ELSE:tsrc3 := SRC3.qword[j]IF MaskBit(j) OR *no writemask*:DEST.qword[j] := concat(SRC2.qword[j], DEST.qword[j]) >> (tsrc3 & 63)ELSE IF *zeroing*:DEST.qword[j] := 0*ELSE DEST.qwoIntel C/C++ Compiler Intrinsic EquivalentVPSHRDVQ __m128i  _mm_shrdv_epi64(__m128i, __m128i, __m128i);VPSHRDVQ __m128i  _mm_mask_shrdv_epi64(__m128i, __mmask8, __m128i, __m128i);VPSHRDVQ __m128i  _mm_maskz_shrdv_epi64(__mmask8, __m128i, __m128i, __m128i);VPSHRDVQ __m256i  _mm256_shrdv_epi64(__m256i, __m256i, __m256i);VPSHRDVQ __m256i  _mm256_mask_shrdv_epi64(__m256i, __mmask8, __m256i, __m256i);VPSHRDVQ __m256i  _mm256_maskz_shrdv_epi64(__mmask8, __m256i, __m256i, __m256i);VPSHRDVQ __m512i  _mm512_shrdv_epi64(__m512i, __m512i, __m512i);VPSHRDVQ __m512i  _mm512_mask_shrdv_epi64(__m512i, __mmask8, __m512i, __m512i);VPSHRDVQ __m512i  _mm512_maskz_shrdv_epi64(__mmask8, __m512i, __m512i, __m512i);VPSHRDVD __m128i _mm_shrdv_epi32(__m128i, __m128i, __m128i);VPSHRDVD __m128i _mm_mask_shrdv_epi32(__m128i, __mmask8, __m128i, __m128i);VPSHRDVD __m128i _mm_maskz_shrdv_epi32(__mmask8, __m128i, __m128i, __m128i);VPSHRDVD __m256i _mm256_shrdv_epi32(__m256i, __m256i, __m256i);VPSHRDVD __m256i _mm256_mask_shrdv_epi32(__m256i, __mmask8, __m256i, __m256i);VPSHRDVD __m256i _mm256_maskz_shrdv_epi32(__mmask8, __m256i, __m256i, __m256i);VPSHRDVD __m512i _mm512_shrdv_epi32(__m512i, __m512i, __m512i);VPSHRDVD __m512i _mm512_mask_shrdv_epi32(__m512i, __mmask16, __m512i, __m512i);VPSHRDVD __m512i _mm512_maskz_shrdv_epi32(__mmask16, __m512i, __m512i, __m512i);VPSHRDVW __m128i _mm_shrdv_epi16(__m128i, __m128i, __m128i);VPSHRDVW __m128i _mm_mask_shrdv_epi16(__m128i, __mmask8, __m128i, __m128i);VPSHRDVW __m128i _mm_maskz_shrdv_epi16(__mmask8, __m128i, __m128i, __m128i);VPSHRDVW __m256i _mm256_shrdv_epi16(__m256i, __m256i, __m256i);VPSHRDVW __m256i _mm256_mask_shrdv_epi16(__m256i, __mmask16, __m256i, __m256i);VPSHRDVW __m256i _mm256_maskz_shrdv_epi16(__mmask16, __m256i, __m256i, __m256i);VPSHRDVW __m512i _mm512_shrdv_epi16(__m512i, __m512i, __m512i);VPSHRDVW __m512i _mm512_mask_shrdv_epi16(__m512i, __mmask32, __m512i, __m512i);VPSHRDVW __m512i _mm512_maskz_shrdv_epi16(__mmask32, __m512i, __m512i, __m512i);
```
