# VFCMADDCSH/VFMADDCSH

Complex Multiply and Accumulate Scalar FP16 Values

This instruction performs a complex multiply and accumulate operation.
There are normal and complex conjugate forms of the operation.The masking for this operation is done on 32-bit quantities representing a pair of FP16 values.Bits 127:32 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.Rounding is performed at every FMA (fused multiply and add) boundary.
Execution occurs as if all MXCSR excep-tions are masked.
MXCSR status bits are updated to reflect exceptional conditions.

## Exceptions

- Other Exceptions
  > EVEX-encoded instructions, see Table2-58, "Type E10 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal

## Operation

```C
VFMADDCSH dest{k1}, src1, src2 (AVX512)IF k1[0] or *no writemask*:tmp[0] := dest.fp16[0] + src1.fp16[0] * src2.fp16[0]tmp[1] := dest.fp16[1] + src1.fp16[1] * src2.fp16[0]// non-conjugate version subtracts last even termdest.fp16[0] := tmp[0] - src1.fp16[1] * src2.fp16[1]dest.fp16[1] := tmp[1] + src1.fp16[0] * src2.fp16[1]ELSE IF *zeroing*:dest.fp16[0] := 0dest.fp16[1] := 0DEST[127:32] := src1[127:32] VFCMADDCSH dest{k1}, src1, src2 (AVX512)IF k1[0] or *no writemask*:tmp[0] := dest.fp16[0] + src1.fp16[0] * src2.fp16[0]tmp[1] := dest.fp16[1] + src1.fp16[1] * src2.fp16[0]// conjugate version subtracts odd final termdest.fp16[0] := tmp[0] + src1.fp16[1] * src2.fp16[1]dest.fp16[1] := tmp[1] - src1.fp16[0] * src2.fp16[1]ELSE IF *zeroing*:dest.fp16[0] := 0dest.fp16[1] := 0DEST[127:32] := src1[127:32] // copy upper part of src1DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVFCMADDCSH __m128h _mm_fcmadd_round_sch (__m128h a, __m128h b, __m128h c, const int rounding);VFCMADDCSH __m128h _mm_mask_fcmadd_round_sch (__m128h a, __mmask8 k, __m128h b, __m128h c, const int rounding);VFCMADDCSH __m128h _mm_mask3_fcmadd_round_sch (__m128h a, __m128h b, __m128h c, __mmask8 k, const int rounding);VFCMADDCSH __m128h _mm_maskz_fcmadd_round_sch (__mmask8 k, __m128h a, __m128h b, __m128h c, const int rounding);VFCMADDCSH __m128h _mm_fcmadd_sch (__m128h a, __m128h b, __m128h c);VFCMADDCSH __m128h _mm_mask_fcmadd_sch (__m128h a, __mmask8 k, __m128h b, __m128h c);VFCMADDCSH __m128h _mm_mask3_fcmadd_sch (__m128h a, __m128h b, __m128h c, __mmask8 k);VFCMADDCSH __m128h _mm_maskz_fcmadd_sch (__mmask8 k, __m128h a, __m128h b, __m128h c);VFCMADDCSH __m128h _mm_mask3_fmadd_round_sch (__m128h a, __m128h b, __m128h c, __mmask8 k, const int rounding);VFCMADDCSH __m128h _mm_mask3_fmadd_sch (__m128h a, __m128h b, __m128h c, __mmask8 k);VFMADDCSH __m128h _mm_fmadd_round_sch (__m128h a, __m128h b, __m128h c, const int rounding);VFMADDCSH __m128h _mm_mask_fmadd_round_sch (__m128h a, __mmask8 k, __m128h b, __m128h c, const int rounding);VFMADDCSH __m128h _mm_maskz_fmadd_round_sch (__mmask8 k, __m128h a, __m128h b, __m128h c, const int rounding);VFMADDCSH __m128h _mm_fmadd_sch (__m128h a, __m128h b, __m128h c);VFMADDCSH __m128h _mm_mask_fmadd_sch (__m128h a, __mmask8 k, __m128h b, __m128h c);VFMADDCSH __m128h _mm_maskz_fmadd_sch (__mmask8 k, __m128h a, __m128h b, __m128h c);
```
