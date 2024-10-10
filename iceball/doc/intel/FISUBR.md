# FSUBR/FSUBRP/FISUBR

Reverse Subtract

Subtracts the destination operand from the source operand and stores the difference in the destination location.
The destination operand is always an FPU register; the source operand can be a register or a memory location.
Source operands in memory can be in single precision or double precision floating-point format or in word or doubleword integer format.These instructions perform the reverse operations of the FSUB, FSUBP, and FISUB instructions.
They are provided to support more efficient coding.The no-operand version of the instruction subtracts the contents of the ST(1) register from the ST(0) register and stores the result in ST(1).
The one-operand version subtracts the contents of the ST(0) register from the contents of a memory location (either a floating-point or an integer value) and stores the result in ST(0).
The two-operand version, subtracts the contents of the ST(i) register from the ST(0) register or vice versa.The FSUBRP instructions perform the additional operation of popping the FPU register stack following the subtrac-tion.
To pop the register stack, the processor marks the ST(0) register as empty and increments the stack pointer (TOP) by 1.
The no-operand version of the floating-point reverse subtract instructions always results in the register stack being popped.
In some assemblers, the mnemonic for this instruction is FSUBR rather than FSUBRP.The FISUBR instructions convert an integer source operand to double extended-precision floating-point format before performing the subtraction.The following table shows the results obtained when subtracting various classes of numbers from one another, assuming that neither overflow nor underflow occurs.
Here, the DEST value is subtracted from the SRC value (SRC   =result).- DESTWhen the difference between two operands of like sign is 0, the result is +0, except for the round toward  mode, -    in which case the result is -0.
This instruction also guarantees that +0 - (-0)=+0, and that -0 - (+0)=-0.
When the source operand is an integer 0, it is treated as a +0.When one operand is , the result is  of the expected sign.
If both operands are Table 3-39.
 FSUBR/FSUBRP/FISUBR ResultsSRC  F or I00F or INaN----++++     *NaN-+++++   FF or 0DESTDESTFNaN- -±±--+ +  0SRC00SRCNaNDEST- -±+ +  0SRC00SRCNaN+ -- ±+  FFDESTDESTF or 0NaN+ -- --±±+      *NaN+-----NaNNaNNaNNaNNaNNaNNaNNaNNOTES:FMeans finite floating-point value.IMeans integer.*Indicates floating-point invalid-arithmetic-operand (#IA) exception.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
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
- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Operand is an SNaN value or unsupported format.
  > Operands are infinities of like sign.
  - #D - Source operand is a denormal value.
  - #U - Result is too small for destination format.
  - #O - Result is too large for destination format.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
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
IF Instruction = FISUBRTHEN-DEST := ConvertToDoubleExtendedPrecisionFP(SRC)  DEST;ELSE (* Source operand is floating-point value *)-DEST := SRC  DEST; FI; IF Instruction =FSUBRP THEN PopRegisterStack; FI;FPU Flags AffectedC1Set to 0 if stack underflow occurred.Set if result was rounded up; cleared otherwise.C0, C2, C3 Undefined.
```
