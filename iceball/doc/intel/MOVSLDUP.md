# MOVSLDUP

Replicate Single Precision Floating-Point Values

Duplicates even-indexed single precision floating-point values from the source operand (the second operand).
See Figure 4-4.
The source operand is an XMM, YMM or ZMM register or 128, 256 or 512-bit memory location and the destination operand is an XMM, YMM or ZMM register.128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding destination register remain unchanged.VEX.128 encoded version: Bits (MAXVL-1:128) of the destination register are zeroed.VEX.256 encoded version: Bits (MAXVL-1:256) of the destination register are zeroed.EVEX encoded version: The destination operand is updated at 32-bit granularity according to the writemask.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.X7X6X5X4X3X2X1X0SRCDESTX6X6X4X4X2X2X0X0

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."
  > EVEX-encoded instruction, see Exceptions Type E4NF.n
  > b in Table2-50, "Type E4NF Class Exception Conditions."

## Operation

```C
VMOVSLDUP (EVEX Encoded Versions)(KL, VL) = (4, 128), (8, 256), (16, 512)TMP_SRC[31:0] := SRC[31:0]TMP_SRC[63:32] := SRC[31:0]TMP_SRC[95:64] := SRC[95:64]TMP_SRC[127:96] := SRC[95:64]IF VL >= 256TMP_SRC[159:128] := SRC[159:128]TMP_SRC[191:160] := SRC[159:128]TMP_SRC[223:192] := SRC[223:192]TMP_SRC[255:224] := SRC[223:192]FI;IF VL >= 512TMP_SRC[287:256] := SRC[287:256]TMP_SRC[319:288] := SRC[287:256]TMP_SRC[351:320] := SRC[351:320]TMP_SRC[383:352] := SRC[351:320]TMP_SRC[415:384] := SRC[415:384]TMP_SRC[447:416] := SRC[415:384]TMP_SRC[479:448] := SRC[479:448]TMP_SRC[511:480] := SRC[479:448]FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_SRC[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0 FIFI;ENDFORDEST[MAXVL-1:VL] := 0VMOVSLDUP (VEX.256 Encoded Version)DEST[31:0] := SRC[31:0]DEST[63:32] := SRC[31:0]DEST[95:64] := SRC[95:64]DEST[127:96] := SRC[95:64]DEST[159:128] := SRC[159:128]DEST[191:160] := SRC[159:128]DEST[223:192] := SRC[223:192]DEST[255:224] := SRC[223:192]DEST[MAXVL-1:256] := 0VMOVSLDUP (VEX.128 Encoded Version)DEST[31:0] := SRC[31:0]DEST[63:32] := SRC[31:0]DEST[95:64] := SRC[95:64]MOVSLDUP (128-bit Legacy SSE Version)DEST[31:0] := SRC[31:0]DEST[63:32] := SRC[31:0]DEST[95:64] := SRC[95:64]DEST[127:96] := SRC[95:64]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVMOVSLDUP __m512 _mm512_moveldup_ps( __m512 a);VMOVSLDUP __m512 _mm512_mask_moveldup_ps(__m512 s, __mmask16 k, __m512 a);VMOVSLDUP __m512 _mm512_maskz_moveldup_ps( __mmask16 k, __m512 a);VMOVSLDUP __m256 _mm256_mask_moveldup_ps(__m256 s, __mmask8 k, __m256 a);VMOVSLDUP __m256 _mm256_maskz_moveldup_ps( __mmask8 k, __m256 a);VMOVSLDUP __m128 _mm_mask_moveldup_ps(__m128 s, __mmask8 k, __m128 a);VMOVSLDUP __m128 _mm_maskz_moveldup_ps( __mmask8 k, __m128 a);VMOVSLDUP __m256 _mm256_moveldup_ps (__m256 a);VMOVSLDUP __m128 _mm_moveldup_ps (__m128 a);
```
