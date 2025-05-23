# VREDUCEPS

Perform Reduction Transformation on Packed Float32 Values

Perform reduction transformation of the packed binary encoded single-precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
The reduction transformation subtracts the integer part and the leading M fractional bits from the binary floating-point source value, where M is a unsigned integer specified by imm8[7:4], see Figure5-28.
Specifically, the reduc-tion transformation can be expressed as:M-M*src))*2;dest = src - (ROUND(2Mwhere "Round()" treats "src", "2", and their product as binary floating-point numbers with normalized signifi-cand and biased exponents.pThe magnitude of the reduced result can be expressed by considering src= 2*man2,where man2' is the normalized significand and p' is the unbiased exponent p-M-1Then if RC = RNE: 0<=|Reduced Result|<=2p-MThen if RC   RNE: 0<=|Reduced Result|<2This instruction might end up with a precision exception set.
However, in case of SPE set (i.e., Suppress Precision Exception, which is imm8[3]=1), no precision exception is reported.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
  > If SPE is enabled, precision exception is not 
  > reported (regardless of MXCSR exception mask).
- Other Exceptions

## Operation

```C
ReduceArgumentSP(SRC[31:0], imm8[7:0]){// Check for NaNIF (SRC [31:0] = NAN) THENRETURN (Convert SRC[31:0] to QNaN); FIM := imm8[7:4]; // Number of fraction bits of the normalized significand to be subtractedRC := imm8[1:0];// Round Control for ROUND() operationRC source := imm[2];SPE := imm[3];// Suppress Precision Exception-MMM  *{ROUND(2*SRC[31:0], SPE, RC_source, RC)}; // ROUND() treats SRC and 2as standard binary FP valuesTMP[31:0] := 2TMP[31:0] := SRC[31:0] - TMP[31:0]; // subtraction under the same RC,SPE controlsRETURN TMP[31:0]; // binary encoded FP with biased exponent and normalized significand}VREDUCEPS (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b == 1) AND (SRC *is memory*)THEN DEST[i+31:i] := ReduceArgumentSP(SRC[31:0], imm8[7:0]);ELSE DEST[i+31:i] := ReduceArgumentSP(SRC[i+31:i], imm8[7:0]);FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] = 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVREDUCEPS __m512 _mm512_mask_reduce_ps( __m512 a, int imm, int sae)VREDUCEPS __m512 _mm512_mask_reduce_ps(__m512 s, __mmask16 k, __m512 a, int imm, int sae)VREDUCEPS __m512 _mm512_maskz_reduce_ps(__mmask16 k, __m512 a, int imm, int sae)VREDUCEPS __m256 _mm256_mask_reduce_ps( __m256 a, int imm)VREDUCEPS __m256 _mm256_mask_reduce_ps(__m256 s, __mmask8 k, __m256 a, int imm)VREDUCEPS __m256 _mm256_maskz_reduce_ps(__mmask8 k, __m256 a, int imm)VREDUCEPS __m128 _mm_mask_reduce_ps( __m128 a, int imm)VREDUCEPS __m128 _mm_mask_reduce_ps(__m128 s, __mmask8 k, __m128 a, int imm)VREDUCEPS __m128 _mm_maskz_reduce_ps(__mmask8 k, __m128 a, int imm)
```
