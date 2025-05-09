# VSCALEFPD

Scale Packed Float64 Values With Float64 Values

Performs a floating-point scale of the packed double precision floating-point values in the first source operand by multiplying them by 2 to the power of the double precision floating-point values in second source operand.The equation of this operation is given by:floor(zmm3).zmm1 := zmm2*2Floor(zmm3) means maximum integer value  " zmm3.If the result cannot be represented in double precision, then the proper overflow response (for positive scaling operand), or the proper underflow response (for negative scaling operand) is issued.
The overflow and underflow responses are dependent on the rounding mode (for IEEE-compliant rounding), as well as on other settings in MXCSR (exception mask bits, FTZ bit), and on the SAE bit.The first source operand is a ZMM/YMM/XMM register.
The second source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.Handling of special-case input values are listed in Table 5-29 and Table 5-30.Table 5-29.
VSCALEFPD/SD/PS/SS Special CasesSrc2Set IE±NaN+Inf-Inf0/Denorm/NormSrc1±QNaNQNaN(Src1)+INF+0QNaN(Src1)IF either source is SNAN±SNaNQNaN(Src1)QNaN(Src1)QNaN(Src1)QNaN(Src1)YES±InfQNaN(Src2)Src1QNaN_IndefiniteSrc1IF Src2 is SNAN or -INF±0QNaN(Src2)QNaN_IndefiniteSrc1Src1IF Src2 is SNAN or +INFTable 5-30.
Additional VSCALEFPD/SD Special CasesSpecial Case Returned value Faults-1074|result| < 2±0 or ±Min-Denormal (Src1 sign)Underflow1024|result|   -  2±INF (Src1 sign) or ±Max-normal (Src1 sign)Overflow

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal (for Src1).
  > Denormal is not reported for Src2.

## Operation

```C
SCALE(SRC1, SRC2){TMP_SRC2 := SRC2TMP_SRC1 := SRC1IF (SRC2 is denormal AND MXCSR.DAZ) THEN TMP_SRC2=0IF (SRC1 is denormal AND MXCSR.DAZ) THEN TMP_SRC1=0/* SRC2 is a 64 bits floating-point value */DEST[63:0] := TMP_SRC1[63:0] * POW(2, Floor(TMP_SRC2[63:0]))}VSCALEFPD (EVEX encoded versions)(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) AND (SRC2 *is register*)THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+63:i] := SCALE(SRC1[i+63:i], SRC2[63:0]);ELSE DEST[i+63:i] := SCALE(SRC1[i+63:i], SRC2[i+63:i]);FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVSCALEFPD __m512d _mm512_scalef_round_pd(__m512d a, __m512d b, int rounding);VSCALEFPD __m512d _mm512_mask_scalef_round_pd(__m512d s, __mmask8 k, __m512d a, __m512d b, int rounding);VSCALEFPD __m512d _mm512_maskz_scalef_round_pd(__mmask8 k, __m512d a, __m512d b, int rounding);VSCALEFPD __m512d _mm512_scalef_pd(__m512d a, __m512d b);VSCALEFPD __m512d _mm512_mask_scalef_pd(__m512d s, __mmask8 k, __m512d a, __m512d b);VSCALEFPD __m512d _mm512_maskz_scalef_pd(__mmask8 k, __m512d a, __m512d b);VSCALEFPD __m256d _mm256_scalef_pd(__m256d a, __m256d b);VSCALEFPD __m256d _mm256_mask_scalef_pd(__m256d s, __mmask8 k, __m256d a, __m256d b);VSCALEFPD __m256d _mm256_maskz_scalef_pd(__mmask8 k, __m256d a, __m256d b);VSCALEFPD __m128d _mm_scalef_pd(__m128d a, __m128d b);VSCALEFPD __m128d _mm_mask_scalef_pd(__m128d s, __mmask8 k, __m128d a, __m128d b);VSCALEFPD __m128d _mm_maskz_scalef_pd(__mmask8 k, __m128d a, __m128d b);
```
