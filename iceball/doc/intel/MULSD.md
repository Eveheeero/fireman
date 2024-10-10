# MULSD

Multiply Scalar Double Precision Floating-Point Value

Multiplies the low double precision floating-point value in the second source operand by the low double precision floating-point value in the first source operand, and stores the double precision floating-point result in the destina-tion operand.
The second source operand can be an XMM register or a 64-bit memory location.
The first source operand and the destination operands are XMM registers.
128-bit Legacy SSE version: The first source operand and the destination operand are the same.
Bits (MAXVL-1:64) of the corresponding destination register remain unchanged.VEX.128 and EVEX encoded version: The quadword at bits 127:64 of the destination operand is copied from the same bits of the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: The low quadword element of the destination operand is updated according to the write-mask.Software should ensure VMULSD is encoded with VEX.L=0.
Encoding VMULSD with VEX.L=1 may encounter unpre-dictable behavior across different processor generations.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.

## Operation

```C
VMULSD (EVEX Encoded Version)IF (EVEX.b = 1) AND SRC2 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[63:0] := SRC1[63:0] * SRC2[63:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingFI;ENDFORDEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VMULSD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] * SRC2[63:0]DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0MULSD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] * SRC[63:0]DEST[MAXVL-1:64] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVMULSD __m128d _mm_mask_mul_sd(__m128d s, __mmask8 k, __m128d a, __m128d b);VMULSD __m128d _mm_maskz_mul_sd( __mmask8 k, __m128d a, __m128d b);VMULSD __m128d _mm_mul_round_sd( __m128d a, __m128d b, int);VMULSD __m128d _mm_mask_mul_round_sd(__m128d s, __mmask8 k, __m128d a, __m128d b, int);VMULSD __m128d _mm_maskz_mul_round_sd( __mmask8 k, __m128d a, __m128d b, int);MULSD __m128d _mm_mul_sd (__m128d a, __m128d b)
```
