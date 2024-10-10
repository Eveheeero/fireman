# PMULLW

Multiply Packed Signed Integers and Store Low Result

Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the low 16 bits of each intermediate 32-bit result in the destina-tion operand.
(Figure4-12 shows this operation when using 64-bit operands.)In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE version 64-bit operand: The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand is an MMX technology register.128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory locatiVEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.L must be 0, otherwise the instruction will #UD.VEX.256 encoded version: The second source operand can be an YMM register or a 256-bit memory location.
The first source and destination operands are YMM registers.EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location.
The destination operand is conditionally updated based on writemask k1.SRCX3X2X1X0DESTY3Y2Y1Y0Z3  X3  Y3Z2  X2  Y2Z1  X1  Y1Z0  X0  Y0====TEMPDESTZ3[15:0]Z2[15:0]Z1[15:0]Z0[15:0]Figure 4-13.
 PMULLU Instruction Operation Using 64-bit Operands

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
PMULLW (With 64-bit Operands)TEMP0[31:0] := DEST[15:0]  SRC[15:0]; (* Signed multiplication *)TEMP1[31:0] := DEST[31:16]  SRC[31:16];TEMP2[31:0] := DEST[47:32]  SRC[47:32];TEMP3[31:0] := DEST[63:48]  SRC[63:48];DEST[15:0] := TEMP0[15:0];DEST[31:16] := TEMP1[15:0];DEST[47:32] := TEMP2[15:0];DEST[63:48] := TEMP3[15:0];PMULLW (With 128-bit Operands) SRC[15:0]; (* Signed multiplication *)TEMP0[31:0] := DEST[15:0] TEMP1[31:0] := DEST[31:16]  SRC[31:16];TEMP2[31:0] := DEST[47:32]  SRC[47:32];TEMP3[31:0] := DEST[63:48]  SRC[63:48];TEMP4[31:0] := DEST[79:64]  SRC[79:64];TEMP5[31:0] := DEST[95:80]  SRC[95:80];TEMP6[31:0] := DEST[111:96]  SRC[111:96];TEMP7[31:0] := DEST[127:112]  SRC[127:112];DEST[15:0] := TEMP0[15:0];DEST[31:16] := TEMP1[15:0];DEST[47:32] := TEMP2[15:0];DEST[63:48] := TEMP3[15:0];DEST[79:64] := TEMP4[15:0];DEST[95:80] := TEMP5[15:0];DEST[111:96] := TEMP6[15:0];VPMULLW (VEX.128 Encoded Version)Temp0[31:0] := SRC1[15:0] * SRC2[15:0]Temp1[31:0] := SRC1[31:16] * SRC2[31:16]Temp2[31:0] := SRC1[47:32] * SRC2[47:32]Temp3[31:0] := SRC1[63:48] * SRC2[63:48]Temp4[31:0] := SRC1[79:64] * SRC2[79:64]Temp5[31:0] := SRC1[95:80] * SRC2[95:80]Temp6[31:0] := SRC1[111:96] * SRC2[111:96]Temp7[31:0] := SRC1[127:112] * SRC2[127:112]DEST[15:0] := Temp0[15:0]DEST[31:16] := Temp1[15:0]DEST[47:32] := Temp2[15:0]DEST[63:48] := Temp3[15:0]DEST[79:64] := Temp4[15:0]DEST[95:80] := Temp5[15:0]DEST[111:96] := Temp6[15:0]DEST[127:112] := Temp7[15:0]DEST[MAXVL-1:128] := 0PMULLW (EVEX Encoded Versions)(KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN temp[31:0] := SRC1[i+15:i] * SRC2[i+15:i]DEST[i+15:i] := temp[15:0]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPMULLW __m512i _mm512_mullo_epi16(__m512i a, __m512i b);VPMULLW __m512i _mm512_mask_mullo_epi16(__m512i s, __mmask32 k, __m512i a, __m512i b);VPMULLW __m512i _mm512_maskz_mullo_epi16( __mmask32 k, __m512i a, __m512i b);VPMULLW __m256i _mm256_mask_mullo_epi16(__m256i s, __mmask16 k, __m256i a, __m256i b);VPMULLW __m256i _mm256_maskz_mullo_epi16( __mmask16 k, __m256i a, __m256i b);VPMULLW __m128i _mm_mask_mullo_epi16(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMULLW __m128i _mm_maskz_mullo_epi16( __mmask8 k, __m128i a, __m128i b);PMULLW __m64 _mm_mullo_pi16(__m64 m1, __m64 m2)(V)PMULLW __m128i _mm_mullo_epi16 ( __m128i a, __m128i b)VPMULLW __m256i _mm256_mullo_epi16 ( __m256i a, __m256i b);
```
