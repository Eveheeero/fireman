# VCVTTSH2SI

Convert with Truncation Low FP16 Value to a Signed Integer

This instruction converts the low FP16 element in the source operand to a signed integer in the destination general purpose register.When a conversion is inexact, a truncated (round toward zero) value is returned.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the integer indefinite value is returned.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTTSH2SI dest, srcIF 64-mode and OperandSize == 64:DEST.qword := Convert_fp16_to_integer64_truncate(SRC.fp16[0])ELSE:DEST.dword := Convert_fp16_to_integer32_truncate(SRC.fp16[0]) Intel C/C++ Compiler Intrinsic EquivalentVCVTTSH2SI int _mm_cvtt_roundsh_i32 (__m128h a, int sae);VCVTTSH2SI __int64 _mm_cvtt_roundsh_i64 (__m128h a, int sae);VCVTTSH2SI int _mm_cvttsh_i32 (__m128h a);VCVTTSH2SI __int64 _mm_cvttsh_i64 (__m128h a);
```
