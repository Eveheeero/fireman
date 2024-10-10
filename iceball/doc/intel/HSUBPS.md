# HSUBPS

Packed Single Precision Floating-Point Horizontal Subtract

Subtracts the single precision floating-point value in the second dword of the destination operand from the first dword of the destination operand and stores the result in the first dword of the destination operand.
Subtracts the single precision floating-point value in the fourth dword of the destination operand from the third dword of the destination operand and stores the result in the second dword of the destination operand.
Subtracts the single precision floating-point value in the second dword of the source operand from the first dword of the source operand and stores the result in the third dword of the destination operand.
Subtracts the single precision floating-point value in the fourth dword of the source operand from the third dword of the source operand and stores the result in the fourth dword of the destination operand.
In 64-bit mode, use of the REX.R prefix permits this HSUBPS xmm1, xmm2/m128xmm2/[127:96][95:64]xmm2/m128[63:32][31:0]xmm1[95:64] - m128xmm1[31:0] - [95:64] - xmm2/xmm1[127:96]xmm1[63:32]m128[127:96]xmm1[127:96][95:64][63:32]xmm2/m128[31:0][31:0] - xmm2/m128[63:32]RESULT:xmm1[127:96][95:64][63:32][31:0]Figure 3-23.
 HSUBPS-Packed Single Precision Floating-Point Horizontal SubtractOM15996X7X6X5X4X3X2X1X0SRC1Y7Y6Y5Y4Y3Y2Y1Y0SRC2DESTY6-Y7Y4-Y5X6-X7X4-X5Y2-Y3Y0-Y1X2-X3X0-X1Figure 3-24.
 VHSUBPS Operation128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding YMM register destination are unmodified.VEX.128 encoded version: the first source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding YMM register destination are zeroed.VEX.256 encoded version: The first source operand is a 

## Exceptions

- Numeric Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.

## Operation

```C
HSUBPS (128-bit Legacy SSE Version)DEST[31:0] := SRC1[31:0] - SRC1[63:32]DEST[63:32] := SRC1[95:64] - SRC1[127:96]DEST[95:64] := SRC2[31:0] - SRC2[63:32]DEST[127:96] := SRC2[95:64] - SRC2[127:96] DEST[MAXVL-1:128] (Unmodified)VHSUBPS (VEX.128 Encoded Version)DEST[31:0] := SRC1[31:0] - SRC1[63:32]DEST[63:32] := SRC1[95:64] - SRC1[127:96]DEST[95:64] := SRC2[31:0] - SRC2[63:32]DEST[127:96] := SRC2[95:64] - SRC2[127:96] DEST[MAXVL-1:128] := 0VHSUBPS (VEX.256 Encoded Version)DEST[31:0] := SRC1[31:0] - SRC1[63:32]DEST[63:32] := SRC1[95:64] - SRC1[127:96]DEST[95:64] := SRC2[31:0] - SRC2[63:32]DEST[127:96] := SRC2[95:64] - SRC2[127:96] DEST[159:128] := SRC1[159:128] - SRC1[191:160]DEST[191:160] := SRC1[223:192] - SRC1[255:224]DEST[223:192] := SRC2[159:128] - SRC2[191:160]DEST[255:224] := SRC2[223:192] - SRC2[255:224]Intel C/C++ Compiler Intrinsic EquivalentHSUBPS __m128 _mm_hsub_ps(__m128 a, __m128 b);VHSUBPS __m256 _mm256_hsub_ps (__m256 a, __m256 b);ExceptionsWhen the source operand is a memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
```
