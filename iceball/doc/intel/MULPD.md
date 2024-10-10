# MULPD

Multiply Packed Double Precision Floating-Point Values

Multiply packed double precision floating-point values from the first source operand with corresponding values in the second source operand, and stores the packed double precision floating-point results in the destination operand.EVEX encoded versions: The first source operand (the second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
Bits (MAXVL-1:256) of the corre-sponding destination ZMM register are zeroed.VEX.128 encoded version: The first source operand is a XMM register.
The second source operand can be a XMM register or a 128-bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:128) of the destination YMM register destination are zeroed.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM regist

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.
- Other Exceptions

## Operation

```C
VMULPD (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) AND SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THENDEST[i+63:i] := SRC1[i+63:i] * SRC2[63:0]ELSE DEST[i+63:i] := SRC1[i+63:i] * SRC2[i+63:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VMULPD (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0] * SRC2[63:0]DEST[127:64] := SRC1[127:64] * SRC2[127:64]DEST[191:128] := SRC1[191:128] * SRC2[191:128]DEST[255:192] := SRC1[255:192] * SRC2[255:192]DEST[MAXVL-1:256] := 0;.VMULPD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] * SRC2[63:0]DEST[127:64] := SRC1[127:64] * SRC2[127:64]DEST[MAXVL-1:128] := 0MULPD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] * SRC[63:0]DEST[127:64] := DEST[127:64] * SRC[127:64]Intel C/C++ Compiler Intrinsic EquivalentVMULPD __m512d _mm512_mul_pd( __m512d a, __m512d b);VMULPD __m512d _mm512_mask_mul_pd(__m512d s, __mmask8 k, __m512d a, __m512d b);VMULPD __m512d _mm512_maskz_mul_pd( __mmask8 k, __m512d a, __m512d b);VMULPD __m512d _mm512_mul_round_pd( __m512d a, __m512d b, int);VMULPD __m512d _mm512_mask_mul_round_pd(__m512d s, __mmask8 k, __m512d a, __m512d b, int);VMULPD __m512d _mm512_maskz_mul_round_pd( __mmask8 k, __m512d a, __m512d b, int);VMULPD __m256d _mm256_mul_pd (__m256d a, __m256d b);MULPD __m128d _mm_mul_pd (__m128d a, __m128d b);
```
