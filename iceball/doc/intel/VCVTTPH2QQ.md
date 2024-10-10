# VCVTTPH2QQ

Convert with Truncation Packed FP16 Values to Signed Quadword Integers

This instruction converts packed FP16 values in the source operand to signed quadword integers in the destination operand.When a conversion is inexact, a truncated (round toward zero) value is returned.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value is returned.The destination elements are updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions

## Operation

```C
VCVTTPH2QQ dest, srcVL = 128, 256 or 512KL := VL / 64FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.fp16[0]ELSEtsrc := SRC.fp16[j]DEST.qword[j] := Convert_fp16_to_integer64_truncate(tsrc)ELSE IF *zeroing*:DEST.qword[j] := 0// else dest.qword[jIntel C/C++ Compiler Intrinsic EquivalentVCVTTPH2QQ __m512i _mm512_cvtt_roundph_epi64 (__m128h a, int sae);VCVTTPH2QQ __m512i _mm512_mask_cvtt_roundph_epi64 (__m512i src, __mmask8 k, __m128h a, int sae);VCVTTPH2QQ __m512i _mm512_maskz_cvtt_roundph_epi64 (__mmask8 k, __m128h a, int sae);VCVTTPH2QQ __m128i _mm_cvttph_epi64 (__m128h a);VCVTTPH2QQ __m128i _mm_mask_cvttph_epi64 (__m128i src, __mmask8 k, __m128h a);VCVTTPH2QQ __m128i _mm_maskz_cvttph_epi64 (__mmask8 k, __m128h a);VCVTTPH2QQ __m256i _mm256_cvttph_epi64 (__m128h a);VCVTTPH2QQ __m256i _mm256_mask_cvttph_epi64 (__m256i src, __mmask8 k, __m128h a);VCVTTPH2QQ __m256i _mm256_maskz_cvttph_epi64 (__mmask8 k, __m128h a);VCVTTPH2QQ __m512i _mm512_cvttph_epi64 (__m128h a);VCVTTPH2QQ __m512i _mm512_mask_cvttph_epi64 (__m512i src, __mmask8 k, __m128h a);VCVTTPH2QQ __m512i _mm512_maskz_cvttph_epi64 (__mmask8 k, __m128h a);
```
