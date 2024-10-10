# DIVSS

Divide Scalar Single Precision Floating-Point Values

Divides the low single precision floating-point value in the first source operand by the low single precision floating-point value in the second source operand, and stores the single precision floating-point result in the destination operand.
The second source operand can be an XMM register or a 32-bit memory location.128-bit Legacy SSE version: The first source operand and the destination operand are the same.
Bits (MAXVL-1:32) of the corresponding YMM destination register remain unchanged.
VEX.128 encoded version: The first source operand is an xmm register encoded by VEX.vvvv.
The three high-order doublewords of the destination operand are copied from the first source operand.
Bits (MAXVL-1:128) of the desti-nation register are zeroed.EVEX.128 encoded version: The first source operand is an xmm register encoded by EVEX.vvvv.
The doubleword elements of the destination operand at bits 127:32 are copied from the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX version: The low doubleword element of the destination is updated according to the writemask.Software should ensure VDIVSS is encoded with VEX.L=

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Divide-by-Zero, Precision, Denormal.

## Operation

```C
VDIVSS (EVEX Encoded Version)IF (EVEX.b = 1) AND SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := SRC1[31:0] / SRC2[31:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0VDIVSS (VEX.128 Encoded Version)DEST[31:0] := SRC1[31:0] / SRC2[31:0]DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0DIVSS (128-bit Legacy SSE Version)DEST[31:0] := DEST[31:0] / SRC[31:0]DEST[MAXVL-1:32] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVDIVSS __m128 _mm_mask_div_ss(__m128 s, __mmask8 k, __m128 a, __m128 b);VDIVSS __m128 _mm_maskz_div_ss( __mmask8 k, __m128 a, __m128 b);VDIVSS __m128 _mm_div_round_ss( __m128 a, __m128 b, int);VDIVSS __m128 _mm_mask_div_round_ss(__m128 s, __mmask8 k, __m128 a, __m128 b, int);VDIVSS __m128 _mm_maskz_div_round_ss( __mmask8 k, __m128 a, __m128 b, int);DIVSS __m128 _mm_div_ss(__m128 a, __m128 b);
```
