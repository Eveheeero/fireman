# DIVPD

Divide Packed Double Precision Floating-Point Values

Performs a SIMD divide of the double precision floating-point values in the first source operand by the floating-point values in the second source operand (the third operand).
Results are written to the destination operand (the first operand).EVEX encoded versions: The first source operand (the second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.VEX.256 encoded version: The first source operand (the second operand) is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding destination are zeroed.VEX.128 encoded version: The first source operand (the second operand) is a XMM register.
The second source operand can be a XMM register or a 128-bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:128) of the corresponding destination are zeroed.128-bit Legacy SSE version: The second source operand (the second operand) can be an XMM register or an 128-bit memory location.
The destination is the same as the fi

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-19, "Type 2 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Divide-by-Zero, Precision, Denormal.

## Operation

```C
VDIVPD (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) AND SRC2 *is a register*THEN® 64 and IA-32 Architectures SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);  ; refer to Table 15-4 in the IntelSoftware Developer's Manual, Volume 1ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THENDEST[i+63:i] := SRC1[i+63:i] / SRC2[63:0]ELSE DEST[i+63:i] := SRC1[i+63:i] / SRC2[i+63:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VDIVPD (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0] / SRC2[63:0]DEST[127:64] := SRC1[127:64] / SRC2[127:64]DEST[191:128] := SRC1[191:128] / SRC2[191:128]DEST[255:192] := SRC1[255:192] / SRC2[255:192]DEST[MAXVL-1:256] := 0;VDIVPD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] / SRC2[63:0]DEST[127:64] := SRC1[127:64] / SRC2[127:64]DEST[MAXVL-1:128] := 0;DIVPD (128-bit Legacy SSE Version)DEST[63:0] := SRC1[63:0] / SRC2[63:0]Intel C/C++ Compiler Intrinsic EquivalentVDIVPD __m512d _mm512_div_pd( __m512d a, __m512d b);VDIVPD __m512d _mm512_mask_div_pd(__m512d s, __mmask8 k, __m512d a, __m512d b);VDIVPD __m512d _mm512_maskz_div_pd( __mmask8 k, __m512d a, __m512d b);VDIVPD __m256d _mm256_mask_div_pd(__m256d s, __mmask8 k, __m256d a, __m256d b);VDIVPD __m256d _mm256_maskz_div_pd( __mmask8 k, __m256d a, __m256d b);VDIVPD __m128d _mm_mask_div_pd(__m128d s, __mmask8 k, __m128d a, __m128d b);VDIVPD __m128d _mm_maskz_div_pd( __mmask8 k, __m128d a, __m128d b);VDIVPD __m512d _mm512_div_round_pd( __m512d a, __m512d b, int);VDIVPD __m512d _mm512_mask_div_round_pd(__m512d s, __mmask8 k, __m512d a, __m512d b, int);VDIVPD __m512d _mm512_maskz_div_round_pd( __mmask8 k, __m512d a, __m512d b, int);VDIVPD __m256d _mm256_div_pd (__m256d a, __m256d b);DIVPD __m128d _mm_div_pd (__m128d a, __m128d b);
```
