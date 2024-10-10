# LGDT/LIDT

Load Global/Interrupt Descriptor Table Register

Loads the values in the source operand into the global descriptor table register (GDTR) or the interrupt descriptor table register (IDTR).
The source operand specifies a 6-byte memory location that contains the base address (a linear address) and the limit (size of table in bytes) of the global descriptor table (GDT) or the interrupt descriptor table (IDT).
If operand-size attribute is 32 bits, a 16-bit limit (lower 2 bytes of the 6-byte data operand) and a 32-bit base address (upper 4 bytes of the data operand) are loaded into the register.
If the operand-size attribute is 16bits, a 16-bit limit (lower 2 bytes) and a 24-bit base address (third, fourth, and fifth byte) are loaded.
Here, the high-order byte of the operand is not used and the high-order byte of the base address in the GDTR or IDTR is filled with zeros.The LGDT and LIDT instructions are used only in operating-system software; they are not used in application programs.
They are the only instructions that directly load a linear address (that is, not a segment-relative address) and a limit in protected mode.
They are commonly executed in real-address mode to allow processor initialization prior to switching to protected mode.In 64-bit mode, the instruction's operand size is fixed at 8+2 bytes (an 8-byte base and a 2-byte limit).
See the summary chart at the beginning of this section for encoding data and limits.® 64 and IA-32 Architectures Soft-See "SGDT-Store Global Descriptor Table Register" in Chapter 4, of the Intel

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  - #GP(0) - If the current privilege level is not 0.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective ad
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #UD - If the LOCK prefix is used.
  - #GP - If the current privilege level is not 0.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the current privilege level is not 0.
  > If the memory address is in a non-canonical form.

## Operation

```C
IF Instruction is LIDTTHEN= IF OperandSize 16THEN IDTR(Limit) := SRC[0:15];IDTR(Base) := SRC[16:47] AND 00FFFFFFH; ELSE IF 32-bit Operand SizeTHENIDTR(Limit) := SRC[0:15];IDTR(Base) := SRC[16:47]; FI;ELSE IF 64-bit Operand Size (* In 64-Bit Mode *)THENIDTR(Limit) := SRC[0:15];IDTR(Base) := SRC[16:79]; FI;FI;ELSE (* Instruction is LGDT *)= 16IF OperandSize THEN GDTR(Limit) := SRC[0:15];GDTR(Base) := SRC[16:47] AND 00FFFFFFH; ELSE IF 32-bit Operand SizeTHENGDTR(Limit) := SRC[0:15];GDTR(Base) := SRC[16:47]; FI;ELSE IF 64-bit Operand Size (* In 64-Bit Mode *)THENGDTR(Limit) := SRC[0:15];GDTR(Base) := SRC[16:79]; FI;FI; FI;
```
