# VCVTPS2PH

Convert Single-Precision FP Value to 16-bit FP Value

Convert packed single-precision floating values in the source operand to half-precision (16-bit) floating-point values and store to the destination operand.
The rounding mode is specified using the immediate field (imm8).Underflow results (i.e., tiny results) are converted to denormals.
MXCSR.FTZ is ignored.
If a source element is denormal relative to the input format with DM masked and at least one of PM or UM unmasked; a SIMD exception will be raised with DE, UE and PE set.VCVTPS2PH xmm1/mem64, xmm2,  imm8convertconvertconvertconvert127                              9695                                6463                                3231                                  0Figure 5-7.
 VCVTPS2PH (128-bit Version)127                              96VS0VS1VS2VS395                                6463           4847           3231           16VH0VH1VH2VH315             0xmm2The immediate byte defines several bit fields that control rounding operation.
The effect and encoding of the RC Table 5-3.
Immediate Byte Encoding for 16-bit Floating-Point Conversion InstructionsBitsField Name/valueDescriptionCommentImm[1:0]RC=00BRound to nearest evenIf Imm[2] = 0RC=01BRound downRC=10BRound upRC=11BTruncateImm[2]MS1=0Use imm[1:0] for roundingIgnore MXCSR.RC MS1=1Use MXCSR.RC for roundingImm[7:3]IgnoredIgnored by processorVEX.128 version: The source operand is a XMM register.
The destination operand is a XMM register or 64-bit memory location.
If the destination operand is a register then the upper bits (MAXVL-1:64) of corresponding register are zeroed.VEX.256 version: The source operand is a YMM register.
The destination operand is a XMM register or 128-bit memory location.
If the destination operand is a register, the upper bits (MAXVL-1:128) of the corresponding desti-nation register are zeroed.Note: VEX.vvvv and EVEX.vvvv are reserved (must be 1111b).EVEX encoded versions: The source operand is a ZMM/YMM/XMM register.
The destination operand is a YMM/XMM/XMM (low 64-bits) register or a 256/128/64-bit memory location, conditionally updated with writemask k1.
Bits (MAXVL-1:256/128/64) of the corresponding destination register are zeroed.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentVCVTPS2PH __m256i _mm512_cvtps_ph(__m512 a);VCVTPS2PH __m256i _mm512_mask_cvtps_ph(__m256i s, __mmask16 k,__m512 a);VCVTPS2PH __m256i _mm512_maskz_cvtps_ph(__mmask16 k,__m512 a);VCVTPS2PH __m256i _mm512_cvt_roundps_ph(__m512 a, const int imm);VCVTPS2PH __m256i _mm512_mask_cvt_roundps_ph(__m256i s, __mmask16 k,__m512 a, const int imm);VCVTPS2PH __m256i _mm512_maskz_cvt_roundps_ph(__mmask16 k,__m512 a, const int imm);VCVTPS2PH __m128i _mm256_mask_cvtps_ph(__m128i s, __mmask8 k,__m256 a);VCVTPS2PH __m128i _mm256_maskz_cvtps_ph(__mmask8 k,__m256 a);VCVTPS2PH __m128i _mm_mask_cvtps_ph(__m128i s, __mmask8 k,__m128 a);VCVTPS2PH __m128i _mm_maskz_cvtps_ph(__mmask8 k,__m128 a);VCVTPS2PH __m128i _mm_cvtps_ph ( __m128 m1, const int imm);VCVTPS2PH __m128i _mm256_cvtps_ph(__m256 m1, const int imm);

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-26, "Type 11 
  > Class Exception Conditions" (do not report #AC); 
  > EVEX-encoded instructions, see Table2-60, "Type E11 Class Exception Conditions."
  > Additionally:

## Operation

```C
vCvt_s2h(SRC1[31:0]){IF Imm[2] = 0THEN ; using Imm[1:0] for rounding control, see Table 5-3RETURN Cvt_Single_Precision_To_Half_Precision_FP_Imm(SRC1[31:0]);ELSE ; using MXCSR.RC for rounding controlRETURN Cvt_Single_Precision_To_Half_Precision_FP_Mxcsr(SRC1[31:0]);FI;}VCVTPS2PH (EVEX Encoded Versions) When DEST is a Register(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 16k := j * 32IF k1[j] OR *no writemask*THEN DEST[i+15:i] :=vCvt_s2h(SRC[k+31:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+15:i] := 0FIFI;VCVTPS2PH (EVEX Encoded Versions) When DEST is Memory(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 16k := j * 32IF k1[j] OR *no writemask*THEN DEST[i+15:i] :=vCvt_s2h(SRC[k+31:k])ELSE *DEST[i+15:i] remains unchanged*; merging-maskingFI;ENDFORVCVTPS2PH (VEX.256 Encoded Version)DEST[15:0] := vCvt_s2h(SRC1[31:0]);DEST[31:16] := vCvt_s2h(SRC1[63:32]);DEST[47:32] := vCvt_s2h(SRC1[95:64]);DEST[63:48] := vCvt_s2h(SRC1[127:96]);DEST[79:64] := vCvt_s2h(SRC1[159:128]);DEST[95:80] := vCvt_s2h(SRC1[191:160]);DEST[111:96] := vCvt_s2h(SRC1[223:192]);DEST[127:112] := vCvt_s2h(SRC1[255:224]);DEST[MAXVL-1:128] := 0VCVTPS2PH (VEX.128 Encoded Version) DEST[15:0] := vCvt_s2h(SRC1[31:0]);DEST[31:16] := vCvt_s2h(SRC1[63:32]);DEST[47:32] := vCvt_s2h(SRC1[95:64]);DEST[63:48] := vCvt_s2h(SRC1[127:96]);DEST[MAXVL-1:64] := 0
```
