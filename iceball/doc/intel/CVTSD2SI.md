# CVTSD2SI

Convert Scalar Double Precision Floating-Point Value to Doubleword Integer

Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer in the destination operand (first operand).
The source operand can be an XMM register or a 64-bit memory location.
The destination operand is a general-purpose register.
When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
If a converted result exceeds the range limits of signed doubleword integer (in non-64-bit modes or 64-bit mode with REX.W/VEX.W/EVEX.W=0), the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value (80000000H) is returned.If a converted result exceeds the range limits of signed quadword integer (in 64-bit mode and REX.W/VEX.W/EVEX.W = 1), the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value (80000000_00000000H) is returned.Legacy SSE instruction: Use of the REX.W prefix promotes the instruction to produce 64-bit data in 64-bit mode.
See the summary chart at the beginning of this section for encoding data and limits.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-48, "Type E3NF Class Exception Conditions."
  > Additionally:

## Operation

```C
VCVTSD2SI (EVEX Encoded Version)IF SRC *is register* AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF 64-Bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0]);ELSEDEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0]);FI(V)CVTSD2SI IF 64-Bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0]);ELSEDEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0]);FI;Intel C/C++ Compiler Intrinsic EquivalentVCVTSD2SI int _mm_cvtsd_i32(__m128d);VCVTSD2SI int _mm_cvt_roundsd_i32(__m128d, int r);VCVTSD2SI __int64 _mm_cvtsd_i64(__m128d);VCVTSD2SI __int64 _mm_cvt_roundsd_i64(__m128d, int r);CVTSD2SI __int64 _mm_cvtsd_si64(__m128d);CVTSD2SI int _mm_cvtsd_si32(__m128d a)
```
