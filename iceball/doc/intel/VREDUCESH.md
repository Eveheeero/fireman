# VREDUCESH

Perform Reduction Transformation on Scalar FP16 Value

This instruction performs a reduction transformation of the low binary encoded FP16 value in the source operand (the second operand) and store the reduced result in binary FP format to the low element of the destination operand (the first operand) under the writemask k1.
For further details see the description of VREDUCEPH.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.This instruction might end up with a precision exception set.
However, in case of SPE set (i.e., Suppress Precision Exception, which is imm8[3]=1), no precision exception is reported.This instruction may generate tiny non-zero result.
If it does so, it does not report underflow exception, even if underflow exceptions are unmasked (UM flag in MXCSR register is 0).For special cases, see Table 5-20.

## Operation

```C
VREDUCESH dest{k1}, src, imm8 IF k1[0] or *no writemask*:dest.fp16[0] := reduce_fp16(src2.fp16[0], imm8) // see VREDUCEPHELSE IF *zeroing*:dest.fp16[0] := 0//else dest.fp16[0] remains unchangedDEST[127:16] := src1[127:16]DEST[MAXVL-1:128] := 0 Intel C/C++ Compiler Intrinsic EquivalentVREDUCESH __m128h _mm_mask_reduce_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int imm8, const int sae);VREDUCESH __m128h _mm_maskz_reduce_round_sh (__mmask8 k, __m128h a, __m128h b, int imm8, const int sae);VREDUCESH __m128h _mm_reduce_round_sh (__m128h a, __m128h b, int imm8, const int sae);VREDUCESH __m128h _mm_mask_reduce_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, int imm8);VREDUCESH __m128h _mm_maskz_reduce_sh (__mmask8 k, __m128h a, __m128h b, int imm8);VREDUCESH __m128h _mm_reduce_sh (__m128h a, __m128h b, int imm8);
```
