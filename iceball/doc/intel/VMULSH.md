# VMULSH

Multiply Scalar FP16 Values

This instruction multiplies the low FP16 value from the source operands and stores the FP16 result in the destina-tion operand.
Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal
- Other Exceptions

## Operation

```C
VMULSH (EVEX encoded versions)IF EVEX.b = 1 and SRC2 is a register:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:DEST.fp16[0] := SRC1.fp16[0] * SRC2.fp16[0]ELSE IF *zeroing*:DEST.fp16[0] := 0// else dest.fp16[0] remains unchangedDEST[127:16] := SRC1[127:16]DEST[MAXVL-1:VL] := 0 Intel C/C++ Compiler Intrinsic EquivalentVMULSH __m128h _mm_mask_mul_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int rounding);VMULSH __m128h _mm_maskz_mul_round_sh (__mmask8 k, __m128h a, __m128h b, int rounding);VMULSH __m128h _mm_mul_round_sh (__m128h a, __m128h b, int rounding);VMULSH __m128h _mm_mask_mul_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VMULSH __m128h _mm_maskz_mul_sh (__mmask8 k, __m128h a, __m128h b);VMULSH __m128h _mm_mul_sh (__m128h a, __m128h b);
```
