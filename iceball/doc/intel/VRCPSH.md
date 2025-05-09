# VRCPSH

Compute Reciprocal of Scalar FP16 Value

This instruction performs a SIMD computation of the approximate reciprocal of the low FP16 value in the second source operand (the third operand) and stores the result in the low word element of the destination operand (the first operand) according to the writemask k1.
Bits 127:16 of the XMM register destination are copied from corre-sponding bits in the first source operand (the second operand).
The maximum relative error for this approximation  í11 í14 + 2.is less than 2Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.For special cases, see Table 5-18.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VRCPSH dest{k1}, src1, src2 IF k1[0] or *no writemask*:DEST.fp16[0] := APPROXIMATE(1.0 / src2.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0//else DEST.fp16[0] remains unchangedDEST[127:16] := src1[127:16]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVRCPSH __m128h _mm_mask_rcp_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VRCPSH __m128h _mm_maskz_rcp_sh (__mmask8 k, __m128h a, __m128h b);VRCPSH __m128h _mm_rcp_sh (__m128h a, __m128h b);
```
