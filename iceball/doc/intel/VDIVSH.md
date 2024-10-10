# VDIVSH

Divide Scalar FP16 Values

This instruction divides the low FP16 value from the first source operand by the corresponding value in the second source operand, storing the FP16 result in the destination operand.
Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal, Zero.
- Other Exceptions

## Operation

```C
VDIVSH (EVEX Encoded Versions)IF EVEX.b = 1 and SRC2 is a register:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:DEST.fp16[0] := SRC1.fp16[0] / SRC2.fp16[0]ELSE IF *zeroing*:DEST.fp16[0] := 0// else dest.fp16[0] remains unchangedDEST[127:16] := SRC1[127:16]DEST[MAXVL-1:128] := 0 Intel C/C++ Compiler Intrinsic EquivalentVDIVSH __m128h _mm_div_round_sh (__m128h a, __m128h b, int rounding);VDIVSH __m128h _mm_mask_div_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int rounding);VDIVSH __m128h _mm_maskz_div_round_sh (__mmask8 k, __m128h a, __m128h b, int rounding);VDIVSH __m128h _mm_div_sh (__m128h a, __m128h b);VDIVSH __m128h _mm_mask_div_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VDIVSH __m128h _mm_maskz_div_sh (__mmask8 k, __m128h a, __m128h b);
```
