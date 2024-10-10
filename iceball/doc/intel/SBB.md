# SBB

Integer Subtraction With Borrow

Adds the source operand (second operand) and the carry (CF) flag, and subtracts the result from the destination operand (first operand).
The result of the subtraction is stored in the destination operand.
The destination operand can be a register or a memory location; the source operan(However, two memory operands cannot be used in one instruction.) The state of the CF flag represents a borrow from a previous subtraction.When an immediate value is used as an operand, it is sign-extended to the length of the destination operand format.The SBB instruction does not distinguish between signed or unsigned operands.
Instead, the processor evaluates the result for both data types and sets the OF and CF flags to indicate a borrow in the signed or unsigned result, respectively.
The SF flag indicates the sign of the signed result.The SBB instruction is usually executed as part of a multibyte or multiword subtraction in which a SUB instruction is followed by a SBB instruction.This instruction can be used with a LOCK prefix to allow the instruction to be executed atomically.In 64-bit mode, the instruction's default operation size is 32 bits.
Using a REX prefix in the form of REX.R permits access to additional registers (R8-R15).
Using a REX prefix in the form of REX.W promotes operation to 64 bits.
See the summary chart at the beginning of this section for encoding data and limits.

## Flags affected

- The OF, SF, ZF, AF, PF, and CF flags are set according to the result.

## Exceptions

- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used but the destination is not a memory operand.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
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
  - #UD - If the LOCK prefix is used but the destination is not a memory operand.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an

## Operation

```C
DEST := (DEST - (SRC + CF));Intel C/C++ Compiler Intrinsic EquivalentSBB extern unsigned char _subborrow_u8(unsigned char c_in, unsigned char src1, unsigned char src2, unsigned char *diff_out);SBB extern unsigned char _subborrow_u16(unsigned char c_in, unsigned short src1, unsigned short src2, unsigned short *diff_out);SBB extern unsigned char _subborrow_u32(unsigned char c_in, unsigned int src1, unsigned char int, unsigned int *diff_out);SBB extern unsigned char _subborrow_u64(unsigned char c_in, unsigned __int64 src1, unsigned __int64 src2, unsigned __int64 *diff_out);
```
