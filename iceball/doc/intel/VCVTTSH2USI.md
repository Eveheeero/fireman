# VCVTTSH2USI

Convert with Truncation Low FP16 Value to an Unsigned Integer

This instruction converts the low FP16 element in the source operand to an unsigned integer in the destination general purpose register.When a conversion is inexact, a truncated (round toward zero) value is returned.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the integer indefinite value is returned.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTTSH2USI dest, src IF 64-mode and OperandSize == 64:DEST.qword := Convert_fp16_to_unsigned_integer64_truncate(SRC.fp16[0])ELSE:DEST.dword := Convert_fp16_to_unsigned_integer32_truncate(SRC.fp16[0]) Intel C/C++ Compiler Intrinsic EquivalentVCVTTSH2USI unsigned int _mm_cvtt_roundsh_u32 (__m128h a, int sae);VCVTTSH2USI unsigned __int64 _mm_cvtt_roundsh_u64 (__m128h a, int sae);VCVTTSH2USI unsigned int _mm_cvttsh_u32 (__m128h a);VCVTTSH2USI unsigned __int64 _mm_cvttsh_u64 (__m128h a);
```
