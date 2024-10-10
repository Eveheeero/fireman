# VCVTPH2PD

Convert Packed FP16 Values to FP64 Values

This instruction converts packed FP16 values to FP64 values in the destination register.
The destination elements are updated according to the writemask.This instruction handles both normal and denormal FP16 inputs.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.

## Operation

```C
VCVTPH2PD DEST, SRC VL = 128, 256, or 512KL := VL/64FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.fp16[0]ELSEtsrc := SRC.fp16[j]DEST.fp64[j] := Convert_fp16_to_fp64(tsrc)ELSE IF *zeroing*:DEST.fp64[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTPH2PD __m512d _mm512_cvt_roundph_pd (__m128h a, int sae);VCVTPH2PD __m512d _mm512_mask_cvt_roundph_pd (__m512d src, __mmask8 k, __m128h a, int sae);VCVTPH2PD __m512d _mm512_maskz_cvt_roundph_pd (__mmask8 k, __m128h a, int sae);VCVTPH2PD __m128d _mm_cvtph_pd (__m128h a);VCVTPH2PD __m128d _mm_mask_cvtph_pd (__m128d src, __mmask8 k, __m128h a);VCVTPH2PD __m128d _mm_maskz_cvtph_pd (__mmask8 k, __m128h a);VCVTPH2PD __m256d _mm256_cvtph_pd (__m128h a);VCVTPH2PD __m256d _mm256_mask_cvtph_pd (__m256d src, __mmask8 k, __m128h a);VCVTPH2PD __m256d _mm256_maskz_cvtph_pd (__mmask8 k, __m128h a);VCVTPH2PD __m512d _mm512_cvtph_pd (__m128h a);VCVTPH2PD __m512d _mm512_mask_cvtph_pd (__m512d src, __mmask8 k, __m128h a);VCVTPH2PD __m512d _mm512_maskz_cvtph_pd (__mmask8 k, __m128h a);
```
