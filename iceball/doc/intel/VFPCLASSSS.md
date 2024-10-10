# VFPCLASSSS

Tests Type of a Scalar Float32 Value

The FPCLASSSS instruction checks the low single-precision floating-point value in the source operand for special categories, specified by the set bits in the imm8 byte.
Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against.
The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element.
The result is written to the low bit in a mask register k2 according to the writemask k1.
Bits MAX_KL-1: 1 of the destination are cleared.The classification categories specified by imm8 are shown in Figure5-13.
The classification test for each category is listed in Table 5-4.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > See Table2-53, "Type E6 Class Exception Conditions."

## Operation

```C
CheckFPClassSP (tsrc[31:0], imm8[7:0]){//* Start checking the source operand for special type *//tsrc[31];NegNum := tsrcIF ([30:23]=0FFh) Then ExpAllOnes := 1; FI;tsrcIF ([30:23]=0h) Then ExpAllZeros := 1;IF (ExpAllZeros AND MXCSR.DAZ) Then MantAllZeros := 1;tsrcELSIF ([22:0]=0h) ThenMantAllZeros := 1;FI;ZeroNumber= ExpAllZeros AND MantAllZerostsrc[22];SignalingBit= sNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND NOT(SignalingBit); // sNaNqNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND SignalingBit; // qNaNPzero_res := NOT(NegNum) AND ExpAllZeros AND MantAllZeros; // +0Nzero_res := NegNum AND ExpAllZeros AND MantAllZeros; // -0PInf_res := NOT(NegNum) AND ExpAllOnes AND MantAllZeros; // +InfNInf_res := NegNum AND ExpAllOnes AND MantAllZeros; // -InfDenorm_res := ExpAllZeros AND NOT(MantAllZeros); // denormFinNeg_res := NegNum AND NOT(ExpAllOnes) AND NOT(ZeroNumber); // -finitebResult = ( imm8[0] AND qNaN_res ) OR (imm8[1] AND Pzero_res ) OR( imm8[2] AND Nzero_res ) OR ( imm8[3] AND PInf_res ) OR( imm8[4] AND NInf_res ) OR ( imm8[5] AND Denorm_res ) OR( imm8[6] AND FinNeg_res ) OR} //* end of CheckSPClassSP() *//VFPCLASSSS (EVEX encoded version)IF k1[0] OR *no writemask*THEN DEST[0] := CheckFPClassSP(SRC1[31:0], imm8[7:0])ELSE  DEST[0] := 0; zeroing-masking onlyFI;DEST[MAX_KL-1:1] := 0Intel C/C++ Compiler Intrinsic EquivalentVFPCLASSSS __mmask8 _mm_fpclass_ss_mask( __m128 a, int c)VFPCLASSSS __mmask8 _mm_mask_fpclass_ss_mask( __mmask8 m, __m128 a, int c)
```
