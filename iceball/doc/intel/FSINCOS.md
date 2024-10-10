# FSINCOS

Sine and Cosine

Computes both the approximate sine and the cosine of the source operand in register ST(0), stores the sine in ST(0), and pushes the cosine onto the top of the FPU register stack.
(This instruction is faster than executing the FSIN and FCOS instructions in succession.)6363 to +2.
The following table shows The source operand must be given in radians and must be within the range -2the results obtained when taking the sine and cosine of various classes of numbers, assuming that underflow does not occur.Table 3-36.
 FSINCOS ResultsSRCDESTST(0)ST(1) CosineST(0) Sine**- F- - +-+1 to 1 1 to  1 00- + - 1010+ + + F+ - +- +1 to 11 to 1   **+NaNNaNNaN NOTES:FMeans finite floating-point value.*Indicates floating-point invalid-arithmetic-operand (#IA) exception.If the source operand is outside the acceptable range, the C2 flag in the FPU status word is set, and the value in register ST(0) remains unchanged.
The instruction does not raise an exception when the source operand is out of range.
It is up to the program to check the C2 flag for out-of-range conditions.
Source values outside the range -63632 to +2 can be reduced to the range of the instruction by subtracting an appropriate integer multiple of 2.
6363However, even within the range -2 to +2, inaccurate results can occur because the finite approximation of  used internally for argument reduction is not sufficient in all cases.
Therefore, for accurate results it is safe to apply FSINCOS only to arguments reduced accurately in software, to a value smaller in absolute value than 3/8.
See the ® 64 and sections titled "Approximation of Pi" and "Transcendental Instruction Accuracy" in Chapter 8 of the IntelIA-32 Architectures Software Developer's Manual, Volume 1, for a discussion of the proper value to use for  in 

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow or overflow occurred.
  - #IA - Source operand is an SNaN value,
  > 
  > , or unsupported format.
  - #D - Source operand is a denormal value.
  - #U - Result is too small for destination format.
  - #P - Value cannot be represented exactly in destination format.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
63IF ST(0) < 2THENC2 := 0;TEMP := fcos(ST(0)); // approximation of cosineST(0) := fsin(ST(0)); // approximation of sineTOP := TOP - 1;ST(0) := TEMP;ELSE (* Source operand out of range *)C2 := 1;FI;FPU Flags AffectedC1Set to 0 if stack underflow occurred; set to 1 of stack overflow occurs.Set if result was rounded up; cleared otherwise.6363C2Set to 1 if outside range (-2 < source operand < +2); otherwise, set to 0.C0, C3 Undefined.
```
