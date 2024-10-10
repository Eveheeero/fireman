# DIVSD

Divide Scalar Double Precision Floating-Point Value

Divides the low double precision floating-point value in the first source operand by the low double precision floating-point value in the second source operand, and stores the double precision floating-point result in the desti-nation operand.
The second source operand can be an XMM register or a 64-bit memory location.
The first source and destination are XMM registers.
128-bit Legacy SSE version: The first source operand and the destination operand are the same.
Bits (MAXVL-1:64) of the corresponding ZMM destination register remain unchanged.VEX.128 encoded version: The first source operand is an xmm register encoded by VEX.vvvv.
The quadword at bits 127:64 of the destination operand is copied from the corresponding quadword of the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX.128 encoded version: The first source operand is an xmm register encoded by EVEX.vvvv.
The quadword element of the destination operand at bits 127:64 are copied from the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX version: The low quadword element of the destination is updated according to the writemask.Software should ensure VDIVSD is encoded with VEX.L=

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Divide-by-Zero, Precision, Denormal.
- Other Exceptions

## Operation

```C
VDIVSD (EVEX Encoded Version)IF (EVEX.b = 1) AND SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[63:0] := SRC1[63:0] / SRC2[63:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[63:0] := 0FI;FI;DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VDIVSD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] / SRC2[63:0]DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0DIVSD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] / SRC[63:0]DEST[MAXVL-1:64] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVDIVSD __m128d _mm_mask_div_sd(__m128d s, __mmask8 k, __m128d a, __m128d b);VDIVSD __m128d _mm_maskz_div_sd( __mmask8 k, __m128d a, __m128d b);VDIVSD __m128d _mm_div_round_sd( __m128d a, __m128d b, int);VDIVSD __m128d _mm_mask_div_round_sd(__m128d s, __mmask8 k, __m128d a, __m128d b, int);VDIVSD __m128d _mm_maskz_div_round_sd( __mmask8 k, __m128d a, __m128d b, int);DIVSD __m128d _mm_div_sd (__m128d a, __m128d b);
```
