# VRNDSCALEPD

Round Packed Float64 Values to Include a Given Number of Fraction Bits

Round the double precision floating-point values in the source operand by the rounding mode specified in the immediate operand (see Figure5-29) and places the result in the destination operand.The destination operand (the first operand) is a ZMM/YMM/XMM register conditionally updated according to the writemask.
The source operand (the second operand) can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 64-bit memory location.The rounding process rounds the input to an integral value, plus number bits of fraction that are specified by imm8[7:4] (to be included in the result) and returns the result as a double precision floating-point value.It should be noticed that no overflow is induced while executing this instruction (although the source is scaled by the imm8[7:4] value).The immediate operand also specifies control fields for the rounding operation, three bit fields are defined and shown in the "Immediate Control Description" figure below.
Bit 3 of the immediate byte controls the processor behavior for a precision exception, bit 2 selects the source of rounding mode control.
Bits 1:0 specify a non-sticky rounding-mode value (immediate control table below lists the encoded values for rounding-mode field).The Precision Floating-Point Exception is signaled according to the immediate operand.
If any source operand is an SNaN then it will be converted to a QNaN.
If DAZ is set to 1 then denormals will be converted to zero before rounding.
The sign of the result of this instruction is preserved, including the sign of zero.The formula of the operation on each data element for VRNDSCALEPD isROUND(x) = 2-MM*Round_to_INT(x*2, round_ctrl), round_ctrl = imm[3:0];M=imm[7:4];MVRNDSCALEPD is a more general form of the VEX-encoded VROUNDPD instruction.
In VROUNDPD, the formula of the operation on each element isROUND(x) = Round_to_INT(x, round_ctrl), round_ctrl = imm[3:0];Note: EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.76531420imm8SPERSFixed point lengthRound Control OverrideSuppress Precision Exception: Imm8[3] Imm8[1:0] = 00b : Round nearest evenImm8[3] = 0b : Use MXCSR exception maskRound Select: Imm8[2] Imm8[7:4] : Number of fixed points to preserveImm8[1:0] = 01b : Round downImm8[3] = 1b : SuppressImm8[2] = 0b : Use Imm8[1:0]Imm8[1:0] = 10b : Round upImm8[2] = 1b : Use MXCSRImm8[1:0] = 11b : TruncateFigure 5-29.
 Imm8 Controls for VRNDSCALEPD/SD/PS/SSHandling of special case of input values are listed in Table 5-21.Table 5-21.
VRNDSCALEPD/SD/PS/SS Special CasesReturned value Src1=±inf Src1Src1=±NANSrc1 converted to QNANSrc1=±0Src1

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
  > If SPE is enabled, precision ex
  > ception is not reported (regardless of MXCSR exception mask).

## Operation

```C
RoundToIntegerDP(SRC[63:0], imm8[7:0]) {if (imm8[2] = 1)rounding_direction := MXCSR:RC; get round control from MXCSRelserounding_direction := imm8[1:0]; get round control from imm8[1:0]FIM := imm8[7:4]; get the scaling factorcase (rounding_direction)M*SRC[63:0])00: TMP[63:0] := round_to_nearest_even_integer(2M01: TMP[63:0] := round_to_equal_or_smaller_integer(2*SRC[63:0])M10: TMP[63:0] := round_to_equal_or_larger_integer(2*SRC[63:0])M11: TMP[63:0] := round_to_nearest_smallest_magnitude_integer(2*SRC[63:0])ESAC-M-MDest[63:0] := 2* TMP[63:0] ; scale down back to 2if (imm8[3] = 0) Then; check SPEif (SRC[63:0] != Dest[63:0]) Then; check precision lostset_precision(); set #PEreturn(Dest[63:0])}VRNDSCALEPD (EVEX encoded versions) (KL, VL) = (2, 128), (4, 256), (8, 512)IF *src is a memory operand*THEN TMP_SRC := BROADCAST64(SRC, VL, k1)ELSE TMP_SRC := SRCFI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := RoundToIntegerDP((TMP_SRC[i+63:i], imm8[7:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVRNDSCALEPD __m512d _mm512_roundscale_pd( __m512d a, int imm);VRNDSCALEPD __m512d _mm512_roundscale_round_pd( __m512d a, int imm, int sae);VRNDSCALEPD __m512d _mm512_mask_roundscale_pd(__m512d s, __mmask8 k, __m512d a, int imm);VRNDSCALEPD __m512d _mm512_mask_roundscale_round_pd(__m512d s, __mmask8 k, __m512d a, int imm, int sae);VRNDSCALEPD __m512d _mm512_maskz_roundscale_pd( __mmask8 k, __m512d a, int imm);VRNDSCALEPD __m512d _mm512_maskz_roundscale_round_pd( __mmask8 k, __m512d a, int imm, int sae);VRNDSCALEPD __m256d _mm256_roundscale_pd( __m256d a, int imm);VRNDSCALEPD __m256d _mm256_mask_roundscale_pd(__m256d s, __mmask8 k, __m256d a, int imm);VRNDSCALEPD __m256d _mm256_maskz_roundscale_pd( __mmask8 k, __m256d a, int imm);VRNDSCALEPD __m128d _mm_roundscale_pd( __m128d a, int imm);VRNDSCALEPD __m128d _mm_mask_roundscale_pd(__m128d s, __mmask8 k, __m128d a, int imm);VRNDSCALEPD __m128d _mm_maskz_roundscale_pd( __mmask8 k, __m128d a, int imm);
```
