# VSCALEFPS

Scale Packed Float32 Values With Float32 Values

Performs a floating-point scale of the packed single-precision floating-point values in the first source operand by multiplying them by 2 to the power of the float32 values in second source operand.The equation of this operation is given by:floor(zmm3).zmm1 := zmm2*2Floor(zmm3) means maximum integer value  " zmm3.If the result cannot be represented in single-precision, then the proper overflow response (for positive scaling operand), or the proper underflow response (for negative scaling operand) is issued.
The overflow and underflow responses are dependent on the rounding mode (for IEEE-compliant rounding), as well as on other settings in MXCSR (exception mask bits, FTZ bit), and on the SAE bit.EVEX.512 encoded version: The first source operand is a ZMM register.
The second source operand is a ZMM register, a 512-bit memory location or a 512-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM register conditionally updated with writemask k1.EVEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register, a 256-bit memory location, or a 256-bit vector broadcasted from a 32-bit memory location.
The destina-tion operand is a YMM register, conditionally updated using writemask k1.
EVEX.128 encoded version: The first source operand is an XMM register.
The second source operand is a XMM register, a 128-bit memory location, or a 128-bit vector broadcasted from a 32-bit memory location.
The destina-tion operand is a XMM register, conditionally updated using writemask k1.
Handling of special-case input values are listed in Table 5-29 and Table 5-33.Table 5-33.
Additional VSCALEFPS/SS Special CasesSpecial Case Returned value Faults-149|result| < 2±0 or ±Min-Denormal (Src1 sign)Underflow128|result|   -  2

## Exceptions

- SIMD Floating-Point Exceptions

## Operation

```C
SCALE(SRC1, SRC2){; Check for denormal operandsTMP_SRC2 := SRC2TMP_SRC1 := SRC1IF (SRC2 is denormal AND MXCSR.DAZ) THEN TMP_SRC2=0IF (SRC1 is denormal AND MXCSR.DAZ) THEN TMP_SRC1=0/* SRC2 is a 32 bits floating-point value */DEST[31:0] := TMP_SRC1[31:0] * POW(2, Floor(TMP_SRC2[31:0]))}VSCALEFPS (EVEX encoded versions)(KL, VL) = (4, 128), (8, 256), (16, 512)IF (VL = 512) AND (EVEX.b = 1) AND (SRC2 *is register*)THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+31:i] := SCALE(SRC1[i+31:i], SRC2[31:0]);ELSE DEST[i+31:i] := SCALE(SRC1[i+31:i], SRC2[i+31:i]);FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0;Intel C/C++ Compiler Intrinsic EquivalentVSCALEFPS __m512 _mm512_scalef_round_ps(__m512 a, __m512 b, int rounding);VSCALEFPS __m512 _mm512_mask_scalef_round_ps(__m512 s, __mmask16 k, __m512 a, __m512 b, int rounding);VSCALEFPS __m512 _mm512_maskz_scalef_round_ps(__mmask16 k, __m512 a, __m512 b, int rounding);VSCALEFPS __m512 _mm512_scalef_ps(__m512 a, __m512 b);VSCALEFPS __m512 _mm512_mask_scalef_ps(__m512 s, __mmask16 k, __m512 a, __m512 b);VSCALEFPS __m512 _mm512_maskz_scalef_ps(__mmask16 k, __m512 a, __m512 b);VSCALEFPS __m256 _mm256_scalef_ps(__m256 a, __m256 b);VSCALEFPS __m256 _mm256_mask_scalef_ps(__m256 s, __mmask8 k, __m256 a, __m256 b);VSCALEFPS __m256 _mm256_maskz_scalef_ps(__mmask8 k, __m256 a, __m256 b);VSCALEFPS __m128 _mm_scalef_ps(__m128 a, __m128 b);VSCALEFPS __m128 _mm_mask_scalef_ps(__m128 s, __mmask8 k, __m128 a, __m128 b);VSCALEFPS __m128 _mm_maskz_scalef_ps(__mmask8 k, __m128 a, __m128 b);
```
