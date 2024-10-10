# PMULUDQ

Multiply Packed Unsigned Doubleword Integers

Multiplies the first operand (destination operand) by the second operand (source operand) and stores the result in the destination operand.
In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).Legacy SSE version 64-bit operand: The source operand can be an unsigned doubleword integer stored in the low doubleword of an MMX technology register or a 64-bit memory location.
The destination operand can be an quadword integer stored in the destination an MMX technology register.
When a quadword result is too large to be represented in 64 bits (overflow), the result is wrapped around and the low 64 bits are written to the destination element (that is, the carry is ignored).For 64-bit memory operands, 64 bits are fetched from memory, but only the low doubleword is used in the compu-tation.128-bit Legacy SSE version: The second source operand is two packed unsigned doubleword integers stored in the first (low) and third doublewords of an XMM register or a 128-bit memory location.
For 128-bit memory operands, 128 bits are fetched from memory, but only the first and third doublewords are used in the computation.
The first source operand is two packed unsigned doubleword integers stored in the first and third doublewords of an XMM register.
The destination contains two packed unsigned quadword integers stored in an XMM register.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: The second source operand is two packed unsigned doubleword integers stored in the first (low) and third doublewords of an XMM register or a 128-bit memory location.
For 128-bit memory operands, 128 bits are fetched from memory, but only the first and third doublewords are used in the computation.
The first source operand is two packed unsigned doubleword integers stored in the first and third doublewords of an XMM register.
The destination contains two packed unsigned quadword integers stored in an XMM register.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded version: The second source operand is four packed unsigned doubleword integers stored in the first (low), third, fifth, and seventh doublewords of a YMM register or a 256-bit memory location.
For 256-bit memory operands, 256 bits are fetched from memory, but only the first, third, fifth, and seventh doublewords are used in the computation.
The first source operand is four packed unsigned doubleword integers stored in the first, third, fifth, and seventh doublewords of an YMM register.
The destination contains four packed unaligned quadword integers stored in an YMM register.EVEX encoded version: The input unsigned doubleword integers are taken from the even-numbered elements of the source operands.
The first source operand is a ZMM/YMM/XMM registers.
The second source operand can be an ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination is a ZMM/YMM/XMM register, and updated according to the writemask at 64-bit granularity.


## Flags affected

- None.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
PMULUDQ (With 64-Bit Operands)DEST[63:0] := DEST[31:0]  SRC[31:0];PMULUDQ (With 128-Bit Operands)DEST[63:0] := DEST[31:0]  SRC[31:0];DEST[127:64] := DEST[95:64]  SRC[95:64];VPMULUDQ (VEX.128 Encoded Version)DEST[63:0] := SRC1[31:0] * SRC2[31:0]DEST[127:64] := SRC1[95:64] * SRC2[95:64]DEST[MAXVL-1:128] := 0VPMULUDQ (VEX.256 Encoded Version)DEST[63:0] := SRC1[31:0] * SRC2[31:0]DEST[127:64] := SRC1[95:64] * SRC2[95:64DEST[191:128] := SRC1[159:128] * SRC2[159:128]DEST[255:192] := SRC1[2VPMULUDQ (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+63:i] := ZeroExtend64( SRC1[i+31:i]) * ZeroExtend64( SRC2[31:0] )ELSE DEST[i+63:i] := ZeroExtend64( SRC1[i+31:i]) * ZeroExtend64( SRC2[i+31:i] )FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPMULUDQ __m512i _mm512_mul_epu32(__m512i a, __m512i b);VPMULUDQ __m512i _mm512_mask_mul_epu32(__m512i s, __mmask8 k, __m512i a, __m512i b);VPMULUDQ __m512i _mm512_maskz_mul_epu32( __mmask8 k, __m512i a, __m512i b);VPMULUDQ __m256i _mm256_mask_mul_epu32(__m256i s, __mmask8 k, __m256i a, __m256i b);VPMULUDQ __m256i _mm256_maskz_mul_epu32( __mmask8 k, __m256i a, __m256i b);VPMULUDQ __m128i _mm_mask_mul_epu32(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMULUDQ __m128i _mm_maskz_mul_epu32( __mmask8 k, __m128i a, __m128i b);PMULUDQ __m64 _mm_mul_su32 (__m64 a, __m64 b)(V)PMULUDQ __m128i _mm_mul_epu32 ( __m128i a, __m128i b)VPMULUDQ __m256i _mm256_mul_epu32( __m256i a, __m256i b);
```
