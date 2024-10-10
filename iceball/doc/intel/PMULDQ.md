# PMULDQ

Multiply Packed Doubleword Integers

Multiplies packed signed doubleword integers in the even-numbered (zero-based reference) elements of the first source operand with the packed signed doubleword integers in the corresponding elements of the second source operand and stores packed signed quadword results in the destination operand.
128-bit Legacy SSE version: The input signed doubleword integers are taken from the even-numbered elements of the source operands, i.e., the first (low) and third doubleword element.
For 128-bit memory operands, 128 bits are fetched from memory, but only the first and third doublewords are used in the computation.
The first source operand and the destination XMM operand is the same.
The second source operand can be an XMM register or 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding destination register remain unchanged.VEX.128 encoded version: The input signed doubleword integers are taken from the even-numbered elements of the source operands, i.e., the first (low) and third doubleword element.
For 128-bit memory operands, 128 bits are fetched from memory, but only the first and third doublewords are used in the computation.The first source operand and the destination operand are XMM registers.
The second source operand can be an XMM register or 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding destination register are zeroed.VEX.256 encoded version: The input signed doubleword integers are taken from the even-numbered elements of the source operands, i.e., the first, 3rd, 5th, 7th doubleword element.
For 256-bit memory operands, 256 bits are fetched from memory, but only the four even-numbered doublewords are used in the computation.
The first source operand and the destination operand are YMM registers.
The second source operand can be a YMM register or 256-EVEX encoded version: The input signed doubleword integers are taken from the even-numbered elements of the source operands.
The first source operand is a ZMM/YMM/XMM registers.
The second source operand can be an ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination is a ZMM/YMM/XMM register, and updated according to the writemask at 64-bit granularity.


## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
VPMULDQ (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+63:i] := SignExtend64( SRC1[i+31:i]) * SignExtend64( SRC2[31:0])ELSE DEST[i+63:i] := SignExtend64( SRC1[i+31:i]) * SignExtend64( SRC2[i+31:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VPMULDQ (VEX.256 Encoded Version)DEST[63:0] := SignExtend64( SRC1[31:0]) * SignExtend64( SRC2[31:0])DEST[127:64] := SignExtend64( SRC1[95:64]) * SignExtend64( SRC2[95:64])DEST[191:128] := SignExtend64( SRC1[159:128]) * SignExtend64( SRC2[159:128])DEST[255:192] := SignExtend64( SRC1[223:192]) * SignExtend64( SRC2[223:192])DEST[MAXVL-1:256] := 0VPMULDQ (VEX.128 Encoded Version)DEST[63:0] := SignExtend64( SRC1[31:0]) * SignExtend64( SRC2[31:0])DEST[127:64] := SignExtend64( SRC1[95:64]) * SignExtend64( SRC2[95:64])DEST[MAXVL-1:128] := 0PMULDQ (128-bit Legacy SSE Version)DEST[63:0] := SignExtend64( DEST[31:0]) * SignExtend64( SRC[31:0])DEST[127:64] := SignExtend64( DEST[95:64]) * SignExtend64( SRC[95:64])DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVPMULDQ __m512i _mm512_mul_epi32(__m512i a, __m512i b);VPMULDQ __m512i _mm512_mask_mul_epi32(__m512i s, __mmask8 k, __m512i a, __m512i b);VPMULDQ __m512i _mm512_maskz_mul_epi32( __mmask8 k, __m512i a, __m512i b);VPMULDQ __m256i _mm256_mask_mul_epi32(__m256i s, __mmask8 k, __m256i a, __m256i b);VPMULDQ __m256i _mm256_mask_mul_epi32( __mmask8 k, __m256i a, __m256i b);VPMULDQ __m128i _mm_mask_mul_epi32(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMULDQ __m256i _mm256_mul_epi32( __m256i a, __m256i b);
```
