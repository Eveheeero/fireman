# SUBSD

Subtract Scalar Double Precision Floating-Point Value

Subtract the low double precision floating-point value in the second source operand from the first source operand and stores the double precision floating-point result in the low quadword of the destination operand.The second source operand can be an XMM register or a 64-bit memory location.
The first source and destination operands are XMM registers.
128-bit Legacy SSE version: The destination and first source operand are the same.
Bits (MAXVL-1:64) of the corresponding destination register remain unchanged.VEX.128 and EVEX encoded versions: Bits (127:64) of the XMM register destination are copied from corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: The low quadword element of the destination operand is updated according to the write-mask.Software should ensure VSUBSD is encoded with VEX.L=0.
Encoding VSUBSD with VEX.L=1 may encounter unpre-dictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.
- Other Exceptions

## Operation

```C
VSUBSD (EVEX Encoded Version)IF (SRC2 *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[63:0] := SRC1[63:0] - SRC2[63:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[63:0] := 0DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VSUBSD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] - SRC2[63:0]DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0SUBSD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] - SRC[63:0]DEST[MAXVL-1:64] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSUBSD __m128d _mm_mask_sub_sd (__m128d s, __mmask8 k, __m128d a, __m128d b);VSUBSD __m128d _mm_maskz_sub_sd (__mmask8 k, __m128d a, __m128d b);VSUBSD __m128d _mm_sub_round_sd (__m128d a, __m128d b, int);VSUBSD __m128d _mm_mask_sub_round_sd (__m128d s, __mmask8 k, __m128d a, __m128d b, int);VSUBSD __m128d _mm_maskz_sub_round_sd (__mmask8 k, __m128d a, __m128d b, int);SUBSD __m128d _mm_sub_sd (__m128d a, __m128d b);
```
