# PMADDUBSW

Multiply and Add Packed Signed and Unsigned Bytes

(V)PMADDUBSW multiplies vertically each unsigned byte of the destination operand (first operand) with the corre-sponding signed byte of the source operand (second operand), producing intermediate signed 16-bit integers.
Each adjacent pair of signed words is added and the saturated result is packed to the destination operand.
For example, the lowest-order bytes (bits 7-0) in the source and destination operands are multiplied and the interme-diate signed word result is added with the corresponding intermediate result from the 2nd lowest-order bytes (bits 15-8) of the operands; the sign-saturated result is stored in the lowest word of the destination register (15-0).
The same operation is performed on the other pairs of adjacent bytes.
Both operands can be MMX register or XMM registers.
When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
128-bit Legacy SSE version: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding destination register remain unchanged.VEX.128 and EVEX.128 encoded versions: The first source and destination operands are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding desti-nation register are zeroed.VEX.256 and EVEX.256 encoded versions: The second source operand can be an YMM register or a 256-bit memory location.
The first source and destination operands are YMM registers.
Bits (MAXVL-1:256) of the corresponding ZMM register are zeroed.EVEX.512 encoded version: The second source operand can be an ZMM register or a 512-bit memory location.
The first source and destination operands are ZMM registers.

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
PMADDUBSW (With 64-bit Operands)DEST[15-0] = SaturateToSignedWord(SRC[15-8]*DEST[15-8]+SRC[7-0]*DEST[7-0]);DEST[31-16] = SaturateToSignedWord(SRC[31-24]*DEST[31-24]+SRC[23-16]*DEST[23-16]);DEST[47-32] = SaturateToSignedWord(SRC[47-40]*DEST[47-40]+SRC[39-32]*DEST[39-32]);DEST[63-48] = SaturateToSignedWord(SRC[63-56]*DEST[63-56]+SRC[55-48]*DEST[55-48]);PMADDUBSW (With 128-bit Operands)DEST[15-0] = SaturateToSignedWord(SRC[15-8]* DEST[15-8]+SRC[7-0]*DEST[7-0]);// Repeat operation for 2nd through 7th word SRC1/DEST[127-112] = SaturateToSignedWord(SRC[127-120]*DEST[127-120]+ SRC[119-112]* DEST[119-112]);VPMADDUBSW (VEX.128 Encoded Version)DEST[15:0] := SaturateToSignedWord(SRC2[15:8]* SRC1[15:8]+SRC2[7:0]*SRC1[7:0])// Repeat operation for 2nd through 7th word DEST[127:112] := SaturateToSignedWord(SRC2[127:120]*SRC1[127:120]+ SRC2[119:112]* SRC1[119:112])DEST[MAXVL-1:128] := 0VPMADDUBSW (VEX.256 Encoded Version)DEST[15:0] := SaturateToSignedWord(SRC2[15:8]* SRC1[15:8]+SRC2[7:0]*SRC1[7:0])// Repeat operation for 2nd through 15th word DEST[255:240] := SaturateToSignedWord(SRC2[255:248]*SRC1[255:248]+ SRC2[247:240]* SRC1[247:240])DEST[MAXVL-1:256] := 0VPMADDUBSW (EVEX Encoded Versions)(KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[i+15:i] := SaturateToSignedWord(SRC2[i+15:i+8]* SRC1[i+15:i+8] + SRC2[i+7:i]*SRC1[i+7:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] = 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentsVPMADDUBSW __m512i _mm512_maddubs_epi16( __m512i a, __m512i b);VPMADDUBSW __m512i _mm512_mask_maddubs_epi16(__m512i s, __mmask32 k, __m512i a, __m512i b);VPMADDUBSW __m512i _mm512_maskz_maddubs_epi16( __mmask32 k, __m512i a, __m512i b);VPMADDUBSW __m256i _mm256_mask_maddubs_epi16(__m256i s, __mmask16 k, __m256i a, __m256i b);VPMADDUBSW __m256i _mm256_maskz_maddubs_epi16( __mmask16 k, __m256i a, __m256i b);VPMADDUBSW __m128i _mm_mask_maddubs_epi16(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMADDUBSW __m128i _mm_maskz_maddubs_epi16( __mmask8 k, __m128i a, __m128i b);PMADDUBSW __m64 _mm_maddubs_pi16 (__m64 a, __m64 b)(V)PMADDUBSW __m128i _mm_maddubs_epi16 (__m128i a, __m128i b)VPMADDUBSW __m256i _mm256_maddubs_epi16 (__m256i a, __m256i b)
```
