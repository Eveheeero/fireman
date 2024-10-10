# BLENDPD

Blend Packed Double Precision Floating-Point Values

Double-precision floating-point values from the second source operand (third operand) are conditionally merged with values from the first source operand (second operand) and written to the destination operand (first operand).
The immediate bits [3:0] determine whether the corresponding double precision floating-point value in the desti-nation is copied from the second source or first source.
If a bit in the mask, corresponding to a word, is "1", then the double precision floating-point value in the second source operand is copied, else the value in the first source operand is copied.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding YMM register destination are unmodified.VEX.128 encoded version: the first source operand is an XMM register.
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
BLENDPD (128-bit Legacy SSE Version)IF (IMM8[0] = 0)THEN DEST[63:0] := DEST[63:0]ELSE DEST [63:0] := SRC[63:0] FIIF (IMM8[1] = 0) THEN DEST[127:64] := DEST[127:64]ELSE DEST [127:64] := SRC[127:64] FIDEST[MAXVL-1:128] (Unmodified)VBLENDPD (VEX.128 Encoded Version)IF (IMM8[0] = 0)THEN DEST[63:0] := SRC1[63:0]ELSE DEST [63:0] := SRC2[63:0] FIIF (IMM8[1] = 0) THEN DEST[127:64] := SRC1[127:64]VBLENDPD (VEX.256 Encoded Version)IF (IMM8[0] = 0)THEN DEST[63:0] := SRC1[63:0]ELSE DEST [63:0] := SRC2[63:0] FIIF (IMM8[1] = 0) THEN DEST[127:64] := SRC1[127:64]ELSE DEST [127:64] := SRC2[127:64] FIIF (IMM8[2] = 0) THEN DEST[191:128] := SRC1[191:128]ELSE DEST [191:128] := SRC2[191:128] FIIF (IMM8[3] = 0) THEN DEST[255:192] := SRC1[255:192]ELSE DEST [255:192] := SRC2[255:192] FIIntel C/C++ Compiler Intrinsic EquivalentBLENDPD __m128d _mm_blend_pd (__m128d v1, __m128d v2, const int mask);VBLENDPD __m256d _mm256_blend_pd (__m256d a, __m256d b, const int mask);
```
