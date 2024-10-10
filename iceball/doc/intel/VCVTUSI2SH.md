# VCVTUSI2SH

Convert Unsigned Doubleword Integer to an FP16 Value

This instruction converts an unsigned doubleword integer (or unsigned quadword integer if operand size is 64 bits) in the second source operand to a FP16 value in the destination operand.
The result is stored in the low word of the destination operand.
When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or embedded rounding controls.The second source operand can be a general-purpose register or a 32/64-bit memory location.
The first source and destination operands are XMM registers.
Bits 127:16 of the XMM register destination are copied from corresponding bits in the first source operand.
Bits MAXVL-1:128 of the destination register are zeroed.If the result of the convert operation is overflow and MXCSR.OM=0 then a SIMD exception will be raised with OE=1, PE=1.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Precision.

## Operation

```C
VCVTUSI2SH dest, src1, src2IF *SRC2 is a register* and (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)IF 64-mode and OperandSize == 64:DEST.fp16[0] := Convert_unsigned_integer64_to_fp16(SRC2.qword)ELSE:DEST.fp16[0] := Convert_unsigned_integer32_to_fp16(SRC2.dword)Intel C/C++ Compiler Intrinsic EquivalentVCVTUSI2SH __m128h _mm_cvt_roundu32_sh (__m128h a, unsigned int b, int rounding);VCVTUSI2SH __m128h _mm_cvt_roundu64_sh (__m128h a, unsigned __int64 b, int rounding);VCVTUSI2SH __m128h _mm_cvtu32_sh (__m128h a, unsigned int b);VCVTUSI2SH __m128h _mm_cvtu64_sh (__m128h a, unsigned __int64 b);
```
