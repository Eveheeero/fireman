# VSCALEFSD

Scale Scalar Float64 Values With Float64 Values

Performs a floating-point scale of the scalar double precision floating-point value in the first source operand by multiplying it by 2 to the power of the double precision floating-point value in second source operand.The equation of this operation is given by:floor(xmm3).xmm1 := xmm2*2Floor(xmm3) means maximum integer value  " xmm3.If the result cannot be represented in double precision, then the proper overflow response (for positive scaling operand), or the proper underflow response (for negative scaling operand) is issued.
The overflow and underflow responses are dependent on the rounding mode (for IEEE-compliant rounding), as well as on other settings in MXCSR (exception mask bits, FTZ bit), and on the SAE bit.EVEX encoded version: The first source operand is an XMM register.
The second source operand is an XMM register or a memory location.
The destination operand is an XMM register conditionally updated with writemask k1.Handling of special-case input values are listed in Table 5-29 and Table 5-30.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal (for Src1).
  > Denormal is not reported for Src2.

## Operation

```C
SCALE(SRC1, SRC2){; Check for denormal operandsTMP_SRC2 := SRC2TMP_SRC1 := SRC1IF (SRC2 is denormal AND MXCSR.DAZ) THEN TMP_SRC2=0IF (SRC1 is denormal AND MXCSR.DAZ) THEN TMP_SRC1=0/* SRC2 is a 64 bits floating-point value */VSCALEFSD (EVEX encoded version)IF (EVEX.b= 1) and SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] OR *no writemask*THEN DEST[63:0] := SCALE(SRC1[63:0], SRC2[63:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingDEST[63:0] := 0FIFI;DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVSCALEFSD __m128d _mm_scalef_round_sd(__m128d a, __m128d b, int);VSCALEFSD __m128d _mm_mask_scalef_round_sd(__m128d s, __mmask8 k, __m128d a, __m128d b, int);VSCALEFSD __m128d _mm_maskz_scalef_round_sd(__mmask8 k, __m128d a, __m128d b, int);
```
