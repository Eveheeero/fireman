# VGETMANTSH

Extract FP16 of Normalized Mantissa from FP16 Scalar

This instruction converts the FP16 value in the low element of the second source operand to FP16 values with the mantissa normalization and sign control specified by the imm8 byte, see Table 5-9.
The converted result is written to the low element of the destination operand using writemask k1.
The normalized mantissa is specified by interv (imm8[1:0]) and the sign control (SC) is specified by bits 3:2 of the immediate byte.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.For each input FP16 value x, The conversion operation is:k|x.significand|GetMant(x) = ±2where:1  |x.significand| < 2Unbiased exponent k depends on the interval range defined by interv and whether the exponent of the source is even or odd.
The sign of the final result is determined by the sign control and the source sign and the leading frac-tion bit.The encoded value of imm8[1:0] and sign control are shown in Table 5-9.Each converted FP16 result is encoded according to the sign control, the unbiased exponent k (adding bias) and a mantissa normalized to the range specified by interv.The GetMant() function follows Table 5-10 when dealing with floating-point special numbers.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal

## Operation

```C
VGETMANTSH dest{k1}, src1, src2, imm8sign_control := imm8[3:2]normalization_interval := imm8[1:0]IF k1[0] or *no writemask*:dest.fp16[0] := getmant_fp16(src2.fp16[0], // see VGETMANTPHsign_control,normalization_interval)ELSE IF *zeroing*:dest.fp16[0] := 0//else dest.fp16[0] remains unchangedIntel C/C++ Compiler Intrinsic EquivalentVGETMANTSH __m128h _mm_getmant_round_sh (__m128h a, __m128h b, _MM_MANTISSA_NORM_ENUM norm, _MM_MANTISSA_SIGN_ENUM sign, const int sae);VGETMANTSH __m128h _mm_mask_getmant_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, _MM_MANTISSA_NORM_ENUM norm, _MM_MANTISSA_SIGN_ENUM sign, const int sae);VGETMANTSH __m128h _mm_maskz_getmant_round_sh (__mmask8 k, __m128h a, __m128h b, _MM_MANTISSA_NORM_ENUM norm, _MM_MANTISSA_SIGN_ENUM sign, const int sae);VGETMANTSH __m128h _mm_getmant_sh (__m128h a, __m128h b, _MM_MANTISSA_NORM_ENUM norm, _MM_MANTISSA_SIGN_ENUM sign);VGETMANTSH __m128h _mm_mask_getmant_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, _MM_MANTISSA_NORM_ENUM norm, _MM_MANTISSA_SIGN_ENUM sign);VGETMANTSH __m128h _mm_maskz_getmant_sh (__mmask8 k, __m128h a, __m128h b, _MM_MANTISSA_NORM_ENUM norm, _MM_MANTISSA_SIGN_ENUM sign);
```
