# VFMSUB132SS/VFMSUB213SS/VFMSUB231SS

Fused Multiply-Subtract of Scalar Single Precision Floating-Point Values

Performs a SIMD multiply-subtract computation on the low packed single precision floating-point values using three source operands and writes the multiply-subtract result in the destination operand.
The destination operand is also the first source operand.
The second operand must be a XMM register.
The third source operand can be a XMM register or a 32-bit memory location.
VFMSUB132SS: Multiplies the low packed single precision floating-point value from the first source operand to the low packed single precision floating-point value in the third source operand.
From the infinite precision interme-diate result, subtracts the low packed single precision floating-point values in the second source operand, performs rounding and stores the resulting packed single precision floating-point value to the destination operand (first source operand).VFMSUB213SS: Multiplies the low packed single precision floating-point value from the second source operand to the low packed single precision floating-point value in the first source operand.
From the infinite precision interme-diate result, subtracts the low packed single precision floating-point value in the third source operand, performs rounding and stores the resulting packed single precision floating-point value to the destination operand (first source operand).VFMSUB231SS: Multiplies the low packed single precision floating-point value from the second source to the low packed single precision floating-point value in the third source operand.
From the infinite precision intermediate result, subtracts the low packed single precision floating-point value in the first source operand, performs rounding and stores the resulting packed single precision floatingVEX.128 and EVEX encoded version: The destination operand (also first source operand) is encoded in reg_field.
The second source operand is encoded in VEX.vvvv/EVEX.vvvv.
The third source operand is encoded in rm_field.
Bits 127:32 of the destination are unchanged.
Bits MAXVL-1:128 of the destination register are zeroed.EVEX encoded version: The low doubleword element of the destination is updated according to the writemask.Compiler tools may optionally support a complementary mnemonic for each instruction mnemonic listed in the opcode/instruction column of the summary table.
The behavior of the complementary mnemonic in situations involving NANs are governed by the definition of the instruction mnemonic defined in the opcode/instruction column.

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal

## Operation

```C
In the operations below, "*" and "-" symbols represent multiplication and subtraction with infinite precision inputs and outputs (no rounding).VFMSUB132SS DEST, SRC2, SRC3 (EVEX encoded version)IF (EVEX.b = 1) and SRC3 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := RoundFPControl(DEST[31:0]*SRC3[31:0] - SRC2[31:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;DEST[127:32] := DEST[127:32]DEST[MAXVL-1:128] := 0VFMSUB213SS DEST, SRC2, SRC3 (EVEX encoded version)IF (EVEX.b = 1) and SRC3 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := RoundFPControl(SRC2[31:0]*DEST[31:0] - SRC3[31:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;VFMSUB231SS DEST, SRC2, SRC3 (EVEX encoded version)IF (EVEX.b = 1) and SRC3 *is a register*THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF k1[0] or *no writemask*THENDEST[31:0] := RoundFPControl(SRC2[31:0]*SRC3[63:0] - DEST[31:0])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;DEST[127:32] := DEST[127:32]DEST[MAXVL-1:128] := 0VFMSUB132SS DEST, SRC2, SRC3 (VEX encoded version)DEST[31:0] := RoundFPControl_MXCSR(DEST[31:0]*SRC3[31:0] - SRC2[31:0])DEST[127:32] := DEST[127:32]DEST[MAXVL-1:128] := 0VFMSUB213SS DEST, SRC2, SRC3 (VEX encoded version)DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*DEST[31:0] - SRC3[31:0])DEST[127:32] := DEST[127:32]DEST[MAXVL-1:128] := 0VFMSUB231SS DEST, SRC2, SRC3 (VEX encoded version)DEST[31:0] := RoundFPControl_MXCSR(SRC2[31:0]*SRC3[31:0] - DEST[31:0])DEST[127:32] := DEST[127:32]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVFMSUBxxxSS __m128 _mm_fmsub_round_ss(__m128 a, __m128 b, __m128 c, int r);VFMSUBxxxSS __m128 _mm_mask_fmsub_ss(__m128 a, __mmask8 k, __m128 b, __m128 c);VFMSUBxxxSS __m128 _mm_maskz_fmsub_ss(__mmask8 k, __m128 a, __m128 b, __m128 c);VFMSUBxxxSS __m128 _mm_mask3_fmsub_ss(__m128 a, __m128 b, __m128 c, __mmask8 k);VFMSUBxxxSS __m128 _mm_mask_fmsub_round_ss(__m128 a, __mmask8 k, __m128 b, __m128 c, int r);VFMSUBxxxSS __m128 _mm_maskz_fmsub_round_ss(__mmask8 k, __m128 a, __m128 b, __m128 c, int r);VFMSUBxxxSS __m128 _mm_mask3_fmsub_round_ss(__m128 a, __m128 b, __m128 c, __mmask8 k, int r);VFMSUBxxxSS __m128 _mm_fmsub_ss (__m128 a, __m128 b, __m128 c);
```
