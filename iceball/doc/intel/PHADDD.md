# PHADDW/PHADDD

Packed Horizontal Add

(V)PHADDW adds two adjacent 16-bit signed integers horizontally from the source and destination operands and packs the 16-bit signed results to the destination operand (first operand).
(V)PHADDD adds two adjacent 32-bit signed integers horizontally from the source and destination operands and packs the 32-bit signed results to the destination operand (first operand).
When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.Note that these instructions can operate on either unsigned or signed (two's complement notation) integers; however, it does not set bits in the EFLAGS register to indicate overflow and/or a carry.
To prevent undetected over-flow conditions, software must control the ranges of the values operated on.
Legacy SSE instructions: Both operands can be MMX registers.
The second source operand can be an MMX register or a 64-bit memory location.128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand can be an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding YMM VEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand can be an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding YMM register are zeroed.VEX.256 encoded version: Horizontal addition of two adjacent data elements of the low 16-bytes of the first and second source operands are packed into the low 16-bytes of the destination operand.
Horizontal addition of two adjacent data elements of the high 16-bytes of the first and second source operands are packed into the high 16-bytes of the destination operand.
The first source and destination operands are YMM registers.
The second source operand can be an YMM register or a 256-bit memory location.
X7X6X5X4X3X2X1X0SRC2SRC1Y7Y6Y5Y4Y3Y2Y1Y0S0S7S2S1S3S3S4S32550DestFigure 4-10.
 256-bit VPHADDD Instruction Operation

## Exceptions

- SIMD Floating-Point Exceptions
  > None. 
- Other Exceptions

## Operation

```C
PHADDW (With 64-bit Operands)mm1[15-0]  = mm1[31-16] + mm1[15-0]; mm1[31-16] = mm1[63-48] + mm1[47-32]; mm1[47-32] = mm2/m64[31-16] + mm2/m64[15-0]; mm1[63-48] = mm2/m64[63-48] + mm2/m64[47-32]; PHADDW (With 128-bit Operands)xmm1[15-0] = xmm1[31-16] + xmm1[15-0]; xmm1[31-16] = xmm1[63-48] + xmm1[47-32]; xmm1[47-32] = xmm1[95-80] + xmm1[79-64]; xmm1[63-48] = xmm1[127-112] + xmm1[111-96]; xmm1[79-64] = xmm2/m128[31-16] + xmm2/m128[15-0]; xmm1[95-80] = xmm2/m128[63-48] + xmm2/m128[47-32]; xmm1[111-96] = xmm2/m128[95-80] + xmm2/m128[79-64]; xmm1[127-112] = xmm2/m128[127-112] + xmm2/m128[111-96]; VPHADDW (VEX.128 Encoded Version)DEST[15:0] := SRC1[31:16] + SRC1[15:0]DEST[31:16] := SRC1[63:48] + SRC1[47:32]DEST[47:32] := SRC1[95:80] + SRC1[79:64]DEST[63:48] := SRC1[127:112] + SRC1[111:96]DEST[79:64] := SRC2[31:16] + SRC2[15:0]DEST[95:80] := SRC2[63:48] + SRC2[47:32]DEST[111:96] := SRC2[95:80] + SRC2[79:64]DEST[127:112] := SRC2[1VPHADDW (VEX.256 Encoded Version)DEST[15:0] := SRC1[31:16] + SRC1[15:0]DEST[31:16] := SRC1[63:48] + SRC1[47:32]DEST[47:32] := SRC1[95:80] + SRC1[79:64]DEST[63:48] := SRC1[127:112] + SRC1[111:96]DEST[79:64] := SRC2[31:16] + SRC2[15:0]DEST[95:80] := SRC2[63:48] + SRC2[47:32]DEST[111:96] := SRC2[95:80] + SRC2[79:64]DEST[127:112] := SRC2[127:112] + SRC2[111:96]DEST[143:128] := SRC1[159:144] + SRC1[143:128]DEST[159:144] := SRC1[191:176] + SRC1[175:160]DEST[175:160] := SRC1[223:208] + SRC1[207:192]DEST[191:176] := SRC1[255:240] + SRC1[239:224]DEST[207:192] := SRC2[127:112] + SRC2[143:128]DEST[223:208] := SRC2[159:144] + SRC2[175:160]DEST[239:224] := SRC2[191:176] + SRC2[207:192]DEST[255:240] := SRC2[223:208] + SRC2[239:224]PHADDD (With 64-bit Operands)mm1[31-0]  = mm1[63-32] + mm1[31-0]; mm1[63-32] = mm2/m64[63-32] + mm2/m64[31-0]; PHADDD (With 128-bit Operands)xmm1[31-0] = xmm1[63-32] + xmm1[31-0]; xmm1[63-32] = xmm1[127-96] + xmm1[95-64]; xmm1[95-64] = xmm2/m128[63-32] + xmm2/m128[31-0]; xmm1[127-96] = xmm2/m128[127-96] + xmm2/m128[95-64]; VPHADDD (VEX.128 Encoded Version)DEST[31-0] := SRC1[63-32] + SRC1[31-0]DEST[63-32] := SRC1[127-96] + SRC1[95-64]DEST[95-64] := SRC2[63-32] + SRC2[31-0]DEST[127-96] := SRC2[127-96] + SRC2[95-64]DEST[MAXVL-1:128] := 0VPHADDD (VEX.256 Encoded Version)DEST[31-0] := SRC1[63-32] + SRC1[31-0]DEST[63-32] := SRC1[127-96] + SRC1[95-64]DEST[95-64] := SRC2[63-32] + SRC2[31-0]DEST[127-96] := SRC2[127-96] + SRC2[95-64]DEST[159-128] := SRC1[191-160] + SRC1[159-128]DEST[191-160] := SRC1[255-224] + SRC1[223-192]DEST[223-192] := SRC2[191-160] + SRC2[159-128]DEST[255-224] := SRC2[255-224] + SRC2[223-192]Intel C/C++ Compiler Intrinsic EquivalentsPHADDW __m64 _mm_hadd_pi16 (__m64 a, __m64 b)PHADDD __m64 _mm_hadd_pi32 (__m64 a, __m64 b)(V)PHADDW __m128i _mm_hadd_epi16 (__m128i a, __m128i b)(V)PHADDD __m128i _mm_hadd_epi32 (__m128i a, __m128i b)
```
