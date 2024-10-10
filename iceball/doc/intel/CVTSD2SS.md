# CVTSD2SS

Convert Scalar Double Precision Floating-Point Value to Scalar Single Precision Floating-Point Value

Converts a double precision floating-point value in the "convert-from" source operand (the second operand in SSE2 version, otherwise the third operand) to a single precision floating-point value in the destination operand.When the "convert-from" operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
The result is stored in the low doubleword of the destination operand.
When the conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.128-bit Legacy SSE version: The "convert-from" source operand (the second operand) is an XMM register or memory location.
Bits (MAXVL-1:32) of the corresponding destination register remain unchanged.
The destination operand is an XMM register.
VEX.128 and EVEX encoded versions: The "convert-from" source operand (the third operand) can be an XMM register or a 64-bit memory location.
The first source and destination operands are XMM registers.
Bits (127:32) of the XMM register destination are copied from the corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: the converted result in written to the low doubleword element of the destination under the writemask.Software should ensure VCVTSD2SS is encoded with VE

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.

## Operation

```C
VCVTSD2SS (EVEX Encoded Version)IF (SRC2 *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC2[63:0]);ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0VCVTSD2SS (VEX.128 Encoded Version)DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC2[63:0]);DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0CVTSD2SS (128-bit Legacy SSE Version)DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[63:0]);(* DEST[MAXVL-1:32] Unmodified *)Intel C/C++ Compiler Intrinsic EquivalentVCVTSD2SS __m128 _mm_mask_cvtsd_ss(__m128 s, __mmask8 k, __m128 a, __m128d b);VCVTSD2SS __m128 _mm_maskz_cvtsd_ss( __mmask8 k, __m128 a,__m128d b);VCVTSD2SS __m128 _mm_cvt_roundsd_ss(__m128 a, __m128d b, int r);VCVTSD2SS __m128 _mm_mask_cvt_roundsd_ss(__m128 s, __mmask8 k, __m128 a, __m128d b, int r);VCVTSD2SS __m128 _mm_maskz_cvt_roundsd_ss( __mmask8 k, __m128 a,__m128d b, int r);CVTSD2SS __m128_mm_cvtsd_ss(__m128 a, __m128d b)
```
