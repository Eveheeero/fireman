# VSCALEFSS

Scale Scalar Float32 Value With Float32 Value

Performs a floating-point scale of the scalar single-precision floating-point value in the first source operand by multiplying it by 2 to the power of the float32 value in second source operand.The equation of this operation is given by:floor(xmm3).xmm1 := xmm2*2Floor(xmm3) means maximum integer value  " xmm3.If the result cannot be represented in single-precision, then the proper overflow response (for positive scaling operand), or the proper underflow response (for negative scaling operand) is issued.
The overflow and underflow responses are dependent on the rounding mode (for IEEE-compliant rounding), as well as on other settings in MXCSR (exception mask bits, FTZ bit), and on the SAE bit.EVEX encoded version: The first source operand is an XMM register.
The second source operand is an XMM register or a memory location.
The destination operand is an 

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal (for Src1).
  > Denormal is not reported for Src2.

## Operation

```C
SCALE(SRC1, SRC2){; Check for denormal operandsTMP_SRC2 := SRC2TMP_SRC1 := SRC1IF (SRC2 is denormal AND MXCSR.DAZ) THEN TMP_SRC2=0IF (SRC1 is denormal AND MXCSR.DAZ) THEN TMP_SRC1=0/* SRC2 is a 32 bits floating-point value */DEST[31:0] := TMP_SRC1[31:0] * POW(2, Floor(TMP_SRC2[31:0]))}VSCALEFSS (EVEX encoded version)IF (EVEX.b= 1) and SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] OR *no writemask*THEN DEST[31:0] := SCALE(SRC1[31:0], SRC2[31:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingDEST[31:0] := 0FIFI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVSCALEFSS __m128 _mm_scalef_round_ss(__m128 a, __m128 b, int);VSCALEFSS __m128 _mm_mask_scalef_round_ss(__m128 s, __mmask8 k, __m128 a, __m128 b, int);VSCALEFSS __m128 _mm_maskz_scalef_round_ss(__mmask8 k, __m128 a, __m128 b, int);
```
