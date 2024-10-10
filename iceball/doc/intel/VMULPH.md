# VMULPH

Multiply Packed FP16 Values

This instruction multiplies packed FP16 values from source operands and stores the packed FP16 result in the desti-nation operand.
The destination elements are updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal

## Operation

```C
VMULPH (EVEX encoded versions) when src2 operand is a registerVL = 128, 256 or 512KL := VL/16IF (VL = 512) AND (EVEX.b = 1):SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:DEST.fp16[j] := SRC1.fp16[j] * SRC2.fp16[j]ELSE IF *zeroing*:DEST.fp16[j] := 0VMULPH (EVEX encoded versions) when src2 operand is a memory sourceVL = 128, 256 or 512KL := VL/16FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF EVEX.b = 1:DEST.fp16[j] := SRC1.fp16[j] * SRC2.fp16[0]ELSE:DEST.fp16[j] := SRC1.fp16[j] * SRC2.fp16[j]ELSE IF *zeroing*:DEST.fp16[j] := 0// else dest.fp16[j] remains unchangedDEST[MAXVL-1:VL] := 0 Intel C/C++ Compiler Intrinsic EquivalentVMULPH __m128h _mm_mask_mul_ph (__m128h src, __mmask8 k, __m128h a, __m128h b);VMULPH __m128h _mm_maskz_mul_ph (__mmask8 k, __m128h a, __m128h b);VMULPH __m128h _mm_mul_ph (__m128h a, __m128h b);VMULPH __m256h _mm256_mask_mul_ph (__m256h src, __mmask16 k, __m256h a, __m256h b);VMULPH __m256h _mm256_maskz_mul_ph (__mmask16 k, __m256h a, __m256h b);VMULPH __m256h _mm256_mul_ph (__m256h a, __m256h b);VMULPH __m512h _mm512_mask_mul_ph (__m512h src, __mmask32 k, __m512h a, __m512h b);VMULPH __m512h _mm512_maskz_mul_ph (__mmask32 k, __m512h a, __m512h b);VMULPH __m512h _mm512_mul_ph (__m512h a, __m512h b);VMULPH __m512h _mm512_mask_mul_round_ph (__m512h src, __mmask32 k, __m512h a, __m512h b, int rounding);VMULPH __m512h _mm512_maskz_mul_round_ph (__mmask32 k, __m512h a, __m512h b, int rounding);VMULPH __m512h _mm512_mul_round_ph (__m512h a, __m512h b, int rounding);
```
