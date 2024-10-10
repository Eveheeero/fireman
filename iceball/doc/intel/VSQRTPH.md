# VSQRTPH

Compute Square Root of Packed FP16 Values

This instruction performs a packed FP16 square-root computation on the values from source operand and stores the packed FP16 result in the destination operand.
The destination elements are updated according to the write-mask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision, Denormal.

## Operation

```C
VSQRTPH dest{k1}, srcVL = 128, 256 or 512KL := VL/16FOR i := 0 to KL-1:IF k1[i] or *no writemask*:IF SRC is memory and (EVEX.b = 1):tsrc := src.fp16[0]ELSE:tsrc := src.fp16[i]DEST.fp16[i] := SQRT(tsrc)ELSE IF *zeroing*:DEST.fp16[i] := 0Intel C/C++ Compiler Intrinsic EquivalentVSQRTPH __m128h _mm_mask_sqrt_ph (__m128h src, __mmask8 k, __m128h a);VSQRTPH __m128h _mm_maskz_sqrt_ph (__mmask8 k, __m128h a);VSQRTPH __m128h _mm_sqrt_ph (__m128h a);VSQRTPH __m256h _mm256_mask_sqrt_ph (__m256h src, __mmask16 k, __m256h a);VSQRTPH __m256h _mm256_maskz_sqrt_ph (__mmask16 k, __m256h a);VSQRTPH __m256h _mm256_sqrt_ph (__m256h a);VSQRTPH __m512h _mm512_mask_sqrt_ph (__m512h src, __mmask32 k, __m512h a);VSQRTPH __m512h _mm512_maskz_sqrt_ph (__mmask32 k, __m512h a);VSQRTPH __m512h _mm512_sqrt_ph (__m512h a);VSQRTPH __m512h _mm512_mask_sqrt_round_ph (__m512h src, __mmask32 k, __m512h a, const int rounding);VSQRTPH __m512h _mm512_maskz_sqrt_round_ph (__mmask32 k, __m512h a, const int rounding);VSQRTPH __m512h _mm512_sqrt_round_ph (__m512h a, const int rounding);
```
