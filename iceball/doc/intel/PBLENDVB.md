# PBLENDVB

Variable Blend Packed Bytes

Conditionally copies byte elements from the source operand (second operand) to the destination operand (first operand) depending on mask bits defined in the implicit third register argument, XMM0.
The mask bits are the most significant bit in each byte element of the XMM0 register.If a mask bit is "1", then the corresponding byte element in the source operand is copied to the destination, else the byte element in the destination operand is left unchanged.The register assignment of the implicit third operand is defined to be the architectural register XMM0.128-bit Legacy SSE version: The first source operand and the destination operand is the same.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.
The mask register operand is implicitly defined to be the architectural register XMM0.
An attempt to execute PBLENDVB with a VEX prefix will cause #UD.VEX.128 encoded version: The first source operand and the destination operand are XMM registers.
The second source operand is an XMM register or 128-bit memory location.
The mask operand is the third source register, and encoded in bits[7:4] of the immediate byte(imm8).
The bits[3:0] of imm8 are ignored.
In 32-bit mode, imm8[7] is ignored.
The upper bits (MAXVL-1:128) of the corresponding YMM register (destination register) are zeroed.
VEX.L must be 0, otherwise the instruction will #UD.
VEX.W must be 0, otherwise, the instruction will #UD.VEX.256 encoded version: The first source operand and the destination operand are YMM registers.
The second source operand is an YMM register or 256-bit memory location.
The third source register is an YMM register and encoded in bits[7:4] of the immediate byte(imm8).
The bits[3:0] of imm8 are ignored.
In 32-bit mode, imm8[7] is ignored.
VPBLENDVB permits the mask to be any XMM or YMM register.
In contrast, PBLENDVB treats XMM0 implicitly as the mask and do not support non-destructive destination operation.
An attempt to execute PBLENDVB encoded with a VEX prefix will cause a #UD exception.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
PBLENDVB (128-bit Legacy SSE Version)MASK := XMM0IF (MASK[7] = 1) THEN DEST[7:0] := SRC[7:0];ELSE DEST[7:0] := DEST[7:0];IF (MASK[15] = 1) THEN DEST[15:8] := SRC[15:8];ELSE DEST[23:16] := DEST[23:16];IF (MASK[31] = 1) THEN DEST[31:24] := SRC[31:24]ELSE DEST[31:24] := DEST[31:24];IF (MASK[39] = 1) THEN DEST[39:32] := SRC[39:32]ELSE DEST[39:32] := DEST[39:32];IF (MASK[47] = 1) THEN DEST[47:40] := SRC[47:40]ELSE DEST[47:40] := DEST[47:40];IF (MASK[55] = 1) THEN DEST[55:48] := SRC[55:48]ELSE DEST[55:48] := DEST[55:48];IF (MASK[63] = 1) THEN DEST[63:56] := SRC[63:56]ELSE DEST[63:56] := DEST[63:56];IF (MASK[71] = 1) THEN DEST[71:64] := SRC[71:64]ELSE DEST[71:64] := DEST[71:64];IF (MASK[79] = 1) THEN DEST[79:72] := SRC[79:72]ELSE DEST[79:72] := DEST[79:72];IF (MASK[87] = 1) THEN DEST[87:80] := SRC[87:80]ELSE DEST[87:80] := DEST[87:80];IF (MASK[95] = 1) THEN DEST[95:88] := SRC[95:88]ELSE DEST[95:88] := DEST[95:88];IF (MASK[103] = 1) THEN DEST[103:96] := SRC[103:96]ELSE DEST[103:96] := DEST[103:96];IF (MASK[111] = 1) THEN DEST[111:104] := SRC[111:104]ELSE DEST[111:104] := DEST[111:104];IF (MASK[119] = 1) THEN DEST[119:112] := SRC[119:112]ELSE DEST[119:112] := DEST[119:112];IF (MASK[127] = 1) THEN DEST[127:120] := SRC[127:120]ELSE DEST[127:120] := DEST[127:120])DEST[MAXVL-1:128] (Unmodified)VPBLENDVB (VEX.128 Encoded Version)MASK := SRC3IF (MASK[7] = 1) THEN DEST[7:0] := SRC2[7:0];ELSE DEST[7:0] := SRC1[7:0];IF (MASK[15] = 1) THEN DEST[15:8] := SRC2[15:8];ELSE DEST[15:8] := SRC1[15:8];IF (MASK[23] = 1) THEN DEST[23:16] := SRC2[23:16]ELSE DEST[23:16] := SRC1[23:16];IF (MASK[31] = 1) THEN DEST[31:24] := SRC2[31:24]ELSE DEST[31:24] := SRC1[31:24];IF (MASK[39] = 1) THEN DEST[39:32] := SRC2[39:32]ELSE DEST[39:32] := SRC1[39:32];IF (MASK[47] = 1) THEN DEST[47:40] := SRC2[47:40]ELSE DEST[47:40] := SRC1[47:40];IF (MASK[55] = 1) THEN DEST[55:48] := SRC2[55:48]ELSE DEST[55:48] := SRC1[55:48];IF (MASK[63] = 1) THEN DEST[63:56] := SRC2[63:56]ELSE DEST[63:56] := SRC1[63:56];IF (MASK[71] = 1) THEN DEST[71:64] := SRC2[71:64]ELSE DEST[71:64] := SRC1[71:64];IF (MASK[79] = 1) THEN DEST[79:72] := SRC2[79:72]ELSE DEST[79:72] := SRC1[79:72];IF (MASK[87] = 1) THEN DEST[87:80] := SRC2[87:80]ELSE DEST[95:88] := SRC1[95:88];IF (MASK[103] = 1) THEN DEST[103:96] := SRC2[103:96]ELSE DEST[103:96] := SRC1[103:96];IF (MASK[111] = 1) THEN DEST[111:104] := SRC2[111:104]ELSE DEST[111:104] := SRC1[111:104];IF (MASK[119] = 1) THEN DEST[119:112] := SRC2[119:112]ELSE DEST[119:112] := SRC1[119:112];IF (MASK[127] = 1) THEN DEST[127:120] := SRC2[127:120]ELSE DEST[127:120] := SRC1[127:120])DEST[MAXVL-1:128] := 0VPBLENDVB (VEX.256 Encoded Version)MASK := SRC3IF (MASK[7] == 1) THEN DEST[7:0] := SRC2[7:0];ELSE DEST[7:0] := SRC1[7:0];IF (MASK[15] == 1) THEN DEST[15:8] := SRC2[15:8];ELSE DEST[15:8] := SRC1[15:8];IF (MASK[23] == 1) THEN DEST[23:16] := SRC2[23:16]ELSE DEST[23:16] := SRC1[23:16];IF (MASK[31] == 1) THEN DEST[31:24] := SRC2[31:24]ELSE DEST[31:24] := SRC1[31:24];IF (MASK[39] == 1) THEN DEST[39:32] := SRC2[39:32]ELSE DEST[39:32] := SRC1[39:32];IF (MASK[47] == 1) THEN DEST[47:40] := SRC2[47:40]ELSE DEST[47:40] := SRC1[47:40];IF (MASK[55] == 1) THEN DEST[55:48] := SRC2[55:48]ELSE DEST[55:48] := SRC1[55:48];IF (MASK[63] == 1) THEN DEST[63:56] := SRC2[63:56]ELSE DEST[63:56] := SRC1[63:56];IF (MASK[71] == 1) THEN DEST[71:64] := SRC2[71:64]ELSE DEST[71:64] := SRC1[71:64];IF (MASK[79] == 1) THEN DEST[79:72] := SRC2[79:72]ELSE DEST[79:72] := SRC1[79:72];IF (MASK[87] == 1) THEN DEST[87:80] := SRC2[87:80]ELSE DEST[87:80] := SRC1[87:80];IF (MASK[95] == 1) THEN DEST[95:88] := SRC2[95:88]ELSE DEST[95:88] := SRC1[95:88];IF (MASK[103] == 1) THEN DEST[103:96] := SRC2[103:96]ELSE DEST[103:96] := SRC1[103:96];IF (MASK[111] == 1) THEN DEST[111:104] := SRC2[111:104]ELSE DEST[111:104] := SRC1[111:104];IF (MASK[119] == 1) THEN DEST[119:112] := SRC2[119:112]ELSE DEST[119:112] := SRC1[119:112];IF (MASK[127] == 1) THEN DEST[127:120] := SRC2[127:120]ELSE DEST[127:120] := SRC1[127:120])IF (MASK[135] == 1) THEN DEST[135:128] := SRC2[135:128];ELSE DEST[135:128] := SRC1[135:128];IF (MASK[143] == 1) THEN DEST[143:136] := SRC2[143:136];ELSE DEST[[143:136] := SRC1[143:136];IF (MASK[151] == 1) THEN DEST[151:144] := SRC2[151:144]ELSE DEST[151:144] := SRC1[151:144];IF (MASK[159] == 1) THEN DEST[159:152] := SRC2[159:152]ELSE DEST[167:160] := SRC1[167:160];IF (MASK[175] == 1) THEN DEST[175:168] := SRC2[175:168]ELSE DEST[175:168] := SRC1[175:168];IF (MASK[183] == 1) THEN DEST[183:176] := SRC2[183:176]ELSE DEST[183:176] := SRC1[183:176];IF (MASK[191] == 1) THEN DEST[191:184] := SRC2[191:184]ELSE DEST[191:184] := SRC1[191:184];IF (MASK[199] == 1) THEN DEST[199:192] := SRC2[199:192]ELSE DEST[199:192] := SRC1[199:192];IF (MASK[207] == 1) THEN DEST[207:200] := SRC2[207:200]ELSE DEST[207:200] := SRC1[207:200]IF (MASK[215] == 1) THEN DEST[215:208] := SRC2[215:208]ELSE DEST[215:208] := SRC1[215:208];IF (MASK[223] == 1) THEN DEST[223:216] := SRC2[223:216]ELSE DEST[223:216] := SRC1[223:216];IF (MASK[231] == 1) THEN DEST[231:224] := SRC2[231:224]ELSE DEST[231:224] := SRC1[231:224];IF (MASK[239] == 1) THEN DEST[239:232] := SRC2[239:232]ELSE DEST[239:232] := SRC1[239:232];IF (MASK[247] == 1) THEN DEST[247:240] := SRC2[247:240]ELSE DEST[247:240] := SRC1[247:240];IF (MASK[255] == 1) THEN DEST[255:248] := SRC2[255:248]ELSE DEST[255:248] := SRC1[255:248]Intel C/C++ Compiler Intrinsic Equivalent(V)PBLENDVB __m128i _mm_blendv_epi8 (__m128i v1, __m128i v2, __m128i mask);VPBLENDVB __m256i _mm256_blendv_epi8 (__m256i v1, __m256i v2, __m256i mask);
```
