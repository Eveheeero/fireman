# UNPCKHPS

Unpack and Interleave High Packed Single Precision Floating-Point Values

Performs an interleaved unpack of the high single precision floating-point values from the first source operand and the second source operand.
128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.
When unpacking from a memory operand, an implementation may fetch only the appropriate 64 bits; however, alignment to 16-byte boundary and normal segment checking will still be enforced.VEX.128 encoded version: The first source operand is a XMM register.
The second source operand can be a XMM register or a 128-bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.VEX.256 encoded version: The second source operand is anX7X6X5X4X3X2X1X0SRC1SRC2Y7Y6Y5Y4Y3Y2Y1Y0DESTY7X7Y6X6Y3X3Y2X2Figure 4-27.
 VUNPCKHPS OperationEVEX.512 encoded version: The first source operand is a ZMM register.
The second source operand is a ZMM register, a 512-bit memory location, or a 512-bit vector broadcasted from a 32-bit memory location.
The destina-tion operand is a ZMM register, conditionally updated using writemask k1.
EVEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register, a 256-bit memory location, or a 256-bit vector broadcasted from a 32-bit memory location.
The destina-tion operand is a YMM register, conditionally updated using writemask k1.
EVEX.128 encoded version: The first source operand is a XMM register.
The second source operand is a XMM register, a 128-bit memory location, or a 128-bit vector broadcasted from a 32-bit memory location.
The destina-tion operand is a XMM register, conditionally updated using writemask k1.


## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
VUNPCKHPS (EVEX Encoded Version When SRC2 is a Register)(KL, VL) = (4, 128), (8, 256), (16, 512)IF VL >= 128TMP_DEST[31:0] := SRC1[95:64]TMP_DEST[63:32] := SRC2[95:64]TMP_DEST[95:64] := SRC1[127:96]TMP_DEST[127:96] := SRC2[127:96]FI;IF VL >= 256TMP_DEST[159:128] := SRC1[223:192]TMP_DEST[191:160] := SRC2[223:192]TMP_DEST[223:192] := SRC1[255:224]TMP_DEST[255:224] := SRC2[255:224]FI;IF VL >= 512TMP_DEST[287:256] := SRC1[351:320]TMP_DEST[319:288] := SRC2[351:320]TMP_DEST[351:320] := SRC1[383:352]TMP_DEST[383:352] := SRC2[383:352]TMP_DEST[415:384] := SRC1[479:448]TMP_DEST[447:416] := SRC2[479:448]TMP_DEST[479:448] := SRC1[511:480]FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VUNPCKHPS (EVEX Encoded Version When SRC2 is Memory)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF (EVEX.b = 1)THEN TMP_SRC2[i+31:i] := SRC2[31:0]ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]FI;ENDFOR;IF VL >= 128TMP_DEST[31:0] := SRC1[95:64]TMP_DEST[63:32] := TMP_SRC2[95:64]TMP_DEST[95:64] := SRC1[127:96]TMP_DEST[127:96] := TMP_SRC2[127:96]FI;IF VL >= 256TMP_DEST[159:128] := SRC1[223:192]TMP_DEST[191:160] := TMP_SRC2[223:192]TMP_DEST[223:192] := SRC1[255:224]TMP_DEST[255:224] := TMP_SRC2[255:224]FI;IF VL >= 512TMP_DEST[287:256] := SRC1[351:320]TMP_DEST[319:288] := TMP_SRC2[351:320]TMP_DEST[351:320] := SRC1[383:352]TMP_DEST[383:352] := TMP_SRC2[383:352]TMP_DEST[415:384] := SRC1[479:448]TMP_DEST[447:416] := TMP_SRC2[479:448]TMP_DEST[479:448] := SRC1[511:480]TMP_DEST[511:480] := TMP_SRC2[511:480]FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*FI;FI;ENDFORDEST[MAXVL-1:VL] := 0VUNPCKHPS (VEX.256 Encoded Version)DEST[31:0] := SRC1[95:64]DEST[63:32] := SRC2[95:64]DEST[95:64] := SRC1[127:96]DEST[127:96] := SRC2[127:96]DEST[159:128] := SRC1[223:192]DEST[191:160] := SRC2[223:192]DEST[223:192] := SRC1[255:224]DEST[255:224] := SRC2[255:224]DEST[MAXVL-1:256] := 0VUNPCKHPS (VEX.128 Encoded Version)DEST[31:0] := SRC1[95:64]DEST[63:32] := SRC2[95:64]DEST[95:64] := SRC1[127:96]DEST[127:96] := SRC2[127:96]DEST[MAXVL-1:128] := 0UNPCKHPS (128-bit Legacy SSE Version)DEST[31:0] := SRC1[95:64]DEST[63:32] := SRC2[95:64]DEST[95:64] := SRC1[127:96]DEST[127:96] := SRC2[127:96]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVUNPCKHPS __m512 _mm512_unpackhi_ps( __m512 a, __m512 b);VUNPCKHPS __m512 _mm512_mask_unpackhi_ps(__m512 s, __mmask16 k, __m512 a, __m512 b);VUNPCKHPS __m512 _mm512_maskz_unpackhi_ps(__mmask16 k, __m512 a, __m512 b);VUNPCKHPS __m256 _mm256_unpackhi_ps (__m256 a, __m256 b);VUNPCKHPS __m256 _mm256_mask_unpackhi_ps(__m256 s, __mmask8 k, __m256 a, __m256 b);VUNPCKHPS __m256 _mm256_maskz_unpackhi_ps(__mmask8 k, __m256 a, __m256 b);UNPCKHPS __m128 _mm_unpackhi_ps (__m128 a, __m128 b);VUNPCKHPS __m128 _mm_mask_unpackhi_ps(__m128 s, __mmask8 k, __m128 a, __m128 b);VUNPCKHPS __m128 _mm_maskz_unpackhi_ps(__mmask8 k, __m128 a, __m128 b);
```
