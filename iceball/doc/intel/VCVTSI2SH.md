# VCVTSI2SH

Convert a Signed Doubleword/Quadword Integer to an FP16 Value

This instruction converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the second source operand to an FP16 value in the destination operand.
The result is stored in the low word of the desti-nation operand.
When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or embedded rounding controls.The second source operand can be a general-purpose register or a 32/64-bit memory location.
The first source and destination operands are XMM registers.
Bits 127:16 of the XMM register destination are copied from corre-sponding bits in the first source operand.
Bits MAXVL-1:128 of the destination register are zeroed.If the result of the convert operation is overflow and MXCSR.OM=0 then a SIMD exception will be raised with OE=1, PE=1.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Precision.

## Operation

```C
VCVTSI2SH dest, src1, src2IF *SRC2 is a register* and (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)IF 64-mode and OperandSize == 64:DEST.fp16[0] := Convert_integer64_to_fp16(SRC2.qword)ELSE:DEST.fp16[0] := Convert_integer32_to_fp16(SRC2.dword)DEST[127:16] := SRC1[127:16]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTSI2SH __m128h _mm_cvt_roundi32_sh (__m128h a, int b, int rounding);VCVTSI2SH __m128h _mm_cvt_roundi64_sh (__m128h a, __int64 b, int rounding);
```
