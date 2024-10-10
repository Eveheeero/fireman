# VPMOVB2M/VPMOVW2M/VPMOVD2M/VPMOVQ2M

Convert a Vector Register to a Mask

Converts a vector register to a mask register.
Each element in the destination register is set to 1 or 0 depending on the value of most significant bit of the corresponding element in the source register.The source operand is a ZMM/YMM/XMM register.


## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > EVEX-encoded instruction, see Table2-55, "Type E7NM Class Exception Conditions."

## Operation

```C
VPMOVB2M (EVEX encoded versions) (KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8IF SRC[i+7]THEN DEST[j] := 1 ELSE DEST[j] := 0FI;ENDFORDEST[MAX_KL-1:KL] := 0VPMOVW2M (EVEX encoded versions) (KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF SRC[i+15]THEN DEST[j] := 1 ELSE DEST[j] := 0FI;ENDFORDEST[MAX_KL-1:KL] := 0VPMOVD2M (EVEX encoded versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF SRC[i+31]THEN DEST[j] := 1 ELSE DEST[j] := 0FI;ENDFORDEST[MAX_KL-1:KL] := 0VPMOVQ2M (EVEX encoded versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF SRC[i+63]THEN DEST[j] := 1 ELSE DEST[j] := 0FI;Intel C/C++ Compiler Intrinsic EquivalentsVPMPOVB2M __mmask64 _mm512_movepi8_mask( __m512i );VPMPOVD2M __mmask16 _mm512_movepi32_mask( __m512i );VPMPOVQ2M __mmask8 _mm512_movepi64_mask( __m512i );VPMPOVW2M __mmask32 _mm512_movepi16_mask( __m512i );VPMPOVB2M __mmask32 _mm256_movepi8_mask( __m256i );VPMPOVD2M __mmask8 _mm256_movepi32_mask( __m256i );VPMPOVQ2M __mmask8 _mm256_movepi64_mask( __m256i );VPMPOVW2M __mmask16 _mm256_movepi16_mask( __m256i );VPMPOVB2M __mmask16 _mm_movepi8_mask( __m128i );VPMPOVD2M __mmask8 _mm_movepi32_mask( __m128i );VPMPOVQ2M __mmask8 _mm_movepi64_mask( __m128i );VPMPOVW2M __mmask8 _mm_movepi16_mask( __m128i );
```
