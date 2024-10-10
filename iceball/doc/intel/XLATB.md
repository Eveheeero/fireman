# XLAT/XLATB

Table Look-up Translation

Locates a byte entry in a table in memory, using the contents of the AL register as a table index, then copies the contents of the table entry back into the AL register.
The index in the AL register is treated as an unsigned integer.
The XLAT and XLATB instructions get the base address of the table in memory from either the DS:EBX or the DS:BX registers (depending on the address-size attribute of the instruction, 32 or 16, respectively).
(The DS segment may be overridden with a segment override prefix.)At the assembly-code level, two forms of this instruction are allowed: the "explicit-operand" form and the "no-operand" form.
The explicit-operand form (specified with the XLAT mnemonic) allows the base address of the table to be specified explicitly with a symbol.
This explicit-operands form is provided to allow documentation; however, note that the documentation provided by this form can be misleading.
That is, the symbol does not have to specify the correct base address.
The base address is always specified by the DS:(E)BX registers, which must be loaded correctly before the XLAT instruction is executed.The no-operands form (XLATB) provides a "short form" of the XLAT instructions.
Here also the processor assumes that the DS:(E)BX registers contain the base address of the table.In 64-bit mode, operation is similar to that in legacy or compatibility mode.
AL is used to specify the table index (the operand size is fixed at 8 bits).
RBX, however, is used to specify the table's base address.
See the summary chart at the beginning of this section for encoding data and limits.

## Exceptions

- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.

## Operation

```C
= IF AddressSize 16THENAL := (DS:BX + ZeroExtend(AL));=ELSE IF (AddressSize  32)AL := (DS:EBX + ZeroExtend(AL)); FI;=ELSE (AddressSize  64)AL := (RBX + ZeroExtend(AL));FI;
```
