# MULSS

Multiply Scalar Single Precision Floating-Point Values

Multiplies the low single precision floating-point value from the second source operand by the low single precision floating-point value in the first source operand, and stores the single precision floating-point result in the destina-tion operand.
The second source operand can be an XMM register or a 32-bit memory location.
The first source operand and the destination operands are XMM registers.
128-bit Legacy SSE version: The first source operand and the destination operand are the same.
Bits (MAXVL-1:32) of the corresponding YMM destination register remain unchanged.VEX.128 and EVEX encoded version: The first source operand is an xmm register encoded by VEX.vvvv.
The three high-order doublewords of the destination operand are copied from the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: The low doubleword element of the destination operand is updated according to the write-mask.Software should ensure VMULSS is encoded with VEX.L=0.
Encoding VMULSS with VEX.L=1 may encounter unpre-dictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Underflow, Overflow, Invalid, Precision, Denormal.
- Other Exceptions

## Operation

```C
VMULSS (EVEX Encoded Version)IF (EVEX.b = 1) AND SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := SRC1[31:0] * SRC2[31:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*FIFI;ENDFORDEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0VMULSS (VEX.128 Encoded Version)DEST[31:0] := SRC1[31:0] * SRC2[31:0]DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0MULSS (128-bit Legacy SSE Version)DEST[31:0] := DEST[31:0] * SRC[31:0]DEST[MAXVL-1:32] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVMULSS __m128 _mm_mask_mul_ss(__m128 s, __mmask8 k, __m128 a, __m128 b);VMULSS __m128 _mm_maskz_mul_ss( __mmask8 k, __m128 a, __m128 b);VMULSS __m128 _mm_mul_round_ss( __m128 a, __m128 b, int);VMULSS __m128 _mm_mask_mul_round_ss(__m128 s, __mmask8 k, __m128 a, __m128 b, int);VMULSS __m128 _mm_maskz_mul_round_ss( __mmask8 k, __m128 a, __m128 b, int);MULSS __m128 _mm_mul_ss(__m128 a, __m128 b)
```
