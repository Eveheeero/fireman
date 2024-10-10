# VFMSUB132SD/VFMSUB213SD/VFMSUB231SD

Fused Multiply-Subtract of Scalar Double Precision Floating-Point Values

Performs a SIMD multiply-subtract computation on the low packed double precision floating-point values using three source operands and writes the multiply-subtract result in the destination operand.
The destination operand is also the first source operand.
The second operand must be a XMM register.
The third source operand can be a XMM register or a 64-bit memory location.
VFMSUB132SD: Multiplies the low packed double precision floating-point value from the first source operand to the low packed double precision floating-point value in the third source operand.
From the infinite precision interme-diate result, subtracts the low packed double precision floating-point values in the second source operand, performs rounding and stores the resulting packed double precision floating-point value to the destination operand (first source operand).VFMSUB213SD: Multiplies the low packed double precision floating-point value from the second source operand to the low packed double precision floating-point value in the first source operand.
From the infinite precision inter-mediate result, subtracts the low packed double precision floating-point value in the third source operand, performs rounding and stores the resulting packed double precision floating-point value to the destination operand (first source operand).VFMSUB231SD: Multiplies the low packed double precision floating-point value from the second source to the low packed double precision floating-point value in the third source operand.
From the infinite precision intermediate result, subtracts the low packed double precision floating-point value in the first source operand, performs rounding and stores the resulting packed double precisioVEX.128 and EVEX encoded version: The destination operand (also first source operand) is encoded in reg_field.
The second source operand is encoded in VEX.vvvv/EVEX.vvvv.
The third source operand is encoded in rm_field.
Bits 127:64 of the destination are unchanged.
Bits MAXVL-1:128 of the destination register are zeroed.EVEX encoded version: The low quadword element of the destination is updated according to the writemask.Compiler tools may optionally support a complementary mnemonic for each instruction mnemonic listed in the opcode/instruction column of the summary table.
The behavior of the complementary mnemonic in situations involving NANs are governed by the definition of the instruction mnemonic defined in the opcode/instruction column.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal
- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions."

## Operation

```C
In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no rounding).VFMSUB132SD DEST, SRC2, SRC3 (EVEX encoded version)IF (EVEX.b = 1) and SRC3 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[63:0] := RoundFPControl(DEST[63:0]*SRC3[63:0] - SRC2[63:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[63:0] := 0FI;FI;DEST[127:64] := DEST[127:64]DEST[MAXVL-1:128] := 0VFMSUB213SD DEST, SRC2, SRC3 (EVEX encoded version)IF (EVEX.b = 1) and SRC3 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[63:0] := RoundFPControl(SRC2[63:0]*DEST[63:0] - SRC3[63:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[63:0] := 0FI;FI;VFMSUB231SD DEST, SRC2, SRC3 (EVEX encoded version)IF (EVEX.b = 1) and SRC3 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[63:0] := RoundFPControl(SRC2[63:0]*SRC3[63:0] - DEST[63:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[63:0] := 0FI;FI;DEST[127:64] := DEST[127:64]DEST[MAXVL-1:128] := 0VFMSUB132SD DEST, SRC2, SRC3 (VEX encoded version)DEST[63:0] := RoundFPControl_MXCSR(DEST[63:0]*SRC3[63:0] - SRC2[63:0])DEST[127:64] := DEST[127:64]DEST[MAXVL-1:128] := 0VFMSUB213SD DEST, SRC2, SRC3 (VEX encoded version)DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*DEST[63:0] - SRC3[63:0])DEST[127:64] := DEST[127:64]DEST[MAXVL-1:128] := 0VFMSUB231SD DEST, SRC2, SRC3 (VEX encoded version)DEST[63:0] := RoundFPControl_MXCSR(SRC2[63:0]*SRC3[63:0] - DEST[63:0])DEST[127:64] := DEST[127:64]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVFMSUBxxxSD __m128d _mm_fmsub_round_sd(__m128d a, __m128d b, __m128d c, int r);VFMSUBxxxSD __m128d _mm_mask_fmsub_sd(__m128d a, __mmask8 k, __m128d b, __m128d c);VFMSUBxxxSD __m128d _mm_maskz_fmsub_sd(__mmask8 k, __m128d a, __m128d b, __m128d c);VFMSUBxxxSD __m128d _mm_mask3_fmsub_sd(__m128d a, __m128d b, __m128d c, __mmask8 k);VFMSUBxxxSD __m128d _mm_mask_fmsub_round_sd(__m128d a, __mmask8 k, __m128d b, __m128d c, int r);VFMSUBxxxSD __m128d _mm_maskz_fmsub_round_sd(__mmask8 k, __m128d a, __m128d b, __m128d c, int r);VFMSUBxxxSD __m128d _mm_mask3_fmsub_round_sd(__m128d a, __m128d b, __m128d c, __mmask8 k, int r);VFMSUBxxxSD __m128d _mm_fmsub_sd (__m128d a, __m128d b, __m128d c);
```
