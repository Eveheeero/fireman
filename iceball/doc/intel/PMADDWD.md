# PMADDWD

Multiply and Add Packed Integers

Multiplies the individual signed words of the destination operand (first operand) by the corresponding signed words of the source operand (second operand), producing temporary signed, doubleword results.
The adjacent double-word results are then summed and stored in the destination operand.
For example, the corresponding low-order words (15-0) and (31-16) in the source and destination operands are multiplied by one another and the double-word results are added together and stored in the low doubleword of the destination register (31-0).
The same operation is performed on the other pairs of adjacent woThe (V)PMADDWD instruction wraps around only in one situation: when the 2 pairs of words being operated on in a group are all 8000H.
In this case, the result wraps around to 80000000H.In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE version: The first source and destination operands are MMX registers.
The second source operand is an MMX register or a 64-bit memory location.
128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding YMM destina-tion register remain unchanged.VEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded version: The second source operand can be an YMM register or a 256-bit memory location.
The first source and destination operands are YMM registers.EVEX.512 encoded version: The second source operand can be an ZMM register or a 512-bit memory location.
The first source and destination operands are ZMM registers.SRCX3X2X1X0DESTY3Y2Y1Y0X3  Y3X2  Y2X1  Y1X0  Y0TEMPDEST(X1Y1)  (X0Y0)+Y3)  (X2Y2) (X3+Figure 4-11.
 PMADDWD Execution Model Using 64-bit Operands

## Flags affected

- None.

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."
- Numeric Exceptions
  > None.

## Operation

```C
PMADDWD (With 64-bit Operands)DEST[31:0] := (DEST[15:0]  SRC[15:0]) + (DEST[31:16]  SRC[31:16]);DEST[63:32] := (DEST[47:32]  SRC[47:32]) + (DEST[63:48]  SRC[63:48]);PMADDWD (With 128-bit Operands)DEST[31:0] := (DEST[15:0]  SRC[15:0]) + (DEST[31:16]  SRC[31:16]);DEST[63:32] := (DEST[47:32]  SRC[47:32]) + (DEST[63:48]  SRC[63:48]);DEST[95:64] := (DEST[79:64]  SRC[79:64]) + (DEST[95:80]  SRC[95:80]);DEST[127:96] := (DEST[111:96]  SRC[111:96]) + (DEST[127:112]  SRC[127:112]);VPMADDWD (VEX.128 Encoded Version)DEST[31:0] := (SRC1[15:0] * SRC2[15:0]) + (SRC1[31:16] * SRC2[31:16])DEST[63:32] := (SRC1[47:32] * SRC2[47:32]) + (SRC1[63:48] * SRC2[63:48])DEST[95:64] := (SRC1[79:64] * SRC2[79:64]) + (SRC1[95:80] * SRC2[95:80])VPMADDWD (VEX.256 Encoded Version)DEST[31:0] := (SRC1[15:0] * SRC2[15:0]) + (SRC1[31:16] * SRC2[31:16])DEST[63:32] := (SRC1[47:32] * SRC2[47:32]) + (SRC1[63:48] * SRC2[63:48])DEST[95:64] := (SRC1[79:64] * SRC2[79:64]) + (SRC1[95:80] * SRC2[95:80])DEST[127:96] := (SRC1[111:96] * SRC2[111:96]) + (SRC1[127:112] * SRC2[127:112])DEST[159:128] := (SRC1[143:128] * SRC2[143:128]) + (SRC1[159:144] * SRC2[159:144])DEST[191:160] := (SRC1[175:160] * SRC2[175:160]) + (SRC1[191:176] * SRC2[191:176])DEST[223:192] := (SRC1[207:192] * SRC2[207:192]) + (SRC1[223:208] * SRC2[223:208])DEST[255:224] := (SRC1[239:224] * SRC2[239:224]) + (SRC1[255:240] * SRC2[255:240])DEST[MAXVL-1:256] := 0VPMADDWD (EVEX Encoded Versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := (SRC2[i+31:i+16]* SRC1[i+31:i+16]) + (SRC2[i+15:i]*SRC1[i+15:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+31:i] = 0FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPMADDWD __m512i _mm512_madd_epi16( __m512i a, __m512i b);VPMADDWD __m512i _mm512_mask_madd_epi16(__m512i s, __mmask32 k, __m512i a, __m512i b);VPMADDWD __m512i _mm512_maskz_madd_epi16( __mmask32 k, __m512i a, __m512i b);VPMADDWD __m256i _mm256_mask_madd_epi16(__m256i s, __mmask16 k, __m256i a, __m256i b);VPMADDWD __m256i _mm256_maskz_madd_epi16( __mmask16 k, __m256i a, __m256i b);VPMADDWD __m128i _mm_mask_madd_epi16(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMADDWD __m128i _mm_maskz_madd_epi16( __mmask8 k, __m128i a, __m128i b);PMADDWD __m64 _mm_madd_pi16(__m64 m1, __m64 m2)(V)PMADDWD __m128i _mm_madd_epi16 ( __m128i a, __m128i b)VPMADDWD __m256i _mm256_madd_epi16 ( __m256i a, __m256i b)
```
