# VCVTSS2SH

Convert Low FP32 Value to an FP16 Value

This instruction converts the low FP32 value in the second source operand to a FP16 value in the low element of the destination operand.When the conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal.
- Other Exceptions

## Operation

```C
VCVTSS2SH dest, src1, src2IF *SRC2 is a register* and (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:DEST.fp16[0] := Convert_fp32_to_fp16(SRC2.fp32[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else dest.fp16[0] remains unchangedDEST[127:16] := SRC1[127:16]DEST[MAXVL-1:128] := 0 Intel C/C++ Compiler Intrinsic EquivalentVCVTSS2SH __m128h _mm_cvt_roundss_sh (__m128h a, __m128 b, const int rounding);VCVTSS2SH __m128h _mm_mask_cvt_roundss_sh (__m128h src, __mmask8 k, __m128h a, __m128 b, const int rounding);VCVTSS2SH __m128h _mm_maskz_cvt_roundss_sh (__mmask8 k, __m128h a, __m128 b, const int rounding);VCVTSS2SH __m128h _mm_cvtss_sh (__m128h a, __m128 b);VCVTSS2SH __m128h _mm_mask_cvtss_sh (__m128h src, __mmask8 k, __m128h a, __m128 b);VCVTSS2SH __m128h _mm_maskz_cvtss_sh (__mmask8 k, __m128h a, __m128 b);
```
