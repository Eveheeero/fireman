# PMULHW

Multiply Packed Signed Integers and Store High Result

Performs a SIMD signed multiply of the packed signed word integers in the destination operand (first operand) and the source operand (second operand), and stores the high 16 bits of each intermediate 32-bit result in the destina-tion operand.
(Figure4-12 shows this operation when using 64-bit operands.) n 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE version 64-bit operand: The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand is an MMX technology register.128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory locatiVEX.128 encoded version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.L must be 0, otherwise the instruction will #UD.VEX.256 encoded version: The second source operand can be an YMM register or a 256-bit memory location.
The first source and destination operands are YMM registers.EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
PMULHW (With 64-bit Operands)TEMP0[31:0] := DEST[15:0]  SRC[15:0]; (* Signed multiplication *)TEMP1[31:0] := DEST[31:16]  SRC[31:16];TEMP2[31:0] := DEST[47:32]  SRC[47:32];TEMP3[31:0] := DEST[63:48]  SRC[63:48];DEST[15:0] := TEMP0[31:16];DEST[31:16] := TEMP1[31:16];DEST[47:32] := TEMP2[31:16];DEST[63:48] := TEMP3[31:16];PMULHW (With 128-bit Operands) SRC[15:0]; (* Signed multiplication *)TEMP0[31:0] := DEST[15:0] TEMP1[31:0] := DEST[31:16]  SRC[31:16];TEMP2[31:0] := DEST[47:32]  SRC[47:32];TEMP3[31:0] := DEST[63:48]  SRC[63:48];TEMP4[31:0] := DEST[79:64]  SRC[79:64];TEMP5[31:0] := DEST[95:80]  SRC[95:80];TEMP6[31:0] := DEST[111:96]  SRC[111:96];TEMP7[31:0] := DEST[127:112]  SRC[127:112];DEST[15:0] := TEMP0[31:16];DEST[31:16] := TEMP1[31:16];DEST[47:32] := TEMP2[31:16];DEST[63:48] := TEMP3[31:16];DEST[79:64] := TEMP4[31:16];DEST[95:80] := TEMP5[31:16];DEST[111:96] := TEMP6[31:16];DEST[127:112] := TEMP7[31:16];VPMULHW (VEX.128 Encoded Version)TEMP0[31:0] := SRC1[15:0] * SRC2[15:0] (*Signed Multiplication*)TEMP1[31:0] := SRC1[31:16] * SRC2[31:16]TEMP2[31:0] := SRC1[47:32] * SRC2[47:32]TEMP3[31:0] := SRC1[63:48] * SRC2[63:48]TEMP4[31:0] := SRC1[79:64] * SRC2[79:64]TEMP5[31:0] := SRC1[95:80] * SRC2[95:80]TEMP6[31:0] := SRC1[111:96] * SRC2[111:96]TEMP7[31:0] := SRC1[127:112] * SRC2[127:112]DEST[15:0] := TEMP0[31:16]DEST[31:16] := TEMP1[31:16]DEST[47:32] := TEMP2[31:16]DEST[63:48] := TEMP3[31:16]DEST[111:96] := TEMP6[31:16]DEST[127:112] := TEMP7[31:16]DEST[MAXVL-1:128] := 0PMULHW (VEX.256 Encoded Version)TEMP0[31:0] := SRC1[15:0] * SRC2[15:0] (*Signed Multiplication*)TEMP1[31:0] := SRC1[31:16] * SRC2[31:16]TEMP2[31:0] := SRC1[47:32] * SRC2[47:32]TEMP3[31:0] := SRC1[63:48] * SRC2[63:48]TEMP4[31:0] := SRC1[79:64] * SRC2[79:64]TEMP5[31:0] := SRC1[95:80] * SRC2[95:80]TEMP6[31:0] := SRC1[111:96] * SRC2[111:96]TEMP7[31:0] := SRC1[127:112] * SRC2[127:112]TEMP8[31:0] := SRC1[143:128] * SRC2[143:128]TEMP9[31:0] := SRC1[159:144] * SRC2[159:144]TEMP10[31:0] := SRC1[175:160] * SRC2[175:160]TEMP11[31:0] := SRC1[191:176] * SRC2[191:176]TEMP12[31:0] := SRC1[207:192] * SRC2[207:192]TEMP13[31:0] := SRC1[223:208] * SRC2[223:208]TEMP14[31:0] := SRC1[239:224] * SRC2[239:224]TEMP15[31:0] := SRC1[255:240] * SRC2[255:240]DEST[15:0] := TEMP0[31:16]DEST[31:16] := TEMP1[31:16]DEST[47:32] := TEMP2[31:16]DEST[63:48] := TEMP3[31:16]DEST[79:64] := TEMP4[31:16]DEST[95:80] := TEMP5[31:16]DEST[111:96] := TEMP6[31:16]DEST[127:112] := TEMP7[31:16]DEST[143:128] := TEMP8[31:16]DEST[159:144] := TEMP9[31:16]DEST[175:160] := TEMP10[31:16]DEST[191:176] := TEMP11[31:16]DEST[207:192] := TEMP12[31:16]DEST[223:208] := TEMP13[31:16]DEST[239:224] := TEMP14[31:16]PMULHW (EVEX Encoded Versions)(KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN temp[31:0] := SRC1[i+15:i] * SRC2[i+15:i]DEST[i+15:i] := tmp[31:16]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPMULHW __m512i _mm512_mulhi_epi16(__m512i a, __m512i b);VPMULHW __m512i _mm512_mask_mulhi_epi16(__m512i s, __mmask32 k, __m512i a, __m512i b);VPMULHW __m512i _mm512_maskz_mulhi_epi16( __mmask32 k, __m512i a, __m512i b);VPMULHW __m256i _mm256_mask_mulhi_epi16(__m256i s, __mmask16 k, __m256i a, __m256i b);VPMULHW __m256i _mm256_maskz_mulhi_epi16( __mmask16 k, __m256i a, __m256i b);VPMULHW __m128i _mm_mask_mulhi_epi16(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMULHW __m128i _mm_maskz_mulhi_epi16( __mmask8 k, __m128i a, __m128i b);PMULHW __m64 _mm_mulhi_pi16 (__m64 m1, __m64 m2)(V)PMULHW __m128i _mm_mulhi_epi16 ( __m128i a, __m128i b)VPMULHW __m256i _mm256_mulhi_epi16 ( __m256i a, __m256i b)
```
