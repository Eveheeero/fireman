# VF[,N]MADD[132,213,231]SH

Fused Multiply-Add of Scalar FP16 Values

Performs a scalar multiply-add or negated multiply-add computation on the low FP16 values using three source operands and writes the result in the destination operand.
The destination operand is also the first source operand.
The "N" (negated) forms of this instruction add the negated infinite precision intermediate product to the corre-sponding remaining operand.
The notation' "132", "213" and "231" indicate the use of the operands in ±A * B + C, where each digit corresponds to the operand number, with the destination being operand 1; see Table 5-3.Bits 127:16 of the destination operand are preserved.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.Table 5-3.
 VF[,N]MADD[132,213,231]SH Notation for OperandsNotationOperands132dest = ± dest*src3+src2231dest = ± src2*src3+dest213dest = ±

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal

## Operation

```C
VF[,N]MADD132SH DEST, SRC2, SRC3 (EVEX encoded versions) IF EVEX.b = 1 and SRC3 is a register:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:IF *negative form*:DEST.fp16[0] := RoundFPControl(-DEST.fp16[0]*SRC3.fp16[0] + SRC2.fp16[0])ELSE:DEST.fp16[0] := RoundFPControl(DEST.fp16[0]*SRC3.fp16[0] + SRC2.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else DEST.fp16[0] remains unchanged//DEST[127:16] remains unchangedDEST[MAXVL-1:128] := 0VF[,N]MADD213SH DEST, SRC2, SRC3 (EVEX encoded versions) IF EVEX.b = 1 and SRC3 is a register:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:IF *negative form:DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*DEST.fp16[0] + SRC3.fp16[0])ELSE:DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*DEST.fp16[0] + SRC3.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else DEST.fp16[0] remains unchanged//DEST[127:16] remains unchangedDEST[MAXVL-1:128] := 0 VF[,N]MADD231SH DEST, SRC2, SRC3 (EVEX encoded versions) IF EVEX.b = 1 and SRC3 is a register:SET_RM(EVEX.RC)ELSESET_RM(MXCSR.RC)IF k1[0] OR *no writemask*:IF *negative form*:DEST.fp16[0] := RoundFPControl(-SRC2.fp16[0]*SRC3.fp16[0] + DEST.fp16[0])ELSE:DEST.fp16[0] := RoundFPControl(SRC2.fp16[0]*SRC3.fp16[0] + DEST.fp16[0])ELSE IF *zeroing*:DEST.fp16[0] := 0// else DEST.fp16[0] remains unchangedIntel C/C++ Compiler Intrinsic EquivalentVFMADD132SH, VFMADD213SH, and VFMADD231SH:__m128h _mm_fmadd_round_sh (__m128h a, __m128h b, __m128h c, const int rounding);__m128h _mm_mask_fmadd_round_sh (__m128h a, __mmask8 k, __m128h b, __m128h c, const int rounding);__m128h _mm_mask3_fmadd_round_sh (__m128h a, __m128h b, __m128h c, __mmask8 k, const int rounding);__m128h _mm_maskz_fmadd_round_sh (__mmask8 k, __m128h a, __m128h b, __m128h c, const int rounding);__m128h _mm_fmadd_sh (__m128h a, __m128h b, __m128h c);__m128h _mm_mask_fmadd_sh (__m128h a, __mmask8 k, __m128h b, __m128h c);__m128h _mm_mask3_fmadd_sh (__m128h a, __m128h b, __m128h c, __mmask8 k);__m128h _mm_maskz_fmadd_sh (__mmask8 k, __m128h a, __m128h b, __m128h c);VFNMADD132SH, VFNMADD213SH, and VFNMADD231SH:__m128h _mm_fnmadd_round_sh (__m128h a, __m128h b, __m128h c, const int rounding);__m128h _mm_mask_fnmadd_round_sh (__m128h a, __mmask8 k, __m128h b, __m128h c, const int rounding);__m128h _mm_mask3_fnmadd_round_sh (__m128h a, __m128h b, __m128h c, __mmask8 k, const int rounding);__m128h _mm_maskz_fnmadd_round_sh (__mmask8 k, __m128h a, __m128h b, __m128h c, const int rounding);__m128h _mm_fnmadd_sh (__m128h a, __m128h b, __m128h c);__m128h _mm_mask_fnmadd_sh (__m128h a, __mmask8 k, __m128h b, __m128h c);__m128h _mm_mask3_fnmadd_sh (__m128h a, __m128h b, __m128h c, __mmask8 k);__m128h _mm_maskz_fnmadd_sh (__mmask8 k, __m128h a, __m128h b, __m128h c);
```
