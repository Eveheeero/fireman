# FIST/FISTP

Store Integer

The FIST instruction converts the value in the ST(0) register to a signed integer and stores the result in the desti-nation operand.
Values can be stored in word or doubleword integer format.
The destination operand specifies the address where the first byte of the destination value is to be stored.The FISTP instruction performs the same operation as the FIST instruction and then pops the register stack.
To pop the register stack, the processor marks the ST(0) register as empty and increments the stack pointer (TOP) by 1.
The FISTP instruction also stores values in quadword integer format.The following table shows the results obtained when storing various classes of numbers in integer format.Table 3-27.
 FIST/FISTP Results ST(0)DEST *or Value Too Large for DEST Format-    F-1-I  **< F< -0-1- 00+ 00    **<F<+ 1+ 0  F+ 1+ I *or Value Too Large for DEST Format+ NaN*NOTES:FMeans finite floating-point value.IMeans integer.*Indicates floating-point invalid-operation (#IA) exception.**0 or ±1, depending on the rounding mode.If the source value is a non-integral value, it is rounded to an integer value, according to the rounding mode spec-ified by the RC field of the FPU control word.
If the converted value is too large for the destination format, or if the source operand is an , SNaN, QNAN, or is in an unsupported format, an invalid-arithmetic-operand condition is signaled.
If the invalid-operation exception is not masked, an invalid-arithmetic-operand exception (#IA) is generated and no value is stored in the destination 

## Exceptions

- Floating-Point Exceptions
  - #IS - Stack underflow occurred.
  - #IA - Converted value is too large for the destination format.
  > Source operand is an SNaN, QNaN, 
  > ±
  > , or unsupported format.
  - #P - Value cannot be represented
  > exactly in destination format.
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
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Protected Mode Exceptions
  - #GP(0) - If the destination is located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
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

## Operation

```C
DEST := Integer(ST(0));=IF Instruction  FISTP THEN PopRegisterStack;FI;FPU Flags AffectedC1Set to 0 if stack underflow occurred.  Indicates rounding direction of if the inexact exception (#P) is generated: 0:= not roundup; 1:= roundup.Set to 0 otherwise.C0, C2, C3 Undefined.
```
