# VREDUCESS

Perform a Reduction Transformation on a Scalar Float32 Value

Perform a reduction transformation of the binary encoded single-precision floating-point value in the low dword element of the second source operand (the third operand) and store the reduced result in binary floating-point format to the low dword element of the destination operand (the first operand) under the writemask k1.
Bits 127:32 of the destination operand are copied from respective dword elements of the first source operand (the second operand).
The reduction transformation subtracts the integer part and the leading M fractional bits from the binary floating-point source value, where M is a unsigned integer specified by imm8[7:4], see Figure5-28.
Specifically, the reduc-tion transformation can be expressed as:M-M*src))*2;dest = src - (ROUND(2Mwhere "Round()" treats "src", "2", and their product as binary floating-point numbers with normalized signifi-cand and biased exponents.pThe magnitude of the reduced result can be expressed by considering src= 2*man2,where man2' is the normalized significand and p' is the unbiased exponent p-M-1Then if RC = RNE: 0<=|Reduced Result|<=2p-MThen if RC   RNE: 0<=|Reduced Result|<2This instruction might end up with a precision exception set.
However, in case of SPE set (i.e., Suppress Precision Exception, which is imm8[3]=1), no precision exception is reported.Handling of special case of input values are listed in Table 5-19.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
  > If SPE is enabled, precision exception is not 
  > reported (regardless of MXCSR exception mask).

## Operation

```C
ReduceArgumentSP(SRC[31:0], imm8[7:0]){// Check for NaNIF (SRC [31:0] = NAN) THENRETURN (Convert SRC[31:0] to QNaN); FIM := imm8[7:4]; // Number of fraction bits of the normalized significand to be subtractedRC := imm8[1:0];// Round Control for ROUND() operationRC source := imm[2];SPE := imm[3];// Suppress Precision Exception-MMM  *{ROUND(2*SRC[31:0], SPE, RC_source, RC)}; // ROUND() treats SRC and 2as standard binary FP valuesTMP[31:0] := 2TMP[31:0] := SRC[31:0] - TMP[31:0]; // subtraction under the same RC,SPE controlsVREDUCESS IF k1[0] or *no writemask*THENDEST[31:0] := ReduceArgumentSP(SRC2[31:0], imm8[7:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] = 0FI;FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVREDUCESS __m128 _mm_mask_reduce_ss( __m128 a, __m128 b, int imm, int sae)VREDUCESS __m128 _mm_mask_reduce_ss(__m128 s, __mmask16 k, __m128 a, __m128 b, int imm, int sae)VREDUCESS __m128 _mm_maskz_reduce_ss(__mmask16 k, __m128 a, __m128 b, int imm, int sae)
```
