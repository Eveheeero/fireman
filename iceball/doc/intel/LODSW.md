# LODS/LODSB/LODSW/LODSD/LODSQ

Load String

Loads a byte, word, or doubleword from the source operand into the AL, AX, or EAX register, respectively.
The source operand is a memory location, the address of which is read from the DS:ESI or the DS:SI registers (depending on the address-size attribute of the instruction, 32 or 16, respectively).
The DS segment may be over-ridden with a segment override prefix.At the assembly-code level, two forms of this instruction are allowed: the "explicit-operands" form and the "no-operands" form.
The explicit-operands form (specified with the LODS mnemonic) allows the source operand to be specified explicitly.
Here, the source operand should be a symbol that indicates the size and location of the source value.
The destination operand is then automatically selected to match the size of the source operand (the AL register for byte operands, AX for word operands, and EAX for doubleword operands).
This explicit-operands form is provided to allow documentation; however, note that the documentation provided by this form can be misleading.
That is, the source operand symbol must specify the correct type (size) of the operand (byte, word, or doubleword), but it does not have to specify the correct location.
The location is always specified by the DS:(E)SI registers, which must be loaded correctly before the load string instruction is executed.The no-operands form provides "short forms" of the byte, word, and doubleword versions of the LODS instructions.
Here also DS:(E)SI is assumed to be the source operand and the AL, AX, or EAX register is assumed to be the desti-nation operand.
The size of the source and destination operands is selected with the mnemonic: LODSB (byte loaded into register AL), LODSW (word loaded into AX), or LODSD (doubleword loaded into EAX).After the byte, word, or doubleword is transferred from the memory location into the AL, AX, or EAX register, the (E)SI register is incremented or decremented automatically according to the setting of the DF flag in the EFLAGS register.
(If the DF flag is 0, the (E)SI register is incremented; if the DF flag is 1, the ESI register is decremented.) The (E)SI register is incremented or decremented by 1 for In 64-bit mode, use of the REX.W prefix promotes operation to 64 bits.
LODS/LODSQ load the quadword at address (R)SI into RAX.
The (R)SI register is then incremented or decremented automatically according to the setting of the DF flag in the EFLAGS register.
The LODS, LODSB, LODSW, and LODSD instructions can be preceded by the REP prefix for block loads of ECX bytes, words, or doublewords.
More often, however, these instructions areused within a LOOP construct because further processing of the data moved into the register is usually necessary before the next transfer can be made.
See ® 64 and IA-32 Archi-"REP/REPE/REPZ /REPNE/REPNZ-Repeat String Operation Prefix" in Chapter 4 of the Inteltectures Software Developer's Manual, Volume 2B, for a description of the REP prefix.

## Flags affected

- None.

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF AL := SRC; (* Byte load *)THEN AL := SRC; (* Byte load *)= IF DF 0THEN (E)SI := (E)SI + 1; ELSE (E)SI := (E)SI - 1; FI;ELSE IF AX := SRC; (* Word load *)= THEN IF DF 0THEN (E)SI := (E)SI + 2; ELSE (E)SI := (E)SI - 2; IF;FI;ELSE IF EAX := SRC; (* Doubleword load *) = THEN IF DF0THEN (E)SI := (E)SI + 4; ELSE (E)SI := (E)SI - 4; FI;FI;ELSE IF RAX := SRC; (* Quadword load *)=THEN IF DF  0THEN (R)SI := (R)SI + 8; ELSE (R)SI := (R)SI - 8; FI;FI;FI;
```
