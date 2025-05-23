# VSCALEFSH

Scale Scalar FP16 Values with FP16 Values

This instruction performs a floating-point scale of the low FP16 element in the first source operand by multiplying it by 2 to the power of the low FP16 element in second source operand, storing the result in the low element of the destination operand.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.The equation of this operation is given by:floor(xmm3).xmm1 := xmm2 * 2Floor(xmm3) means maximum integer value  " xmm3.If the result cannot be represented in FP16, then the proper overflow response (for positive scaling operand), or the proper underflow response (for negative scaling operand), is issued.
The overflow and underflow responses are dependent on the rounding mode (for IEEE-compliant rounding), as well as on other settings in MXCSR (exception mask bits, FTZ bit), and on the SAE bit.Handling of special-case input values are listed in Table 5-31 and Table 5-32.

## Exceptions

- Other Exceptions
  > EVEX-encoded instructions, see Table2-47, "Type E3 Class Exception Conditions."
  > Denormal-operand exception (#D) is checked and signal
  > ed for src1 operand, but not for src2 operand. The 
  > denormal-operand exception is checked for src1 operand only 
  > if the src2 operand is not Na
  > N. If the src2 operand is 
  > NaN, the processor generates NaN and does not signal de
- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal.

## Operation

```C
VSCALEFSH dest{k1}, src1, src2IF (EVEX.b = 1) and no memory operand:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] or *no writemask*:dest.fp16[0] := scale_fp16(src1.fp16[0], src2.fp16[0]) // see VSCALEFPHELSE IF *zeroing*:dest.fp16[0] := 0//else DEST.fp16[0] remains unchangedIntel C/C++ Compiler Intrinsic EquivalentVSCALEFSH __m128h _mm_mask_scalef_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, const int rounding);VSCALEFSH __m128h _mm_maskz_scalef_round_sh (__mmask8 k, __m128h a, __m128h b, const int rounding);VSCALEFSH __m128h _mm_scalef_round_sh (__m128h a, __m128h b, const int rounding);VSCALEFSH __m128h _mm_mask_scalef_sh (__m128h src, __mmask8 k, __m128h a, __m128h b);VSCALEFSH __m128h _mm_maskz_scalef_sh (__mmask8 k, __m128h a, __m128h b);VSCALEFSH __m128h _mm_scalef_sh (__m128h a, __m128h b);
```
