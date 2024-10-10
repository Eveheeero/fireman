# FUCOM/FUCOMP/FUCOMPP

Unordered Compare Floating-Point Values

Performs an unordered comparison of the contents of register ST(0) and ST(i) and sets condition code flags C0, C2, and C3 in the FPU status word according to the results (see the table below).
If no operand is specified, the contents of registers ST(0) and ST(1) are compared.
The sign of zero is ignored, so that -0.0 is equal to +0.0.Table 3-41.
 FUCOM/FUCOMP/FUCOMPP ResultsComparison Results*C3C2C0ST0  ST(i)000>ST0  ST(i)001< ST(i)100ST0 =Unordered111NOTES:*Flags not set if unmasked invalid-arithmetic-operand (#IA) exception is generated.An unordered comparison checks the class of the numbers being compared (see "FXAM-Examine Floating-Point" in this chapter).
The FUCOM/FUCOMP/FUCOMPP instructions perform the same operations as the FCOM/FCOMP/FCOMPP instructions.
The only difference is that the FUCOM/FUCOMP/FUCOMPP instructions raise the invalid-arithmetic-operand exception (#IA) only when either or both operands are an SNaN or are in an unsup-ported format; QNaNs cause the condition code flags to be set to unordered, but do not cause an exception to be generated.
The FCOM/FCOMP/FCOMPP instructions raise an invalid-operation exception when either or both of the operands are a NaN value of any kind or are in an unsupported format.As with the FCOM/FCOMP/FCOMPP instructions, if the operation results in an invalid-arithmetic-operand exception being raised, the condition code flags are set only if the exception is masked.The FUCOMP instruction pops the register stack following the comparison operation and the FUCOMPP instruction pops the register stack twice following the comparison operation.
To pop the register stack, the processor marks 

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - One or both operands are SNaN values or
  > have unsupported formats. Detection of a QNaN 
  > value in and of itself does not raise an invalid-operand exception.
  - #D - One or both operands are denormal values.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.

## Operation

```C
CASE (relation of operands) OFST > SRC:C3, C2, C0 := 000;ST < SRC:C3, C2, C0 := 001;=ST  SRC:C3, C2, C0 := 100;ESAC;=IF ST(0) or SRC  QNaN, but not SNaN or unsupported formatTHEN C3, C2, C0 := 111;ELSE (* ST(0) or SRC is SNaN or unsupported format *) #IA;= 1IF FPUControlWord.IM THEN C3, C2, C0 := 111;FI;FI;=IF Instruction  FUCOMP THEN PopRegisterStack;FI;=IF Instruction  FUCOMPP THEN PopRegisterStack; FI;FPU Flags AffectedC1Set to 0 if stack underflow occurred.C0, C2, C3See Table 3-41.
```
