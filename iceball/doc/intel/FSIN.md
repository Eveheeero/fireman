# FSIN

Sine

Computes an approximation of the sine of the source operand in register ST(0) and stores the result in ST(0).
The 6363source operand must be given in radians and must be within the range -2 to +2.
The following table shows the results obtained when taking the sine of various classes of numbers, assuming that underflow does not occur.Table 3-35.
 FSIN ResultsSRC (ST(0))DEST (ST(0))-* - -+F 1 to 1 00- -0 0+ +-+F+  1 to 1 +*NaNNaN NOTES:FMeans finite floating-point value.*Indicates floating-point invalid-arithmetic-operand (#IA) exception.If the source operand is outside the acceptable range, the C2 flag in the FPU status word is set, and the value in register ST(0) remains unchanged.
The instruction does not raise an exception when the source operand is out of range.
It is up to the program to check the C2 flag for out-of-range conditions.
Source values outside the range -6363 to +2 can be reduced to the range of the instruction by subtracting an appropriate integer multiple of 2.
26363However, even within the range -2 to +2, inaccurate results can occur because the finite approximation of  used internally for argument reduction is not sufficient in all cases.
Therefore, for accurate results it is safe to apply FSIN only to arguments reduced accurately in software, to a value smaller in absolute value than 3/4.
See the ® 64 and sections titled "Approximation of Pi" and "Transcendental Instruction Accuracy" in Chapter 8 of the IntelIA-32 Architectures Software Developer's Manual, Volume 1, for a discussion of the proper value to use for  in performing such reductions.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Source operand is an SNaN value,
  > 
  > , or unsupported format.
  - #D - Source operand is a denormal value.
  - #P - Value cannot be represented exactly in destination format.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
63 63IF -2< ST(0) < 2THENC2 := 0;ST(0) := fsin(ST(0)); // approximation of the mathematical sin functionELSE (* Source operand out of range *)C2 := 1;FI;FPU Flags AffectedC1Set to 0 if stack underflow occurred.Set if result was rounded up; cleared otherwise.6363C2Set to 1 if outside range (-2 < source operand < +2
```
