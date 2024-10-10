# VMINSH

Return Minimum Scalar FP16 Value

This instruction performs a compare of the low packed FP16 values in the first source operand and the second source operand and returns the minimum value for the pair of values to the destination operand.If the values being compared are both 0.0s (of either sign), the value in the second operand (source operand) is returned.
If a value in the second operand is an SNaN, then SNaN is forwarded unchanged to the destination (that is, a QNaN version of the SNaN is not returned).If only one value is a NaN (SNaN or QNaN) for this instruction, the second operand (source operand), either a NaN or a valid floating-point value, is written to the result.
If instead of this behavior, it is required that the NaN source operand (from either the first or second operand) be returned, the action of VMINSH can be emulated using a sequence of instructions, such as, a comparison followed by AND, ANDN, and OR.EVEX encoded versions: The first source operand (the second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcast from a 16-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal
- Other Exceptions

## Operation

```C
def MIN(SRC1, SRC2):IF (SRC1 = 0.0) and (SRC2 = 0.0):DEST := SRC2ELSE IF (SRC1 = NaN):DEST := SRC2ELSE IF (SRC2 = NaN):DEST := SRC2ELSE IF (SRC1 < SRC2):DEST := SRC1VMINSH dest, src1, src2 IF k1[0] OR *no writemask*:DEST.fp16[0] := MIN(SRC1.fp16[0], SRC2.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else dest.fp16[j] remains unchangedDEST[127:16] := SRC1[127:16]DEST[MAXVL-1:128] := 0 Intel C/C++ Compiler Intrinsic EquivalentVMINSH __m128h _mm_mask_min_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int sae);VMINSH __m128h _mm_maskz_min_round_sh (__mmask8 k, __m128h a, __m128h b, int sae);VMINSH __m128h _mm_min_round_sh (__m128h a, __m128h b, int sae);VMINSH __m128h _mm_mask_min_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VMINSH __m128h _mm_maskz_min_sh (__mmask8 k, __m128h a, __m128h b);VMINSH __m128h _mm_min_sh (__m128h a, __m128h b);
```
