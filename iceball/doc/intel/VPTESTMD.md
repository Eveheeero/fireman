# VPTESTMB/VPTESTMW/VPTESTMD/VPTESTMQ

Logical AND and Set Mask

Performs a bitwise logical AND operation on the first source operand (the second operand) and second source operand (the third operand) and stores the result in the destination operand (the first operand) under the write-mask.
Each bit of the result is set to 1 if the bitwise AND of the corresponding elements of the first and second src operands is non-zero; otherwise it is set to 0.VPTESTMD/VPTESTMQ: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32/64-bit memory location.
The destination operand is a mask register updated under the writemask.VPTESTMB/VPTESTMW: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location.
The destination operand is a mask register updated under the writemask.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > VPTESTMD/Q: See Table2-49, "Type E4 Class Exception Conditions."

## Operation

```C
VPTESTMB (EVEX encoded versions) (KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8IF k1[j] OR *no writemask*THEN DEST[j] := (SRC1[i+7:i] BITWISE AND SRC2[i+7:i] != 0)? 1 : 0;ELSE DEST[j] = 0; zeroing-masking onlyFI;ENDFORDEST[MAX_KL-1:KL] := 0VPTESTMW (EVEX encoded versions) (KL, VL) = (8, 128), (16, 256), (32, 512)FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[j] := (SRC1[i+15:i] BITWISE AND SRC2[i+15:i] != 0)? 1 : 0;ELSE DEST[j] = 0; zeroing-masking onlyFI;ENDFORDEST[MAX_KL-1:KL] := 0VPTESTMD (EVEX encoded versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[31:0] != 0)? 1 : 0;ELSE DEST[j] := (SRC1[i+31:i] BITWISE AND SRC2[i+31:i] != 0)? 1 : 0;FI;ELSE DEST[j] := 0; zeroing-masking onlyFI;VPTESTMQ (EVEX encoded versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[63:0] != 0)? 1 : 0;ELSE DEST[j] := (SRC1[i+63:i] BITWISE AND SRC2[i+63:i] != 0)? 1 : 0;FI;ELSE DEST[j] := 0; zeroing-masking onlyFI;ENDFORDEST[MAX_KL-1:KL] := 0Intel C/C++ Compiler Intrinsic EquivalentsVPTESTMB __mmask64 _mm512_test_epi8_mask( __m512i a, __m512i b);VPTESTMB __mmask64 _mm512_mask_test_epi8_mask(__mmask64, __m512i a, __m512i b);VPTESTMW __mmask32 _mm512_test_epi16_mask( __m512i a, __m512i b);VPTESTMW __mmask32 _mm512_mask_test_epi16_mask(__mmask32, __m512i a, __m512i b);VPTESTMD __mmask16 _mm512_test_epi32_mask( __m512i a, __m512i b);VPTESTMD __mmask16 _mm512_mask_test_epi32_mask(__mmask16, __m512i a, __m512i b);VPTESTMQ __mmask8 _mm512_test_epi64_mask(__m512i a, __m512i b);VPTESTMQ __mmask8 _mm512_mask_test_epi64_mask(__mmask8, __m512i a, __m512i b);
```
