# MUL

Unsigned Multiply

Performs an unsigned multiplication of the first operand (destination operand) and the second operand (source operand) and stores the result in the destination operand.
The destination operand is an implied operand located in register AL, AX or EAX (depending on the size of the operand); the source operand is located in a general-purpose register or a memory location.
The action of this instruction and the location of the result depends on the opcode and the operand size as shown in Table 4-9.The result is stored in register AX, register pair DX:AX, or register pair EDX:EAX (depending on the operand size), with the high-order bits of the product contained in register AH, DX, or EDX, respectively.
If the high-order bits of the product are 0, the CF and OF flags are cleared; otherwise, the flags are set.In 64-bit mode, the instruction's default operation size is 32 bits.
Use of the REX.R prefix permits access to addi-tional registers (R8-R15).
Use of the REX.W prefix promotes operation to 64 bits.
See the summary chart at the beginning of this section for encoding data and limits.Table 4-9.
 MUL ResultsOperand SizeSource 1Source 2DestinationByteALr/m8AXWordAXr/m16DX:AXDoublewordEAXr/m32EDX:EAXQuadwordRAXr/m64RDX:RAX

## Flags affected

- The OF and CF flags are set to 0 if the upper half of the result is 0; otherwise, they are set to 1. The SF, ZF, AF, and PF flags are undefined.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an

## Operation

```C
IF (Byte operation)THEN AX := AL  SRC;ELSE (* Word or doubleword operation *) IF OperandSize =16THEN DX:AX := AX  SRC;ELSE IF OperandSize = 32THEN EDX:EAX := EAX  SRC; FI;ELSE (* OperandSize = 64 *)RDX:RAX := RAX FI;FI;
```
