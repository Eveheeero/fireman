# VGETEXPPS

Convert Exponents of Packed Single Precision Floating-Point Values to Single Precision Floating-Point Values

Extracts the biased exponents from the normalized single-precision floating-point representation of each dword element of the source operand (the second operand) as unbiased signed integer value, or convert the denormal representation of input data to unbiased negative integer values.
Each integer value of the unbiased exponent is converted to single-precision floating-point value and written to the corresponding dword elements of the destina-tion operand (the first operand) as single-precision floating-point numbers.
The destination operand is a ZMM/YMM/XMM register and updated under the writemask.
The source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 32-bit memory location.EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.Each GETEXP operation converts the exponent value into a floating-point number (permitting input value in denormal representation).
Special cases of input values are listed in Table 5-7.The formula is:GETEXP(x) = floor(log(|x|)) 2Notation floor(x) stands for maximal integer not exceeding real number x.
Software usage of VGETEXPxx and VGETMANTxx instructions generally involve a combination of GETEXP operation and GETMANT operation (see VGETMANTPD).
Thus VGETEXPxx instruction do not require software to handle SIMD floating-point exceptions.Table 5-7.
VGETEXPPS/SS Special CasesInput OperandResultCommentssrc1 = NaNQNaN(src1)If (SRC = SNaN) then #IE0 < |src1| < INFfloor(log(|src1|)) If (SRC = denormal) then #DE2Figure5-14 illustrates the VGETEXPPS functionality on input values with normalized representation.expFraction313029282726252423222120191817161514131211109876543210sSrc = 2^101000000000000000000000000000000SAR Src, 23 = 080h00000000000000000000000010000000-Bias11111111111111111111111110000001Tmp - Bias = 100000000000000000000000000000001Cvt_PI2PS(01h) = 2^000111111100000000000000000000000Figure 5-14.
 VGETEXPPS Functionality On Normal Input values

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.
- Other Exceptions
  > See Table2-46, "Type E2 Class Exception Conditions."

## Operation

```C
NormalizeExpTinySPFP(SRC[31:0]){// Jbit is the hidden integral bit of a floating-point number. In case of denormal number it has the value of ZERO.Src.Jbit := 0;Dst.exp := 1; Dst.fraction := SRC[22:0];WHILE(Src.Jbit = 0){Src.Jbit := Dst.fraction[22];// Get the fraction MSBDst.fraction := Dst.fraction << 1;// One bit shift leftDst.exp--;// Decrement the exponent}Dst.fraction := 0;// zero out fraction bitsDst.sign := 1;// Return negative signTMP[31:0] := MXCSR.DAZ? 0 : (Dst.sign << 31) OR (Dst.exp << 23) OR (Dst.fraction);Return (TMP[31:0]);}ConvertExpSPFP(SRC[31:0]){Src.sign := 0;// Zero out sign bitSrc.exp := SRC[30:23];Src.fraction := SRC[22:0];// Check for NaNIF (SRC = NaN) {IF ( SRC = SNAN ) SET IE;Return QNAN(SRC);}// Check for +INFIF (Src = +INF) RETURN (Src);// check if zero operandIF ((Src.exp = 0) AND ((Src.fraction = 0) OR (MXCSR.DAZ = 1))) Return (-INF);}IF ((Src.exp = 0) AND (Src.fraction != 0)) {TMP[31:0] := NormalizeExpTinySPFP(SRC[31:0]);// Get Normalized ExponentSet #DE}ELSE// exponent value is correct{TMP[31:0] := (Src.sign << 31) OR (Src.exp << 23) OR (Src.fraction);}TMP := SAR(TMP, 23);// Shift Arithmetic RightTMP := TMP - 127;// Subtract BiasReturn CvtI2S(TMP);// Convert INT to single precision floating-point number}}VGETEXPPS (EVEX encoded versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC *is memory*)THENDEST[i+31:i] :=ConvertExpSPFP(SRC[31:0])ELSE DEST[i+31:i] :=ConvertExpSPFP(SRC[i+31:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVGETEXPPS __m512 _mm512_getexp_ps( __m512 a);VGETEXPPS __m512 _mm512_mask_getexp_ps(__m512 s, __mmask16 k, __m512 a);VGETEXPPS __m512 _mm512_maskz_getexp_ps( __mmask16 k, __m512 a);VGETEXPPS __m512 _mm512_getexp_round_ps( __m512 a, int sae);VGETEXPPS __m512 _mm512_mask_getexp_round_ps(__m512 s, __mmask16 k, __m512 a, int sae);VGETEXPPS __m512 _mm512_maskz_getexp_round_ps( __mmask16 k, __m512 a, int sae);VGETEXPPS __m256 _mm256_getexp_ps(__m256 a);VGETEXPPS __m256 _mm256_mask_getexp_ps(__m256 s, __mmask8 k, __m256 a);VGETEXPPS __m256 _mm256_maskz_getexp_ps( __mmask8 k, __m256 a);VGETEXPPS __m128 _mm_getexp_ps(__m128 a);VGETEXPPS __m128 _mm_mask_getexp_ps(__m128 s, __mmask8 k, __m128 a);
```
