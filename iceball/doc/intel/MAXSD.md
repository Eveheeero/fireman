# MAXSD

Return Maximum Scalar Double Precision Floating-Point Value

Compares the low double precision floating-point values in the first source operand and the second source operand, and returns the maximum value to the low quadword of the destination operand.
The second source operand can be an XMM register or a 64-bit memory location.
The first source and destination operands are XMM registers.
When the second source operand is a memory operand, only 64 bits are accessed.
If the values being compared are both 0.0s (of either sign), the value in the second source operand is returned.
If a value in the second source operand is an SNaN, that SNaN is returned unchanged to the destination (that is, a QNaN version of the SNaN is not returned).If only one value is a NaN (SNaN or QNaN) for this instruction, the second source operand, either a NaN or a valid floating-point value, is written to the result.
If instead of this behavior, it is required that the NaN of either source operand be returned, the action of MAXSD can be emulated using a sequence of instructions, such as, a comparison followed by AND, ANDN, and OR.
128-bit Legacy SSE version: The destination and first source operand are the same.
Bits (MAXVL-1:64) of the corresponding destination register remain unchanged.VEX.128 and EVEX encoded version: Bits (127:64) of the XMM register destination are copied from corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: The low quadword element of the destination operand is updated according to the writemask.Software should ensure VMAXSD is encoded with VEX.L=0.
Encoding VMAXSD with VEX.L=1 may encounter unpre-dictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid (Including QNaN Source Operand), Denormal.
- Other Exceptions

## Operation

```C
MAX(SRC1, SRC2){IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;ELSE IF (SRC1 > SRC2) THEN DEST := SRC1;ELSE DEST := SRC2; VMAXSD (EVEX Encoded Version)IF k1[0] or *no writemask*THENDEST[63:0] := MAX(SRC1[63:0], SRC2[63:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingDEST[63:0] := 0FI;FI;DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VMAXSD (VEX.128 Encoded Version)DEST[63:0] := MAX(SRC1[63:0], SRC2[63:0])DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0MAXSD (128-bit Legacy SSE Version)DEST[63:0] := MAX(DEST[63:0], SRC[63:0])DEST[MAXVL-1:64] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVMAXSD __m128d _mm_max_round_sd( __m128d a, __m128d b, int);VMAXSD __m128d _mm_mask_max_round_sd(__m128d s, __mmask8 k, __m128d a, __m128d b, int);VMAXSD __m128d _mm_maskz_max_round_sd( __mmask8 k, __m128d a, __m128d b, int);MAXSD __m128d _mm_max_sd(__m128d a, __m128d b)
```
