# MOVDDUP

Replicate Double Precision Floating-Point Values

InstructionMode Feature SupportFlagF2 0F 12 /rAV/VSSE3Move double precision floating-point value from MOVDDUP xmm1, xmm2/m64xmm2/m64 and duplicate into xmm1.VEX.128.F2.0F.WIG 12 /rAV/VAVXMove double precision floating-point value from VMOVDDUP xmm1, xmm2/m64xmm2/m64 and duplicate into xmm1.VEX.256.F2.0F.WIG 12 /rAV/VAVXMove even index double precision floating-point values VMOVDDUP ymm1, ymm2/m256from ymm2/mem and duplicate each element into ymm1.BV/VAVX512VLMove double precision floating-point value from EVEX.128.F2.0F.W1 12 /rAVX512Fxmm2/m64 and duplicate each element into xmm1 VMOVDDUP xmm1 {k1}{z}, subject to writemask k1.xmm2/m64BV/VAVX512VLMove even index double precision floating-point values EVEX.256.F2.0F.W1 12 /rAVX512Ffrom ymm2/m256 and duplicate each element into VMOVDDUP ymm1 {k1}{z}, ymm1 subject to writemask k1.ymm2/m256EVEX.512.F2.0F.W1 12 /rBV/VAVX512FMove even index double precision floating-point values VMOVDDUP zmm1 {k1}{z}, from zmm2/m512 and duplicate each element into zmm2/m512zmm1 subject to writemask k1.Instruction Operand EncodingOp/EnTuple TypeOperand 1Operand 2Operand 3Operand 4AN/AModRM:reg (w)ModRM:r/m (r)N/AN/ABMOVDDUPModRM:reg (w)ModRM:r/m (r)N/AN/AFor 256-bit or higher versions: Duplicates even-indexed double precision floating-point values from the source operand (the second operand) and into adjacent pair and store to the destination operand (the first operand).For 128-bit versions: Duplicates the low double precision floating-point value from the source operand (the second operand) and store to the destination operand (the first operand).128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding destination register are unchanged.
The source operand is XMM register or a 64-bit memory location.VEX.128 and EVEX.128 encoded version: Bits (MAXVL-1:128) of the destination register are zeroed.
The source operand is XMM register or a 64-bit memory location.
The destination is updated conditionally under the writemask for EVEX version.VEX.256 and EVEX.256 encoded version: Bits (MAXVL-1:256) of the destination register are zeroed.
The source operand is YMM register or a 256-bit memory location.
The destination is updated conditionally under the write-mask for EVEX version.EVEX.512 encoded version: The destination is updated according to the writemask.
The source operand is ZMM X3X2X1X0SRCDESTX2X2X0X0Figure 4-2.
 VMOVDDUP Operation

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-22, "Type 5 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-52, "Type E5NF Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VMOVDDUP (EVEX Encoded Versions) (KL, VL) = (2, 128), (4, 256), (8, 512)TMP_SRC[63:0] := SRC[63:0] TMP_SRC[127:64] := SRC[63:0]IF VL >= 256TMP_SRC[191:128] := SRC[191:128]TMP_SRC[255:192] := SRC[191:128]FI;IF VL >= 512TMP_SRC[319:256] := SRC[319:256]TMP_SRC[383:320] := SRC[319:256]TMP_SRC[477:384] := SRC[477:384]TMP_SRC[511:484] := SRC[477:384]FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := TMP_SRC[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0 ; zeroing-maskingFIFI;ENDFORDEST[MAXVL-1:VL] := 0VMOVDDUP (VEX.256 Encoded Version)DEST[63:0] := SRC[63:0]DEST[127:64] := SRC[63:0]DEST[191:128] := SRC[191:128]DEST[255:192] := SRC[191:128]DEST[MAXVL-1:256] := 0VMOVDDUP (VEX.128 Encoded Version)DEST[63:0] := SRC[63:0]MOVDDUP (128-bit Legacy SSE Version)DEST[63:0] := SRC[63:0]DEST[127:64] := SRC[63:0]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVMOVDDUP __m512d _mm512_movedup_pd( __m512d a);VMOVDDUP __m512d _mm512_mask_movedup_pd(__m512d s, __mmask8 k, __m512d a);VMOVDDUP __m512d _mm512_maskz_movedup_pd( __mmask8 k, __m512d a);VMOVDDUP __m256d _mm256_mask_movedup_pd(__m256d s, __mmask8 k, __m256d a);VMOVDDUP __m256d _mm256_maskz_movedup_pd( __mmask8 k, __m256d a);VMOVDDUP __m128d _mm_mask_movedup_pd(__m128d s, __mmask8 k, __m128d a);VMOVDDUP __m128d _mm_maskz_movedup_pd( __mmask8 k, __m128d a);MOVDDUP __m256d _mm256_movedup_pd (__m256d a);MOVDDUP __m128d _mm_movedup_pd (__m128d a);
```
