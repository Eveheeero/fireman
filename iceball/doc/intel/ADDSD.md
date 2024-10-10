# ADDSD

Add Scalar Double Precision Floating-Point Values

Adds the low double precision floating-point values from the second source operand and the first source operand and stores the double precision floating-point result in the destination operand.The second source operand can be an XMM register or a 64-bit memory location.
The first source and destination operands are XMM registers.
128-bit Legacy SSE version: The first source and destination operands are the same.
Bits (MAXVL-1:64) of the corresponding destination register remain unchanged.EVEX and VEX.128 encoded version: The first source operand is encoded by EVEX.vvvv/VEX.vvvv.
Bits (127:64) of the XMM register destination are copied from corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.
EVEX version: The low quadword element of the destination is updated according to the writemask.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.

## Operation

```C
VADDSD (EVEX Encoded Version)IF (EVEX.b = 1) AND SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[63:0] := SRC1[63:0] + SRC2[63:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[63:0] := 0FI;FI;DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VADDSD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] + SRC2[63:0]DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0ADDSD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] + SRC[63:0]DEST[MAXVL-1:64] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVADDSD __m128d _mm_mask_add_sd (__m128d s, __mmask8 k, __m128d a, __m128d b);VADDSD __m128d _mm_maskz_add_sd (__mmask8 k, __m128d a, __m128d b);VADDSD __m128d _mm_add_round_sd (__m128d a, __m128d b, int);VADDSD __m128d _mm_mask_add_round_sd (__m128d s, __mmask8 k, __m128d a, __m128d b, int);VADDSD __m128d _mm_maskz_add_round_sd (__mmask8 k, __m128d a, __m128d b, int);ADDSD __m128d _mm_add_sd (__m128d a, __m128d b);
```
