# PSUBSB/PSUBSW

Subtract Packed Signed Integers With Signed Saturation

Performs a SIMD subtract of the packed signed integers of the source operand (second operand) from the packed signed integers of the destination operand (first operand), and stores the packed integer results in the destination ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for an operand.
See Figure 9-4 in the Intelillustration of a SIMD operation.
Overflow is handled with signed saturation, as described in the following para-graphs.The (V)PSUBSB instruction subtracts packed signed byte integers.
When an individual byte result is beyond the range of a signed byte integer (that is, greater than 7FH or less than 80H), the saturated value of 7FH or 80H, respectively, is written to the destination operand.The (V)PSUBSW instruction subtracts packed signed word integers.
When an individual word result is beyond the range of a signed word integer (that is, greater than 7FFFH or less than 8000H), the saturated value of 7FFFH or 8000H, respectively, is written to the destination operand.In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE version 64-bit operand: The destination operand must be an MMX technology register and the source operand can be either an MMX technology register or a 64-bit memory location.128-bit Legacy SSE version: The second source operand is an XMM register or a 128-bit memory location.
The first source operand and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corresponding YMM desti-nation register remain unchanged.VEX.128 encoded version: The second source operand is an XMM register or a 128-bit memory location.
The first source operand and destination operands are XMM registers.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded versions: The second source operand is an YMM register or an 256-bit memory location.
The first source operand and destination operands are YMM registers.
Bits (MAXVL-1:256) of the corresponding ZMM register are zeroed.EVEX encoded version: The second source operand is an ZMM/YMM/XMM register or an 512/256/128-bit memory location.
The first source operand and destination operands are ZMM/YMM/XMM registers.
The destination is condi-tionally updated with writemask k1.

## Flags affected

- None.

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."
- Numeric Exceptions
  > None.

## Operation

```C
PSUBSB (With 64-bit Operands)-DEST[7:0] := SaturateToSignedByte (DEST[7:0]  SRC (7:0]);(* Repeat subtract operation for 2nd through 7th bytes *)-PSUBSW (With 64-bit Operands)-DEST[15:0] := SaturateToSignedWord (DEST[15:0]  SRC[15:0] );(* Repeat subtract operation for 2nd and 7th words *)- SRC[63:48] );DEST[63:48] := SaturateToSignedWord (DEST[63:48] VPSUBSB (EVEX Encoded Versions) (KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8;IF k1[j] OR *no writemask*THEN DEST[i+7:i] := SaturateToSignedByte (SRC1[i+7:i] - SRC2[i+7:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+7:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+7:i] := 0;FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0VPSUBSW (EVEX Encoded Versions) (KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[i+15:i] := SaturateToSignedWord (SRC1[i+15:i] - SRC2[i+15:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] := 0;FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0;VPSUBSB (VEX.256 Encoded Version)DEST[7:0] := SaturateToSignedByte (SRC1[7:0] - SRC2[7:0]);(* Repeat subtract operation for 2nd through 31th bytes *)DEST[255:248] := SaturateToSignedByte (SRC1[255:248] - SRC2[255:248]);DEST[MAXVL-1:256] := 0;VPSUBSB (VEX.128 Encoded Version)DEST[7:0] := SaturateToSignedByte (SRC1[7:0] - SRC2[7:0]);(* Repeat subtract operation for 2nd through 14th bytes *)DEST[127:120] := SaturateToSignedByte (SRC1[127:120] - SRC2[127:120]);DEST[MAXVL-1:128] := 0;PSUBSB (128-bit Legacy SSE Version)DEST[7:0] := SaturateToSignedByte (DEST[7:0] - SRC[7:0]);(* Repeat subtract operation for 2nd through 14th bytes *)DEST[127:120] := SaturateToSignedByte (DEST[127:120] - SRC[127:120]);VPSUBSW (VEX.256 Encoded Version)DEST[15:0] := SaturateToSignedWord (SRC1[15:0] - SRC2[15:0]);(* Repeat subtract operation for 2nd through 15th words *)DEST[255:240] := SaturateToSignedWord (SRC1[255:240] - SRC2[255:240]);DEST[MAXVL-1:256] := 0;VPSUBSW (VEX.128 Encoded Version)DEST[15:0] := SaturateToSignedWord (SRC1[15:0] - SRC2[15:0]);(* Repeat subtract operation for 2nd through 7th words *)DEST[127:112] := SaturateToSignedWord (SRC1[127:112] - SRC2[127:112]);DEST[MAXVL-1:128] := 0;PSUBSW (128-bit Legacy SSE Version) DEST[15:0] := SaturateToSignedWord (DEST[15:0] - SRC[15:0]);(* Repeat subtract operation for 2nd through 7th words *)DEST[127:112] := SaturateToSignedWord (DEST[127:112] - SRC[127:112]);DEST[MAXVL-1:128] (Unmodified);Intel C/C++ Compiler Intrinsic EquivalentsVPSUBSB __m512i _mm512_subs_epi8(__m512i a, __m512i b);VPSUBSB __m512i _mm512_mask_subs_epi8(__m512i s, __mmask64 k, __m512i a, __m512i b);VPSUBSB __m512i _mm512_maskz_subs_epi8( __mmask64 k, __m512i a, __m512i b);VPSUBSB __m256i _mm256_mask_subs_epi8(__m256i s, __mmask32 k, __m256i a, __m256i b);VPSUBSB __m256i _mm256_maskz_subs_epi8( __mmask32 k, __m256i a, __m256i b);VPSUBSB __m128i _mm_mask_subs_epi8(__m128i s, __mmask16 k, __m128i a, __m128i b);VPSUBSB __m128i _mm_maskz_subs_epi8( __mmask16 k, __m128i a, __m128i b);VPSUBSW __m512i _mm512_subs_epi16(__m512i a, __m512i b);VPSUBSW __m512i _mm512_mask_subs_epi16(__m512i s, __mmask32 k, __m512i a, __m512i b);VPSUBSW __m512i _mm512_maskz_subs_epi16( __mmask32 k, __m512i a, __m512i b);VPSUBSW __m256i _mm256_mask_subs_epi16(__m256i s, __mmask16 k, __m256i a, __m256i b);VPSUBSW __m256i _mm256_maskz_subs_epi16( __mmask16 k, __m256i a, __m256i b);VPSUBSW __m128i _mm_mask_subs_epi16(__m128i s, __mmask8 k, __m128i a, __m128i b);VPSUBSW __m128i _mm_maskz_subs_epi16( __mmask8 k, __m128i a, __m128i b);PSUBSB __m64 _mm_subs_pi8(__m64 m1, __m64 m2)(V)PSUBSB __m128i _mm_subs_epi8(__m128i m1, __m128i m2)VPSUBSB __m256i _mm256_subs_epi8(__m256i m1, __m256i m2)PSUBSW __m64 _mm_subs_pi16(__m64 m1, __m64 m2)(V)PSUBSW __m128i _mm_subs_epi16(__m128i m1, __m128i m2)VPSUBSW __m256i _mm256_subs_epi16(__m256i m1, __m256i m2)
```
