# VSQRTSH

Compute Square Root of Scalar FP16 Value

This instruction performs a scalar FP16 square-root computation on the source operand and stores the FP16 result in the destination operand.
Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision, Denormal

## Operation

```C
VSQRTSH dest{k1}, src1, src2IF k1[0] or *no writemask*:DEST.fp16[0] := SQRT(src2.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0//else DEST.fp16[0] remains unchangedDEST[127:16] := src1[127:16]DEST[MAXVL-1:128] := 0 Intel C/C++ Compiler Intrinsic EquivalentVSQRTSH __m128h _mm_mask_sqrt_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, const int rounding);VSQRTSH __m128h _mm_maskz_sqrt_round_sh (__mmask8 k, __m128h a, __m128h b, const int rounding);VSQRTSH __m128h _mm_sqrt_round_sh (__m128h a, __m128h b, const int rounding);VSQRTSH __m128h _mm_mask_sqrt_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VSQRTSH __m128h _mm_maskz_sqrt_sh (__mmask8 k, __m128h a, __m128h b);VSQRTSH __m128h _mm_sqrt_sh (__m128h a, __m128h b);
```
