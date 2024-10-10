# VFPCLASSPD

Tests Types of Packed Float64 Values

The FPCLASSPD instruction checks the packed double precision floating-point values for special categories, speci-fied by the set bits in the imm8 byte.
Each set bit in imm8 specifies a category of floating-point values that the input data element is classified against.
The classified results of all specified categories of an input value are ORed together to form the final boolean result for the input element.
The result of each element is written to the corre-sponding bit in a mask register k2 according to the writemask k1.
Bits [MAX_KL-1:8/4/2] of the destination are cleared.The classification categories specified by imm8 are shown in Figure5-13.
The classification test for each category is listed in Table 5-4.76531420SNaNNeg.
INFNeg.
FiniteDenormal+INFNeg.
0QNaN+0Figure 5-13.
 Imm8 Byte Specifier of Special Case Floating-Point Values for VFPCLASSPD/SD/PS/SSTable 5-4.
Classifier Operations for VFPCLASSPD/SD/PS/SSBitsImm8[0]Imm8[1]Imm8[2]Imm8[3]Imm8[4]Imm8[5]Imm8[6]Imm8[7]CategoryQNANPosZeroNegZeroPosINFNegINFDenormalNegativeSNANClassifierChecks for Checks for Checks for -Checks for Checks for -Checks for Checks for Checks for QNaN+00+INFINFDenormalNegative finiteSNaNThe source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector 

## Exceptions

- Other Exceptions
  > See Table2-49, "Type E4 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
(tsrc[63:0], imm8[7:0]){CheckFPClassDP //* Start checking the source operand for special type *//tsrcNegNum := [63];tsrcIF ([62:52]=07FFh) Then ExpAllOnes := 1; FI;tsrcIF ([62:52]=0h) Then ExpAllZeros := 1;IF (ExpAllZeros AND MXCSR.DAZ) Then MantAllZeros := 1;tsrcELSIF ([51:0]=0h) ThenMantAllZeros := 1;FI;ZeroNumber := ExpAllZeros AND MantAllZerostsrc[51];SignalingBit := sNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND NOT(SignalingBit); // sNaNqNaN_res := ExpAllOnes AND NOT(MantAllZeros) AND SignalingBit; // qNaNPzero_res := NOT(NegNum) AND ExpAllZeros AND MantAllZeros; // +0Nzero_res := NegNum AND ExpAllZeros AND MantAllZeros; // -0PInf_res := NOT(NegNum) AND ExpAllOnes AND MantAllZeros; // +InfNInf_res := NegNum AND ExpAllOnes AND MantAllZeros; // -InfDenorm_res := ExpAllZeros AND NOT(MantAllZeros); // denormFinNeg_res := NegNum AND NOT(ExpAllOnes) AND NOT(ZeroNumber); // -finitebResult = ( imm8[0] AND qNaN_res ) OR (imm8[1] AND Pzero_res ) OR( imm8[2] AND Nzero_res ) OR ( imm8[3] AND PInf_res ) OR( imm8[4] AND NInf_res ) OR ( imm8[5] AND Denorm_res ) OR( imm8[6] AND FinNeg_res ) OR ( imm8[7] AND sNaN_res );Return bResult;} //* end of CheckFPClassDP() *//VFPCLASSPD (EVEX Encoded versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b == 1) AND (SRC *is memory*)THENDEST[j] := CheckFPClassDP(SRC1[63:0], imm8[7:0]);ELSE DEST[j] := CheckFPClassDP(SRC1[i+63:i], imm8[7:0]);FI;ELSE DEST[j] := 0; zeroing-masking onlyFI;Intel C/C++ Compiler Intrinsic EquivalentVFPCLASSPD __mmask8 _mm512_fpclass_pd_mask( __m512d a, int c);VFPCLASSPD __mmask8 _mm512_mask_fpclass_pd_mask( __mmask8 m, __m512d a, int c)VFPCLASSPD __mmask8 _mm256_fpclass_pd_mask( __m256d a, int c)VFPCLASSPD __mmask8 _mm256_mask_fpclass_pd_mask( __mmask8 m, __m256d a, int c)VFPCLASSPD __mmask8 _mm_fpclass_pd_mask( __m128d a, int c)VFPCLASSPD __mmask8 _mm_mask_fpclass_pd_mask( __mmask8 m, __m128d a, int c)
```
