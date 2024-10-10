# MOVS/MOVSB/MOVSW/MOVSD/MOVSQ

Move Data From StringtoString

Moves the byte, word, or doubleword specified with the second operand (source operand) to the location specified with the first operand (destination operand).
Both the source and destination operands are located in memory.
The address of the source operand is read from the DS:ESI or the DS:SI registers (depending on the address-size attri-bute of the instruction, 32 or 16, respectively).
The address of the destination operand is read from the ES:EDI or the ES:DI registers (again depending on the address-size attribute of the instruction).
The DS segment may be overridden with a segment override prefix, but the ES segment cannot be overridden.At the assembly-code level, two forms of this instruction are allowed: the "explicit-operands" form and the "no-operands" form.
The explicit-operands form (specified with the MOVS mnemonic) allows the source and destination operands to be specified explicitly.
Here, the source and destination operands should be symbols that indicate the size and location of the source value and the destination, respectively.
This explicit-operands form is provided to allow documentation; however, note that the documentation provided by this form can be misleading.
That is, the source and destination operand symbols must specify the correct type (size) of the operands (bytes, words, or doublewords), but they do not have to specify the correct location.
The locations of the source and destination operands are always specified by the DS:(E)SI and ES:(E)DI registers, which must be loaded correctly before the move string instruction is executed.
The no-operands form provides "short forms" of the byte, word, and doubleword versions of the MOVS instructions.
Here also DS:(E)SI and ES:(E)DI are assumed to be the source and destination operands, respectively.
The size of the source and destination operands is selected with the mnemonic: MOVSB (byte move), MOVSW (word move), or MOVSD (doubleword move).After the move operation, the (E)SI and (E)DI registers are incremented or decremented automatically according to the setting of the DF flag in the mented; if the DF flag is 1, the (E)SI and (E)DI registers are decremented.) The registers are incremented or decremented by 1 for byte operations, by 2 for word operations, or by 4 for doubleword operations.NOTETo improve performance, more recent processors support modifications to the processor's operation during the string store operations initiated with MOVS and MOVSB.
See Section 7.3.9.3 ®in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for additional information on fast-string operation.The MOVS, MOVSB, MOVSW, and MOVSD instructions can be preceded by the REP prefix (see "REP/REPE/REPZ /REPNE/REPNZ-Repeat String Operation Prefix" for a description of the REP prefix) for block moves of ECX bytes, words, or doublewords.In 64-bit mode, the instruction's default address size is 64 bits, 32-bit address size is supported using the prefix 67H.
The 64-bit addresses are specified by RSI and RDI; 32-bit address are specified by ESI and EDI.
Use of the REX.W prefix promotes doubleword operation to 64 bits.
See the summary chart at the beginning of this section for encoding data and limits.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the destination is located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.

## Operation

```C
DEST := SRC;Non-64-bit Mode:IF (Byte move)= 0THEN IF DF THEN (E)SI := (E)SI + 1; (E)DI := (E)DI + 1; ELSE (E)SI := (E)SI - 1; (E)DI := (E)DI - 1; FI;ELSE IF (Word move)= 0THEN IF DF (E)SI := (E)SI + 2; (E)DI := (E)DI + 2; FI;ELSE (E)SI := (E)SI - 2; (E)DI := (E)DI - 2; FI;ELSE IF (Doubleword move)= 0THEN IF DF (E)SI := (E)SI + 4; (E)DI := (E)DI + 4; FI;ELSE (E)SI := (E)SI - 4; (E)DI := (E)DI - 4; FI;FI;64-bit Mode:IF (Byte move)= 0THEN IF DF THEN (R|E)SI := (R|E)SI + 1; (R|E)DI := (R|E)DI ELSE (R|E)SI := (R|E)SI - 1; (R|E)DI := (R|E)DI - 1; FI;ELSE IF (Word move)= 0THEN IF DF (R|E)SI := (R|E)SI + 2; (R|E)DI := (R|E)DI + 2; FI;ELSE (R|E)SI := (R|E)SI - 2; (R|E)DI := (R|E)DI - 2; FI;ELSE IF (Doubleword move)= 0THEN IF DF (R|E)SI := (R|E)SI + 4; (R|E)DI := (R|E)DI + 4; FI;ELSE (R|E)SI := (R|E)SI - 4; (R|E)DI := (R|E)DI - 4; FI;ELSE IF (Quadword move) = 0THEN IF DF(R|E)SI := (R|E)SI + 8; (R|E)DI := (R|E)DI + 8;FI;ELSE (R|E)SI := (R|E)SI - 8; (R|E)DI := (R|E)DI - 8; FI;FI;
```
