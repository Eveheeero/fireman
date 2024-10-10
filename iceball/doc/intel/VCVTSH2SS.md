# VCVTSH2SS

Convert Low FP16 Value to FP32 Value

This instruction converts the low FP16 element in the second source operand to the low FP32 element of the desti-nation operand.Bits 127:32 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.
- Other Exceptions

## Operation

```C
VCVTSH2SS dest, src1, src2 IF k1[0] OR *no writemask*:DEST.fp32[0] := Convert_fp16_to_fp32(SRC2.fp16[0])ELSE IF *zeroing*:DEST.fp32[0] := 0// else dest.fp32[0] remains unchangedDEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0 Intel C/C++ Compiler Intrinsic EquivalentVCVTSH2SS __m128 _mm_cvt_roundsh_ss (__m128 a, __m128h b, const int sae);VCVTSH2SS __m128 _mm_mask_cvt_roundsh_ss (__m128 src, __mmask8 k, __m128 a, __m128h b, const int sae);VCVTSH2SS __m128 _mm_maskz_cvt_roundsh_ss (__mmask8 k, __m128 a, __m128h b, const int sae);VCVTSH2SS __m128 _mm_cvtsh_ss (__m128 a, __m128h b);VCVTSH2SS __m128 _mm_mask_cvtsh_ss (__m128 src, __mmask8 k, __m128 a, __m128h b);VCVTSH2SS __m128 _mm_maskz_cvtsh_ss (__mmask8 k, __m128 a, __m128h b);
```
