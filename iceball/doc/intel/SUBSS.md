# SUBSS

Subtract Scalar Single Precision Floating-Point Value

Subtract the low single precision floating-point value from the second source operand and the first source operand and store the double precision floating-point result in the low doubleword of the destination operand.The second source operand can be an XMM register or a 32-bit memory location.
The first source and destination operands are XMM registers.
128-bit Legacy SSE version: The destination and first source operand are the same.
Bits (MAXVL-1:32) of the corresponding destination register remain unchanged.VEX.128 and EVEX encoded versions: Bits (127:32) of the XMM register destination are copied from corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: The low doubleword element of the destination operand is updated according to the write-mask.Software should ensure VSUBSS is encoded with VEX.L=0.
Encoding VSUBSD with VEX.L=1 may encounter unpre-dictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.
- Other Exceptions

## Operation

```C
VSUBSS (EVEX Encoded Version)IF (SRC2 *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := SRC1[31:0] - SRC2[31:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0VSUBSS (VEX.128 Encoded Version)DEST[31:0] := SRC1[31:0] - SRC2[31:0]DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0SUBSS (128-bit Legacy SSE Version)DEST[31:0] := DEST[31:0] - SRC[31:0]DEST[MAXVL-1:32] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSUBSS __m128 _mm_mask_sub_ss (__m128 s, __mmask8 k, __m128 a, __m128 b);VSUBSS __m128 _mm_maskz_sub_ss (__mmask8 k, __m128 a, __m128 b);VSUBSS __m128 _mm_sub_round_ss (__m128 a, __m128 b, int);VSUBSS __m128 _mm_mask_sub_round_ss (__m128 s, __mmask8 k, __m128 a, __m128 b, int);VSUBSS __m128 _mm_maskz_sub_round_ss (__mmask8 k, __m128 a, __m128 b, int);SUBSS __m128 _mm_sub_ss (__m128 a, __m128 b);
```
