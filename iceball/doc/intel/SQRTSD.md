# SQRTSD

Compute Square Root of Scalar Double Precision Floating-Point Value

Computes the square root of the low double precision floating-point value in the second source operand and stores the double precision floating-point result in the destination operand.
The second source operand can be an XMM register or a 64-bit memory location.
The first source and destination operands are XMM registers.
128-bit Legacy SSE version: The first source operand and the destination operand are the same.
The quadword at bits 127:64 of the destination operand remains unchanged.
Bits (MAXVL-1:64) of the corresponding destination register remain unchanged.VEX.128 and EVEX encoded versions: Bits 127:64 of the destination operand are copied from the corresponding bits of the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: The low quadword element of the destination operand is updated according to the write-mask.Software should ensure VSQRTSD is encoded with VEX.L=0.
Encoding VSQRTSD with VEX.L=1 may encounter unpredictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision, Denormal.
- Other Exceptions

## Operation

```C
VSQRTSD (EVEX Encoded Version)IF (EVEX.b = 1) AND (SRC2 *is register*)THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[63:0] := SQRT(SRC2[63:0])ELSE IF *merging-masking*; merging-maskingTHEN DEST[63:0] := 0FI;FI;DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VSQRTSD (VEX.128 Encoded Version)DEST[63:0] := SQRT(SRC2[63:0])DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0SQRTSD (128-bit Legacy SSE Version)DEST[63:0] := SQRT(SRC[63:0])DEST[MAXVL-1:64] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSQRTSD __m128d _mm_sqrt_round_sd(__m128d a, __m128d b, int r);VSQRTSD __m128d _mm_mask_sqrt_round_sd(__m128d s, __mmask8 k, __m128d a, __m128d b, int r);VSQRTSD __m128d _mm_maskz_sqrt_round_sd(__mmask8 k, __m128d a, __m128d b, int r);SQRTSD __m128d _mm_sqrt_sd (__m128d a, __m128d b)
```
