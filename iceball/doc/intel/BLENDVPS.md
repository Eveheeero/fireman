# BLENDVPS

Variable Blend Packed Single Precision Floating-Point Values

Conditionally copy each dword data element of single precision floating-point value from the second source operand and the first source operand depending on mask bits defined in the mask register operand.
The mask bits are the most significant bit in each dword element of the mask register.Each quadword element of the destination operand is copied from: - the corresponding dword element in the second source operand, if a mask bit is "1"; or - the corresponding dword element in the first source operand, if a mask bit is "0".The register assignment of the implicit mask operand for BLENDVPS is defined to be the architectural register XMM0.128-bit Legacy SSE version: The first source operand and the destination operand is the same.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.
The mask register operand is implicitly defined to be the architectural register XMM0.
An attempt to execute BLENDVPS with a VEX prefix will cause #UD.VEX.128 encoded version: The first source operand and the destination operand are XMM registers.
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
VEX.W must be 0, otherwise, the instruction will #UD.VBLENDVPS permits the mask to be any XMM or YMM register

## Exceptions

- Other Exceptions
  > See Table2-21, "Type 4 Class Exce

## Operation

```C
BLENDVPS (128-bit Legacy SSE Version)MASK := XMM0IF (MASK[31] = 0) THEN DEST[31:0] := DEST[31:0]ELSE DEST [31:0] := SRC[31:0] FIIF (MASK[63] = 0) THEN DEST[63:32] := DEST[63:32]ELSE DEST [63:32] := SRC[63:32] FIIF (MASK[95] = 0) THEN DEST[95:64] := DEST[95:64]ELSE DEST [95:64] := SRC[95:64] FIIF (MASK[127] = 0) THEN DEST[127:96] := DEST[127:96]ELSE DEST [127:96] := SRC[127:96] FIDEST[MAXVL-1:128] (Unmodified)VBLENDVPS (VEX.128 Encoded Version)MASK := SRC3IF (MASK[31] = 0) THEN DEST[31:0] := SRC1[31:0]ELSE DEST [31:0] := SRC2[31:0] FIIF (MASK[63] = 0) THEN DEST[63:32] := SRC1[63:32]ELSE DEST [63:32] := SRC2[63:32] FIIF (MASK[95] = 0) THEN DEST[95:64] := SRC1[95:64]ELSE DEST [95:64] := SRC2[95:64] FIIF (MASK[127] = 0) THEN DEST[127:96] := SRC1[127:96]ELSE DEST [127:96] := SRC2[127:96] FIDEST[MAXVL-1:128] := 0VBLENDVPS (VEX.256 Encoded Version)MASK := SRC3IF (MASK[31] = 0) THEN DEST[31:0] := SRC1[31:0]ELSE DEST [31:0] := SRC2[31:0] FIIF (MASK[63] = 0) THEN DEST[63:32] := SRC1[63:32]ELSE DEST [63:32] := SRC2[63:32] FIIF (MASK[95] = 0) THEN DEST[95:64] := SRC1[95:64]ELSE DEST [95:64] := SRC2[95:64] FIIF (MASK[127] = 0) THEN DEST[127:96] := SRC1[127:96]ELSE DEST [127:96] := SRC2[127:96] FIIF (MASK[159] = 0) THEN DEST[159:128] := SRC1[159:128]ELSE DEST [159:128] := SRC2[159:128] FIIF (MASK[191] = 0) THEN DEST[191:160] := SRC1[191:160]ELSE DEST [191:160] := SRC2[191:160] FIIF (MASK[223] = 0) THEN DEST[223:192] := SRC1[223:192]ELSE DEST [223:192] := SRC2[223:192] FIIF (MASK[255] = 0) THEN DEST[255:224] := SRC1[255:224]ELSE DEST [255:224] := SRC2[255:224] FIIntel C/C++ Compiler Intrinsic EquivalentBLENDVPS __m128 _mm_blendv_ps(__m128 v1, __m128 v2, __m128 v3);VBLENDVPS __m128 _mm_blendv_ps (__m128 a, __m128 b, __m128 mask);VBLENDVPS __m256 _mm256_blendv_ps (__m256 a, __m256 b, __m256 mask);
```
