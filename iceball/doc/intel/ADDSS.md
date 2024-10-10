# ADDSS

Add Scalar Single Precision Floating-Point Values

Adds the low single precision floating-point values from the second source operand and the first source operand, and stores the double precision floating-point result in the destination operand.The second source operand can be an XMM register or a 64-bit memory location.
The first source and destination operands are XMM registers.
128-bit Legacy SSE version: The first source and destination operands are the same.
Bits (MAXVL-1:32) of the corresponding the destination register remain unchanged.EVEX and VEX.128 encoded version: The first source operand is encoded by EVEX.vvvv/VEX.vvvv.
Bits (127:32) of the XMM register destination are copied from corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX version: The low doubleword element of the destination is updated according to the writemask.Software should ensure VADDSS is encoded with VEX.L=

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.
- Other Exceptions

## Operation

```C
VADDSS (EVEX Encoded Versions)IF (EVEX.b = 1) AND SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := SRC1[31:0] + SRC2[31:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0VADDSS DEST, SRC1, SRC2 (VEX.128 Encoded Version)DEST[31:0] := SRC1[31:0] + SRC2[31:0]DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0ADDSS DEST, SRC (128-bit Legacy SSE Version)DEST[31:0] := DEST[31:0] + SRC[31:0]DEST[MAXVL-1:32] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVADDSS __m128 _mm_mask_add_ss (__m128 s, __mmask8 k, __m128 a, __m128 b);VADDSS __m128 _mm_maskz_add_ss (__mmask8 k, __m128 a, __m128 b);VADDSS __m128 _mm_add_round_ss (__m128 a, __m128 b, int);VADDSS __m128 _mm_mask_add_round_ss (__m128 s, __mmask8 k, __m128 a, __m128 b, int);VADDSS __m128 _mm_maskz_add_round_ss (__mmask8 k, __m128 a, __m128 b, int);ADDSS __m128 _mm_add_ss (__m128 a, __m128 b);
```
