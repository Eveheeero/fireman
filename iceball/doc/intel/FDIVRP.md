# FDIVR/FDIVRP/FIDIVR

Reverse Divide

Divides the source operand by the destination operand and stores the result in the destination location.
The desti-nation operand (divisor) is always in an FPU register; the source operand (dividend) can be a register or a memory location.
Source operands in memory can be in single precision or double precision floating-point format, word or doubleword integer format.These instructions perform the reverse operations of the FDIV, FDIVP, and FIDIV instructions.
They are provided to support more efficient coding.The no-operand version of the instruction divides the contents of the ST(0) register by the contents of the ST(1) register.
The one-operand version divides the contents of a memory location (either a floating-point or an integer value) by the contents of the ST(0) register.
The two-operand version, divides the contents of the ST(i) register by the contents of the ST(0) register or vice versa.The FDIVRP instructions perform the additional operation of popping the FPU register stack after storing the result.
To pop the register stack, the processor marks the ST(0) register as empty and increments the stack pointer (TOP) by 1.
The no-operand version of the floating-point divide instructions always results in the register stack being popped.
In some assemblers, the mnemonic for this instruction is FDIVR rather than FDIVRP.The FIDIVR instructions convert an integer source operand to double extended-precision floating-point format before performing the division.If an unmasked divide-by-zero exception (#Z) is generated, no result is stored; if the exception is masked, an  of the appropriate sign is stored in the destination operand.The following table shows the results obtained when dividiTable 3-25.
 FDIVR/FDIVRP/FIDIVR ResultsDEST  F00FNaN-- - + + +  **NaN-++--   F0F**** F0NaNSRC- + + -- I0F**** F0NaN- + + -- 000**00NaN- + + - - 000**00NaN+ - - + + I0 F**** F0NaN+ - -++ F0 F****F0NaN+ - -+ +    **NaN+--++  NaNNaNNaNNaNNaNNaNNaNNaNNOTES:FMeans finite floating-point value.IMeans integer.*Indicates floating-point invalid-arithmetic-operand (#IA) exception.** Indicates floating-point zero-divide (#Z) exception.When the source operand is an integer 0, it is treated as a +0.
This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Operand is an SNaN value or unsupported format.
  > ± / ±
  > ; 
  > ±
  > 0 / 
  > ±
  > 0
  - #D - Source is a denormal value.
  - #Z - SRC /
  > ±
  > 0, where SRC is not equal to 
  > ±
  > 0.
  - #U - Result is too small for destination format.
  - #O - Result is too large for destination format.
  - #P - Value cannot be represented
  > exactly in destination format.

## Operation

```C
= IF DEST 0THEN#Z;ELSE= IF Instruction FIDIVRTHENDEST := ConvertToDoubleExtendedPrecisionFP(SRC) / DEST;ELSE (* Source operand is floating-point value *)DEST := SRC / DEST;FI;FI;= IF Instruction FDIVRP THEN PopRegisterStack;FI;FPU Flags AffectedC1Set to 0 if stack underflow occurred.
```
