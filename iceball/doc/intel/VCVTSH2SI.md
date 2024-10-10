# VCVTSH2SI

Convert Low FP16 Value to Signed Integer

This instruction converts the low FP16 element in the source operand to a signed integer in the destination general purpose register.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the integer indefinite value is returned.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTSH2SI dest, src IF *SRC is a register* and (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)IF 64-mode and OperandSize == 64:DEST.qword := Convert_fp16_to_integer64(SRC.fp16[0])ELSE:DEST.dword := Convert_fp16_to_integer32(SRC.fp16[0]) Intel C/C++ Compiler Intrinsic EquivalentVCVTSH2SI int _mm_cvt_roundsh_i32 (__m128h a, int rounding);VCVTSH2SI __int64 _mm_cvt_roundsh_i64 (__m128h a, int rounding);VCVTSH2SI int _mm_cvtsh_i32 (__m128h a);VCVTSH2SI __int64 _mm_cvtsh_i64 (__m128h a);
```
