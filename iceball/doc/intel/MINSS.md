# MINSS

Return Minimum Scalar Single Precision Floating-Point Value

Compares the low single precision floating-point values in the first source operand and the second source operand and returns the minimum value to the low doubleword of the destination operand.If the values being compared are both 0.0s (of either sign), the value in the second source operand is returned.
If a value in the second operand is an SNaN, that SNaN is returned unchanged to the destination (that is, a QNaN version of the SNaN is not returned).If only one value is a NaN (SNaN or QNaN) for this instruction, the second source operand, either a NaN or a valid floating-point value, is written to the result.
If instead of this behavior, it is required that the NaN in either source operand be returned, the action of MINSD can be emulated using a sequence of instructions, such as, a comparison followed by AND, ANDN, and OR.
The second source operand can be an XMM register or a 32-bit memory location.
The first source and destination operands are XMM registers.
128-bit Legacy SSE version: The destination and first source operand are the same.
Bits (MAXVL:32) of the corre-sponding destination register remain unchanged.VEX.128 and EVEX encoded version: The first source operand is an xmm register encoded by (E)VEX.vvvv.
Bits (127:32) of the XMM register destination are copied from corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: The low doubleword element of the destination operand is updated according to the writemask.Software should ensure VMINSS is encoded with VEX.L=0.
Encoding VMINSS with VEX.L=1 may encounter unpre-dictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid (Including QNaN Source Operand), Denormal.
- Other Exceptions

## Operation

```C
MIN(SRC1, SRC2){IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;ELSE IF (SRC1 < SRC2) THEN DEST := SRC1;}MINSS (EVEX Encoded Version)IF k1[0] or *no writemask*THENDEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0VMINSS (VEX.128 Encoded Version)DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0MINSS (128-bit Legacy SSE Version)DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVMINSS __m128 _mm_min_round_ss( __m128 a, __m128 b, int);VMINSS __m128 _mm_mask_min_round_ss(__m128 s, __mmask8 k, __m128 a, __m128 b, int);VMINSS __m128 _mm_maskz_min_round_ss( __mmask8 k, __m128 a, __m128 b, int);MINSS __m128 _mm_min_ss(__m128 a, __m128 b)
```
