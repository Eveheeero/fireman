# VGETEXPSS

Convert Exponents of Scalar Single Precision Floating-Point Value to Single Precision Floating-Point Value

Extracts the biased exponent from the normalized single-precision floating-point representation of the low double-word data element of the source operand (the third operand) as unbiased signed integer value, or convert the denormal representation of input data to unbiased negative integer values.
The integer value of the unbiased expo-nent is converted to single-precision floating-point value and written to the destination operand (the first operand) as single-precision floating-point numbers.
Bits (127:32) of the XMM register destination are copied from corre-sponding bits in the first source operand.The destination must be a XMM register, the source operand can be a XMM register or a float32 memory location.If writemasking is used, the low doubleword element of the destination operand is conditionally updated depending on the value of writemask register k1.
If writemasking is not used, the low doubleword element of the destination operand is unconditionally updated.Each GETEXP operation converts the exponent value into a floating-point number (permitting input value in denormal representation).
Special cases of input values are listed in Table 5-7.The formula is:GETEXP(x) = floor(log(|x|)) 2Notation floor(x) stands for maximal integer not exceeding real number x.
Software usage of VGETEXPxx and VGETMANTxx instructions generally involve a combination of GETEXP operation and GETMANT operation (see VGETMANTPD).
Thus VGETEXPxx instruction do not require software to handle SIMD floating-point exceptions.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal

## Operation

```C
// NormalizeExpTinySPFP(SRC[31:0]) is defined in the Operation section of VGETEXPPS// ConvertExpSPFP(SRC[31:0]) is defined in the Operation section of VGETEXPPSVGETEXPSS (EVEX encoded version) IF k1[0] OR *no writemask*THEN DEST[31:0] :=ConvertExpDPFP(SRC2[31:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingDEST[31:0]:= 0FIDEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVGETEXPSS __m128 _mm_getexp_ss( __m128 a, __m128 b);VGETEXPSS __m128 _mm_mask_getexp_ss(__m128 s, __mmask8 k, __m128 a, __m128 b);VGETEXPSS __m128 _mm_maskz_getexp_ss( __mmask8 k, __m128 a, __m128 b);VGETEXPSS __m128 _mm_getexp_round_ss( __m128 a, __m128 b, int sae);VGETEXPSS __m128 _mm_mask_getexp_round_ss(__m128 s, __mmask8 k, __m128 a, __m128 b, int sae);VGETEXPSS __m128 _mm_maskz_getexp_round_ss( __mmask8 k, __m128 a, __m128 b, int sae);
```
