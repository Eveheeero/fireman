# OUT

Output to Port

EnModeLeg ModeE6 ibOUT imm8, ALIValidValidOutput byte in AL to I/O port address imm8.E7 ibOUT imm8, AXIValidValidOutput word in AX to I/O port address imm8.
E7 ibOUT imm8, EAXIValidValidOutput doubleword in EAX to I/O port address imm8.EEOUT DX, ALZOValidValidOutput byte in AL to I/O port address in DX.EFOUT DX, AXZOValidValidOutput word in AX to I/O port address in DX.EFOUT DX, EAXZOValidValidOutput doubleword in EAX to I/O port address in DX.NOTES:1.
See the IA-32 Architecture Compatibility section below.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3Operand 4Iimm8N/AN/AN/AZON/AN/AN/AN/ACopies the value from the second operand (source operand) to the I/O port specified with the destination operand (first operand).
The source operand can be register AL, AX, or EAX, depending on the size of the port being accessed (8, 16, or 32 bits, respectively); the destination operand can be a byte-immediate or the DX register.
Using a byte immediate allows I/O port addresses 0 to 255 to be accessed; using the DX register as a source operand allows I/O ports from 0 to 65,535 to be accessed.The size of the I/O port being accessed is determined by the opcode for an 8-bit I/O port or by the operand-size attribute of the instruction for a 16- or 32-bit I/O port.At the machine code level, I/O instructions are shorter when accessing 8-bit I/O ports.
Here, the upper eight bits of the port address will be 0.This instruction is only useful for accessing I/O ports located in the processor's I/O address space.
See Chapter 19, ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for more infor-"Input/Output," in the Intelmation on accessing I/O ports in the I/O address space.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture Compatibility®After executing an OUT instruction, the Pentium processor ensures that the EWBE# pin has been sampled active before it begins to execute the next instruction.
(Note that the instruction can be prefetched if EWBE# is not active, but it will not be executed until the EWBE# pin is samp

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the CPL is greater than (has less privileg
  > e) the I/O privilege level (IOPL) and any of the 
  > corresponding I/O permission bits in TS
  > S for the I/O port being accessed is 1.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same as protected mode exceptions.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If any of the I/O permission bits in
  > the TSS for the I/O port being accessed is 1.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.

## Operation

```C
==IF ((PE  1) and ((CPL > IOPL) or (VM  1)))THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)= IF (Any I/O Permission Bit for I/O port being accessed 1)THEN (* I/O operation is not allowed *)#GP(0);ELSE ( * I/O operation is allowed *) DEST := SRC; (* Writes to selected I/O port *)FI;ELSE (Real Mode or Protected Mode with CPL  IOPL *)DEST := SRC; (* Writes to selected I/O port *)FI;
```
