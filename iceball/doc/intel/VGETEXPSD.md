# VGETEXPSD

Convert Exponents of Scalar Double Precision Floating-Point Value to Double Precision Floating-Point Value

Extracts the biased exponent from the normalized double precision floating-point representation of the low qword data element of the source operand (the third operand) as unbiased signed integer value, or convert the denormal representation of input data to unbiased negative integer values.
The integer value of the unbiased exponent is converted to double precision floating-point value and written to the destination operand (the first operand) as double precision floating-point numbers.
Bits (127:64) of the XMM register destination are copied from corre-sponding bits in the first source operand.The destination must be a XMM register, the source operand can be a XMM register or a float64 memory location.If writemasking is used, the low quadword element of the destination operand is conditionally updated depending on the value of writemask register k1.
If writemasking is not used, the low quadword element of the destination operand is unconditionally updated.Each GETEXP operation converts the exponent value into a floating-point number (permitting input value in denormal representation).
Special cases of input values are listed in Table 5-5.The formula is:GETEXP(x) = floor(log(|x|)) 2Notation floor(x) stands for maximal integer not exceeding real number x.


## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal

## Operation

```C
// NormalizeExpTinyDPFP(SRC[63:0]) is defined in the Operation section of VGETEXPPD// ConvertExpDPFP(SRC[63:0]) is defined in the Operation section of VGETEXPPDVGETEXPSD (EVEX encoded version) IF k1[0] OR *no writemask*THEN DEST[63:0] :=ConvertExpDPFP(SRC2[63:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingDEST[63:0] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVGETEXPSD __m128d _mm_getexp_sd( __m128d a, __m128d b);VGETEXPSD __m128d _mm_mask_getexp_sd(__m128d s, __mmask8 k, __m128d a, __m128d b);VGETEXPSD __m128d _mm_maskz_getexp_sd( __mmask8 k, __m128d a, __m128d b);VGETEXPSD __m128d _mm_getexp_round_sd( __m128d a, __m128d b, int sae);VGETEXPSD __m128d _mm_mask_getexp_round_sd(__m128d s, __mmask8 k, __m128d a, __m128d b, int sae);VGETEXPSD __m128d _mm_maskz_getexp_round_sd( __mmask8 k, __m128d a, __m128d b, int sae);
```
