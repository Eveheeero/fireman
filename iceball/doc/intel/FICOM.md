# FICOM/FICOMP

Compare Integer

Compares the value in ST(0) with an integer source operand and sets the condition code flags C0, C2, and C3 in the FPU status word according to the results (see table below).
The integer value is converted to double extended-precision floating-point format before the comparison is made.Table 3-26.
 FICOM/FICOMP ResultsConditionC3C2C0ST(0) > SRC000 < SRC001ST(0) SRC100ST(0) =Unordered111These instructions perform an "unordered comparison." An unordered comparison also checks the class of the numbers being compared (see "FXAM-Examine Floating-Point" in this chapter).
If either operand is a NaN or is in an undefined format, the condition flags are set to "unordered." The sign of zero is ignored, so that -0.0:= +0.0.The FICOMP instructions pop the register stack following the comparison.
To pop the register stack, the processor marks the ST(0) register empty and increments the stack pointer (TOP) by 1.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 

## Operation

```C
CASE (relation of operands) OFST(0) > SRC:C3, C2, C0 := 000;ST(0) < SRC:C3, C2, C0 := 001;= SRC:C3, C2, C0 := 100;ST(0) Unordered:C3, C2, C0 := 111;ESAC;= IF Instruction FICOMP THEN PopRegisterStack; FI;FPU Flags AffectedC1Set to 0.C0, C2, C3See table on previous page.
```
