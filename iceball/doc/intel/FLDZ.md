# FLD1/FLDL2T/FLDL2E/FLDPI/FLDLG2/FLDLN2/FLDZ

Load Constant

Push one of seven commonly used constants (in double extended-precision floating-point format) onto the FPU 10, loge, , log2, register stack.
The constants that can be loaded with these instructions include +1.0, +0.0, log2210and log2.
For each constant, an internal 66-bit constant is rounded (as specified by the RC field in the FPU control eword) to double extended-precision floating-point format.
The inexact-result exception (#P) is not generated as a result of the rounding, nor is the C1 flag set in the x87 FPU status word if the value is rounded up.
® 64 and IA-32 Architectures Software Devel-See the section titled "Approximation of Pi" in Chapter 8 of the Inteloper's Manual, Volume 1, for a description of the  constant.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture CompatibilityWhen the RC field is set to round-to-nearest, the FPU produces the same constants that is produced by the Intel 8087 and Intel 287 math coprocessors.

## Exceptions

- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Floating-Point Exceptions
  - #IS - Stack overflow occurred.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
-TOP := TOP  1;ST(0) := CONSTANT;FPU Flags AffectedC1Set to 1 if stack overflow occurred; otherwise, set to 0.C0, C2, C3 Undefined.
```
