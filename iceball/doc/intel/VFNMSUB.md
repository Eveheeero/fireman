# VF[,N]MSUB[132,213,231]SH

Fused Multiply-Subtract of Scalar FP16 Values

This instruction performs a scalar multiply-subtract or negated multiply-subtract computation on the low FP16 values using three source operands and writes the result in the destination operand.
The destination operand is also the first source operand.
The "N" (negated) forms of this instruction subtract the remaining operand from the negated infinite precision intermediate product.
The notation' "132", "213" and "231" indicate the use of the oper-C, where each digit corresponds to the operand number, with the destination being operand 1; ands in ±A * B  í see Table 5-6.Bits 127:16 of the destination operand are preserved.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.Table 5-6.
 VF[,N]MSUB[132,213,231]SH Notation for OperandsNotationOperands132dest = ± dest*src3-src2231dest = ± src2*src3-dest213dest = ±

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal

## Operation

```C
VF[,N]MSUB132SH DEST, SRC2, SRC3 (EVEX encoded versions)IF EVEX.b = 1 and SRC3 is a register:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:IF *negative form*:DEST.fp16[0] := RoundFPControl(-DEST.fp16[0]*SRC3.fp16[0] - SRC2.fp16[0])ELSE:DEST.fp16[0] := RoundFPControl(DEST.fp16[0]*SRC3.fp16[0] - SRC2.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else DEST.fp16[0] remains unchanged//DEST[127:16] remains unchangedDEST[MAXVL-1:128] := 0 VF[,N]MSUB213SH DEST, SRC2, SRC3 (EVEX encoded versions) IF EVEX.b = 1 and SRC3 is a register:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:IF *negative form:DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*DEST.fp16[0] - SRC3.fp16[0])ELSE:DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*DEST.fp16[0] - SRC3.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else DEST.fp16[0] remains unchanged//DEST[127:16] remains unchangedDEST[MAXVL-1:128] := 0 VF[,N]MSUB231SH DEST, SRC2, SRC3 (EVEX encoded versions) IF EVEX.b = 1 and SRC3 is a register:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:IF *negative form*:DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*SRC3.fp16[0] - DEST.fp16[0])ELSE:DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*SRC3.fp16[0] - DEST.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else DEST.fp16[0] remains unchangedIntel C/C++ Compiler Intrinsic EquivalentVFMSUB132SH, VFMSUB213SH, and VFMSUB231SH:__m128h _mm_fmsub_round_sh (__m128h a, __m128h b, __m128h c, const int rounding);__m128h _mm_mask_fmsub_round_sh (__m128h a, __mmask8 k, __m128h b, __m128h c, const int rounding);__m128h _mm_mask3_fmsub_round_sh (__m128h a, __m128h b, __m128h c, __mmask8 k, const int rounding);__m128h _mm_maskz_fmsub_round_sh (__mmask8 k, __m128h a, __m128h b, __m128h c, const int rounding);__m128h _mm_fmsub_sh (__m128h a, __m128h b, __m128h c);__m128h _mm_mask_fmsub_sh (__m128h a, __mmask8 k, __m128h b, __m128h c);__m128h _mm_mask3_fmsub_sh (__m128h a, __m128h b, __m128h c, __mmask8 k);__m128h _mm_maskz_fmsub_sh (__mmask8 k, __m128h a, __m128h b, __m128h c);VFNMSUB132SH, VFNMSUB213SH, and VFNMSUB231SH:__m128h _mm_fnmsub_round_sh (__m128h a, __m128h b, __m128h c, const int rounding);__m128h _mm_mask_fnmsub_round_sh (__m128h a, __mmask8 k, __m128h b, __m128h c, const int rounding);__m128h _mm_mask3_fnmsub_round_sh (__m128h a, __m128h b, __m128h c, __mmask8 k, const int rounding);__m128h _mm_maskz_fnmsub_round_sh (__mmask8 k, __m128h a, __m128h b, __m128h c, const int rounding);__m128h _mm_fnmsub_sh (__m128h a, __m128h b, __m128h c);__m128h _mm_mask_fnmsub_sh (__m128h a, __mmask8 k, __m128h b, __m128h c);__m128h _mm_mask3_fnmsub_sh (__m128h a, __m128h b, __m128h c, __mmask8 k);__m128h _mm_maskz_fnmsub_sh (__mmask8 k, __m128h a, __m128h b, __m128h c);
```
