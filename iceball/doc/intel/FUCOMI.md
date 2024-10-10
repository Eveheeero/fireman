# FCOMI/FCOMIP/ FUCOMI/FUCOMIP

Compare Floating-Point Values and Set EFLAGS

Performs an unordered comparison of the contents of registers ST(0) and ST(i) and sets the status flags ZF, PF, and CF in the EFLAGS register according to the results (see the table below).
The sign of zero is ignored for compari-sons, so that -0.0 is equal to +0.0.
Table 3-22.
 FCOMI/FCOMIP/ FUCOMI/FUCOMIP ResultsComparison Results*ZFPFCF  >ST(i)000ST0  <ST(i)001ST0  =ST(i)100ST0Unordered**111NOTES:*See the IA-32 Architecture Compatibility section below.**Flags not set if unmasked invalid-arithmetic-operand (#IA) exception is generated.An unordered comparison checks the class of the numbers being compared (see "FXAM-Examine Floating-Point" in this chapter).
The FUCOMI/FUCOMIP instructions perform the same operations as the FCOMI/FCOMIP instruc-tions.
The only difference is that the FUCOMI/FUCOMIP instructions raise the invalid-arithmetic-operand exception (#IA) only when either or both operands are an SNaN or are in an unsupported format; QNaNs cause the condition code flags to be set to unordered, but do not cause an exception to be generated.
The FCOMI/FCOMIP instructions raise an invalid-operation exception when either or both of the operands are a NaN value of any kind or are in an unsupported format.If the operation results in an invalid-arithmetic-operand exception being raised, the status flags in the EFLAGS register are set only if the exception is masked.
The FCOMI/FCOMIP and FUCOMI/FUCOMIP instructions set the OF, SF, and AF flags to zero in the EFLAGS register (regardless of whether an invalid-operation exception is detected).The FCOMIP and FUCOMIP instructions also pop the register stack following the comparison operation.
To pop the register stack, the processor marks the ST(0) register as empty and increments the stack pointer (TOP) by 1.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture CompatibilityThe FCOMI/FCOMIP/FUCOMI/FUCOMIP instructions were intr

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - (FCOMI or FCOMIP instruction) One or both operands are NaN values or have unsupported
  > formats.
  > (FUCOMI or FUCOMIP instruction) One or both op
  > erands are SNaN values (but not QNaNs) or 
  > have undefined formats. Detection of a QNaN 
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
CASE (relation of operands) OF> ST(0) ST(i):ZF, PF, CF := 000;<ST(0)  ST(i):ZF, PF, CF := 001;= ST(0) ST(i):ZF, PF, CF := 100;ESAC;IF Instruction is FCOMI or FCOMIPTHEN= IF ST(0) or ST(i) NaN or unsupported formatTHEN #IA= 1IF FPUControlWord.IM THEN ZF, PF, CF := 111;FI;FI;FI;IF Instruction is FUCOMI or FUCOMIPTHEN=IF ST(0) or ST(i)  QNaN, but not SNaN or unsupported formatTHEN ZF, PF, CF := 111;ELSE (* ST(0) or ST(i) is SNaN or unsupported format *) #IA;= 1IF FPUControlWord.IM THEN ZF, PF, CF := 111;FI;FI;FI;IF Instruction is FCOMIP or FUCOMIP THEN PopRegisterStack;FI;FPU Flags AffectedC1Set to 0.C0, C2, C3 Not affected.
```
