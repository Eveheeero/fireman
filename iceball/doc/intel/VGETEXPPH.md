# VGETEXPPH

Convert Exponents of Packed FP16 Values to FP16 Values

This instruction extracts the biased exponents from the normalized FP16 representation of each word element of the source operand (the second operand) as unbiased signed integer value, or convert the denormal representa-tion of input data to unbiased negative integer values.
Each integer value of the unbiased exponent is converted to an FP16 value and written to the corresponding word elements of the destination operand (the first operand) as FP16 numbers.The destination elements are updated according to the writemask.Each GETEXP operation converts the exponent value into a floating-point number (permitting input value in denormal representation).
Special cases of input values are listed in Table 5-5.The formula is:GETEXP(x) = floor(log(|x|))2Notation floor(x) stands for maximal integer not exceeding real number x.Software usage of VGETEXPxx and VGETMANTxx instructions generally involve a combination of GETEXP operation and GETMANT operation (see VGETMANTPH).
Thus, the VGETEXPPH instruction does not require software to handle SIMD floating-point exceptions.Table 5-6.
VGETEXPPH/VGETEXPSH Special CasesInput OperandResultCommentssrc1 = NaNQNaN(src1)If (SRC = SNaN), then #IE.0 < |src1| < INFfloor(log(|src1|)) 2If (SRC = denormal), then #DE.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.
- Other Exceptions

## Operation

```C
def normalize_exponent_tiny_fp16(src):jbit := 0// src & dst are FP16 numbers with sign(1b), exp(5b) and fraction (10b) fieldsdst.exp := 1 // write bits 14:10dst.fraction := src.fraction // copy bits 9:0while jbit == 0:jbit := dst.fraction[9] // msb of the fractiondst.fraction := dst.fraction << 1dst.exp := dst.exp - 1dst.fraction := 0return dstdef getexp_fp16(src):src.sign := 0 // make positiveexponent_all_ones := (src[14:10] == 0x1F)exponent_all_zeros := (src[14:10] == 0)mantissa_all_zeros := (src[9:0] == 0)zero := exponent_all_zeros and mantissa_all_zerossignaling_bit := src[9]nan := exponent_all_ones and not(mantissa_all_zeros)snan := nan and not(signaling_bit)qnan := nan and signaling_bitpositive_infinity := not(negative) and exponent_all_ones and mantissa_all_zerosdenormal := exponent_all_zeros and not(mantissa_all_zeros)if nan:if snan:MXCSR.IE := 1return qnan(src) // convert snan to a qnanif positive_infinity:return srcif zero:return -INFif denormal:tmp := normalize_exponent_tiny_fp16(src)MXCSR.DE := 1else:tmp := srctmp := SAR(tmp, 10) // shift arithmetic rightVGETEXPPH dest{k1}, srcVL = 128, 256 or 512KL := VL/16FOR i := 0 to KL-1:IF k1[i] or *no writemask*:IF SRC is memory and (EVEX.b = 1):tsrc := src.fp16[0]ELSE:tsrc := src.fp16[i]DEST.fp16[i] := getexp_fp16(tsrc)ELSE IF *zeroing*:DEST.fp16[i] := 0//else DEST.fp16[i] remains unchangedDEST[MAXVL-1:VL] := 0 Intel C/C++ Compiler Intrinsic EquivalentVGETEXPPH __m128h _mm_getexp_ph (__m128h a);VGETEXPPH __m128h _mm_mask_getexp_ph (__m128h src, __mmask8 k, __m128h a);VGETEXPPH __m128h _mm_maskz_getexp_ph (__mmask8 k, __m128h a);VGETEXPPH __m256h _mm256_getexp_ph (__m256h a);VGETEXPPH __m256h _mm256_mask_getexp_ph (__m256h src, __mmask16 k, __m256h a);VGETEXPPH __m256h _mm256_maskz_getexp_ph (__mmask16 k, __m256h a);VGETEXPPH __m512h _mm512_getexp_ph (__m512h a);VGETEXPPH __m512h _mm512_mask_getexp_ph (__m512h src, __mmask32 k, __m512h a);VGETEXPPH __m512h _mm512_maskz_getexp_ph (__mmask32 k, __m512h a);VGETEXPPH __m512h _mm512_getexp_round_ph (__m512h a, const int sae);VGETEXPPH __m512h _mm512_mask_getexp_round_ph (__m512h src, __mmask32 k, __m512h a, const int sae);VGETEXPPH __m512h _mm512_maskz_getexp_round_ph (__mmask32 k, __m512h a, const int sae);
```
