# VREDUCEPD

Perform Reduction Transformation on Packed Float64 Values

Perform reduction transformation of the packed binary encoded double precision floating-point values in the source operand (the second operand) and store the reduced results in binary floating-point format to the destination operand (the first operand) under the writemask k1.
The reduction transformation subtracts the integer part and the leading M fractional bits from the binary floating-point source value, where M is a unsigned integer specified by imm8[7:4], see Figure5-28.
Specifically, the reduc-tion transformation can be expressed as:M-M*src))*2;dest = src - (ROUND(2Mwhere "Round()" treats "src", "2", and their product as binary floating-point numbers with normalized signifi-cand and biased exponents.pThe magnitude of the reduced result can be expressed by considering src= 2*man2,where man2' is the normalized significand and p' is the unbiased exponent p-M-1Then if RC = RNE: 0<=|Reduced Result|<=2p-MThen if RC   RNE: 0<=|Reduced Result|<2This instruction might end up with a precision exception set.
However, in case of SPE set (i.e., Suppress Precision Exception, which is imm8[3]=1), no precision exception is reported.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.76531420imm8SPERSFixed point lengthRound Control OverrideSuppress Precision Exception: Imm8[3] Imm8[1:0] = 00b : Round nearest evenRound Select: Imm8[2] Imm8[3] = 0b : Use MXCSR exception maskImm8[7:4] : Number of fixed points to subtractImm8[1:0] = 01b : Round downImm8[2] = 0b : Use Imm8[1:0]Imm8[3] = 1b : SuppressImm8[1:0] = 10b : Round upImm8[2] = 1b : Use MXCSRImm8[1:0] = 11b : TruncateHandling of special case of input values are listed in Table 5-19.Table 5-19.
VREDUCEPD/SD/PS/SS Special CasesRound ModeReturned value -M-1|Src1| < 2RNESrc1-MRPI, Src1 > 0Round (Src1-2) *RPI, Src1  " 0Src1RNI, Src1   -  0Src1-M-M|Src1| < 2RNI, Src1 < 0Round (Src1+2) *NOT RNI+0.0Src1 = ±0, orDest = ±0 RNI-0.0(Src1!=INF)Src1 = ±INFany+0.0Src1= ±NANn/aQNaN(Src1)* Round control = (imm8.MS1)? MXCSR.RC: imm8.RC

## Exceptions

- Other Exceptions
  > See Table2-46, "Type E2 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Invalid, Precision.
  > If SPE is enabled, precision ex
  > ception is not reported (regardless of MXCSR exception mask).

## Operation

```C
ReduceArgumentDP(SRC[63:0], imm8[7:0]){// Check for NaNIF (SRC [63:0] = NAN) THENRETURN (Convert SRC[63:0] to QNaN); FI;M := imm8[7:4]; // Number of fraction bits of the normalized significand to be subtractedRC := imm8[1:0];// Round Control for ROUND() operationRC source := imm[2];SPE := imm[3];// Suppress Precision Exception-MMM  *{ROUND(2*SRC[63:0], SPE, RC_source, RC)}; // ROUND() treats SRC and 2as standard binary FP valuesTMP[63:0] := 2TMP[63:0] := SRC[63:0] - TMP[63:0]; // subtraction under the same RC,SPE controlsRETURN TMP[63:0]; // binary encoded FP with biased exponent and normalized significand}VREDUCEPD (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b == 1) AND (SRC *is memory*)THEN DEST[i+63:i] := ReduceArgumentDP(SRC[63:0], imm8[7:0]);ELSE DEST[i+63:i] := ReduceArgumentDP(SRC[i+63:i], imm8[7:0]);FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] = 0FI;FI;Intel C/C++ Compiler Intrinsic EquivalentVREDUCEPD __m512d _mm512_mask_reduce_pd( __m512d a, int imm, int sae)VREDUCEPD __m512d _mm512_mask_reduce_pd(__m512d s, __mmask8 k, __m512d a, int imm, int sae)VREDUCEPD __m512d _mm512_maskz_reduce_pd(__mmask8 k, __m512d a, int imm, int sae)VREDUCEPD __m256d _mm256_mask_reduce_pd( __m256d a, int imm)VREDUCEPD __m256d _mm256_mask_reduce_pd(__m256d s, __mmask8 k, __m256d a, int imm)VREDUCEPD __m256d _mm256_maskz_reduce_pd(__mmask8 k, __m256d a, int imm)VREDUCEPD __m128d _mm_mask_reduce_pd( __m128d a, int imm)VREDUCEPD __m128d _mm_mask_reduce_pd(__m128d s, __mmask8 k, __m128d a, int imm)VREDUCEPD __m128d _mm_maskz_reduce_pd(__mmask8 k, __m128d a, int imm)
```
