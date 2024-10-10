# PAVGB/PAVGW

Average Packed Integers

Performs a SIMD average of the packed unsigned integers from the source operand (second operand) and the destination operand (first operand), and stores the results in the destination operand.
For each corresponding pair of data elements in the first and second operands, the elements are added together, a 1 is added to the temporary sum, and that result is shifted right one bit position.
The (V)PAVGB instruction operates on packed unsigned bytes and the (V)PAVGW instruction operates on packed unsigned words.In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE instructions: The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand can be an MMX technology register.128-bit Legacy SSE version: The first source operand is an XMM register.
The second operand can be an XMM register or an 128-bit memory location.
The destination is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding register destination are unmodified.EVEX.512 encoded version: The first source operand is a ZMM register.
The second source operand is a ZMM register or a 512-bit memory location.
The destination operand is a ZMM register.VEX.256 and EVEX.256 encoded versions: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
VEX.128 and EVEX.128 encoded versions: The first source operand is an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding register destination are zeroed.

## Flags affected

- None.

## Exceptions

- Numeric Exceptions
  > None.
- Other Exceptions

## Operation

```C
PAVGB (With 64-bit Operands)DEST[7:0] := (SRC[7:0] + DEST[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)(* Repeat operation performed for bytes 2 through 6 *)DEST[63:56] := (SRC[63:56] + DEST[63:56] + 1) >> 1;PAVGW (With 64-bit Operands)DEST[15:0] := (SRC[15:0] + DEST[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)(* Repeat operation performed for words 2 and 3 *)DEST[63:48] := (SRC[63:48] + DEST[63:48] + 1) >> 1;PAVGB (With 128-bit Operands)DEST[7:0] := (SRC[7:0] + DEST[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)(* Repeat operation performed for bytes 2 through 14 *)DEST[127:120] := (SRC[127:120] + DEST[127:120] + 1) >> 1;PAVGW (With 128-bit Operands)DEST[15:0] := (SRC[15:0] + DEST[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)VPAVGB (VEX.128 Encoded Version)DEST[7:0] := (SRC1[7:0] + SRC2[7:0] + 1) >> 1; (* Repeat operation performed for bytes 2 through 15 *)DEST[127:120] := (SRC1[127:120] + SRC2[127:120] + 1) >> 1DEST[MAXVL-1:128] := 0VPAVGW (VEX.128 Encoded Version)DEST[15:0] := (SRC1[15:0] + SRC2[15:0] + 1) >> 1; (* Repeat operation performed for 16-bit words 2 through 7 *)DEST[127:112] := (SRC1[127:112] + SRC2[127:112] + 1) >> 1DEST[MAXVL-1:128] := 0VPAVGB (VEX.256 Encoded Instruction)DEST[7:0] := (SRC1[7:0] + SRC2[7:0] + 1) >> 1; (* Temp sum before shifting is 9 bits *)(* Repeat operation performed for bytes 2 through 31)DEST[255:248] := (SRC1[255:248] + SRC2[255:248] + 1) >> 1;VPAVGW (VEX.256 Encoded Instruction)DEST[15:0] := (SRC1[15:0] + SRC2[15:0] + 1) >> 1; (* Temp sum before shifting is 17 bits *)(* Repeat operation performed for words 2 through 15)DEST[255:14]) := (SRC1[255:240] + SRC2[255:240] + 1) >> 1;VPAVGB (EVEX encoded versions)(KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8IF k1[j] OR *no writemask*THEN DEST[i+7:i] := (SRC1[i+7:i] + SRC2[i+7:i] + 1) >> 1; (* Temp sum before shifting is 9 bits *)ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+7:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+7:i] = 0FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0VPAVGW (EVEX Encoded Versions)(KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[i+15:i] := (SRC1[i+15:i] + SRC2[i+15:i] + 1) >> 1; (* Temp sum before shifting is 17 bits *)ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] = 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentsVPAVGB __m512i _mm512_avg_epu8( __m512i a, __m512i b);VPAVGW __m512i _mm512_avg_epu16( __m512i a, __m512i b);VPAVGB __m512i _mm512_mask_avg_epu8(__m512i s, __mmask64 m, __m512i a, __m512i b);VPAVGW __m512i _mm512_mask_avg_epu16(__m512i s, __mmask32 m, __m512i a, __m512i b);VPAVGB __m512i _mm512_maskz_avg_epu8( __mmask64 m, __m512i a, __m512i b);VPAVGW __m512i _mm512_maskz_avg_epu16( __mmask32 m, __m512i a, __m512i b);VPAVGB __m256i _mm256_mask_avg_epu8(__m256i s, __mmask32 m, __m256i a, __m256i b);VPAVGW __m256i _mm256_mask_avg_epu16(__m256i s, __mmask16 m, __m256i a, __m256i b);VPAVGB __m256i _mm256_maskz_avg_epu8( __mmask32 m, __m256i a, __m256i b);VPAVGW __m256i _mm256_maskz_avg_epu16( __mmask16 m, __m256i a, __m256i b);VPAVGB __m128i _mm_mask_avg_epu8(__m128i s, __mmask16 m, __m128i a, __m128i b);VPAVGW __m128i _mm_mask_avg_epu16(__m128i s, __mmask8 m, __m128i a, __m128i b);VPAVGB __m128i _mm_maskz_avg_epu8( __mmask16 m, __m128i a, __m128i b);VPAVGW __m128i _mm_maskz_avg_epu16( __mmask8 m, __m128i a, __m128i b);PAVGB __m64 _mm_avg_pu8 (__m64 a, __m64 b)PAVGW __m64 _mm_avg_pu16 (__m64 a, __m64 b)(V)PAVGB __m128i _mm_avg_epu8 ( __m128i a, __m128i b)(V)PAVGW __m128i _mm_avg_epu16 ( __m128i a, __m128i b)VPAVGB __m256i _mm256_avg_epu8 ( __m256i a, __m256i b)VPAVGW __m256i _mm256_avg_epu16 ( __m256i a, __m256i b)
```
