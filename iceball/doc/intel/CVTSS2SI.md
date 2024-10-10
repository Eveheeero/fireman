# CVTSS2SI

Convert Scalar Single Precision Floating-Point Value to Doubleword Integer

Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand).
The source operand can be an XMM register or a memory location.
The destination operand is a general-purpose register.
When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value w-1, where w represents the number of bits in the destination format) is returned.(2Legacy SSE instructions: In 64-bit mode, Use of the REX.W prefix promotes the instruction to produce 64-bit data.
See the summary chart at the beginning of this section for encoding data and limits.VEX.W1 and EVEX.W1 versions: promotes the instruction to produce 64-bit data in 64-bit mode.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b, otherwise instructions will #UD.Software should ensure VCVTSS2SI is encoded with VE

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 
  > 3 Class Exception Conditions," additionally:

## Operation

```C
VCVTSS2SI (EVEX Encoded Version)IF (SRC *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF 64-bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);ELSEDEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);FI;(V)CVTSS2SI (Legacy and VEX.128 Encoded Version) IF 64-bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);ELSEDEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0]);FI;Intel C/C++ Compiler Intrinsic EquivalentVCVTSS2SI int _mm_cvtss_i32( __m128 a);VCVTSS2SI int _mm_cvt_roundss_i32( __m128 a, int r);VCVTSS2SI __int64 _mm_cvtss_i64( __m128 a);VCVTSS2SI __int64 _mm_cvt_roundss_i64( __m128 a, int r);
```
