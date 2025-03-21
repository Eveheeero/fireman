# VRSQRTSH

Compute Approximate Reciprocal of Square Root of Scalar FP16 Value

This instruction performs the computation of the approximate reciprocal square-root of the low FP16 value in the second source operand (the third operand) and stores the result in the low word element of the destination operand (the first operand) according to the writemask k1. í11  í14+ 2.The maximum relative error for this approximation is less than 2Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL í1:128 of the destination operand are zeroed.For special cases, see Table 5-28.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VRSQRTSH dest{k1}, src1, src2VL = 128, 256 or 512KL := VL/16IF k1[0] or *no writemask*:DEST.fp16[0] := APPROXIMATE(1.0 / SQRT(src2.fp16[0]))ELSE IF *zeroing*:DEST.fp16[0] := 0//else DEST.fp16[0] remains unchangedDEST[127:16] := src1[127:16]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVRSQRTSH __m128h _mm_mask_rsqrt_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VRSQRTSH __m128h _mm_maskz_rsqrt_sh (__mmask8 k, __m128h a, __m128h b);VRSQRTSH __m128h _mm_rsqrt_sh (__m128h a, __m128h b);
```
