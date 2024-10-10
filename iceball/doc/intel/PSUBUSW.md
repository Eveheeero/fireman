# PSUBUSB/PSUBUSW

Subtract Packed Unsigned Integers With Unsigned Saturation

Performs a SIMD subtract of the packed unsigned integers of the source operand (second operand) from the packed unsigned integers of the destination operand (first operand), and stores the packed unsigned integer ® 64 and IA-32 Architectures Software Developer's results in the destination operand.
See Figure 9-4 in the IntelManual, Volume 1, for an illustration of a SIMD operation.
Overflow is handled with unsigned saturation, as described in the following paragraphs.These instructions can operate on either 64-bit or 128-bit operands.The (V)PSUBUSB instruction subtracts packed unsigned byte integers.
When an individual byte result is less than zero, the saturated value of 00H is written to the destination operand.The (V)PSUBUSW instruction subtracts packed unsigned word integers.
When an individual word result is less than zero, the saturated value of 0000H is written to the destination operand.In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE version 64-bit operand: The destination operand must be an MMX technology register and the source operand can be either an MMX technology register or a 64-bit memory location.
128-bit Legacy SSE version: The second source operand is an XMM register or a 128-bit memory location.
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
- Numeric Exceptions
  > None.

## Operation

```C
PSUBUSB (With 64-bit Operands)-DEST[7:0] := SaturateToUnsignedByte (DEST[7:0]  SRC (7:0] );(* Repeat add operation for 2nd through 7th bytes *)- SRC[63:56]; DEST[63:56] := SaturateToUnsignedByte (DEST[63:56] PSUBUSW (With 64-bit Operands)- SRC[15:0] );DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] (* Repeat add operation for 2nd and 3rd words *)-VPSUBUSB (EVEX Encoded Versions) (KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8;IF k1[j] OR *no writemask*THEN DEST[i+7:i] := SaturateToUnsignedByte (SRC1[i+7:i] - SRC2[i+7:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+7:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+7:i] := 0;FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0;VPSUBUSW (EVEX Encoded Versions) (KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16;IF k1[j] OR *no writemask*THEN DEST[i+15:i] := SaturateToUnsignedWord (SRC1[i+15:i] - SRC2[i+15:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] := 0;FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0;VPSUBUSB (VEX.256 Encoded Version)DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] - SRC2[7:0]);(* Repeat subtract operation for 2nd through 31st bytes *)DEST[255:148] := SaturateToUnsignedByte (SRC1[255:248] - SRC2[255:248]);DEST[MAXVL-1:256] := 0;VPSUBUSB (VEX.128 Encoded Version)DEST[7:0] := SaturateToUnsignedByte (SRC1[7:0] - SRC2[7:0]);(* Repeat subtract operation for 2nd through 14th bytes *)DEST[127:120] := SaturateToUnsignedByte (SRC1[127:120] - SRC2[127:120]);DEST[MAXVL-1:128] := 0SB (128-bit Legacy SSE Version)PSUBUDEST[7:0] := SaturateToUnsignedByte (DEST[7:0] - SRC[7:0]);(* Repeat subtract operation for 2nd through 14th bytes *)VPSUBUSW (VEX.256 Encoded Version)DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] - SRC2[15:0]);(* Repeat subtract operation for 2nd through 15th words *)DEST[255:240] := SaturateToUnsignedWord (SRC1[255:240] - SRC2[255:240]);DEST[MAXVL-1:256] := 0;VPSUBUSW (VEX.128 Encoded Version)DEST[15:0] := SaturateToUnsignedWord (SRC1[15:0] - SRC2[15:0]);(* Repeat subtract operation for 2nd through 7th words *)DEST[127:112] := SaturateToUnsignedWord (SRC1[127:112] - SRC2[127:112]);DEST[MAXVL-1:128] := 0PSUBUSW (128-bit Legacy SSE Version)DEST[15:0] := SaturateToUnsignedWord (DEST[15:0] - SRC[15:0]);(* Repeat subtract operation for 2nd through 7th words *)DEST[127:112] := SaturateToUnsignedWord (DEST[127:112] - SRC[127:112]);DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentsVPSUBUSB __m512i _mm512_subs_epu8(__m512i a, __m512i b);VPSUBUSB __m512i _mm512_mask_subs_epu8(__m512i s, __mmask64 k, __m512i a, __m512i b);VPSUBUSB __m512i _mm512_maskz_subs_epu8( __mmask64 k, __m512i a, __m512i b);VPSUBUSB __m256i _mm256_mask_subs_epu8(__m256i s, __mmask32 k, __m256i a, __m256i b);VPSUBUSB __m256i _mm256_maskz_subs_epu8( __mmask32 k, __m256i a, __m256i b);VPSUBUSB __m128i _mm_mask_subs_epu8(__m128i s, __mmask16 k, __m128i a, __m128i b);VPSUBUSB __m128i _mm_maskz_subs_epu8( __mmask16 k, __m128i a, __m128i b);VPSUBUSW __m512i _mm512_subs_epu16(__m512i a, __m512i b);VPSUBUSW __m512i _mm512_mask_subs_epu16(__m512i s, __mmask32 k, __m512i a, __m512i b);VPSUBUSW __m512i _mm512_maskz_subs_epu16( __mmask32 k, __m512i a, __m512i b);VPSUBUSW __m256i _mm256_mask_subs_epu16(__m256i s, __mmask16 k, __m256i a, __m256i b);VPSUBUSW __m256i _mm256_maskz_subs_epu16( __mmask16 k, __m256i a, __m256i b);VPSUBUSW __m128i _mm_mask_subs_epu16(__m128i s, __mmask8 k, __m128i a, __m128i b);VPSUBUSW __m128i _mm_maskz_subs_epu16( __mmask8 k, __m128i a, __m128i b);PSUBUSB __m64 _mm_subs_pu8(__m64 m1, __m64 m2)(V)PSUBUSB __m128i _mm_subs_epu8(__m128i m1, __m128i m2)VPSUBUSB __m256i _mm256_subs_epu8(__m256i m1, __m256i m2)PSUBUSW __m64 _mm_subs_pu16(__m64 m1, __m64 m2)(V)PSUBUSW __m128i _mm_subs_epu16(__m128i m1, __m128i m2)VPSUBUSW __m256i _mm256_subs_epu16(__m256i m1, __m256i m2)
```
