# VCVTPH2UW

Convert Packed FP16 Values to Unsigned Word Integers

This instruction converts packed FP16 values in the source operand to unsigned word integers in the destination operand.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value is returned.The destination elements are updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTPH2UW DEST, SRC VL = 128, 256 or 512KL := VL / 16IF *SRC is a register* and (VL = 512) and (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.fp16[0]ELSEtsrc := SRC.fp16[j]DEST.word[j] := Convert_fp16_to_unsigned_integer16(tsrc)ELSE IF *zeroing*:DEST.word[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTPH2UW __m512i _mm512_cvt_roundph_epu16 (__m512h a, int sae);VCVTPH2UW __m512i _mm512_mask_cvt_roundph_epu16 (__m512i src, __mmask32 k, __m512h a, int sae);VCVTPH2UW __m512i _mm512_maskz_cvt_roundph_epu16 (__mmask32 k, __m512h a, int sae);VCVTPH2UW __m128i _mm_cvtph_epu16 (__m128h a);VCVTPH2UW __m128i _mm_mask_cvtph_epu16 (__m128i src, __mmask8 k, __m128h a);VCVTPH2UW __m128i _mm_maskz_cvtph_epu16 (__mmask8 k, __m128h a);VCVTPH2UW __m256i _mm256_cvtph_epu16 (__m256h a);VCVTPH2UW __m256i _mm256_mask_cvtph_epu16 (__m256i src, __mmask16 k, __m256h a);VCVTPH2UW __m256i _mm256_maskz_cvtph_epu16 (__mmask16 k, __m256h a);VCVTPH2UW __m512i _mm512_cvtph_epu16 (__m512h a);VCVTPH2UW __m512i _mm512_mask_cvtph_epu16 (__m512i src, __mmask32 k, __m512h a);VCVTPH2UW __m512i _mm512_maskz_cvtph_epu16 (__mmask32 k, __m512h a);
```
