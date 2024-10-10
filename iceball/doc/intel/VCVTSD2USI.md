# VCVTSD2USI

Convert Scalar Double Precision Floating-Point Value to Unsigned Doubleword Integer

Converts a double precision floating-point value in the source operand (the second operand) to an unsigned doubleword integer in the destination operand (the first operand).
The source operand can be an XMM register or a 64-bit memory location.
The destination operand is a general-purpose register.
When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination w - 1 is format, the floating-point invalid exception is raised, and if this exception is masked, the integer value 2returned, where w represents the number of bits in the destination format.

## Operation

```C
VCVTSD2USI (EVEX Encoded Version)IF (SRC *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF 64-Bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Double_Precision_Floating_Point_To_UInteger(SRC[63:0]);ELSEDEST[31:0] := Convert_Double_Precision_Floating_Point_To_UInteger(SRC[63:0]);FIIntel C/C++ Compiler Intrinsic EquivalentVCVTSD2USI unsigned int _mm_cvtsd_u32(__m128d);VCVTSD2USI unsigned int _mm_cvt_roundsd_u32(__m128d, int r);VCVTSD2USI unsigned __int64 _mm_cvtsd_u64(__m128d);VCVTSD2USI unsigned __int64 _mm_cvt_roundsd_u64(__m128d, int r);
```
