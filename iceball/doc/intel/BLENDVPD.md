# BLENDVPD

Variable Blend Packed Double Precision Floating-Point Values

Conditionally copy each quadword data element of double precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand.
The mask bits are the most significant bit in each quadword element of the mask register.Each quadword element of the destination operand is copied from: - the corresponding quadword element in the second source operand, if a mask bit is "1"; or - the corresponding quadword element in the first source operand, if a mask bit is "0"The register assignment of the implicit mask operand for BLENDVPD is defined to be the architectural register XMM0.128-bit Legacy SSE version: The first source operand and the destination operand is the same.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.
The mask register operand is implicitly defined to be the architectural register XMM0.
An attempt to execute BLENDVPD with a VEX prefix will cause #UD.VEX.128 encoded version: The first source operand and the destination operand are XMM registers.
The second source operand is an XMM register or 128-bit memory location.
The mask operand is the third source register, and encoded in bits[7:4] of the immediate byte(imm8).
The bits[3:0] of imm8 are ignored.
In 32-bit mode, imm8[7] is ignored.
The upper bits (MAXVL-1:128) of the corresponding YMM register (destination register) are zeroed.
VEX.W must be 0, otherwise, the instruction will #UD.VEX.256 encoded version: The first source operand and destination operand are YMM registers.
The second source operand can be a YMM register or a 256-bit memory location.
The mask operand is the third source register, and encoded in bits[7:4] of the immediate byte(imm8).
The bits[3:0] of imm8 are ignored.
In 32-bit mode, imm8[7] is ignored.
VEX.W must be 0, otherwise, the instruction will #UD.VBLENDVPD permits the mask to be any 

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
BLENDVPD (128-bit Legacy SSE Version)MASK := XMM0IF (MASK[63] = 0) THEN DEST[63:0] := DEST[63:0]ELSE DEST [63:0] := SRC[63:0] FIIF (MASK[127] = 0) THEN DEST[127:64] := DEST[127:64]ELSE DEST [127:64] := SRC[127:64] FIDEST[MAXVL-1:128] (Unmodified)VBLENDVPD (VEX.128 Encoded Version)MASK := SRC3IF (MASK[63] = 0) THEN DEST[63:0] := SRC1[63:0]ELSE DEST [63:0] := SRC2[63:0] FIIF (MASK[127] = 0) THEN DEST[127:64] := SRC1[127:64]ELSE DEST [127:64] := SRC2[127:64] FIDEST[MAXVL-1:128] := 0VBLENDVPD (VEX.256 Encoded Version)MASK := SRC3IF (MASK[63] = 0) THEN DEST[63:0] := SRC1[63:0]ELSE DEST [63:0] := SRC2[63:0] FIIF (MASK[127] = 0) THEN DEST[127:64] := SRC1[127:64]ELSE DEST [127:64] := SRC2[127:64] FIIF (MASK[191] = 0) THEN DEST[191:128] := SRC1[191:128]ELSE DEST [191:128] := SRC2[191:128] FIIF (MASK[255] = 0) THEN DEST[255:192] := SRC1[255:192]ELSE DEST [255:192] := SRC2[255:192] FIIntel C/C++ Compiler Intrinsic EquivalentBLENDVPD __m128d _mm_blendv_pd(__m128d v1, __m128d v2, __m128d v3);VBLENDVPD __m128 _mm_blendv_pd (__m128d a, __m128d b, __m128d mask);VBLENDVPD __m256 _mm256_blendv_pd (__m256d a, __m256d b, __m256d mask);
```
