# PADDSB/PADDSW

Add Packed Signed Integers with Signed Saturation

Performs a SIMD add of the packed signed integers from the source operand (second operand) and the destination operand (first operand), and stores the packed integer results in the destination operand.
See Figure 9-4 in the ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for an illustration of a SIMD operation.
IntelOverflow is handled with signed saturation, as described in the following paragraphs.(V)PADDSB performs a SIMD add of the packed signed integers with saturation from the first source operand and second source operand and stores the packed integer results in the destination operand.
When an individual byte result is beyond the range of a signed byte integer (that is, greater than 7FH or less than 80H), the saturated value of 7FH or 80H, respectively, is written to the destination operand.(V)PADDSW performs a SIMD add of the packed signed word integers with saturation from the first source operand and second source operand and stores the packed integer results in the destination operand.
When an individual word result is beyond the range of a signed word integer (that is, greater than 7FFFH or less than 8000H), the satu-rated value of 7FFFH or 8000H, respectively, is written to the destination operand.EVEX encoded versions: The first source operand is an ZMM/YMM/XMM register.
The second source operand is an ZMM/YMM/XMM register or a memory location.
The destination operand is an ZMM/YMM/XMM register.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
VEX.128 encoded version: The first source operand is an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding register destination are zeroed.128-bit Legacy SSE version: The first source operand is an XMM register.
The second operand can be an XMM register or an 128-bit memory location.
The destination is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding register destination are unmodified.

## Flags affected

- None.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
PADDSB (With 64-bit Operands)DEST[7:0] := SaturateToSignedByte(DEST[7:0] + SRC (7:0]);(* Repeat add operation for 2nd through 7th bytes *)DEST[63:56] := SaturateToSignedByte(DEST[63:56] + SRC[63:56] );PADDSB (With 128-bit Operands)DEST[7:0] := SaturateToSignedByte (DEST[7:0] + SRC[7:0]);(* Repeat add operation for 2nd through 14th bytes *)DEST[127:120] := SaturateToSignedByte (DEST[111:120] + SRC[127:120]);VPADDSB (VEX.128 Encoded Version)DEST[7:0] := SaturateToSignedByte (SRC1[7:0] + SRC2[7:0]);(* Repeat subtract operation for 2nd through 14th bytes *)DEST[127:120] := SaturateToSignedByte (SRC1[111:120] + SRC2[127:120]);DEST[MAXVL-1:128] := 0VPADDSB (VEX.256 Encoded Version)DEST[7:0] := SaturateToSignedByte (SRC1[7:0] + SRC2[7:0]);VPADDSB (EVEX Encoded Versions)(KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8IF k1[j] OR *no writemask*THEN DEST[i+7:i] := SaturateToSignedByte (SRC1[i+7:i] + SRC2[i+7:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+7:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+7:i] = 0FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0PADDSW (with 64-bit operands)DEST[15:0] := SaturateToSignedWord(DEST[15:0] + SRC[15:0] );(* Repeat add operation for 2nd and 7th words *)DEST[63:48] := SaturateToSignedWord(DEST[63:48] + SRC[63:48] );PADDSW (with 128-bit operands)DEST[15:0] := SaturateToSignedWord (DEST[15:0] + SRC[15:0]);(* Repeat add operation for 2nd through 7th words *)DEST[127:112] := SaturateToSignedWord (DEST[127:112] + SRC[127:112]);VPADDSW (VEX.128 Encoded Version)DEST[15:0] := SaturateToSignedWord (SRC1[15:0] + SRC2[15:0]);(* Repeat subtract operation for 2nd through 7th words *)DEST[127:112] := SaturateToSignedWord (SRC1[127:112] + SRC2[127:112]);DEST[MAXVL-1:128] := 0VPADDSW (VEX.256 Encoded Version)DEST[15:0] := SaturateToSignedWord (SRC1[15:0] + SRC2[15:0]);(* Repeat add operation for 2nd through 15th words *)DEST[255:240] := SaturateToSignedWord (SRC1[255:240] + SRC2[255:240])VPADDSW (EVEX Encoded Versions)(KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[i+15:i] := SaturateToSignedWord (SRC1[i+15:i] + SRC2[i+15:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] = 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentsPADDSB __m64 _mm_adds_pi8(__m64 m1, __m64 m2)(V)PADDSB  __m128i _mm_adds_epi8 ( __m128i a, __m128i b)VPADDSB __m256i _mm256_adds_epi8 ( __m256i a, __m256i b)PADDSW __m64 _mm_adds_pi16(__m64 m1, __m64 m2)(V)PADDSW __m128i _mm_adds_epi16 ( __m128i a, __m128i b)VPADDSW __m256i _mm256_adds_epi16 ( __m256i a, __m256i b)VPADDSB __m512i _mm512_adds_epi8 ( __m512i a, __m512i b)VPADDSW __m512i _mm512_adds_epi16 ( __m512i a, __m512i b)VPADDSB __m512i _mm512_mask_adds_epi8 ( __m512i s, __mmask64 m, __m512i a, __m512i b)VPADDSW __m512i _mm512_mask_adds_epi16 ( __m512i s, __mmask32 m, __m512i a, __m512i b)VPADDSB __m512i _mm512_maskz_adds_epi8 (__mmask64 m, __m512i a, __m512i b)VPADDSW __m512i _mm512_maskz_adds_epi16 (__mmask32 m, __m512i a, __m512i b)VPADDSB __m256i _mm256_mask_adds_epi8 (__m256i s, __mmask32 m, __m256i a, __m256i b)VPADDSW __m256i _mm256_mask_adds_epi16 (__m256i s, __mmask16 m, __m256i a, __m256i b)VPADDSB __m256i _mm256_maskz_adds_epi8 (__mmask32 m, __m256i a, __m256i b)VPADDSW __m256i _mm256_maskz_adds_epi16 (__mmask16 m, __m256i a, __m256i b)VPADDSB __m128i _mm_mask_adds_epi8 (__m128i s, __mmask16 m, __m128i a, __m128i b)VPADDSW __m128i _mm_mask_adds_epi16 (__m128i s, __mmask8 m, __m128i a, __m128i b)VPADDSB __m128i _mm_maskz_adds_epi8 (__mmask16 m, __m128i a, __m128i b)VPADDSW __m128i _mm_maskz_adds_epi16 (__mmask8 m, __m128i a, __m128i b)
```
