# POPA/POPAD

Pop All General-Purpose Registers

Pops doublewords (POPAD) or words (POPA) from the stack into the general-purpose registers.
The registers are loaded in the following order: EDI, ESI, EBP, EBX, EDX, ECX, and EAX (if the operand-size attribute is 32) and DI, SI, BP, BX, DX, CX, and AX (if the operand-size attribute is 16).
(These instructions reverse the operation of the PUSHA/PUSHAD instructions.) The value on the stack for the ESP or SP register is ignored.
Instead, the ESP or SP register is incremented after each register is loaded.The POPA (pop all) and POPAD (pop all double) mnemonics reference the same opcode.
The POPA instruction is intended for use when the operand-size attribute is 16 and the POPAD instruction for when the operand-size attri-bute is 32.
Some assemblers may force the operand size to 16 when POPA is used and to 32 when POPAD is used (using the operand-size override prefix [66H] if necessary).
Others may treat these mnemonics as synonyms (POPA/POPAD) and use the current setting of the operand-size attribute to determine the size of values to be popped from the stack, regardless of the mnemonic used.
(The D flag in the current code segment's segment descriptor determines the operand-size attribute.)This instruction executes as described in non-64-bit modes.
It is not valid in 64-bit mode.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #SS(0) - If the starting or ending stack a
  > ddress is not within the stack segment. 
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is made wh
  > ile the current privilege level is 3 and alignment 
  > checking is enabled.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #SS - If the starting or ending stack address is not within the stack segment.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #SS(0) - If the starting or ending stack
  > address is not within the stack segment.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is
  > made while alignment checking is enabled.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same as for protected mode exceptions.

## Operation

```C
IF 64-Bit ModeTHEN#UD;ELSE POPAD *)IF OperandSize = 32 (* Instruction =THENEDI := Pop();ESI := Pop();EBP := Pop();Increment ESP by 4; (* Skip next 4 bytes of stack *)EBX := Pop();EDX := Pop();ECX := Pop();EAX := Pop();ELSE (* OperandSize = 16, instruction = POPA *)DI := Pop();SI := Pop();BP := Pop();Increment ESP by 2; (* Skip next 2 bytes of stack *)BX := Pop();DX := Pop();CX := Pop();AX := Pop();
```
