# FYL2X

Compute y * log2x

Computes (ST(1)  log (ST(0))), stores the result in register ST(1), and pops the FPU register stack.
The source 2operand in ST(0) must be a non-zero positive number.The following table shows the results obtained when taking the log of various classes of numbers, assuming that neither overflow nor underflow occurs.Table 3-48.
 FYL2X ResultsST(0)  F 1F  1- ±0++ >+ +-NaN+0<+F<+1    -***++--NaN  -****F0FST(1)+ - - - FNaN-***000*+ - -  0NaN+***00 0*- + + 0NaN +****F0F- + + + FNaN     +-++**-*NaNNaNNaNNaNNaNNaNNaNNaNNaNNaNNOTES:FMeans finite floating-point value.*Indicates floating-point invalid-operation (#IA) exception.**Indicates floating-point zero-divide (#Z) exception.If the divide-by-zero exception is masked and register ST(0) contains ±0, the instruction returns  with a sign that is the opposite of the sign of the source operand in register ST(1).The FYL2X instruction is designed with a built-in multiplication to optimize the calculation of logarithms with an arbitrary positive base (b):-1logx := (logb)  logxb22This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Either operand is an SNaN or unsupported format.
  > Source operand in register ST(0) is a negative finite value 
  > (not 
  > 0).
  > -
  - #Z - Source operand in register ST(0) is
  > ±
  > 0.
  - #D - Source operand is a denormal value.
  - #U - Result is too small for destination format.
  - #O - Result is too large for destination format.
  - #P - Value cannot be represented
  > exactly in destination format.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
ST(1) := ST(1)  logST(0);2PopRegisterStack;FPU Flags AffectedC1Set to 0 if stack underflow occurred.
```
