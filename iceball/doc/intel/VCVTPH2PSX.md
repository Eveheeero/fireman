# VCVTPH2PS/VCVTPH2PSX

Convert Packed FP16 Values to Single Precision Floating-Point Values

This instruction converts packed half precision (16-bits) floating-point values in the low-order bits of the source operand (the second operand) to packed single precision floating-point values and writes the converted values into the destination operand (the first operand).If case of a denormal operand, the correct normal result is returned.
MXCSR.DAZ is ignored and is treated as if it 0.
No denormal exception is reported on MXCSR.VEX.128 version: The source operand is a XMM register or 64-bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:128) of the corresponding destination register are zeroed.VEX.256 version: The source operand is a XMM register or 128-bit memory location.
The destination operand is a YMM register.
Bits (MAXVL-1:256) of the corresponding destination register are zeroed.EVEX encoded versions: The source operand is a YMM/XMM/XMM (low 64-bits) register or a 256/128/64-bit memory location.
The destination operThe diagram below illustrates how data is converted from four packed half precision (in 64 bits) to four single preci-sion (in 128 bits) floating-point values.Note: VEX.vvvv and EVEX.vvvv are reserved (must be 1111b).VCVTPH2PS xmm1, xmm2/mem64,  imm8xmm2/mem64127                              96convertconvertconvertconvert95                                6463                                3231                                  0Figure 5-6.
 VCVTPH2PS (128-bit Version)127                              96VS0VS1VS2VS395                                6463           4847           3231           16VH0VH1VH2VH315             0xmm1The VCVTPH2PSX instruction is a new form of the PH to PS conversion instruction, encoded in map 6.
The previous version of the instruction, VCVTPH2PS, that is present in AVX512F (encoded in map 2, 0F38) does not support embedded broadcasting.
The VCVTPH2PSX instruction has the embedded broadcasting option available.The instructions associated with AVX512_FP16 always handle FP16 denormal number inputs; denormal inputs are not treated as zero.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentVCVTPH2PS __m512 _mm512_cvtph_ps( __m256i a);VCVTPH2PS __m512 _mm512_mask_cvtph_ps(__m512 s, __mmask16 k, __m256i a);VCVTPH2PS __m512 _mm512_maskz_cvtph_ps(__mmask16 k, __m256i a);VCVTPH2PS __m512 _mm512_cvt_roundph_ps( __m256i a, int sae);VCVTPH2PS __m512 _mm512_mask_cvt_roundph_ps(__m512 s, __mmask16 k, __m256i a, int sae);VCVTPH2PS __m512 _mm512_maskz_cvt_roundph_ps( __mmask16 k, __m256i a, int sae);VCVTPH2PS __m256 _mm256_mask_cvtph_ps(__m256 s, __mmask8 k, __m128i a);VCVTPH2PS __m256 _mm256_maskz_cvtph_ps(__mmask8 k, __m128i a);VCVTPH2PS __m128 _mm_mask_cvtph_ps(__m128 s, __mmask8 k, __m128i a);VCVTPH2PS __m128 _mm_maskz_cvtph_ps(__mmask8 k, __m128i a);VCVTPH2PS __m128 _mm_cvtph_ps ( __m128i m1);VCVTPH2PS __m256 _mm256_cvtph_ps ( __m128i m1)VCVTPH2PSX __m512 _mm512_cvtx_roundph_ps (__m256h a, int sae);VCVTPH2PSX __m512 _mm512_maskz_cvtx_roundph_ps (__mmask16 k, __m256h a, int sae);VCVTPH2PSX __m128 _mm_cvtxph_ps (__m128h a);VCVTPH2PSX __m128 _mm_mask_cvtxph_ps (__m128 src, __mmask8 k, __m128h a);VCVTPH2PSX __m128 _mm_maskz_cvtxph_ps (__mmask8 k, __m128h a);VCVTPH2PSX __m256 _mm256_cvtxph_ps (__m128h a);VCVTPH2PSX __m256 _mm256_mask_cvtxph_ps (__m256 src, __mmask8 k, __m128h a);VCVTPH2PSX __m256 _mm256_maskz_cvtxph_ps (__mmask8 k, __m128h a);VCVTPH2PSX __m512 _mm512_cvtxph_ps (__m256h a);VCVTPH2PSX __m512 _mm512_mask_cvtxph_ps (__m512 src, __mmask16 k, __m256h a);VCVTPH2PSX __m512 _mm512_maskz_cvtxph_ps (__mmask16 k, __m256h a);

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-26, "Type 11 
  > Class Exception Conditions" (do not report #AC).
  > EVEX-encoded instructions, see Table2-60, "Type E11 Class Exception Conditions."
  > EVEX-encoded instructions with broadcast (VCVTPH2PSX), see Table 2-46, "Type E2 Class Exception Conditions."
  > Additionally:
- SIMD Floating-Point Exceptions
  > VEX-encoded instructions: Invalid.
  > EVEX-encoded instructions: Invalid.
  > EVEX-encoded instructions with broa
  > dcast (VCVTPH2PSX): Invalid, Denormal.

## Operation

```C
vCvt_h2s(SRC1[15:0]){RETURN Cvt_Half_Precision_To_Single_Precision(SRC1[15:0]);}VCVTPH2PS (EVEX Encoded Versions) (KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32k := j * 16IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=vCvt_h2s(SRC[k+15:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VCVTPH2PS (VEX.256 Encoded Version)DEST[31:0] := vCvt_h2s(SRC1[15:0]);DEST[63:32] := vCvt_h2s(SRC1[31:16]);DEST[95:64] := vCvt_h2s(SRC1[47:32]);DEST[127:96] := vCvt_h2s(SRC1[63:48]);DEST[159:128] := vCvt_h2s(SRC1[79:64]);DEST[191:160] := vCvt_h2s(SRC1[95:80]);DEST[223:192] := vCvt_h2s(SRC1[111:96]);DEST[255:224] := vCvt_h2s(SRC1[127:112]);DEST[MAXVL-1:256] := 0VCVTPH2PS (VEX.128 Encoded Version) DEST[31:0] := vCvt_h2s(SRC1[15:0]);DEST[63:32] := vCvt_h2s(SRC1[31:16]);DEST[95:64] := vCvt_h2s(SRC1[47:32]);DEST[127:96] := vCvt_h2s(SRC1[63:48]);DEST[MAXVL-1:128] := 0VCVTPH2PSX DEST, SRC VL = 128, 256, or 512KL := VL/32FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.fp16[0]ELSEtsrc := SRC.fp16[j]DEST.fp32[j] := Convert_fp16_to_fp32(tsrc)ELSE IF *zeroing*:DEST.fp32[j] := 0// else dest.fp32[j] remains unchangedDEST[MAXVL-1:VL] := 0 
```
