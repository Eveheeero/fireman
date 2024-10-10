# SETcc

Set Byte on Condition

Sets the destination operand to 0 or 1 depending on the settings of the status flags (CF, SF, OF, ZF, and PF) in the EFLAGS register.
The destination operand points to a byte register or a byte in memory.
The condition code suffix (cc) indicates the condition being tested for.
The terms "above" and "below" are associated with the CF flag and refer to the relationship between two unsigned integer values.
The terms "greater" and "less" are associated with the SF and OF flags and refer to the relationship between two signed integer values.Many of the SETcc instruction opcodes have alternate mnemonics.
For example, SETG (set byte if greater) and SETNLE (set if not less or equal) have the same opcode and test for the same condition: ZF equals 0 and SF equals OF.
These alternate mnemonics are provided to make code more intelligible.
Appendix B, "EFLAGS Condition ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, shows the alternate Codes," in the Intelmnemonics for various test conditions.Some languages represent a logical one as an integer with all bits set.
This representation can be obtained by choosing the logically opposite condition for the SETccThe reg field of the ModR/M byte is not used for the SETCC instruction and those opcode bits are ignored by the processor.In IA-64 mode, the operand size is fixed at 8 bits.
Use of REX prefix enable uniform addressing to additional byte registers.
Otherwise, this instruction's operation is the same as in legacy mode and compatibility mode.


## Flags affected

- None.

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
- Protected Mode Exceptions
  - #GP(0) - If the destination is located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF conditionTHEN DEST := 1; ELSE DEST := 0; FI;
```
