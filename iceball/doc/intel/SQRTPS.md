# SQRTPS

Square Root of Single Precision Floating-Point Values

Performs a SIMD computation of the square roots of the four, eight or sixteen packed single precision floating-point values in the source operand (second operand) stores the packed single precision floating-point results in the desti-nation operand.
EVEX.512 encoded versions: The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register updated according to the writemask.VEX.256 encoded version: The source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: the source operand second source operand or a 128-bit memory location.
The destina-tion operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The second source can be an XMM register or 128-bit memory location.
The destina-tion is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding ZMM 

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision, Denormal.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-19, "Type 2 Class Exception Conditions," additionally:
  - #UD - If VEX.vvvv != 1111B.

## Operation

```C
VSQRTPS (EVEX Encoded Versions)(KL, VL) = (4, 128), (8, 256), (16, 512)IF (VL = 512) AND (EVEX.b = 1) AND (SRC *is register*)THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC *is memory*)THEN DEST[i+31:i] := SQRT(SRC[31:0])ELSE DEST[i+31:i] := SQRT(SRC[i+31:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VSQRTPS (VEX.256 Encoded Version)DEST[31:0] := SQRT(SRC[31:0])DEST[63:32] := SQRT(SRC[63:32])DEST[95:64] := SQRT(SRC[95:64])DEST[127:96] := SQRT(SRC[127:96])DEST[159:128] := SQRT(SRC[159:128])DEST[191:160] := SQRT(SRC[191:160])DEST[223:192] := SQRT(SRC[223:192])DEST[255:224] := SQRT(SRC[255:224])VSQRTPS (VEX.128 Encoded Version)DEST[31:0] := SQRT(SRC[31:0])DEST[63:32] := SQRT(SRC[63:32])DEST[95:64] := SQRT(SRC[95:64])DEST[127:96] := SQRT(SRC[127:96])DEST[MAXVL-1:128] := 0SQRTPS (128-bit Legacy SSE Version)DEST[31:0] := SQRT(SRC[31:0])DEST[63:32] := SQRT(SRC[63:32])DEST[95:64] := SQRT(SRC[95:64])DEST[127:96] := SQRT(SRC[127:96])Intel C/C++ Compiler Intrinsic EquivalentVSQRTPS __m512 _mm512_sqrt_round_ps(__m512 a, int r);VSQRTPS __m512 _mm512_mask_sqrt_round_ps(__m512 s, __mmask16 k, __m512 a, int r);VSQRTPS __m512 _mm512_maskz_sqrt_round_ps( __mmask16 k, __m512 a, int r);VSQRTPS __m256 _mm256_sqrt_ps (__m256 a);VSQRTPS __m256 _mm256_mask_sqrt_ps(__m256 s, __mmask8 k, __m256 a, int r);VSQRTPS __m256 _mm256_maskz_sqrt_ps( __mmask8 k, __m256 a, int r);SQRTPS __m128 _mm_sqrt_ps (__m128 a);VSQRTPS __m128 _mm_mask_sqrt_ps(__m128 s, __mmask8 k, __m128 a, int r);VSQRTPS __m128 _mm_maskz_sqrt_ps( __mmask8 k, __m128 a, int r);
```
