# VFPCLASSPS

Tests Types of Packed Float32 Values

The FPCLASSPS instruction checks the packed single-precision floating-point values for special categories, speci-fied by the set bits in the imm8 byte.
Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against.
The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element.
The result of each element is written to the corre-sponding bit in a mask register k2 according to the writemask k1.
Bits [MAX_KL-1:16/8/4] of the destination are cleared.The classification categories specified by imm8 are shown in Figure5-13.
The classification test for each category is listed in Table 5-4.The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 32-bit memory location.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > See Table2-49, "Type E4 Class Exception Conditions."

## Operation

```C
(tsrc[31:0], imm8[7:0]){CheckFPClassSP //* Start checking the source operand for special type *//tsrcNegNum := [31];tsrcIF ([30:23]=0FFh) Then ExpAllOnes := 1; FI;tsrcIF ([30:23]=0h) Then ExpAllZeros := 1;IF (ExpAllZeros AND MXCSR.DAZ) Then MantAllZeros := 1;tsrcELSIF ([22:0]=0h) ThenMantAllZeros := 1;FI;ZeroNumber= ExpAllZeros AND MantAllZerostsrc[22];SignalingBit= sNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND NOT(SignalingBit); // sNaNqNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND SignalingBit; // qNaNNzero_res := NegNum AND ExpAllZeros AND MantAllZeros; // -0PInf_res := NOT(NegNum) AND ExpAllOnes AND MantAllZeros; // +InfNInf_res := NegNum AND ExpAllOnes AND MantAllZeros; // -InfDenorm_res := ExpAllZeros AND NOT(MantAllZeros); // denormFinNeg_res := NegNum AND NOT(ExpAllOnes) AND NOT(ZeroNumber); // -finitebResult = ( imm8[0] AND qNaN_res ) OR (imm8[1] AND Pzero_res ) OR( imm8[2] AND Nzero_res ) OR ( imm8[3] AND PInf_res ) OR( imm8[4] AND NInf_res ) OR ( imm8[5] AND Denorm_res ) OR( imm8[6] AND FinNeg_res ) OR ( imm8[7] AND sNaN_res );Return bResult;} //* end of CheckSPClassSP() *//VFPCLASSPS (EVEX encoded versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b == 1) AND (SRC *is memory*)THENDEST[j] := CheckFPClassDP(SRC1[31:0], imm8[7:0]);ELSE DEST[j] := CheckFPClassDP(SRC1[i+31:i], imm8[7:0]);FI;ELSE  DEST[j] := 0; zeroing-masking onlyFI;ENDFORDEST[MAX_KL-1:KL] := 0Intel C/C++ Compiler Intrinsic EquivalentVFPCLASSPS __mmask16 _mm512_fpclass_ps_mask( __m512 a, int c);VFPCLASSPS __mmask16 _mm512_mask_fpclass_ps_mask( __mmask16 m, __m512 a, int c)VFPCLASSPS __mmask8 _mm256_fpclass_ps_mask( __m256 a, int c)VFPCLASSPS __mmask8 _mm256_mask_fpclass_ps_mask( __mmask8 m, __m256 a, int c)VFPCLASSPS __mmask8 _mm_fpclass_ps_mask( __m128 a, int c)VFPCLASSPS __mmask8 _mm_mask_fpclass_ps_mask( __mmask8 m, __m128 a, int c)
```
