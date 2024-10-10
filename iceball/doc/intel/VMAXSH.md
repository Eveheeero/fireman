# VMAXSH

Return Maximum of Scalar FP16 Values

This instruction performs a compare of the low packed FP16 values in the first source operand and the second source operand and returns the maximum value for the pair of values to the destination operand.If the values being compared are both 0.0s (of either sign), the value in the second operand (source operand) is returned.
If a value in the second operand is an SNaN, then SNaN is forwarded unchanged to the destination (that is, a QNaN version of the SNaN is not returned).If only one value is a NaN (SNaN or QNaN) for this instruction, the second operand (source operand), either a NaN or a valid floating-point value, is written to the result.
If instead of this behavior, it is required that the NaN source operand (from either the first or second operand) be returned, the action of VMAXSH can be emulated using a sequence of instructions, such as, a comparison followed by AND, ANDN, and OR.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal
- Other Exceptions

## Operation

```C
def MAX(SRC1, SRC2):IF (SRC1 = 0.0) and (SRC2 = 0.0):DEST := SRC2ELSE IF (SRC1 = NaN):DEST := SRC2ELSE IF (SRC2 = NaN):DEST := SRC2ELSE IF (SRC1 > SRC2):DEST := SRC1ELSE:DEST := SRC2VMAXSH dest, src1, src2IF k1[0] OR *no writemask*:DEST.fp16[0] := MAX(SRC1.fp16[0], SRC2.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else dest.fp16[j] remains unchangedIntel C/C++ Compiler Intrinsic EquivalentVMAXSH __m128h _mm_mask_max_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int sae);VMAXSH __m128h _mm_maskz_max_round_sh (__mmask8 k, __m128h a, __m128h b, int sae);VMAXSH __m128h _mm_max_round_sh (__m128h a, __m128h b, int sae);VMAXSH __m128h _mm_mask_max_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VMAXSH __m128h _mm_maskz_max_sh (__mmask8 k, __m128h a, __m128h b);VMAXSH __m128h _mm_max_sh (__m128h a, __m128h b);
```
