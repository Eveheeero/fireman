# VCVTPH2DQ

Convert Packed FP16 Values to Signed Doubleword Integers

This instruction converts packed FP16 values in the source operand to signed doubleword integers in destination operand.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value is returned.The destination elements are updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTPH2DQ DEST, SRC VL = 128, 256 or 512KL := VL / 32IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.fp16[0]ELSEtsrc := SRC.fp16[j]DEST.dword[j] := Convert_fp16_to_integer32(tsrc)ELSE IF *zeroing*:DEST.dword[j] := 0// else dest.dword[jIntel C/C++ Compiler Intrinsic EquivalentVCVTPH2DQ __m512i _mm512_cvt_roundph_epi32 (__m256h a, int rounding);VCVTPH2DQ __m512i _mm512_mask_cvt_roundph_epi32 (__m512i src, __mmask16 k, __m256h a, int rounding);VCVTPH2DQ __m512i _mm512_maskz_cvt_roundph_epi32 (__mmask16 k, __m256h a, int rounding);VCVTPH2DQ __m128i _mm_cvtph_epi32 (__m128h a);VCVTPH2DQ __m128i _mm_mask_cvtph_epi32 (__m128i src, __mmask8 k, __m128h a);VCVTPH2DQ __m128i _mm_maskz_cvtph_epi32 (__mmask8 k, __m128h a);VCVTPH2DQ __m256i _mm256_cvtph_epi32 (__m128h a);VCVTPH2DQ __m256i _mm256_mask_cvtph_epi32 (__m256i src, __mmask8 k, __m128h a);VCVTPH2DQ __m256i _mm256_maskz_cvtph_epi32 (__mmask8 k, __m128h a);VCVTPH2DQ __m512i _mm512_cvtph_epi32 (__m256h a);VCVTPH2DQ __m512i _mm512_mask_cvtph_epi32 (__m512i src, __mmask16 k, __m256h a);VCVTPH2DQ __m512i _mm512_maskz_cvtph_epi32 (__mmask16 k, __m256h a);
```
