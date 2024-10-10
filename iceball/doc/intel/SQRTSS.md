# SQRTSS

Compute Square Root of Scalar Single Precision Value

Computes the square root of the low single precision floating-point value in the second source operand and stores the single precision floating-point result in the destination operand.
The second source operand can be an XMM register or a 32-bit memory location.
The first source and destination operands is an XMM register.
128-bit Legacy SSE version: The first source operand and the destination operand are the same.
Bits (MAXVL-1:32) of the corresponding YMM destination register remain unchanged.VEX.128 and EVEX encoded versions: Bits 127:32 of the destination operand are copied from the corresponding bits of the first source operand.
Bits (MAXVL-1:128) of the destination ZMM register are zeroed.EVEX encoded version: The low doubleword element of the destination operand is updated according to the write-mask.Software should ensure VSQRTSS is encoded with VEX.L=0.
Encoding VSQRTSS with VEX.L=1 may encounter unpredictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision, Denormal.
- Other Exceptions

## Operation

```C
VSQRTSS (EVEX Encoded Version)IF (EVEX.b = 1) AND (SRC2 *is register*)THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := SQRT(SRC2[31:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingFI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0VSQRTSS (VEX.128 Encoded Version)DEST[31:0] := SQRT(SRC2[31:0])DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0SQRTSS (128-bit Legacy SSE Version)DEST[31:0] := SQRT(SRC2[31:0])DEST[MAXVL-1:32] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSQRTSS __m128 _mm_sqrt_round_ss(__m128 a, __m128 b, int r);VSQRTSS __m128 _mm_mask_sqrt_round_ss(__m128 s, __mmask8 k, __m128 a, __m128 b, int r);VSQRTSS __m128 _mm_maskz_sqrt_round_ss( __mmask8 k, __m128 a, __m128 b, int r);SQRTSS __m128 _mm_sqrt_ss(__m128 a)
```
