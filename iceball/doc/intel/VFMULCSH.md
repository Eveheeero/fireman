# VFCMULCSH/VFMULCSH

Complex Multiply Scalar FP16 Values

This instruction performs a complex multiply operation.
There are normal and complex conjugate forms of the oper-ation.
The masking for this operation is done on 32-bit quantities representing a pair of FP16 values.Bits 127:32 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.Rounding is performed at every FMA (fused multiply and add) boundary.
Execution occurs as if all MXCSR excep-tions are masked.
MXCSR status bits are updated to reflect exceptional conditions.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal
- Other Exceptions
  > EVEX-encoded instructions, see Table2-58, "Type E10 Class Exception Conditions."

## Operation

```C
VFMULCSH dest{k1}, src1, src2 (AVX512)KL := VL / 32IF k1[0] or *no writemask*:// non-conjugate version subtracts last even termtmp.fp16[0] := src1.fp16[0] * src2.fp16[0]tmp.fp16[1] := src1.fp16[1] * src2.fp16[0]dest.fp16[0] := tmp.fp16[0] - src1.fp16[1] * src2.fp16[1]dest.fp16[1] := tmp.fp16[1] + src1.fp16[0] * src2.fp16[1]ELSE IF *zeroing*:dest.fp16[0] := 0dest.fp16[1] := 0VFCMULCSH dest{k1}, src1, src2 (AVX512)KL := VL / 32IF k1[0] or *no writemask*:tmp.fp16[0] := src1.fp16[0] * src2.fp16[0]tmp.fp16[1] := src1.fp16[1] * src2.fp16[0]// conjugate version subtracts odd final termdest.fp16[0] := tmp.fp16[0] + src1.fp16[1] * src2.fp16[1]dest.fp16[1] := tmp.fp16[1] - src1.fp16[0] * src2.fp16[1]ELSE IF *zeroing*:dest.fp16[0] := 0dest.fp16[1] := 0DEST[127:32] := src1[127:32] // copy upper part of src1DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVFCMULCSH __m128h _mm_cmul_round_sch (__m128h a, __m128h b, const int rounding);VFCMULCSH __m128h _mm_mask_cmul_round_sch (__m128h src, __mmask8 k, __m128h a, __m128h b, const int rounding);VFCMULCSH __m128h _mm_maskz_cmul_round_sch (__mmask8 k, __m128h a, __m128h b, const int rounding);VFCMULCSH __m128h _mm_cmul_sch (__m128h a, __m128h b);VFCMULCSH __m128h _mm_mask_cmul_sch (__m128h src, __mmask8 k, __m128h a, __m128h b);VFCMULCSH __m128h _mm_maskz_cmul_sch (__mmask8 k, __m128h a, __m128h b);VFCMULCSH __m128h _mm_fcmul_round_sch (__m128h a, __m128h b, const int rounding);VFCMULCSH __m128h _mm_mask_fcmul_round_sch (__m128h src, __mmask8 k, __m128h a, __m128h b, const int rounding);VFCMULCSH __m128h _mm_maskz_fcmul_round_sch (__mmask8 k, __m128h a, __m128h b, const int rounding);VFCMULCSH __m128h _mm_fcmul_sch (__m128h a, __m128h b);VFCMULCSH __m128h _mm_mask_fcmul_sch (__m128h src, __mmask8 k, __m128h a, __m128h b);VFCMULCSH __m128h _mm_maskz_fcmul_sch (__mmask8 k, __m128h a, __m128h b);VFMULCSH __m128h _mm_fmul_round_sch (__m128h a, __m128h b, const int rounding);VFMULCSH __m128h _mm_mask_fmul_round_sch (__m128h src, __mmask8 k, __m128h a, __m128h b, const int rounding);VFMULCSH __m128h _mm_maskz_fmul_round_sch (__mmask8 k, __m128h a, __m128h b, const int rounding);VFMULCSH __m128h _mm_fmul_sch (__m128h a, __m128h b);VFMULCSH __m128h _mm_mask_fmul_sch (__m128h src, __mmask8 k, __m128h a, __m128h b);VFMULCSH __m128h _mm_maskz_fmul_sch (__mmask8 k, __m128h a, __m128h b);VFMULCSH __m128h _mm_mask_mul_round_sch (__m128h src, __mmask8 k, __m128h a, __m128h b, const int rounding);VFMULCSH __m128h _mm_maskz_mul_round_sch (__mmask8 k, __m128h a, __m128h b, const int rounding);VFMULCSH __m128h _mm_mul_round_sch (__m128h a, __m128h b, const int rounding);VFMULCSH __m128h _mm_mask_mul_sch (__m128h src, __mmask8 k, __m128h a, __m128h b);VFMULCSH __m128h _mm_maskz_mul_sch (__mmask8 k, __m128h a, __m128h b);VFMULCSH __m128h _mm_mul_sch (__m128h a, __m128h b);
```
