# ADDSUBPD

Packed Double Precision Floating-Point Add/Subtract

Adds odd-numbered double precision floating-point values of the first source operand (second operand) with the corresponding double precision floating-point values from the second source operand (third operand); stores the result in the odd-numbered values of the destination operand (first operand).
Subtracts the even-numbered double precision floating-point values from the second source operand from the corresponding double precision floating values in the first source operand; stores the result into the even-numbered values of the destination operand.
In 64-bit mode, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding YMM register destination are unmodified.
See Figure3-3.VEX.128 encoded version: the first source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding YMM register destination are zeroed.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM ADDSUBPD xmm1, xmm2/m128xmm2/m128[127:64][63:0]RESULT:xmm1[127:64] + xmm2/m128[127:64]xmm1[63:0] - xmm2/m128[63:0]xmm1[127:64][63:0]Figure 3-3.
 ADDSUBPD-Packed Double Precision Floating-Point Add/Subtract

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.

## Operation

```C
ADDSUBPD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] - SRC[63:0]DEST[127:64] := DEST[127:64] + SRC[127:64]DEST[MAXVL-1:128] (Unmodified)VADDSUBPD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] - SRC2[63:0]DEST[127:64] := SRC1[127:64] + SRC2[127:64]DEST[MAXVL-1:128] := 0VADDSUBPD (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0] - SRC2[63:0]DEST[127:64] := SRC1[127:64] + SRC2[127:64]DEST[191:128] := SRC1[191:128] - SRC2[191:128]DEST[255:192] := SRC1[255:192] + SRC2[255:192]Intel C/C++ Compiler Intrinsic EquivalentADDSUBPD __m128d _mm_addsub_pd(__m128d a, __m128d b)VADDSUBPD __m256d _mm256_addsub_pd (__m256d a, __m256d b)ExceptionsWhen the source operand is a memory operand, it must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
```
