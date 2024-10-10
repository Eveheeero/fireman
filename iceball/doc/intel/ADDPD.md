# ADDPD

Add Packed Double Precision Floating-Point Values

Adds two, four or eight packed double precision floating-point values from the first source operand to the second source operand, and stores the packed double precision floating-point result in the destination operand.EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: the first source operand is a XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM regist

## Exceptions

- Other Exceptions
  > VEX-encoded instruction, see Table2-19,
- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.

## Operation

```C
VADDPD (EVEX Encoded Versions) When SRC2 Operand is a Vector Register(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VADDPD (EVEX Encoded Versions) When SRC2 Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+63:i] := SRC1[i+63:i] + SRC2[63:0]ELSE DEST[i+63:i] := SRC1[i+63:i] + SRC2[i+63:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VADDPD (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0] + SRC2[63:0]DEST[127:64] := SRC1[127:64] + SRC2[127:64]DEST[191:128] := SRC1[191:128] + SRC2[191:128]DEST[255:192] := SRC1[255:192] + SRC2[255:192]VADDPD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] + SRC2[63:0]DEST[127:64] := SRC1[127:64] + SRC2[127:64]DEST[MAXVL-1:128] := 0ADDPD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] + SRC[63:0]DEST[127:64] := DEST[127:64] + SRC[127:64]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVADDPD __m512d _mm512_add_pd (__m512d a, __m512d b);VADDPD __m512d _mm512_mask_add_pd (__m512d s, __mmask8 k, __m512d a, __m512d b);VADDPD __m512d _mm512_maskz_add_pd (__mmask8 k, __m512d a, __m512d b);VADDPD __m256d _mm256_mask_add_pd (__m256d s, __mmask8 k, __m256d a, __m256d b);VADDPD __m256d _mm256_maskz_add_pd (__mmask8 k, __m256d a, __m256d b);VADDPD __m128d _mm_mask_add_pd (__m128d s, __mmask8 k, __m128d a, __m128d b);VADDPD __m128d _mm_maskz_add_pd (__mmask8 k, __m128d a, __m128d b);VADDPD __m512d _mm512_add_round_pd (__m512d a, __m512d b, int);VADDPD __m512d _mm512_mask_add_round_pd (__m512d s, __mmask8 k, __m512d a, __m512d b, int);VADDPD __m512d _mm512_maskz_add_round_pd (__mmask8 k, __m512d a, __m512d b, int);ADDPD __m256d _mm256_add_pd (__m256d a, __m256d b);ADDPD __m128d _mm_add_pd (__m128d a, __m128d b);
```
