# VCVTW2PH

Convert Packed Signed Word Integers to FP16 Values

This instruction converts packed signed word integers in the source operand to FP16 values in the destination operand.
When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or embedded rounding controls.The destination elements are updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Precision.
- Other Exceptions

## Operation

```C
VCVTW2PH dest, srcVL = 128, 256 or 512KL := VL / 16IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.word[0]ELSEtsrc := SRC.word[j]DEST.fp16[j] := Convert_integer16_to_fp16(tsrc)ELSE IF *zeroing*:DEST.fp16[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTW2PH __m512h _mm512_cvt_roundepi16_ph (__m512i a, int rounding);VCVTW2PH __m512h _mm512_mask_cvt_roundepi16_ph (__m512h src, __mmask32 k, __m512i a, int rounding);VCVTW2PH __m512h _mm512_maskz_cvt_roundepi16_ph (__mmask32 k, __m512i a, int rounding);VCVTW2PH __m128h _mm_cvtepi16_ph (__m128i a);VCVTW2PH __m128h _mm_mask_cvtepi16_ph (__m128h src, __mmask8 k, __m128i a);VCVTW2PH __m128h _mm_maskz_cvtepi16_ph (__mmask8 k, __m128i a);VCVTW2PH __m256h _mm256_cvtepi16_ph (__m256i a);VCVTW2PH __m256h _mm256_mask_cvtepi16_ph (__m256h src, __mmask16 k, __m256i a);VCVTW2PH __m256h _mm256_maskz_cvtepi16_ph (__mmask16 k, __m256i a);VCVTW2PH __m512h _mm512_cvtepi16_ph (__m512i a);VCVTW2PH __m512h _mm512_mask_cvtepi16_ph (__m512h src, __mmask32 k, __m512i a);VCVTW2PH __m512h _mm512_maskz_cvtepi16_ph (__mmask32 k, __m512i a);
```
