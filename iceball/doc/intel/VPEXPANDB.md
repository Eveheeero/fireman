# VPEXPANDB/VPEXPANDW

Expand Byte/Word Values

Expands (loads) up to 64 byte integer values or 32 word integer values from the source operand (memory operand) to the destination operand (register operand), based on the active elements determined by the writemask operand.Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.Moves 128, 256 or 512 bits of packed byte integer values from the source operand (memory operand) to the desti-nation operand (register operand).
This instruction is used to load from an int8 vector register or memory location while inserting the data into sparse elements of destination vector register using the active elements pointed out by the operand writemask.This instruction supports memory fault suppression.Note that the compressed displacement assumes a pre-scaling

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPEXPANDB(KL, VL) = (16, 128), (32, 256), (64, 512)k := 0FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:DEST.byte[j] := SRC.byte[k];k := k + 1ELSE:IF *merging-masking*:*DEST.byte[j] remains unchanged*ELSE: ; zeroing-maskingDEST.byte[j] := 0DEST[MAX_VL-1:VL] := 0VPEXPANDW(KL, VL) = (8,128), (16,256), (32, 512)k := 0FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:DEST.word[j] := SRC.word[k];k := k + 1ELSE:IF *merging-masking*:*DEST.word[j] remains unchanged*ELSE: ; zeroing-maskingDEST.word[j] := 0DEST[MAX_VL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPEXPAND __m128i _mm_mask_expand_epi8(__m128i, __mmask16, __m128i);VPEXPAND __m128i _mm_maskz_expand_epi8(__mmask16, __m128i);VPEXPAND __m128i _mm_mask_expandloadu_epi8(__m128i, __mmask16, const void*);VPEXPAND __m128i _mm_maskz_expandloadu_epi8(__mmask16, const void*);VPEXPAND __m256i _mm256_mask_expand_epi8(__m256i, __mmask32, __m256i);VPEXPAND __m256i _mm256_maskz_expand_epi8(__mmask32, __m256i);VPEXPAND __m256i _mm256_mask_expandloadu_epi8(__m256i, __mmask32, const void*);VPEXPAND __m256i _mm256_maskz_expandloadu_epi8(__mmask32, const void*);VPEXPAND __m512i _mm512_mask_expand_epi8(__m512i, __mmask64, __m512i);VPEXPAND __m512i _mm512_maskz_expand_epi8(__mmask64, __m512i);VPEXPAND __m512i _mm512_mask_expandloadu_epi8(__m512i, __mmask64, const void*);VPEXPAND __m512i _mm512_maskz_expandloadu_epi8(__mmask64, const void*);VPEXPANDW __m128i _mm_mask_expand_epi16(__m128i, __mmask8, __m128i);VPEXPANDW __m128i _mm_maskz_expand_epi16(__mmask8, __m128i);VPEXPANDW __m128i _mm_mask_expandloadu_epi16(__m128i, __mmask8, const void*);VPEXPANDW __m128i _mm_maskz_expandloadu_epi16(__mmask8, const void *);VPEXPANDW __m256i _mm256_mask_expand_epi16(__m256i, __mmask16, __m256i);VPEXPANDW __m256i _mm256_maskz_expand_epi16(__mmask16, __m256i);VPEXPANDW __m256i _mm256_mask_expandloadu_epi16(__m256i, __mmask16, const void*);VPEXPANDW __m256i _mm256_maskz_expandloadu_epi16(__mmask16, const void*);VPEXPANDW __m512i _mm512_mask_expand_epi16(__m512i, __mmask32, __m512i);VPEXPANDW __m512i _mm512_maskz_expand_epi16(__mmask32, __m512i);VPEXPANDW __m512i _mm512_mask_expandloadu_epi16
```
