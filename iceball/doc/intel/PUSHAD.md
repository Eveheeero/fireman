# PUSHA/PUSHAD

Push All General-Purpose Registers

Pushes the contents of the general-purpose registers onto the stack.
The registers are stored on the stack in the following order: EAX, ECX, EDX, EBX, ESP (original value), EBP, ESI, and EDI (if the current operand-size attribute is 32) and AX, CX, DX, BX, SP (original value), BP, SI, and DI (if the operand-size attribute is 16).
These instruc-tions perform the reverse operation of the POPA/POPAD instructions.
The value pushed for the ESP or SP register is its value before prior to pushing the first register (see the "Operation" section below).The PUSHA (push all) and PUSHAD (push all double) mnemonics reference the same opcode.
The PUSHA instruc-tion is intended for use when the operand-size attribute is 16 and the PUSHAD instruction for when the operand-size attribute is 32.
Some assemblers may force the operand size to 16 when PUSHA is used and to 32 when PUSHAD is used.
Others may treat these mnemonics as synonyms (PUSHA/PUSHAD) and use the current setting of the operand-size attribute to determine the size of values to be pushed from the stack, regardless of the mnemonic used.In the real-address mode, if the ESP or SP register is 1, 3, or 5 when PUSHA/PUSHAD executes: an #SS exception is generated but not delivered (the stack error reported prevents #SS delivery).
Next, the processor generates a ® 64 and #DF exception and enters a shutdown state as described in the #DF discussion in Chapter 6 of the IntelIA-32 Architectures Software Developer's Manual, Volume 3A.This instruction executes as described in compatibility mode and legacy mode.
It is not valid in 64-bit mode.

## Flags affected

- None.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #SS(0) - If the starting or ending stack a
  > ddress is outside the stack segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is made wh
  > ile the current privilege level is 3 and alignment 
  > checking is enabled.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If the ESP or SP register contains 7, 9, 11, 13, or 15.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is
  > made while alignment checking is enabled.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If the ESP or SP register contains 7, 9, 11, 13, or 15.
  - #UD - If the LOCK prefix is used.

## Operation

```C
IF 64-bit Mode THEN #UDFI;  =32 (* PUSHAD instruction *)IF OperandSizeTHENTemp := (ESP);Push(EAX);Push(ECX);Push(EDX);Push(EBX);Push(Temp);Push(EBP);Push(ESI);Push(EDI); = 16, PUSHA instruction *)ELSE (* OperandSizeTemp := (SP);Push(AX);Push(CX);Push(DX);Push(BP);Push(SI);Push(DI);FI;
```
