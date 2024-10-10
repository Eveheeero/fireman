# LOOP/LOOPcc

Loop According to ECX Counter

Performs a loop operation using the RCX, ECX or CX register as a counter (depending on whether address size is 64 bits, 32 bits, or 16 bits).
Note that the LOOP instruction ignores REX.W; but 64-bit address size can be over-ridden using a 67H prefix.Each time the LOOP instruction is executed, the count register is decremented, then checked for 0.
If the count is 0, the loop is terminated and program execution continues with the instruction following the LOOP instruction.
If the count is not zero, a near jump is performed to the destination (target) operand, which is presumably the instruction at the beginning of the loop.The target instruction is specified with a relative offset (a signed offset relative to the current value of the instruc-tion pointer in the IP/EIP/RIP register).
This offset is generally specified as a label in assembly code, but at the machine code level, it is encoded as a signed, 8-bit immediate value, which is added to the instruction pointer.
Offsets of -128 to +127 are allowed with this instruction.Some forms of the loop instruction (LOOPcc) also accept the ZF flag as a condition for terminating the loop before the count reaches zero.
With these forms of the instruction, a condition code (cc) is associated with each instruction to indicate the condition being tested for.
Here, the LOOPcc instruction itself does not affect the state of the ZF flag; the ZF flag is changed by other instructions in the loop.

## Flags affected

- None.

## Exceptions

- Real-Address Mode Exceptions
  - #GP - If the offset being jumped to is beyond the limi
  > ts of the CS segment or is outside of the effec-
  > tive address space from 0 to FFFFH. This conditio
  > n can occur if a 32-bit address size override 
  > prefix is used.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
- Protected Mode Exceptions
  - #GP(0) - If the offset being jumped to is
  >  beyond the limits of the CS segment.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions

## Operation

```C
=IF (AddressSize  32)THEN Count is ECX;ELSE IF (AddressSize = 64)Count is RCX;ELSE Count is CX; FI;Count := Count - 1;IF Instruction is not LOOPTHENIF (Instruction := LOOPE) or (Instruction := LOOPZ)THEN IF (ZF =  1) and (Count 0)THEN BranchCond := 1;ELSE BranchCond := 0;FI;== LOOPNE) or (Instruction  LOOPNZ)ELSE (Instruction =IF (ZF  0 ) and (Count  0)THEN BranchCond := 1;FI;=ELSE (* Instruction  LOOP *) IF (Count 0)THEN BranchCond := 1;ELSE BranchCond := 0;FI;FI;= 1IF BranchCond THENIF in 64-bit mode (* OperandSize = 64 *)THENtempRIP := RIP + SignExtend(DEST);IF tempRIP is not canonicalTHEN #GP(0);ELSE RIP := tempRIP;FI;ELSEtempEIP := EIP   SignExtend(DEST);IF OperandSize   16THEN tempEIP := tempEIP AND 0000FFFFH;FI;IF tempEIP is not within code segment limitTHEN #GP(0);ELSE EIP := tempEIP;FI;FI;ELSETerminate loop and continue program execution at (R/E)IP;FI;
```
