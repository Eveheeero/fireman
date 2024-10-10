# FTST

TEST

Compares the value in the ST(0) register with 0.0 and sets the condition code flags C0, C2, and C3 in the FPU status word according to the results (see table below).Table 3-40.
 FTST ResultsConditionC3C2C0ST(0)  0.0000>ST(0)  0.0001<ST(0)  0.0100=Unordered111This instruction performs an "unordered comparison." An unordered comparison also checks the class of the numbers being compared (see "FXAM-Examine Floating-Point" in this chapter).
If the value in register ST(0) is a NaN or is in an undefined format, the condition flags are set to "unordered" and the invalid operation exception is generated.
:= +0.0).The sign of zero is ignored, so that (- 0.0This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - The source operand is a NaN value or is in an unsupported format.
  - #D - The source operand is a denormal value.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.

## Operation

```C
CASE (relation of operands) OFNot comparable:C3, C2, C0 := 111;ST(0) > 0.0:C3, C2, C0 := 000;ST(0) < 0.0:C3, C2, C0 := 001;= 0.0:C3, C2, C0 := 100;ST(0) ESAC;FPU Flags AffectedC1Set to 0.C0, C2, C3See Table 3-40.
```
