# PHADDSW

Packed Horizontal Add and Saturate

(V)PHADDSW adds two adjacent signed 16-bit integers horizontally from the source and destination operands and saturates the signed results; packs the signed, saturated 16-bit results to the destination operand (first operand) When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
Legacy SSE version: Both operands can be MMX registers.
The second source operand can be an MMX register or a 64-bit memory location.128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding YMM destina-tion register remain unchanged.
In 64-bit mode, use the REX prefix to access additional registers.VEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded version: The first source and destination operands are YMM registers.
The second source operand can be an YMM register or a 256-bit memory location.

## Exceptions

- SIMD Floating-Point Exceptions
  > None. 
- Other Exceptions

## Operation

```C
PHADDSW (With 64-bit Operands)mm1[15-0]  = SaturateToSignedWord((mm1[31-16] + mm1[15-0]); mm1[31-16] = SaturateToSignedWord(mm1[63-48] + mm1[47-32]);PHADDSW (With 128-bit Operands)xmm1[15-0]= SaturateToSignedWord(xmm1[31-16] + xmm1[15-0]);xmm1[31-16] = SaturateToSignedWord(xmm1[63-48] + xmm1[47-32]);xmm1[47-32] = SaturateToSignedWord(xmm1[95-80] + xmm1[79-64]);xmm1[63-48] = SaturateToSignedWord(xmm1[127-112] + xmm1[111-96]); xmm1[79-64] = SaturateToSignedWord(xmm2/m128[31-16] + xmm2/m128[15-0]);=SaturateToSignedWord(xmm2/m128[63-48]+xmm2/m128[47-32]);xmm1[95-80]    xmm1[111-96] = SaturateToSignedWord(xmm2/m128[95-80] + xmm2/m128[79-64]);xmm1[127-112] = SaturateToSignedWord(xmm2/m128[127-112] + xmm2/m128[111-96]); VPHADDSW (VEX.128 Encoded Version)DEST[15:0]= SaturateToSignedWord(SRC1[31:16] + SRC1[15:0])DEST[31:16] = SaturateToSignedWord(SRC1[63:48] + SRC1[47:32])DEST[47:32] = SaturateToSignedWord(SRC1[95:80] + SRC1[79:64])DEST[63:48] = SaturateToSignedWord(SRC1[127:112] + SRC1[111:96])DEST[79:64] = SaturateToSignedWord(SRC2[31:16] + SRC2[15:0])DEST[95:80] = SaturateToSignedWord(SRC2[63:48] + SRC2[47:32])DEST[111:96] = SaturateToSignedWord(SRC2[95:80] + SRC2[79:64])DEST[127:112] = SaturateToSignedWord(SRC2[127:112] + SRC2[111:96])DEST[MAXVL-1:128] := 0VPHADDSW (VEX.256 Encoded Version)DEST[15:0]= SaturateToSignedWord(SRC1[31:16] + SRC1[15:0])DEST[31:16] = SaturateToSignedWord(SRC1[63:48] + SRC1[47:32])DEST[47:32] = SaturateToSignedWord(SRC1[95:80] + SRC1[79:64])DEST[63:48] = SaturateToSignedWord(SRC1[127:112] + SRC1[111:96])DEST[79:64] = SaturateToSignedWord(SRC2[31:16] + SRC2[15:0])DEST[95:80] = SaturateToSignedWord(SRC2[63:48] + SRC2[47:32])DEST[111:96] = SaturateToSignedWord(SRC2[95:80] + SRC2[79:64])DEST[127:112] = SaturateToSignedWord(SRC2[127:112] + SRC2[111:96])DEST[143:128]= SaturateToSignedWord(SRC1[159:144] + SRC1[143:128])DEST[159:144] = SaturateToSignedWord(SRC1[191:176] + SRC1[175:160])DEST[175:160] = SaturateToSignedWord( SRC1[223:208] + SRC1[207:192])DEST[191:176] = SaturateToSignedWord(SRC1[255:240] + SRC1[239:224])DEST[207:192] = SaturateToSignedWord(SRC2[127:112] + SRC2[143:128])DEST[223:208] = SaturateToSignedWord(SRC2[159:144] + SRC2[175:160])DEST[239:224] = SaturateToSignedWord(SRC2[191-160] + SRC2[159-128])DEST[255:240] = SaturateToSignedWord(SRC2[255:240] + SRC2[239:224])Intel C/C++ Compiler Intrinsic EquivalentPHADDSW __m64 _mm_hadds_pi16 (__m64 a, __m64 b)(V)PHADDSW __m128i _mm_hadds_epi16 (__m128i a, __m128i b)VPHADDSW __m256i _mm256_hadds_epi16 (__m256i a, __m256i b)
```
