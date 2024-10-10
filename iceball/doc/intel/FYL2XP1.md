# FYL2XP1

Compute y * log2(x +1)

Computes (ST(1)  log(ST(0) + 1.0)), stores the result in register ST(1), and pops the FPU register stack.
The 2source operand in ST(0) must be in the range:-())122-to122()-The source operand in ST(1) can range from - to +.
If the ST(0) operand is outside of its acceptable range, the result is undefined and software should not rely on an exception being generated.
Under some circumstances exceptions may be generated when ST(0) is out of range, but this behavior is implementation specific and not guaranteed.The following table shows the results obtained when taking the log epsilon of various classes of numbers, assuming that underflow does not occur.Table 3-49.
 FYL2XP1 ResultsST(0)(1 )) to 00 to (1 ())NaN-- (--0+0++ -2222  NaN-**-+NaNST(1)- F+F+0-0- FNaN- 0+0+0-0- 0NaN+0- 0- 0+0+0NaN+F- F- 0+0+F NaN+-**+NaNNaNNaNNaNNaNNaNNOTES:FMeans finite floating-point value.*Indicates floating-point invalid-operation (#IA) exception.This instruction provides optimal accuracy for values of epsilon [the value in register ST(0)] that are close to 0.
For small epsilon (	) values, more significant digits can be retained by using the FYL2XP1 instruction than by using (	+1) as an argument to the FYL2X instruction.
The (	+1) expression is commonly found in compound interest and annuity calculations.
The result can be simply converted into a value in another logarithm base by including a scale factor in the ST(1) source operand.
The following equation is used to calculate the scale factor for a particular loga-rithm base, where n is the logarithm base desired for the result of the FYL2XP1 instruction: 2scale factor := lognThis instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Either operand is an SNaN value or unsupported format.
  - #D - Source operand is a denormal value.
  - #U - Result is too small for destination format.
  - #O - Result is too large for destination format.
  - #P - Value cannot be represented
  > exactly in destination format.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.

## Operation

```C
ST(1) := ST(1)  log(ST(0) + 1.0);FPU Flags AffectedC1Set to 0 if stack underflow occurred.Set if result was rounded up; cleared otherwise.C0, C2, C3 Undefined.
```
