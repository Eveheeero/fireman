# VCVTPH2UQQ

Convert Packed FP16 Values to Unsigned Quadword Integers

This instruction converts packed FP16 values in the source operand to unsigned quadword integers in destination operand.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value is returned.The destination elements are updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTPH2UQQ DEST, SRC VL = 128, 256 or 512KL := VL / 64IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.fp16[0]ELSEtsrc := SRC.fp16[j]DEST.qword[j] := Convert_fp16_to_unsigned_integer64(tsrc)ELSE IF *zeroing*:DEST.qword[j] := 0// else dest.qword[jIntel C/C++ Compiler Intrinsic EquivalentVCVTPH2UQQ __m512i _mm512_cvt_roundph_epu64 (__m128h a, int rounding);VCVTPH2UQQ __m512i _mm512_mask_cvt_roundph_epu64 (__m512i src, __mmask8 k, __m128h a, int rounding);VCVTPH2UQQ __m512i _mm512_maskz_cvt_roundph_epu64 (__mmask8 k, __m128h a, int rounding);VCVTPH2UQQ __m128i _mm_cvtph_epu64 (__m128h a);VCVTPH2UQQ __m128i _mm_mask_cvtph_epu64 (__m128i src, __mmask8 k, __m128h a);VCVTPH2UQQ __m128i _mm_maskz_cvtph_epu64 (__mmask8 k, __m128h a);VCVTPH2UQQ __m256i _mm256_cvtph_epu64 (__m128h a);VCVTPH2UQQ __m256i _mm256_mask_cvtph_epu64 (__m256i src, __mmask8 k, __m128h a);VCVTPH2UQQ __m256i _mm256_maskz_cvtph_epu64 (__mmask8 k, __m128h a);VCVTPH2UQQ __m512i _mm512_cvtph_epu64 (__m128h a);VCVTPH2UQQ __m512i _mm512_mask_cvtph_epu64 (__m512i src, __mmask8 k, __m128h a);VCVTPH2UQQ __m512i _mm512_maskz_cvtph_epu64 (__mmask8 k, __m128h a);
```
