# SUBPS

Subtract Packed Single Precision Floating-Point Values

Performs a SIMD subtract of the packed single precision floating-point values in the second Source operand from the First Source operand, and stores the packed single precision floating-point results in the destination operand.VEX.128 and EVEX.128 encoded versions: The second source operand is an XMM register or an 128-bit memory location.
The first source operand and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corre-sponding destination register are zeroed.VEX.256 and EVEX.256 encoded versions: The second source operand is an YMM register or an 256-bit memory location.
The first source operand and destination operands are YMM registers.
Bits (MAXVL-1:256) of the corre-sponding destination register are zeroed.EVEX.512 encoded version: The second source operand is a ZMM register, a 512-bit memory location or a 512-bit vector broadcasted from a 32-bit memory location.
The first source operand and destination operands are ZMM registers.
The destination operand is conditionally updated according to the writemask.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper Bits (MAXVL-1:128) of the corresponding register destination are unmodified.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.
- Other Exceptions

## Operation

```C
VSUBPS (EVEX Encoded Versions When SRC2 Operand is a Vector Register)(KL, VL) = (4, 128), (8, 256), (16, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := SRC1[i+31:i] - SRC2[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingDEST[31:0] := 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0VSUBPS (EVEX Encoded Versions When SRC2 Operand is a Memory Source)(KL, VL) = (4, 128), (8, 256),(16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask* THENIF (EVEX.b = 1)THEN DEST[i+31:i] := SRC1[i+31:i] - SRC2[31:0];ELSE DEST[i+31:i] := SRC1[i+31:i] - SRC2[i+31:i];FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingDEST[31:0] := 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0VSUBPS (VEX.256 Encoded Version)DEST[31:0] := SRC1[31:0] - SRC2[31:0]DEST[63:32] := SRC1[63:32] - SRC2[63:32]DEST[95:64] := SRC1[95:64] - SRC2[95:64]DEST[127:96] := SRC1[127:96] - SRC2[127:96]DEST[159:128] := SRC1[159:128] - SRC2[159:128]DEST[191:160] := SRC1[191:160] - SRC2[191:160]DEST[223:192] := SRC1[223:192] - SRC2[223:192]DEST[255:224] := SRC1[255:224] - SRC2[255:224].DEST[MAXVL-1:256] := 0VSUBPS (VEX.128 Encoded Version)DEST[31:0] := SRC1[31:0] - SRC2[31:0]DEST[63:32] := SRC1[63:32] - SRC2[63:32]DEST[95:64] := SRC1[95:64] - SRC2[95:64]SUBPS (128-bit Legacy SSE Version)DEST[31:0] := SRC1[31:0] - SRC2[31:0]DEST[63:32] := SRC1[63:32] - SRC2[63:32]DEST[95:64] := SRC1[95:64] - SRC2[95:64]DEST[127:96] := SRC1[127:96] - SRC2[127:96]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSUBPS __m512 _mm512_sub_ps (__m512 a, __m512 b);VSUBPS __m512 _mm512_mask_sub_ps (__m512 s, __mmask16 k, __m512 a, __m512 b);VSUBPS __m512 _mm512_maskz_sub_ps (__mmask16 k, __m512 a, __m512 b);VSUBPS __m512 _mm512_sub_round_ps (__m512 a, __m512 b, int);VSUBPS __m512 _mm512_mask_sub_round_ps (__m512 s, __mmask16 k, __m512 a, __m512 b, int);VSUBPS __m512 _mm512_maskz_sub_round_ps (__mmask16 k, __m512 a, __m512 b, int);VSUBPS __m256 _mm256_sub_ps (__m256 a, __m256 b);VSUBPS __m256 _mm256_mask_sub_ps (__m256 s, __mmask8 k, __m256 a, __m256 b);VSUBPS __m256 _mm256_maskz_sub_ps (__mmask16 k, __m256 a, __m256 b);SUBPS __m128 _mm_sub_ps (__m128 a, __m128 b);VSUBPS __m128 _mm_mask_sub_ps (__m128 s, __mmask8 k, __m128 a, __m128 b);VSUBPS __m128 _mm_maskz_sub_ps (__mmask16 k, __m128 a, __m128 b);
```
