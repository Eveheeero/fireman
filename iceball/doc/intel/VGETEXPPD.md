# VGETEXPPD

Convert Exponents of Packed Double Precision Floating-Point Values to Double Precision Floating-Point Values

Extracts the biased exponents from the normalized double precision floating-point representation of each qword data element of the source operand (the second operand) as unbiased signed integer value, or convert the denormal representation of input data to unbiased negative integer values.
Each integer value of the unbiased exponent is converted to double precision floating-point value and written to the corresponding qword elements of the destination operand (the first operand) as double precision floating-point numbers.
The destination operand is a ZMM/YMM/XMM register and updated under the writemask.
The source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 64-bit memory location.EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.Each GETEXP operation converts the exponent value into a floating-point number (permitting input value in denormal representation).
Special cases of input values are listed in Table 5-5.The formula is:GETEXP(x) = floor(log(|x|)) 2Notation floor(x) stands for the greatest integer not exceeding real number x.
Table 5-5.
VGETEXPPD/SD Special CasesInput OperandResultCommentssrc1 = NaNQNaN(src1)If (SRC = SNaN) then #IE0 < |src1| < INFfloor(log(|src1|)) 2If (SRC = denormal) then #DE

## Exceptions

- Other Exceptions
  > See Table2-46, "Type E2 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Invalid, Denormal.

## Operation

```C
NormalizeExpTinyDPFP(SRC[63:0]){// Jbit is the hidden integral bit of a floating-point number. In case of denormal number it has the value of ZERO.Src.Jbit := 0;Dst.exp := 1; Dst.fraction := SRC[51:0];WHILE(Src.Jbit = 0){Src.Jbit := Dst.fraction[51];// Get the fraction MSBDst.fraction := Dst.fraction << 1;// One bit shift leftDst.exp--;// Decrement the exponent}Dst.fraction := 0;// zero out fraction bitsDst.sign := 1;// Return negative signTMP[63:0] := MXCSR.DAZ? 0 : (Dst.sign << 63) OR (Dst.exp << 52) OR (Dst.fraction);Return (TMP[63:0]);}ConvertExpDPFP(SRC[63:0]){Src.sign := 0;// Zero out sign bitSrc.exp := SRC[62:52];Src.fraction := SRC[51:0];// Check for NaNIF (SRC = NaN) {IF ( SRC = SNAN ) SET IE;Return QNAN(SRC);}// Check for +INFIF (Src = +INF) RETURN (Src);// check if zero operandIF ((Src.exp = 0) AND ((Src.fraction = 0) OR (MXCSR.DAZ = 1))) Return (-INF);}ELSE // check if denormal operand (notice that MXCSR.DAZ = 0){IF ((Src.exp = 0) AND (Src.fraction != 0)) {TMP[63:0] := NormalizeExpTinyDPFP(SRC[63:0]);// Get Normalized ExponentSet #DE}ELSE// exponent value is correct{TMP[63:0] := (Src.sign << 63) OR (Src.exp << 52) OR (Src.fraction);}TMP := SAR(TMP, 52);// Shift Arithmetic RightTMP := TMP - 1023;// Subtract BiasReturn CvtI2D(TMP);// Convert INT to double precision floating-point numberVGETEXPPD (EVEX encoded versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC *is memory*)THENDEST[i+63:i] :=ConvertExpDPFP(SRC[63:0])ELSE DEST[i+63:i] :=ConvertExpDPFP(SRC[i+63:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVGETEXPPD __m512d _mm512_getexp_pd(__m512d a);VGETEXPPD __m512d _mm512_mask_getexp_pd(__m512d s, __mmask8 k, __m512d a);VGETEXPPD __m512d _mm512_maskz_getexp_pd( __mmask8 k, __m512d a);VGETEXPPD __m512d _mm512_getexp_round_pd(__m512d a, int sae);VGETEXPPD __m512d _mm512_mask_getexp_round_pd(__m512d s, __mmask8 k, __m512d a, int sae);VGETEXPPD __m512d _mm512_maskz_getexp_round_pd( __mmask8 k, __m512d a, int sae);VGETEXPPD __m256d _mm256_getexp_pd(__m256d a);VGETEXPPD __m256d _mm256_mask_getexp_pd(__m256d s, __mmask8 k, __m256d a);VGETEXPPD __m256d _mm256_maskz_getexp_pd( __mmask8 k, __m256d a);VGETEXPPD __m128d _mm_getexp_pd(__m128d a);VGETEXPPD __m128d _mm_mask_getexp_pd(__m128d s, __mmask8 k, __m128d a);VGETEXPPD __m128d _mm_maskz_getexp_pd( __mmask8 k, __m128d a);
```
