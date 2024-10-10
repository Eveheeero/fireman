# Jcc

Jump if Condition Is Met

Checks the state of one or more of the status flags in the EFLAGS register (CF, OF, PF, SF, and ZF) and, if the flags are in the specified state (condition), performs a jump to the target instruction specified by the destination operand.
A condition code (cc) is associated with each instruction to indicate the condition being tested for.
If the condition is not satisfied, the jump is not performed and execution continues with the instruction following the Jcc instruction.
The target instruction is specified with a relative offset (a signed offset relative to the current value of the instruc-tion pointer in the EIP register).
A relative offset (rel8, rel16, or rel32) is generally specified as a label in assembly code, but at the machine code level, it is encoded as a signed, 8-bit or 32-bit immediate value, which is added to the instruction pointer.
Instruction coding is most efficient for offsets of -128 to +127.
If the operand-size attribute is 16, the upper two bytes of the EIP register are cleared, resulting in a maximum instruction pointer size of 16 bits.
The conditions for each Jcc mnemonic are given in the "Description" column of the table on the preceding page.
The terms "less" and "greater" are used for comparisons of signed integers and the terms "above" and "below" are used for unsigned integers.Because a particular state of the status flags can sometimes be interpreted in two ways, two mnemonics are defined for some opcodes.
For example, the JA (jump if above) instruction and the JNBE (jump if not below or equal) instruction are alternate mnemonics for the opcode 77H.The Jcc instruction does not support far jumps (jumps to other code segments).
When the target for the conditional jump is in a different segment, use the opposite condition from the condition being tested for the Jcc instruction, and then access the target with an unconditional far jump (JMP instruction) to the other segment.
For example, the following conditional far jump is illegal:JZ FARLABEL;To accomplish this far jump, use the following two instructions:JNZ BEYOND;JMP FARLABEL;BEYOND:The JRCXZ, JECXZ, and JCXZ instructions differ from other Jcc instructions because they do not check status flags.
Instead, they check RCX, ECX or CX for 0.
The register checked is determined by the address-size attribute.
These instructions are useful when used at the beginning of a loop that terminates with a conditional loop instruction (such as LOOPNE).
They can be used to prevent an instruction sequence from entering a loop when RCX, ECX or CX 6432, 2 or 64K times (not zero times).is 0.
This would cause the loop to execute 2All conditional jumps are converted to code fetches of one or two cache lines, regardless of jump address or cache-ability.In 64-bit mode, operand size is fixed at 64 bits.
JMP Short 

## Flags affected

- None.

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
- Protected Mode Exceptions
  - #GP(0) - If the offset being jumped to is
  >  beyond the limits of the CS segment.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #GP - If the offset being jumped to is beyond the limi
  > ts of the CS segment or is outside of the effec-
  > tive address space from 0 to FFFFH. This conditio
  > n can occur if a 32-bit address size override 
  > prefix is used.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions

## Operation

```C
IF conditionTHEN tempEIP := EIP + SignExtend(DEST); IF OperandSize = 16THEN tempEIP := tempEIP AND 0000FFFFH; FI;IF tempEIP is not within code segment limitTHEN #GP(0); ELSE EIP := tempEIP FI;FI;
```
