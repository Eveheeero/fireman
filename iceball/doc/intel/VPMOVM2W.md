# VPMOVM2B/VPMOVM2W/VPMOVM2D/VPMOVM2Q

Convert a Mask Register to a Vector Register

Converts a mask register to a vector register.
Each element in the destination register is set to all 1's or all 0's depending on the value of the corresponding bit in the source mask register.The source operand is a mask register.
The de

## Exceptions

- Other Exceptions
  > EVEX-encoded instruction, see Table2-55,
  >  "Type E7NM Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPMOVM2B (EVEX encoded versions) (KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8IF SRC[j]THEN DEST[i+7:i] := -1 ELSE DEST[i+7:i] := 0FI;ENDFORDEST[MAXVL-1:VL] := 0VPMOVM2W (EVEX encoded versions) (KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF SRC[j]THEN DEST[i+15:i] := -1 ELSE DEST[i+15:i] := 0FI;ENDFORDEST[MAXVL-1:VL] := 0VPMOVM2D (EVEX encoded versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF SRC[j]THEN DEST[i+31:i] := -1 ELSE DEST[i+31:i] := 0FI;ENDFORDEST[MAXVL-1:VL] := 0VPMOVM2Q (EVEX encoded versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF SRC[j]THEN DEST[i+63:i] := -1 ELSE DEST[i+63:i] := 0FI;Intel C/C++ Compiler Intrinsic EquivalentsVPMOVM2B __m512i _mm512_movm_epi8(__mmask64 );VPMOVM2D __m512i _mm512_movm_epi32(__mmask8 );VPMOVM2Q __m512i _mm512_movm_epi64(__mmask16 );VPMOVM2W __m512i _mm512_movm_epi16(__mmask32 );VPMOVM2B __m256i _mm256_movm_epi8(__mmask32 );VPMOVM2D __m256i _mm256_movm_epi32(__mmask8 );VPMOVM2Q __m256i _mm256_movm_epi64(__mmask8 );VPMOVM2W __m256i _mm256_movm_epi16(__mmask16 );VPMOVM2B __m128i _mm_movm_epi8(__mmask16 );VPMOVM2D __m128i _mm_movm_epi32(__mmask8 );VPMOVM2Q __m128i _mm_movm_epi64(__mmask8 );VPMOVM2W __m128i _mm_movm_epi16(__mmask8 );
```
