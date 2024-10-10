# VCVTTPH2UW

Convert Packed FP16 Values to Unsigned Word Integers

This instruction converts packed FP16 values in the source operand to unsigned word integers in the destination operand.When a conversion is inexact, a truncated (round toward zero) value is returned.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the integer indefinite value is returned.The destination elements are updated according to the writemask.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTTPH2UW dest, srcVL = 128, 256 or 512KL := VL / 16FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.fp16[0]ELSEtsrc := SRC.fp16[j]DEST.word[j] := Convert_fp16_to_unsigned_integer16_truncate(tsrc)ELSE IF *zeroing*:DEST.word[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTTPH2UW __m512i _mm512_cvtt_roundph_epu16 (__m512h a, int sae);VCVTTPH2UW __m512i _mm512_mask_cvtt_roundph_epu16 (__m512i src, __mmask32 k, __m512h a, int sae);VCVTTPH2UW __m512i _mm512_maskz_cvtt_roundph_epu16 (__mmask32 k, __m512h a, int sae);VCVTTPH2UW __m128i _mm_cvttph_epu16 (__m128h a);VCVTTPH2UW __m128i _mm_mask_cvttph_epu16 (__m128i src, __mmask8 k, __m128h a);VCVTTPH2UW __m128i _mm_maskz_cvttph_epu16 (__mmask8 k, __m128h a);VCVTTPH2UW __m256i _mm256_cvttph_epu16 (__m256h a);VCVTTPH2UW __m256i _mm256_mask_cvttph_epu16 (__m256i src, __mmask16 k, __m256h a);VCVTTPH2UW __m256i _mm256_maskz_cvttph_epu16 (__mmask16 k, __m256h a);VCVTTPH2UW __m512i _mm512_cvttph_epu16 (__m512h a);VCVTTPH2UW __m512i _mm512_mask_cvttph_epu16 (__m512i src, __mmask32 k, __m512h a);VCVTTPH2UW __m512i _mm512_maskz_cvttph_epu16 (__mmask32 k, __m512h a);
```
