# IN

Input From Port

Copies the value from the I/O port specified with the second operand (source operand) to the destination operand (first operand).
The source operand can be a byte-immediate or the DX register; the destination operand can be register AL, AX, or EAX, depending on the size of the port being accessed (8, 16, or 32 bits, respectively).
Using the DX register as a source operand allows I/O port addresses from 0 to 65,535 to be accessed; using a byte immediate allows I/O port addresses 0 to 255 to be accessed.When accessing an 8-bit I/O port, the opcode determines the port size; when accessing a 16- and 32-bit I/O port, the operand-size attribute determines the port size.
At the machine code level, I/O instructions are shorter when accessing 8-bit I/O ports.
Here, the upper eight bits of the port address will be 0.This instruction is only useful for accessing I/O ports located in the processor's I/O address space.
See Chapter 19, ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for more infor-"Input/Output," in the Intelmation on accessing I/O ports in the I/O address space.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- 64-Bit Mode Exceptions
  - #GP(0) - If the CPL is greater than (has less privileg
  > e) the I/O privilege level (IOPL) and any of the 
  > corresponding I/O permission bits in TS
  > S for the I/O port being accessed is 1.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If any of the I/O permission bits in
  > the TSS for the I/O port being accessed is 1.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #GP(0) - If the CPL is greater than (has less privileg
  > e) the I/O privilege level (IOPL) and any of the 
  > corresponding I/O permission bits in TS
  > S for the I/O port being accessed is 1.
  - #PF(fault-code) - If a page fault occurs.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.

## Operation

```C
==IF ((PE  1) and ((CPL > IOPL) or (VM  1)))THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)=IF (Any I/O Permission Bit for I/O port being accessed  1)THEN (* I/O operation is not allowed *)#GP(0);ELSE ( * I/O operation is allowed *) DEST := SRC; (* Read from selected I/O port *)FI;ELSE (Real Mode or Protected Mode with CPL  IOPL *)DEST := SRC; (* Read from selected I/O port *)FI;
```
