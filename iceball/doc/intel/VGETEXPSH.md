# VGETEXPSH

Convert Exponents of Scalar FP16 Values to FP16 Values

This instruction extracts the biased exponents from the normalized FP16 representation of the low word element of the source operand (the second operand) as unbiased signed integer value, or convert the denormal representation of input data to an unbiased negative integer value.
The integer value of the unbiased exponent is converted to an FP16 value and written to the low word element of the destination operand (the first operand) as an FP16 number.Bits 127:16 of the destination operand are copied from the corresponding bits of the first source operand.
Bits MAXVL-1:128 of the destination operand are zeroed.
The low FP16 element of the destination is updated according to the writemask.Each GETEXP operation converts the exponent value into a floating-point number (permitting input value in denormal representation).
Special cases of input values are listed in Table 5-6.The formula is:GETEXP(x) = floor(log(|x|))2Notation floor(x) stands for maximal integer not exceeding real number x.Software usage of VGETEXPxx and VGETMANTxx instructions generally involve a combination of GETEXP operation and GETMANT operation (see VGETMANTSH).
Thus, the VGETEXPSH instruction does not require software to handle SIMD floating-point exceptions.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Invalid, Denormal

## Operation

```C
VGETEXPSH dest{k1}, src1, src2 IF k1[0] or *no writemask*:DEST.fp16[0] := getexp_fp16(src2.fp16[0]) // see VGETEXPPHELSE IF *zeroing*:DEST.fp16[0] := 0//else DEST.fp16[0] remains unchangedDEST[127:16] := src1[127:16]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVGETEXPSH __m128h _mm_getexp_round_sh (__m128h a, __m128h b, const int sae);VGETEXPSH __m128h _mm_mask_getexp_round_sh (__m128h src, __mmask8 k, __m128h a, __m128h b, const int sae);VGETEXPSH __m128h _mm_maskz_getexp_round_sh (__mmask8 k, __m128h a, __m128h b, const int sae);VGETEXPSH __m128h _mm_getexp_sh (__m128h a, __m128h b);VGETEXPSH __m128h _mm_mask_getexp_sh (__m128h
```
