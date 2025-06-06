# ANDNPS

Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values

Performs a bitwise logical AND NOT of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The first source operand is an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM regist

## Exceptions

- Other Exceptions
  > VEX-encoded instruction, see Table2-21,
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VANDNPS (EVEX Encoded Versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*IF (EVEX.b == 1) AND (SRC2 *is memory*)THENDEST[i+31:i] := (NOT(SRC1[i+31:i])) BITWISE AND SRC2[31:0]ELSE DEST[i+31:i] := (NOT(SRC1[i+31:i])) BITWISE AND SRC2[i+31:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] = 0FI;FI;ENDFORDEST[MAXVL-1:VL] := 0VANDNPS (VEX.256 Encoded Version)DEST[31:0] := (NOT(SRC1[31:0])) BITWISE AND SRC2[31:0]DEST[63:32] := (NOT(SRC1[63:32])) BITWISE AND SRC2[63:32]DEST[95:64] := (NOT(SRC1[95:64])) BITWISE AND SRC2[95:64]DEST[127:96] := (NOT(SRC1[127:96])) BITWISE AND SRC2[127:96]DEST[159:128] := (NOT(SRC1[159:128])) BITWISE AND SRC2[159:128]DEST[191:160] := (NOT(SRC1[191:160])) BITWISE AND SRC2[191:160]DEST[223:192] := (NOT(SRC1[223:192])) BITWISE AND SRC2[223:192]DEST[255:224] := (NOT(SRC1[255:224])) BITWISE AND SRC2[255:224].DEST[MAXVL-1:256] := 0VANDNPS (VEX.128 Encoded Version)DEST[31:0] := (NOT(SRC1[31:0])) BITWISE AND SRC2[31:0]DEST[63:32] := (NOT(SRC1[63:32])) BITWISE AND SRC2[63:32]DEST[95:64] := (NOT(SRC1[95:64])) BITWISE AND SRC2[95:64]DEST[127:96] := (NOT(SRC1[127:96])) BITWISE AND SRC2[127:96]DEST[MAXVL-1:128] := 0ANDNPS (128-bit Legacy SSE Version)DEST[31:0] := (NOT(DEST[31:0])) BITWISE AND SRC[31:0]DEST[63:32] := (NOT(DEST[63:32])) BITWISE AND SRC[63:32]DEST[95:64] := (NOT(DEST[95:64])) BITWISE AND SRC[95:64]DEST[127:96] := (NOT(DEST[127:96])) BITWISE AND SRC[127:96]Intel C/C++ Compiler Intrinsic EquivalentVANDNPS __m512 _mm512_andnot_ps (__m512 a, __m512 b);VANDNPS __m512 _mm512_mask_andnot_ps (__m512 s, __mmask16 k, __m512 a, __m512 b);VANDNPS __m512 _mm512_maskz_andnot_ps (__mmask16 k, __m512 a, __m512 b);VANDNPS __m256 _mm256_mask_andnot_ps (__m256 s, __mmask8 k, __m256 a, __m256 b);VANDNPS __m256 _mm256_maskz_andnot_ps (__mmask8 k, __m256 a, __m256 b);VANDNPS __m128 _mm_mask_andnot_ps (__m128 s, __mmask8 k, __m128 a, __m128 b);VANDNPS __m128 _mm_maskz_andnot_ps (__mmask8 k, __m128 a, __m128 b);VANDNPS __m256 _mm256_andnot_ps (__m256 a, __m256 b);ANDNPS __m128 _mm_andnot_ps (__m128 a, __m128 b);
```
