# ADDSUBPS

Packed Single Precision Floating-Point Add/Subtract

Adds odd-numbered single precision floating-point values of the first source operand (second operand) with the corresponding single precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand).
Subtracts the even-numbered single precision floating-point values from the second source operand from the corresponding single precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.In 64-bit mode, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding YMM register destination are unmodified.
See Figure3-4.VEX.128 encoded version: the first source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding YMM register destination are zeroed.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM ADDSUBPS xmm1, xmm2/m128xmm2/[127:96][95:64]xmm1[127:96] + [63:32]xmm1[95:64] - xmm2/[31:0]xmm1[63:32] + m128xmm1[31:0] - xmm2/m128[127:96]m128[95:64]xmm2/m128[63:32]xmm2/m128[31:0]RESULT:xmm1[127:96][95:64][63:32][31:0]OM15992Figure 3-4.
 ADDSUBPS-Packed Single Precision Floating-Point Add/Subtract

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.

## Operation

```C
ADDSUBPS (128-bit Legacy SSE Version)DEST[31:0] := DEST[31:0] - SRC[31:0]DEST[63:32] := DEST[63:32] + SRC[63:32]DEST[95:64] := DEST[95:64] - SRC[95:64]DEST[127:96] := DEST[127:96] + SRC[127:96]DEST[MAXVL-1:128] (Unmodified)VADDSUBPS (VEX.128 Encoded Version)DEST[31:0] := SRC1[31:0] - SRC2[31:0]DEST[63:32] := SRC1[63:32] + SRC2[63:32]DEST[95:64] := SRC1[95:64] - SRC2[95:64]DEST[127:96] := SRC1[127:96] + SRC2[127:96]DEST[MAXVL-1:128] := 0VADDSUBPS (VEX.256 Encoded Version)DEST[31:0] := SRC1[31:0] - SRC2[31:0]DEST[63:32] := SRC1[63:32] + SRC2[63:32]DEST[95:64] := SRC1[95:64] - SRC2[95:64]DEST[127:96] := SRC1[127:96] + SRC2[127:96]DEST[159:128] := SRC1[159:128] - SRC2[159:128]DEST[191:160] := SRC1[191:160] + SRC2[191:160]DEST[223:192] := SRC1[223:192] - SRC2[223:192]DEST[255:224] := SRC1[255:224] + SRC2[255:224]Intel C/C++ Compiler Intrinsic EquivalentADDSUBPS __m128 _mm_addsub_ps(__m128 a, __m128 b)VADDSUBPS __m256 _mm256_addsub_ps (__m256 a, __m256 b)ExceptionsWhen the source operand is a memory operand, the oper
```
