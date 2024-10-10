# FPATAN

Partial Arctangent

Computes the arctangent of the source operand in register ST(1) divided by the source operand in register ST(0), stores the result in ST(1), and pops the FPU register stack.
The result in register ST(0) has the same sign as the source operand ST(1) and a magnitude less than +.The FPATAN instruction returns the angle between the X axis and the line from the origin to the point (X,Y), where Y (the ordinate) is ST(1) and X (the abscissa) is ST(0).
The angle depends on the sign of X and Y independently, not just on the sign of the ratio Y/X.
This is because a point (-X,Y) is in the second quadrant, resulting in an angle between /2 and , while a point (X,-Y) is in the fourth quadrant, resulting in an angle between 0 and -/2.
A point (-X,-Y) is in the third quadrant, giving an angle between -/2 and -.The following table shows the results obtained when computing the arctangent of various classes of numbers, assuming that underflow does not occur.Table 3-30.
 FPATAN ResultsST(0) -- F- 0+ 0+ F+ NaN **-- 3/4- /2- /2- /2- /2- /4NaNST(1)- F-p- to -/2-/2-/2-/2 to -0- 0NaN**- 0-p-p-p- 0- 0- 0NaN**+ 0+p+ p+ + 0+ 0+ 0NaN+ F+p+ to +/2+ /2+/2+/2 to +0+ 0NaN **++3/4+/2+/2+/2+ /2+ /4NaNNaNNaNNaNNaNNaNNaNNaNNaNNOTES:FMeans finite floating-point value.® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, specifies that the ratios 0/0 and / *Table 8-10 in the Intelgenerate the floating-point invalid arithmetic-operation exception and, if this exception is masked, the floating-point QNaN indefi-/ value is actually not calculated using division.
Instead, the arc-nite value is returned.
With the FPATAN instruction, the 0/0 or tangent of the two variables is derived from a standard mathematical formulation that is generalized to allow complex numbers as arguments.
In this complex variable formulation, arctangent(0,0) etc.
has well defined values.
These values are needed to develop a library to compute transcendental functions with complex arguments, based on the FPU functions that only allow floating-point values as arguments.There is no restriction on the range of source operands that FPATAN can accept.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture CompatibilityThe source operands for this instruction are restricted for the 80287 math coprocessor to the following range:0  |ST(1)| < |ST(0)| <

## Exceptions

- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Source operand is an SNaN value or unsupported format.
  - #D - Source operand is a denormal value.
  - #U - Result is too small for destination format.
  - #P - Value cannot be represented exactly in destination format.

## Operation

```C
ST(1) := arctan(ST(1) / ST(0));PopRegisterStack;FPU Flags AffectedC1Set to 0 if stack underflow occurred.Set if result was rounded up; cleared otherwise.C0, C2, C3 Undefined.
```
