# PADDUSB/PADDUSW

Add Packed Unsigned Integers With Unsigned Saturation

Performs a SIMD add of the packed unsigned integers from the source operand (second operand) and the destina-tion operand (first operand), and stores the packed integer results in the destination operand.
See Figure 9-4 in the ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for an illustration of a SIMD operation.
IntelOverflow is handled with unsigned saturation, as described in the following paragraphs.(V)PADDUSB performs a SIMD add of the packed unsigned integers with saturation from the first source operand and second source operand and stores the packed integer results in the destination operand.
When an individual byte result is beyond the range of an unsigned byte integer (that is, greater than FFH), the saturated value of FFH is written to the destination operand.(V)PADDUSW performs a SIMD add of the packed unsigned word integers with saturation from the first source operand and second source operand and stores the packed integer results in the destination operand.
When an individual word result is beyond the range of an unsigned word integer (that is, greater than FFFFH), the saturated value of FFFFH is written to the destination operand.EVEX encoded versions: The first source operand is an ZMM/YMM/XMM register.
The second source operand is an ZMM/YMM/XMM register or a 512/256/128-bit memory location.
The destination is an ZMM/YMM/XMM register.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
VEX.128 encoded version: The first source operand is an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding destination register destination are zeroed.128-bit Legacy SSE version: The first source operand is an XMM register.
The second operand can be an XMM register or an 128-bit memory location.
The destination is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding register destination are unmodified.

## Flags affected

- None.

## Exceptions

- Numeric Exceptions
  > None.
- Other Exceptions

## Operation

```C
PADDUSB (With 64-bit Operands)DEST[7:0] := SaturateToUnsignedByte(DEST[7:0] + SRC (7:0] );(* Repeat add operation for 2nd through 7th bytes *)DEST[63:56] := SaturateToUnsignedByte(DEST[63:56] + SRC[63:56] PADDUSB (With 128-bit Operands)DEST[7:0] := SaturateToUnsignedByte (DEST[7:0] + SRC[7:0]);(* Repeat add operation for 2nd through 14th bytes *)SRC[127:120]);DEST[127:120] := SaturateToUnSignedByte (DEST[127:120] + VPADDUSB (VEX.128 Encoded Version)DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] + SRC2[7:0]);(* Repeat subtract operation for 2nd through 14th bytes *)VPADDUSB (VEX.256 Encoded Version)DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] + SRC2[7:0]);(* Repeat add operation for 2nd through 31st bytes *)DEST[255:248] := SaturateToUnsignedByte (SRC1[255:248] + SRC2[255:248]);PADDUSW (With 64-bit Operands)DEST[15:0] := SaturateToUnsignedWord(DEST[15:0] + SRC[15:0] );(* Repeat add operation for 2nd and 3rd words *)DEST[63:48] := SaturateToUnsignedWord(DEST[63:48] + SRC[63:48] );PADDUSW (With 128-bit Operands)DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] + SRC[15:0]);(* Repeat add operation for 2nd through 7th words *)DEST[127:112] := SaturateToUnSignedWord (DEST[127:112] + SRC[127:112]);VPADDUSW (VEX.128 Encoded Version)DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] + SRC2[15:0]);(* Repeat subtract operation for 2nd through 7th words *)DEST[127:112] := SaturateToUnsignedWord (SRC1[127:112] + SRC2[127:112]);DEST[MAXVL-1:128] := 0VPADDUSW (VEX.256 Encoded Version)DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] + SRC2[15:0]);(* Repeat add operation for 2nd through 15th words *)DEST[255:240] := SaturateToUnsignedWord (SRC1[255:240] + SRC2[255:240])VPADDUSB (EVEX Encoded Versions)(KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8IF k1[j] OR *no writemask*THEN DEST[i+7:i] := SaturateToUnsignedByte (SRC1[i+7:i] + SRC2[i+7:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+7:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+7:i] = 0FIFI;VPADDUSW (EVEX Encoded Versions)(KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[i+15:i] := SaturateToUnsignedWord (SRC1[i+15:i] + SRC2[i+15:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] = 0FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentsPADDUSB __m64 _mm_adds_pu8(__m64 m1, __m64 m2)PADDUSW __m64 _mm_adds_pu16(__m64 m1, __m64 m2)(V)PADDUSB __m128i _mm_adds_epu8 ( __m128i a, __m128i b)(V)PADDUSW __m128i _mm_adds_epu16 ( __m128i a, __m128i b)VPADDUSB __m256i _mm256_adds_epu8 ( __m256i a, __m256i b)VPADDUSW __m256i _mm256_adds_epu16 ( __m256i a, __m256i b)VPADDUSB __m512i _mm512_adds_epu8 ( __m512i a, __m512i b)VPADDUSW __m512i _mm512_adds_epu16 ( __m512i a, __m512i b)VPADDUSB __m512i _mm512_mask_adds_epu8 ( __m512i s, __mmask64 m, __m512i a, __m512i b)VPADDUSW __m512i _mm512_mask_adds_epu16 ( __m512i s, __mmask32 m, __m512i a, __m512i b)VPADDUSB __m512i _mm512_maskz_adds_epu8 (__mmask64 m, __m512i a, __m512i b)VPADDUSW __m512i _mm512_maskz_adds_epu16 (__mmask32 m, __m512i a, __m512i b)VPADDUSB __m256i _mm256_mask_adds_epu8 (__m256i s, __mmask32 m, __m256i a, __m256i b)VPADDUSW __m256i _mm256_mask_adds_epu16 (__m256i s, __mmask16 m, __m256i a, __m256i b)VPADDUSB __m256i _mm256_maskz_adds_epu8 (__mmask32 m, __m256i a, __m256i b)VPADDUSW __m256i _mm256_maskz_adds_epu16 (__mmask16 m, __m256i a, __m256i b)VPADDUSB __m128i _mm_mask_adds_epu8 (__m128i s, __mmask16 m, __m128i a, __m128i b)VPADDUSW __m128i _mm_mask_adds_epu16 (__m128i s, __mmask8 m, __m128i a, __m128i b)VPADDUSB __m128i _mm_maskz_adds_epu8 (__mmask16 m, __m128i a, __m128i b)VPADDUSW __m128i _mm_maskz_adds_epu16 (__mmask8 m, __m128i a, __m128i b)
```
