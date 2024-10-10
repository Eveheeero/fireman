# VADDSH

Add Scalar FP16 Values

This instruction adds the low FP16 value from the source operands and stores the FP16 result in the destination operand.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal.

## Operation

```C
VADDSH (EVEX Encoded Versions) IF EVEX.b = 1 and SRC2 is a register: SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:DEST.fp16[0] := SRC1.fp16[0] + SRC2.fp16[0]ELSEIF *zeroing*:DEST.fp16[0] := 0// else dest.fp16[0] remains unchangedDEST[127:16] := SRC1[127:16] DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVADDSH __m128h _mm_add_round_sh (__m128h a, __m128h b, int rounding);VADDSH ___m128h _mm_mask_add_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int rounding);VADDSH ___m128h _mm_maskz_add_round_sh (__mmask8 k, __m128h a, __m128h b, int rounding);VADDSH ___m128h _mm_add_sh (__m128h a, __m128h b);VADDSH ___m128h _mm_mask_add_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VADDSH ___m128h _mm_maskz_add_sh (__mmask8 k, __m128h a, __m128h b);
```
