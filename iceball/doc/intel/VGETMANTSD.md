# VGETMANTSD

Extract Float64 of Normalized Mantissas From Float64 Scalar

Convert the double precision floating values in the low quadword element of the second source operand (the third operand) to double precision floating-point value with the mantissa normalization and sign control specified by the .
The converted result is written to the low quadword element of the destination imm8 byte, see Figure5-15operand (the first operand) using writemask k1.
Bits (127:64) of the XMM register destination are copied from corresponding bits in the first source operand.
The normalized mantissa is specified by interv (imm8[1:0]) and the sign control (sc) is specified by bits 3:2 of the immediate byte.
The conversion operation is:GetMant(x) = ±2|x.significand|kwhere:1 <= |x.significand| < 2Unbiased exponent k can be either 0 or -1, depending on the interval range defined by interv, the range of the significand and whether the exponent of the source is even or odd.
The sign of the final result is determined by sc and the source sign.
The encoded value of imm8[1:0] and sign control are shown in Figure5-15.The converted double precision floating-point result is encoded according to the sign control, the unbiased expo-nent k (adding bias) and a mantissa normalized to the range specified by interv.The GetMant() function follows Table 5-8 when dealing with floating-point special numbers.If writemasking is used, the low quadword element of the destination operand is conditionally updated depending on the value of writemask register k1.
If writemasking 

## Exceptions

- SIMD Floating-Point Exceptions
  > Denormal, Invalid

## Operation

```C
// getmant_fp64(src, sign_control, normalization_interval) is defined in the operation section of VGETMANTPDVGETMANTSD (EVEX encoded version) SignCtrl[1:0] := IMM8[3:2];Interv[1:0] := IMM8[1:0];IF k1[0] OR *no writemask*THEN DEST[63:0] :=getmant_fp64(src, sign_control, normalization_interval)ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingDEST[63:0] := 0FIFI;DEST[127:64] := SRC1[127:64] DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVGETMANTSD __m128d _mm_getmant_sd( __m128d a, __m128 b, enum intv, enum sgn);VGETMANTSD __m128d _mm_mask_getmant_sd(__m128d s, __mmask8 k, __m128d a, __m128d b, enum intv, enum sgn);VGETMANTSD __m128d _mm_maskz_getmant_sd( __mmask8 k, __m128 a, __m128d b, enum intv, enum sgn);VGETMANTSD __m128d _mm_getmant_round_sd( __m128d a, __m128 b, enum intv, enum sgn, int r);VGETMANTSD __m128d _mm_mask_getmant_round_sd(__m128d s, __mmask8 k, __m128d a, __m128d b, enum intv, enum sgn, int r);VGETMANTSD __m128d _mm_maskz_getmant_round_sd( __mmask8 k, __m128d a, __m128d b, enum intv, enum sgn, int r);
```
