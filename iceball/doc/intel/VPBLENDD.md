# VPBLENDD

Blend Packed Dwords

Dword elements from the source operand (second operand) are conditionally written to the destination operand (first operand) depending on bits in the immediate operand (third operand).
The immediate bits (bits 7:0) form a mask that determines whether the corresponding word in the destination is copied from the source.
If a bit in the mask, corresponding to a word, is "1", then the word is copied, else the word is unchanged.VEX.128 encoded version: The second source operand can be an XMM register or a 128-bit memory location.
The first source and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corresponding YMM register are zeroed.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.


## Exceptions

- Other Exceptions
  > See Table2-21, "Type 4 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > None

## Operation

```C
VPBLENDD (VEX.256 encoded version)IF (imm8[0] == 1) THEN DEST[31:0] := SRC2[31:0]ELSE DEST[31:0] := SRC1[31:0]IF (imm8[1] == 1) THEN DEST[63:32] := SRC2[63:32]ELSE DEST[63:32] := SRC1[63:32]IF (imm8[2] == 1) THEN DEST[95:64] := SRC2[95:64]ELSE DEST[95:64] := SRC1[95:64]IF (imm8[3] == 1) THEN DEST[127:96] := SRC2[127:96]ELSE DEST[127:96] := SRC1[127:96]IF (imm8[4] == 1) THEN DEST[159:128] := SRC2[159:128]ELSE DEST[159:128] := SRC1[159:128]IF (imm8[5] == 1) THEN DEST[191:160] := SRC2[191:160]ELSE DEST[191:160] := SRC1[191:160]IF (imm8[6] == 1) THEN DEST[223:192] := SRC2[223:192]ELSE DEST[223:192] := SRC1[223:192]VPBLENDD (VEX.128 encoded version)IF (imm8[0] == 1) THEN DEST[31:0] := SRC2[31:0]ELSE DEST[31:0] := SRC1[31:0]IF (imm8[1] == 1) THEN DEST[63:32] := SRC2[63:32]ELSE DEST[63:32] := SRC1[63:32]IF (imm8[2] == 1) THEN DEST[95:64] := SRC2[95:64]ELSE DEST[95:64] := SRC1[95:64]IF (imm8[3] == 1) THEN DEST[127:96] := SRC2[127:96]ELSE DEST[127:96] := SRC1[127:96]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVPBLENDD:__m128i _mm_blend_epi32 (__m128i v1, __m128i v2, const int mask)VPBLENDD:__m256i _mm256_blend_epi32 (__m256i v1, __m256i v2, const int mask)
```
