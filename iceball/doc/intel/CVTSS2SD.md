# CVTSS2SD

Convert Scalar Single Precision Floating-Point Value to Scalar Double Precision Floating-Point Value

Converts a single precision floating-point value in the "convert-from" source operand to a double precision floating-point value in the destination operand.
When the "convert-from" source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
The result is stored in the low quadword of the destination operand.128-bit Legacy SSE version: The "convert-from" source operand (the second operand) is an XMM register or memory location.
Bits (MAXVL-1:64) of the corresponding destination register remain unchanged.
The destination operand is an XMM register.
VEX.128 and EVEX encoded versions: The "convert-from" source operand (the third operand) can be an XMM register or a 32-bit memory location.
The first source and destination operands are XMM registers.
Bits (127:64) of the XMM register destination are copied from the corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.Software should ensure VCVTSS2SD is encoded with VEX.L=0.
Encoding VCVTSS2SD with VEX.L=1 may encounter unpredictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.
- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions."

## Operation

```C
VCVTSS2SD (EVEX Encoded Version)IF k1[0] or *no writemask*THENDEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC2[31:0]);ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[63:0] = 0FI;FI;VCVTSS2SD (VEX.128 Encoded Version)DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC2[31:0])DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0CVTSS2SD (128-bit Legacy SSE Version)DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0]);DEST[MAXVL-1:64] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVCVTSS2SD __m128d _mm_cvt_roundss_sd(__m128d a, __m128 b, int r);VCVTSS2SD __m128d _mm_mask_cvt_roundss_sd(__m128d s, __mmask8 m, __m128d a,__m128 b, int r);VCVTSS2SD __m128d _mm_maskz_cvt_roundss_sd(__mmask8 k, __m128d a, __m128 a, int r);VCVTSS2SD __m128d _mm_mask_cvtss_sd(__m128d s, __mmask8 m, __m128d a,__m128 b);VCVTSS2SD __m128d _mm_maskz_cvtss_sd(__mmask8 m, __m128d a,__m128 b);CVTSS2SD __m128d_mm_cvtss_sd(__m128d a, __m128 a);
```
