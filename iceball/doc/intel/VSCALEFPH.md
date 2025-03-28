# VSCALEFPH

Scale Packed FP16 Values with FP16 Values

This instruction performs a floating-point scale of the packed FP16 values in the first source operand by multiplying it by 2 to the power of the FP16 values in second source operand.
The destination elements are updated according to the writemask.The equation of this operation is given by:floor(zmm3).zmm1 := zmm2 * 2Floor(zmm3) means maximum integer value  " zmm3.If the result cannot be represented in FP16, then the proper overflow response (for positive scaling operand), or the proper underflow response (for negative scaling operand), is issued.
The overflow and underflow responses are dependent on the rounding mode (for IEEE-compliant rounding), as well as on other settings in MXCSR (exception mask bits), and on the SAE bit.Handling of special-case input values are listed in Table 5-31 and Table 5-32.Table 5-31.
VSCALEFPH/VSCALEFSH Special CasesSrc2Src1Set IE±NaN+INF îINF0/Denorm/Norm±QNaNQNaN(Src1)+INF+0QNaN(Src1)IF either source is SNaN±SNaNQNaN(Src1)QNaN(Src1)QNaN(Src1)QNaN(Src1)YES±INFQNaN(Src2)Src1QNaN_IndefiniteSrc1IF Src2 is SNaN or  îINF±0QNaN(Src2)QNaN_IndefiniteSrc1Src1IF Src2 is SNaN or +INFDenorm/NormQNaN(Src2)±INF (Src1 sign)±0 (Src1 sign)Compute ResultIF Src2 is SNaNTable 5-32.
Additional VSCALEFPH/VSCALEFSH Special CasesSpecial Case Returned Value Faults-24|result| < 2±0 or ±Min-Denormal (Src1 sign)Underflow16±INF (Src1 sign) or ±Max-Denormal (Src1 sign)Overflow|result| 

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal.
- Other Exceptions
  > EVEX-encoded instruction, see Table2-46, "Type E2 Class Exception Conditions".
  > Denormal-operand exception (#D) is 
  > checked and signaled for src1 operand, but not for src2 operand. The 
  > denormal-operand exception is checked for src1 operand only 
  > if the src2 operand is not NaN. If the src2 operand is 
  > NaN, the processor generates NaN and does not signal de

## Operation

```C
def scale_fp16(src1,src2):tmp1 := src1tmp2 := src2return tmp1 * POW(2, FLOOR(tmp2)) VSCALEFPH dest{k1}, src1, src2VL = 128, 256, or 512KL := VL / 16IF (VL = 512) AND (EVEX.b = 1) and no memory operand:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)FOR i := 0 to KL-1:IF k1[i] or *no writemask*:IF SRC2 is memory and (EVEX.b = 1):tsrc := src2.fp16[0]ELSE:tsrc := src2.fp16[i]dest.fp16[i] := scale_fp16(src1.fp16[i],tsrc)ELSE IF *zeroing*:dest.fp16[i] := 0//else dest.fp16[i] remains unchangedDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVSCALEFPH __m128h _mm_mask_scalef_ph (__m128h src, __mmask8 k, __m128h a, __m128h b);VSCALEFPH __m128h _mm_maskz_scalef_ph (__mmask8 k, __m128h a, __m128h b);VSCALEFPH __m128h _mm_scalef_ph (__m128h a, __m128h b);VSCALEFPH __m256h _mm256_mask_scalef_ph (__m256h src, __mmask16 k, __m256h a, __m256h b);VSCALEFPH __m256h _mm256_maskz_scalef_ph (__mmask16 k, __m256h a, __m256h b);VSCALEFPH __m256h _mm256_scalef_ph (__m256h a, __m256h b);VSCALEFPH __m512h _mm512_mask_scalef_ph (__m512h src, __mmask32 k, __m512h a, __m512h b);VSCALEFPH __m512h _mm512_maskz_scalef_ph (__mmask32 k, __m512h a, __m512h b);VSCALEFPH __m512h _mm512_scalef_ph (__m512h a, __m512h b);VSCALEFPH __m512h _mm512_mask_scalef_round_ph (__m512h src, __mmask32 k, __m512h a, __m512h b, const int rounding);VSCALEFPH __m512h _mm512_maskz_scalef_round_ph (__mmask32 k, __m512h a, __m512h b, const int;VSCALEFPH __m512h _mm512_scalef_round_ph (__m512h a, __m512h b, const int rounding);
```
