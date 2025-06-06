# VFIXUPIMMSS

Fix Up Special Scalar Float32 Value

Perform a fix-up of the low doubleword element encoded in single precision floating-point format in the first source operand (the second operand) using a 32-bit, two-level look-up table specified in the low doubleword element of the second source operand (the third operand) with exception reporting specifier imm8.
The element that is fixed-up is selected by mask bit of 1 specified in the opmask k1.
Mask bit of 0 in the opmask k1 or table response action of 0000b preserves the corresponding element of the first operand.
The fixed-up element from the first source operand or the preserved element in the first operand becomes the low doubleword element of the destination operand (the first operand) Bits 127:32 of the destination operand is copied from the corresponding bits of the first source operand.
The destination and first source operands are XMM registers.
The second source operand can be a XMM register or a 32-bit memory location.The two-level look-up table perform a fix-up of each single precision floating-point input data in the first source operand by decoding the input data encoding into 8 token types.
A response table is defined for each token type that converts the input encoding in the first source operand with one of 16 response actions.
This instruction is specifically intended for use in fixing up the results of arithmetic calculations involving one source so that they match the spec, although it is generally useful for fixing up the results of multiple-instruction sequences to reflect special-number inputs.
For example, consider rcp(0).
Input 0 to rcp, and you should get INF according to the DX10 spec.
However, evaluating rcp via Newton-Raphson, where x=approx(1/0), yields an incor-rect result.
To deal with this, VFIXUPIMMPD can be used after the N-R reciprocal sequence to set the result to the correct value (i.e., INF when the input is 0).If MXCSR.DAZ is not set, denormal input elements in the first source operand are considered as normal inputs and do not trigger any fixup nor fault reporting.Imm8 is used to set the required flags reporting.
It supports #ZE and #IE fault reporting (see details below).MXCSR.DAZ is used and refer to zmm2 only (i.e., zmm1 is not considered as zero in case MXCSR.DAZ is set).MXCSR mask bits are ignored and are treated as if all mask bits are set to masked response).
If any of the imm8 bits is set and the condition met for fault reporting, MXCSR.IE or MXCSR.ZE might be updated.

## Exceptions

- SIMD Floating-Point Exceptions
  > Zero, Invalid

## Operation

```C
enum TOKEN_TYPE{QNAN_TOKEN := 0,SNAN_TOKEN := 1,ZERO_VALUE_TOKEN := 2,POS_ONE_VALUE_TOKEN := 3,NEG_INF_TOKEN := 4,POS_INF_TOKEN := 5,NEG_VALUE_TOKEN := 6,FIXUPIMM (dest[31:0], ){_SPsrc1[31:0],tbl3[31:0], imm8 [7:0]tsrc[31:0] := ((src1[30:23] = 0) AND (MXCSR.DAZ =1)) ? 0.0 : src1[31:0]CASE(tsrc[63:0] of TOKEN_TYPE) {QNAN_TOKEN: j := 0;SNAN_TOKEN: j := 1;ZERO_VALUE_TOKEN: j := 2;POS_ONE_VALUE_TOKEN: j := 3;NEG_INF_TOKEN: j := 4;POS_INF_TOKEN: j := 5;NEG_VALUE_TOKEN: j := 6;POS_VALUE_TOKEN: j := 7;}; end source special CASE(tsrc); The required response from src3 table is extracted token_response[3:0] = tbl3[3+4*j:4*j];CASE(token_response[3:0]) {0000: dest[31:0] := dest[31:0];  ; preserve content of DEST0001: dest[31:0] := tsrc[31:0];   ; pass through src1 normal input value, denormal as zero0010: dest[31:0] := QNaN(tsrc[31:0]);0011: dest[31:0] := QNAN_Indefinite;0100: dest[31:0] := -INF;0101: dest[31:0] := +INF;0110: dest[31:0] := tsrc.sign? -INF : +INF;0111: dest[31:0] := -0;1000: dest[31:0] := +0;1001: dest[31:0] := -1;1010: dest[31:0] := +1;1011: dest[31:0] := ½;1100: dest[31:0] := 90.0;1101: dest[31:0] := PI/2;1110: dest[31:0] := MAX_FLOAT;1111: dest[31:0] := -MAX_FLOAT;} ; end of token_response CASE ; The required fault reporting from imm8 is extracted ; TOKENs are mutually exclusive and TOKENs priority defines the order. ¬ .; Multiple faults related to a single token can occur simultaneouslyIF (tsrc[31:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[0] then set #ZE;IF (tsrc[31:0] of TOKEN_TYPE: ZERO_VALUE_TOKEN) AND imm8[1] then set #IE;IF (tsrc[31:0] of TOKEN_TYPE: ONE_VALUE_TOKEN) AND imm8[2] then set #ZE;IF (tsrc[31:0] of TOKEN_TYPE: ONE_VALUE_TOKEN) AND imm8[3] then set #IE;IF (tsrc[31:0] of TOKEN_TYPE: SNAN_TOKEN) AND imm8[4] then set #IE;IF (tsrc[31:0] of TOKEN_TYPE: NEG_INF_TOKEN) AND imm8[5] then set #IE;IF (tsrc[31:0] of TOKEN_TYPE: NEG_VALUE_TOKEN) AND imm8[6] then set #IE;IF (tsrc[31:0] of TOKEN_TYPE: POS_INF_TOKEN) AND imm8[7] then set #IE;; end fault reporting VFIXUPIMMSS (EVEX encoded version)IF k1[0] OR *no writemask*THEN DEST[31:0] := FIXUPIMM_SP(DEST[31:0], SRC1[31:0], SRC2[31:0], imm8 [7:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE  DEST[31:0] := 0; zeroing-maskingFIFI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0Immediate Control Description: Æ76543210+INF  Æ#IE-INF  Æ#IESNaN  Æ#IE-VE   Æ#IEONE  #IEONE   Æ#ZEZERO  Æ#IEZERO  Æ#ZEFigure 5-12.  VFIXUPIMMSS Immediate Control DescriptionIntel C/C++ Compiler Intrinsic EquivalentVFIXUPIMMSS __m128 _mm_fixupimm_ss( __m128 a, __m128i tbl, int imm);VFIXUPIMMSS __m128 _mm_mask_fixupimm_ss(__m128 s, __mmask8 k, __m128 a, __m128i tbl, int imm);VFIXUPIMMSS __m128 _mm_maskz_fixupimm_ss( __mmask8 k, __m128 a, __m128i tbl, int imm);VFIXUPIMMSS __m128 _mm_fixupimm_round_ss( __m128 a, __m128i tbl, int imm, int sae);VFIXUPIMMSS __m128 _mm_mask_fixupimm_round_ss(__m128 s, __mmask8 k, __m128 a, __m128i tbl, int imm, int sae);VFIXUPIMMSS __m128 _mm_maskz_fixupimm_round_ss( __mmask8 k, __m128 a, __m128i tbl, int imm, int sae);
```
