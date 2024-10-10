# VPMADD52HUQ

Packed Multiply of Unsigned 52-Bit Unsigned Integers and Add High 52-Bit Products to 64-Bit Accumulators

Multiplies packed unsigned 52-bit integers in each qword element of the first source operand (the second oper-and) with the packed unsigned 52-bit integers in the corresponding elements of the second source operand (the third operand) to form packed 104-bit intermediate results.
The high 52-bit, unsigned integer of each 104-bit product is added to the corresponding qword unsigned integer of the destination operand (the first operand) under the writemask k1.The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM reg-ister, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory loca-tion.
The destination operand is a ZMM/YMM/XMM register

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPMADD52HUQ (EVEX encoded)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64;IF k1[j] OR *no writemask* THENIF src2 is Memory AND EVEX.b=1 THENtsrc2[63:0] := ZeroExtend64(src2[51:0]);ELSEtsrc2[63:0] := ZeroExtend64(src2[i+51:i];FI;Temp128[127:0] := ZeroExtend64(src1[i+51:i]) * tsrc2[63:0];Temp2[63:0] := DEST[i+63:i] + ZeroExtend64(temp128[103:52]) ;DEST[i+63:i] := Temp2[63:0];ELSE IF *zeroing-masking* THENDEST[i+63:i] := 0;ELSE *merge-masking*DEST[i+63:i] is unchanged;FI;FI;ENDFORDEST[MAX_VL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPMADD52HUQ __m512i _mm512_madd52hi_epu64( __m512i a, __m512i b, __m512i c);VPMADD52HUQ __m512i _mm512_mask_madd52hi_epu64(__m512i s, __mmask8 k, __m512i a, __m512i b, __m512i c);VPMADD52HUQ __m512i _mm512_maskz_madd52hi_epu64( __mmask8 k, __m512i a, __m512i b, __m512i c);VPMADD52HUQ __m256i _mm256_madd52hi_epu64( __m256i a, __m256i b, __m256i c);VPMADD52HUQ __m256i _mm256_mask_madd52hi_epu64(__m256i s, __mmask8 k, __m256i a, __m256i b, __m256i c);VPMADD52HUQ __m256i _mm256_maskz_madd52hi_epu64( __mmask8 k, __m256i a, __m256i b, __m256i c);VPMADD52HUQ __m128i _mm_madd52hi_epu64( __m128i a, __m128i b, __m128i c);VPMADD52HUQ __m128i _mm_mask_madd52hi_epu64(__m128i s, __mmask8 k, __m128i a, __m128i b, __m128i c);VPMADD52HUQ __m128i _mm_maskz_madd52hi_epu64( __mmask8 k, __m128i a, __m128i b, __m128i c);
```
