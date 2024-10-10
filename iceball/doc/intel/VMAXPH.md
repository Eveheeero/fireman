# VMAXPH

Return Maximum of Packed FP16 Values

This instruction performs a SIMD compare of the packed FP16 values in the first source operand and the second source operand and returns the maximum value for each pair of values to the destination operand.If the values being compared are both 0.0s (of either sign), the value in the second operand (source operand) is returned.
If a value in the second operand is an SNaN, then SNaN is forwarded unchanged to the destination (that is, a QNaN version of the SNaN is not returned).If only one value is a NaN (SNaN or QNaN) for this instruction, the second operand (source operand), either a NaN or a valid floating-point value, is written to the result.
If instead of this behavior, it is required that the NaN source operand (from either the first or second operand) be returned, the action of VMAXPH can be emulated using a sequence of instructions, such as, a comparison followed by AND, ANDN and OR.EVEX encoded versions: The first source operand (the second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcast from a 16-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Invalid, Denormal.

## Operation

```C
def MAX(SRC1, SRC2):IF (SRC1 = 0.0) and (SRC2 = 0.0):DEST := SRC2ELSE IF (SRC1 = NaN):DEST := SRC2ELSE IF (SRC2 = NaN):DEST := SRC2ELSE IF (SRC1 > SRC2):DEST := SRC1VMAXPH dest, src1, src2VL = 128, 256 or 512KL := VL/16FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF EVEX.b = 1:tsrc2 := SRC2.fp16[0]ELSE:tsrc2 := SRC2.fp16[j]DEST.fp16[j] := MAX(SRC1.fp16[j], tsrc2)ELSE IF *zeroing*:DEST.fp16[j] := 0// else dest.fp16[j] remains unchangedDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVMAXPH __m128h _mm_mask_max_ph (__m128h src, __mmask8 k, __m128h a, __m128h b);VMAXPH __m128h _mm_maskz_max_ph (__mmask8 k, __m128h a, __m128h b);VMAXPH __m128h _mm_max_ph (__m128h a, __m128h b);VMAXPH __m256h _mm256_mask_max_ph (__m256h src, __mmask16 k, __m256h a, __m256h b);VMAXPH __m256h _mm256_maskz_max_ph (__mmask16 k, __m256h a, __m256h b);VMAXPH __m256h _mm256_max_ph (__m256h a, __m256h b);VMAXPH __m512h _mm512_mask_max_ph (__m512h src, __mmask32 k, __m512h a, __m512h b);VMAXPH __m512h _mm512_maskz_max_ph (__mmask32 k, __m512h a, __m512h b);VMAXPH __m512h _mm512_max_ph (__m512h a, __m512h b);VMAXPH __m512h _mm512_mask_max_round_ph (__m512h src, __mmask32 k, __m512h a, __m512h b, int sae);VMAXPH __m512h _mm512_maskz_max_round_ph (__mmask32 k, __m512h a, __m512h b, int sae);VMAXPH __m512h _mm512_max_round_ph (__m512h a, __m512h b, int sae);
```
