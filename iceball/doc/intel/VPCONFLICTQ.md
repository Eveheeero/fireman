# VPCONFLICTD/Q

Detect Conflicts Within a Vector of Packed Dword/Qword Values Into Dense Memory/ Register

Test each dword/qword element of the source operand (the second operand) for equality with all other elements in the source operand closer to the least significant element.
Each element's comparison results form a bit vector, which is then zero extended and written to the destination according to the writemask.EVEX.512 encoded version: The source operand is a ZMM register, a 512-bit memory location, or a 512-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a ZMM register, conditionally updated using writemask k1.
EVEX.256 encoded version: The source operand is a YMM register, a 256-bit memory location, or a 256-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a YMM register, conditionally updated using writemask k1.
EVEX.128 encoded version: The source operand is a XMM register, a 128-bit memory location, or a 128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a XMM register, conditionally updated 

## Exceptions

- SIMD Floating-Point Exceptions
  > None

## Operation

```C
VPCONFLICTD(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j*32IF MaskBit(j) OR *no writemask*THEN FOR k := 0 TO j-1m := k*32IF ((SRC[i+31:i] = SRC[m+31:m])) THEN DEST[i+k] := 1ELSE DEST[i+k] := 0FIENDFORDEST[i+31:i+j] := 0ELSEIF *merging-masking* THEN *DEST[i+31:i] remains unchanged*ELSE DEST[i+31:i] := 0FIFIENDFOR := 0DEST[MAXVL-1:VL]VPCONFLICTQ(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j*64IF MaskBit(j) OR *no writemask*THEN FOR k := 0 TO j-1  m:=k*64IF ((SRC[i+63:i] = SRC[m+63:m])) THEN  DEST[i+k] :=1ELSE   DEST[i+k]:=0FIENDFOR  :=0DEST[i+63:i+j]ELSEIF *merging-masking* THEN *DEST[i+63:i] remains unchanged*ELSE  DEST[i+63:i]:=0 FIFIIntel C/C++ Compiler Intrinsic EquivalentVPCONFLICTD __m512i _mm512_conflict_epi32( __m512i a);VPCONFLICTD __m512i _mm512_mask_conflict_epi32(__m512i s, __mmask16 m, __m512i a);VPCONFLICTD __m512i _mm512_maskz_conflict_epi32(__mmask16 m, __m512i a);VPCONFLICTQ __m512i _mm512_conflict_epi64( __m512i a);VPCONFLICTQ __m512i _mm512_mask_conflict_epi64(__m512i s, __mmask8 m, __m512i a);VPCONFLICTQ __m512i _mm512_maskz_conflict_epi64(__mmask8 m, __m512i a);VPCONFLICTD __m256i _mm256_conflict_epi32( __m256i a);VPCONFLICTD __m256i _mm256_mask_conflict_epi32(__m256i s, __mmask8 m, __m256i a);VPCONFLICTD __m256i _mm256_maskz_conflict_epi32(__mmask8 m, __m256i a);VPCONFLICTQ __m256i _mm256_conflict_epi64( __m256i a);VPCONFLICTQ __m256i _mm256_mask_conflict_epi64(__m256i s, __mmask8 m, __m256i a);VPCONFLICTQ __m256i _mm256_maskz_conflict_epi64(__mmask8 m, __m256i a);VPCONFLICTD __m128i _mm_conflict_epi32( __m128i a);VPCONFLICTD __m128i _mm_mask_conflict_epi32(__m128i s, __mmask8 m, __m128i a);VPCONFLICTD __m128i _mm_maskz_conflict_epi32(__mmask8 m, __m128i a);VPCONFLICTQ __m128i _mm_conflict_epi64( __m128i a);VPCONFLICTQ __m128i _mm_mask_conflict_epi64(__m128i s, __mmask8 m, __m128i a);VPCONFLICTQ __m128i _mm_maskz_conflict_epi64(__mmask8 m, __m128i a);
```
