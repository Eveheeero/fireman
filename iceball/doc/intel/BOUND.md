# BOUND

Check Array Index Against Bounds

BOUND determines if the first operand (array index) is within the bounds of an array specified the second operand (bounds operand).
The array index is a signed integer located in a register.
The bounds operand is a memory loca-tion that contains a pair of signed doubleword-integers (when the operand-size attribute is 32) or a pair of signed word-integers (when the operand-size attribute is 16).
The first doubleword (or word) is the lower bound of the array and the second doubleword (or word) is the upper bound of the array.
The array index must be greater than or equal to the lower bound and less than or equal to the upper bound plus the operand size in bytes.
If the index is not within bounds, a BOUND range exceeded exception (#BR) is signaled.
When this exception is generated, the saved return instruction pointer points to the BOUND instruction.The bounds limit data structure (two words or doublewords containing the lower and upper limits of the array) is usually placed just before the array itself, making the limits addressable via a constant offset from the beginning of the array.
Because the address of the array already will be present in a register, this practice avoids extra bus cycles to obtain the effective address of the array bounds.This instruction executes as described in compatibility mode and legacy mode.
It is not valid in 64-bit mode.

## Exceptions

- Protected Mode Exceptions
  - #BR - If the bounds test fails.
  - #UD - If second operand is not a memory location.
  > If the LOCK prefix is used.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
- Virtual-8086 Mode Exceptions
  - #BR - If the bounds test fails.
  - #UD - If second operand is not a memory location.
  > If the LOCK prefix is used.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #BR - If the bounds test fails.
  - #UD - If second operand is not a memory location.
  > If the LOCK prefix is used.
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.

## Operation

```C
IF 64bit ModeTHEN#UD;ELSEIF (ArrayIndex < LowerBound OR ArrayIndex > UpperBound) THEN(* Below lower bound or above upper bound *)IF <equation for PL enabled> THEN BNDSTATUS := 0#BR; FI;FI;
```
