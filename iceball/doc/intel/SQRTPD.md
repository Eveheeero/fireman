# SQRTPD

Square Root of Double Precision Floating-Point Values

Performs a SIMD computation of the square roots of the two, four or eight packed double precision floating-point values in the source operand (the second operand) stores the packed double precision floating-point results in the destination operand (the first operand).
EVEX encoded versions: The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register updated according to the writemask.VEX.256 encoded version: The source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: the source operand second source operand or a 128-bit memory location.
The destina-tion operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The second source can be an XMM register or 128-bit memory location.
The destina-tion is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding ZMM 

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-19, "T
  > ype 2 Class Exception Conditions," additionally:
  - #UD - If VEX.vvvv != 1111B.
- SIMD Floating-Point Exceptions
  > Invalid, Precision, Denormal.

## Operation

```C
VSQRTPD (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) AND (SRC *is register*)THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC *is memory*)THEN DEST[i+63:i] := SQRT(SRC[63:0])ELSE DEST[i+63:i] := SQRT(SRC[i+63:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VSQRTPD (VEX.256 Encoded Version)DEST[63:0] := SQRT(SRC[63:0])DEST[127:64] := SQRT(SRC[127:64])DEST[191:128] := SQRT(SRC[191:128])DEST[255:192] := SQRT(SRC[255:192])DEST[MAXVL-1:256] := 0.VSQRTPD (VEX.128 Encoded Version)DEST[63:0] := SQRT(SRC[63:0])DEST[127:64] := SQRT(SRC[127:64])DEST[MAXVL-1:128] := 0SQRTPD (128-bit Legacy SSE Version)DEST[63:0] := SQRT(SRC[63:0])DEST[127:64] := SQRT(SRC[127:64])DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSQRTPD __m512d _mm512_sqrt_round_pd(__m512d a, int r);VSQRTPD __m512d _mm512_mask_sqrt_round_pd(__m512d s, __mmask8 k, __m512d a, int r);VSQRTPD __m512d _mm512_maskz_sqrt_round_pd( __mmask8 k, __m512d a, int r);VSQRTPD __m256d _mm256_sqrt_pd (__m256d a);VSQRTPD __m256d _mm256_mask_sqrt_pd(__m256d s, __mmask8 k, __m256d a, int r);VSQRTPD __m256d _mm256_maskz_sqrt_pd( __mmask8 k, __m256d a, int r);SQRTPD __m128d _mm_sqrt_pd (__m128d a);VSQRTPD __m128d _mm_mask_sqrt_pd(__m128d s, __mmask8 k, __m128d a, int r);
```
