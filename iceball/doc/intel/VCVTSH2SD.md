# VCVTSH2SD

Convert Low FP16 Value to an FP64 Value

This instruction converts the low FP16 element in the second source operand to a FP64 element in the low element of the destination operand.Bits 127:64 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP64 element of the destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.
- Other Exceptions

## Operation

```C
VCVTSH2SD dest, src1, src2 IF k1[0] OR *no writemask*:DEST.fp64[0] := Convert_fp16_to_fp64(SRC2.fp16[0])ELSE IF *zeroing*:DEST.fp64[0] := 0// else dest.fp64[0] remains unchangedDEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0 Intel C/C++ Compiler Intrinsic EquivalentVCVTSH2SD __m128d _mm_cvt_roundsh_sd (__m128d a, __m128h b, const int sae);VCVTSH2SD __m128d _mm_mask_cvt_roundsh_sd (__m128d src, __mmask8 k, __m128d a, __m128h b, const int sae);VCVTSH2SD __m128d _mm_maskz_cvt_roundsh_sd (__mmask8 k, __m128d a, __m128h b, const int sae);VCVTSH2SD __m128d _mm_cvtsh_sd (__m128d a, __m128h b);VCVTSH2SD __m128d _mm_mask_cvtsh_sd (__m128d src, __mmask8 k, __m128d a, __m128h b);VCVTSH2SD __m128d _mm_maskz_cvtsh_sd (__mmask8 k, __m128d a, __m128h b);
```
