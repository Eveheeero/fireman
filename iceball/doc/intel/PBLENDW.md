# PBLENDW

Blend Packed Words

Words from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand).
The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source.
If a bit in the mask, corresponding to a word, is "1", then the word is copied, else the word element in the destination operand is unchanged.128-bit Legacy SSE version: The second source operand can be an XMM register or a 128-bit memory location.
The first source and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: The second source operand can be an XMM register or a 128-bit memory location.
The first source and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corresponding YMM register are zeroed.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.


## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
PBLENDW (128-bit Legacy SSE Version)IF (imm8[0] = 1) THEN DEST[15:0] := SRC[15:0]ELSE DEST[15:0] := DEST[15:0]IF (imm8[1] = 1) THEN DEST[31:16] := SRC[31:16]ELSE DEST[31:16] := DEST[31:16]IF (imm8[2] = 1) THEN DEST[47:32] := SRC[47:32]ELSE DEST[47:32] := DEST[47:32]IF (imm8[3] = 1) THEN DEST[63:48] := SRC[63:48]ELSE DEST[63:48] := DEST[63:48]IF (imm8[4] = 1) THEN DEST[79:64] := SRC[79:64]ELSE DEST[79:64] := DEST[79:64]IF (imm8[5] = 1) THEN DEST[95:80] := SRC[95:80]ELSE DEST[95:80] := DEST[95:80]IF (imm8[6] = 1) THEN DEST[111:96] := SRC[111:96]ELSE DEST[127:112] := DEST[127:112]VPBLENDW (VEX.128 Encoded Version)IF (imm8[0] = 1) THEN DEST[15:0] := SRC2[15:0]ELSE DEST[15:0] := SRC1[15:0]IF (imm8[1] = 1) THEN DEST[31:16] := SRC2[31:16]ELSE DEST[31:16] := SRC1[31:16]IF (imm8[2] = 1) THEN DEST[47:32] := SRC2[47:32]ELSE DEST[47:32] := SRC1[47:32]IF (imm8[3] = 1) THEN DEST[63:48] := SRC2[63:48]ELSE DEST[63:48] := SRC1[63:48]IF (imm8[4] = 1) THEN DEST[79:64] := SRC2[79:64]ELSE DEST[79:64] := SRC1[79:64]IF (imm8[5] = 1) THEN DEST[95:80] := SRC2[95:80]ELSE DEST[95:80] := SRC1[95:80]IF (imm8[6] = 1) THEN DEST[111:96] := SRC2[111:96]ELSE DEST[111:96] := SRC1[111:96]IF (imm8[7] = 1) THEN DEST[127:112] := SRC2[127:112]ELSE DEST[127:112] := SRC1[127:112]DEST[MAXVL-1:128] := 0VPBLENDW (VEX.256 Encoded Version)IF (imm8[0] == 1) THEN DEST[15:0] := SRC2[15:0]ELSE DEST[15:0] := SRC1[15:0]IF (imm8[1] == 1) THEN DEST[31:16] := SRC2[31:16]ELSE DEST[31:16] := SRC1[31:16]IF (imm8[2] == 1) THEN DEST[47:32] := SRC2[47:32]ELSE DEST[47:32] := SRC1[47:32]IF (imm8[3] == 1) THEN DEST[63:48] := SRC2[63:48]ELSE DEST[63:48] := SRC1[63:48]IF (imm8[4] == 1) THEN DEST[79:64] := SRC2[79:64]ELSE DEST[79:64] := SRC1[79:64]IF (imm8[5] == 1) THEN DEST[95:80] := SRC2[95:80]ELSE DEST[95:80] := SRC1[95:80]IF (imm8[6] == 1) THEN DEST[111:96] := SRC2[111:96]ELSE DEST[111:96] := SRC1[111:96]IF (imm8[7] == 1) THEN DEST[127:112] := SRC2[127:112]ELSE DEST[127:112] := SRC1[127:112]IF (imm8[0] == 1) THEN DEST[143:128] := SRC2[143:128]ELSE DEST[143:128] := SRC1[143:128]IF (imm8[1] == 1) THEN DEST[159:144] := SRC2[159:144]ELSE DEST[159:144] := SRC1[159:144]IF (imm8[2] == 1) THEN DEST[175:160] := SRC2[175:160]ELSE DEST[175:160] := SRC1[175:160]IF (imm8[3] == 1) THEN DEST[191:176] := SRC2[191:176]ELSE DEST[191:176] := SRC1[191:176]IF (imm8[4] == 1) THEN DEST[207:192] := SRC2[207:192]ELSE DEST[207:192] := SRC1[207:192]IF (imm8[5] == 1) THEN DEST[223:208] := SRC2[223:208]ELSE DEST[223:208] := SRC1[223:208]IF (imm8[6] == 1) THEN DEST[239:224] := SRC2[239:224]ELSE DEST[239:224] := SRC1[239:224]IF (imm8[7] == 1) THEN DEIntel C/C++ Compiler Intrinsic Equivalent(V)PBLENDW __m128i _mm_blend_epi16 (__m128i v1, __m128i v2, const int mask);VPBLENDW __m256i _mm256_blend_epi16 (__m256i v1, __m256i v2, const int mask)
```
