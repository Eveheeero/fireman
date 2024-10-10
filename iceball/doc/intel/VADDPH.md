# VADDPH

Add Packed FP16 Values

This instruction adds packed FP16 values from source operands and stores the packed FP16 result in the destina-tion operand.
The destination elements are updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal.

## Operation

```C
VADDPH (EVEX Encoded Versions) When SRC2 Operand is a Register VL = 128, 256 or 512KL := VL/16IF (VL = 512) AND (EVEX.b = 1): SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:DEST.fp16[j] := SRC1.fp16[j] + SRC2.fp16[j]ELSEIF *zeroing*:DEST.fp16[j] := 0// else dest.fp16[j] remains unchangedDEST[MAXVL-1:VL] := 0VADDPH (EVEX Encoded Versions) When SRC2 Operand is a Memory SourceVL = 128, 256 or 512KL := VL/16FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF EVEX.b = 1:DEST.fp16[j] := SRC1.fp16[j] + SRC2.fp16[0]ELSE:DEST.fp16[j] := SRC1.fp16[j] + SRC2.fp16[j]ELSE IF *zeroing*:DEST.fp16[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVADDPH __m128h _mm_add_ph (__m128h a, __m128h b);VADDPH __m128h _mm_mask_add_ph (__m128h src, __mmask8 k, __m128h a, __m128h b);VADDPH __m128h _mm_maskz_add_ph (__mmask8 k, __m128h a, __m128h b);VADDPH __m256h _mm256_add_ph (__m256h a, __m256h b);VADDPH __m256h _mm256_mask_add_ph (__m256h src, __mmask16 k, __m256h a, __m256h b);VADDPH __m256h _mm256_maskz_add_ph (__mmask16 k, __m256h a, __m256h b);VADDPH __m512h _mm512_add_ph (__m512h a, __m512h b);VADDPH __m512h _mm512_add_ph (__m512h a, __m512h b);VADDPH __m512h _mm512_mask_add_ph (__m512h src, __mmask32 k, __m512h a, __m512h b);VADDPH __m512h _mm512_maskz_add_ph (__mmask32 k, __m512h a, __m512h b);VADDPH __m512h _mm512_add_round_ph (__m512h a, __m512h b, int rounding);VADDPH __m512h _mm512_mask_add_round_ph (__m512h src, __mmask32 k, __m512h a, __m512h b, int rounding);VADDPH __m512h _mm512_maskz_add_round_ph (__mmask32 k, __m512h a, __m512h b, int rounding);
```
