# VCVTUW2PH

Convert Packed Unsigned Word Integers to FP16 Values

This instruction converts packed unsigned word integers in the source operand to FP16 values in the destination operand.
When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or embedded rounding controls.The destination elements are updated according to the writemask.If the result of the convert operation is overflow and MXCSR.OM=0 then a SIMD exception will be raised with OE=1, PE=1.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Overflow, Precision.

## Operation

```C
VCVTUW2PH dest, srcVL = 128, 256 or 512KL := VL / 16IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.word[0]ELSEtsrc := SRC.word[j]DEST.fp16[j] := Convert_unsignd_integer16_to_fp16(tsrc)ELSE IF *zeroing*:DEST.fp16[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTUW2PH __m512h _mm512_cvt_roundepu16_ph (__m512i a, int rounding);VCVTUW2PH __m512h _mm512_mask_cvt_roundepu16_ph (__m512h src, __mmask32 k, __m512i a, int rounding);VCVTUW2PH __m512h _mm512_maskz_cvt_roundepu16_ph (__mmask32 k, __m512i a, int rounding);VCVTUW2PH __m128h _mm_cvtepu16_ph (__m128i a);VCVTUW2PH __m128h _mm_mask_cvtepu16_ph (__m128h src, __mmask8 k, __m128i a);VCVTUW2PH __m128h _mm_maskz_cvtepu16_ph (__mmask8 k, __m128i a);VCVTUW2PH __m256h _mm256_cvtepu16_ph (__m256i a);VCVTUW2PH __m256h _mm256_mask_cvtepu16_ph (__m256h src, __mmask16 k, __m256i a);VCVTUW2PH __m256h _mm256_maskz_cvtepu16_ph (__mmask16 k, __m256i a);VCVTUW2PH __m512h _mm512_cvtepu16_ph (__m512i a);VCVTUW2PH __m512h _mm512_mask_cvtepu16_ph (__m512h src, __mmask32 k, __m512i a);VCVTUW2PH __m512h _mm512_maskz_cvtepu16_ph (__mmask32 k, __m512i a);
```
