# VRCPPH

Compute Reciprocals of Packed FP16 Values

This instruction performs a SIMD computation of the approximate reciprocals of 8/16/32 packed FP16 values in the source operand (the second operand) and stores the packed FP16 results in the destination operand.
The maximum  í11 í14 + 2.relative error for this approximation is less than 2For special cases, see Table 5-18.Table 5-18.
VRCPPH/VRCPSH Special CasesInput ValueResult ValueComments0  " X  " 2-16INFVery small denormal-16 «2 " X  " -0«INFVery small denormalX > +»+0X < «»«0-nnX = 22-nnX = «2«2

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VRCPPH dest{k1}, src VL = 128, 256 or 512KL := VL/16FOR i := 0 to KL-1:IF k1[i] or *no writemask*:IF SRC is memory and (EVEX.b = 1):tsrc := src.fp16[0]ELSE:tsrc := src.fp16[i]DEST.fp16[i] := APPROXIMATE(1.0 / tsrc)ELSE IF *zeroing*:DEST.fp16[i] := 0Intel C/C++ Compiler Intrinsic EquivalentVRCPPH __m128h _mm_mask_rcp_ph (__m128h src, __mmask8 k, __m128h a);VRCPPH __m128h _mm_maskz_rcp_ph (__mmask8 k, __m128h a);VRCPPH __m128h _mm_rcp_ph (__m128h a);VRCPPH __m256h _mm256_mask_rcp_ph (__m256h src, __mmask16 k, __m256h a);VRCPPH __m256h _mm256_maskz_rcp_ph (__mmask16 k, __m256h a);VRCPPH __m256h _mm256_rcp_ph (__m256h a);VRCPPH __m512h _mm512_mask_rcp_ph (__m512h src, __mmask32 k, __m512h a);VRCPPH __m512h _mm512_maskz_rcp_ph (__mmask32 k, __m512h a);VRCPPH __m512h _mm512_rcp_ph (__m512h a);
```
