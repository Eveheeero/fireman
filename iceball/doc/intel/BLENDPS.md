# BLENDPS

Blend Packed Single Precision Floating-Point Values

Packed single precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand).
The immediate bits [7:0] determine whether the corresponding single precision floating-point value in the destination is copied from the second source or first source.
If a bit in the mask, corresponding to a word, is "1", then the single precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding YMM register destination are unmodified.VEX.128 encoded version: The first source operand an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding YMM register destination are zeroed.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.


## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
BLENDPS (128-bit Legacy SSE Version)IF (IMM8[0] = 0) THEN DEST[31:0] :=DEST[31:0]ELSE DEST [31:0] := SRC[31:0] FIIF (IMM8[1] = 0) THEN DEST[63:32] := DEST[63:32]ELSE DEST [63:32] := SRC[63:32] FIIF (IMM8[2] = 0) THEN DEST[95:64] := DEST[95:64]ELSE DEST [95:64] := SRC[95:64] FIIF (IMM8[3] = 0) THEN DEST[127:96] := DEST[127:96]VBLENDPS (VEX.128 Encoded Version)IF (IMM8[0] = 0) THEN DEST[31:0] :=SRC1[31:0]ELSE DEST [31:0] := SRC2[31:0] FIIF (IMM8[1] = 0) THEN DEST[63:32] := SRC1[63:32]ELSE DEST [63:32] := SRC2[63:32] FIIF (IMM8[2] = 0) THEN DEST[95:64] := SRC1[95:64]ELSE DEST [95:64] := SRC2[95:64] FIIF (IMM8[3] = 0) THEN DEST[127:96] := SRC1[127:96]ELSE DEST [127:96] := SRC2[127:96] FIDEST[MAXVL-1:128] := 0VBLENDPS (VEX.256 Encoded Version)IF (IMM8[0] = 0) THEN DEST[31:0] :=SRC1[31:0]ELSE DEST [31:0] := SRC2[31:0] FIIF (IMM8[1] = 0) THEN DEST[63:32] := SRC1[63:32]ELSE DEST [63:32] := SRC2[63:32] FIIF (IMM8[2] = 0) THEN DEST[95:64] := SRC1[95:64]ELSE DEST [95:64] := SRC2[95:64] FIIF (IMM8[3] = 0) THEN DEST[127:96] := SRC1[127:96]ELSE DEST [127:96] := SRC2[127:96] FIIF (IMM8[4] = 0) THEN DEST[159:128] := SRC1[159:128]ELSE DEST [159:128] := SRC2[159:128] FIIF (IMM8[5] = 0) THEN DEST[191:160] := SRC1[191:160]ELSE DEST [191:160] := SRC2[191:160] FIIF (IMM8[6] = 0) THEN DEST[223:192] := SRC1[223:192]ELSE DEST [223:192] := SRC2[223:192] FIIF (IMM8[7] = 0) THEN DEST[255:224] := SRC1[255:224]ELSE DEST [255:224] := SRC2[255:224] FI.Intel C/C++ Compiler Intrinsic EquivalentBLENDPS __m128 _mm_blend_ps (__m128 v1, __m128 v2, const int mask);VBLENDPS __m256 _mm256_blend_ps (__m256 a, __m256 b, const int mask);
```
