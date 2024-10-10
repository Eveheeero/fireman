# VGETMANTSS

Extract Float32 Vector of Normalized Mantissa From Float32 Vector

Convert the single-precision floating values in the low doubleword element of the second source operand (the third operand) to single-precision floating-point value with the mantissa normalization and sign control specified by the .
The converted result is written to the low doubleword element of the destination imm8 byte, see Figure5-15operand (the first operand) using writemask k1.
Bits (127:32) of the XMM register destination are copied from corresponding bits in the first source operand.
The normalized mantissa is specified by interv (imm8[1:0]) and the sign control (sc) is specified by bits 3:2 of the immediate byte.
The conversion operation is:k|x.significand|GetMant(x) = ±2where:1 <= |x.significand| < 2Unbiased exponent k can be either 0 or -1, depending on the interval range defined by interv, the range of the significand and whether the exponent of the source is even or odd.
The sign of the final result is determined by sc and the source sign.
The encoded value of imm8[1:0] and sign control are shown in Figure5-15.The converted single-precision floating-point result is encoded according to the sign control, the unbiased expo-nent k (adding bias) and a mantissa normalized to the range specified by interv.The GetMant() function follows Table 5-8 when dealing with floating-point special numbers.If writemasking is used, the low doubleword element of the destination operand is conditionally updated depending on the value of writemask register k1.
If writemasking is

## Exceptions

- SIMD Floating-Point Exceptions
  > Denormal, Invalid

## Operation

```C
// getmant_fp32(src, sign_control, normalization_interval) is defined in the operation section of VGETMANTPSVGETMANTSS (EVEX encoded version) SignCtrl[1:0] := IMM8[3:2];Interv[1:0] := IMM8[1:0];IF k1[0] OR *no writemask*THEN DEST[31:0] :=getmant_fp32(src, sign_control, normalization_interval)ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingDEST[31:0] := 0FIFI;DEST[127:32] := SRC1[127:32] DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVGETMANTSS __m128 _mm_getmant_ss( __m128 a, __m128 b, enum intv, enum sgn);VGETMANTSS __m128 _mm_mask_getmant_ss(__m128 s, __mmask8 k, __m128 a, __m128 b, enum intv, enum sgn);VGETMANTSS __m128 _mm_maskz_getmant_ss( __mmask8 k, __m128 a, __m128 b, enum intv, enum sgn);VGETMANTSS __m128 _mm_getmant_round_ss( __m128 a, __m128 b, enum intv, enum sgn, int r);VGETMANTSS __m128 _mm_mask_getmant_round_ss(__m128 s, __mmask8 k, __m128 a, __m128 b, enum intv, enum sgn, int r);VGETMANTSS __m128 _mm_maskz_getmant_round_ss( __mmask8 k, __m128 a, __m128 b, enum intv, enum sgn, int r);
```
