# LOCK

Assert LOCK# Signal Prefix

EnModeLeg ModeF0LOCKZOValidValidAsserts LOCK# signal for duration of the accompanying instruction.NOTES:1.
See IA-32 Architecture Compatibility section below.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3Operand 4ZON/AN/AN/AN/ACauses the processor's LOCK# signal to be asserted during execution of the accompanying instruction (turns the instruction into an atomic instruction).
In a multiprocessor environment, the LOCK# signal ensures that the processor has exclusive use of any shared memory while the signal is asserted.In most IA-32 and all Intel 64 processors, locking may occur without the LOCK# signal being asserted.
See the "IA-32 Architecture Compatibility" section below for more details.The LOCK prefix can be prepended only to the following instructions and only to those forms of the instructions where the destination operand is a memory operand: ADD, ADC, AND, BTC, BTR, BTS, CMPXCHG, CMPXCH8B, CMPXCHG16B, DEC, INC, NEG, NOT, OR, SBB, SUB, XOR, XADD, and XCHG.
If the LOCK prefix is used with one of these instructions and the source operand is a memory operand, an undefined opcode exception (#UD) may be generated.
An undefined opcode exception will also be generated if the LOCK prefix is used with any instruction not in the above list.
The XCHG instruction always asserts the LOCK# signal regardless of the presence or absence of the LOCK prefix.The LOCK prefix is typically used with the BTS instruction to perform a read-modify-write operation on a memory location in shared memory environment.The integrity of the LOCK prefix is not affected by the alignment of the memory field.
Memory locking is observed for arbitrarily misaligned fields.This instruction's operation is the same in non-64-bit modes and 64-bit mode.IA-32 Architecture CompatibilityBeginning with the P6 family processors, when the LOCK prefix is prefixed to an instruction and the memory area being accessed is cached internally in the processor, the LOCK# signal is generally not asserted.
Instead, only the processor's cache is locked.
Here, the processor's cache coherency mechanism ensures that the operation is carried out atomically with regards to memory.
See "Effects of a Locked Operation on Internal Processor Caches" ® 64 and IA-32 Architectures Software Developer's Manual, Volume 3A, the for more informa-in Chapter 9 of Inteltion on locking of caches.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used with an instruction not listed: ADD, ADC, AND, BTC, BTR, BTS,
  > CMPXCHG, CMPXCH8B, CMPXCHG16B, DEC, INC, NEG, NOT, OR, SBB, SUB, XOR, XADD, 
  > XCHG.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
AssertLOCK#(DurationOfAccompaningInstruction);
```
