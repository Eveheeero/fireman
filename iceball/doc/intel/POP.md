# POP

Pop a Value From the Stack

Loads the value from the top of the stack to the location specified with the destination operand (or explicit opcode) and then increments the stack pointer.
The destination operand can be a general-purpose register, memory loca-tion, or segment register.Address and operand sizes are determined and used as follows: - Address size.
The D flag in the current code-segment descriptor determines the default address size; it may be overridden by an instruction prefix (67H).The address size is used only when writing to a destination operand in memory.
- Operand size.
The D flag in the current code-segment descriptor determines the default operand size; it may be overridden by instruction prefixes (66H or REX.W).The operand size (16, 32, or 64 bits) determines the amount by which the stack pointer is incremented (2, 4or 8).
- Stack-address size.
Outside of 64-bit mode, the B flag in the current stack-segment descriptor determines the size of the stack pointer (16 or 32 bits); in 64-bit mode, the size of the stack pointer is always 64 bits.The stack-address size determines the width of the stack pointer when reading from the stack in memory andwhen incrementing the stack pointer.
(As stated above, the amount by which the stack pointer is incrementedis determined by the operand size.)If the destination operand is one of the segment registers DS, ES, FS, GS, or SS, the value loaded into the register ically causes the descriptor information associated with that segment selector to be loaded into the hidden (shadow) part of the segment register and causes the selector and the descriptor information to be validated (see the "Operation" section below).A NULL value (0000-0003) may be popped into the DS, ES, FS, or GS register without causing a general protection fault.
However, any subsequent attempt to reference a segment whose corresponding segment register is loaded with a NULL value causes a general protection exception (#GP).
In this situation, no memory reference occurs and the saved value of the segment register is NULL.The POP instruction cannot pop a value into the CS register.
To load the CS register from the stack, use the RET instruction.If the ESP register is used as a base register for addressing a destination operand in memory, the POP instruction computes the effective address of the operand after it increments the ESP register.
For the case of a 16-bit stack where ESP wraps to 0H as a result of the POP instruction, the resulting location of the memory write is processor-family-specific.The POP ESP instruction increments the stack pointer (ESP) before data at the old top of stack is written into the destination.Loading the SS register with a POP instruction suppresses or inhibits some debug exceptions and inhibits interrupts on the following instruction boundary.
(The inhibition ends after delivery of an exception or the execution of the next instruction.) This behavior allows a stack pointer to be loaded into the ESP register with the next instruction (POP ESP) before an event can be delivered.
See Section 6.8.3, "Masking Exceptions and Interrupts When ® 64 and IA-32 Architectures Software Developer's Manual, Volume 3A.
Intel recom-Switching Stacks," in the Intelmends that software use the LSS instruction to load the SS register and ESP together.In 64-bit mode, using a REX prefix in the form of REX.R permits access to additional registers (R8-R15).
When in 64-bit mode, POPs using 32-bit operands are not encodable and POPs to DS, ES, SS are not valid.
See the summary chart at the beginning of this section for encoding data and limits.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If attempt is made to load SS
  > register with NULL segment selector.
  > If the destination operand is in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #GP(selector) - If segment selector index is outside descriptor table limits.
  > If the SS register is being loaded and the se
  > gment selector's RPL and the segment descriptor's 
  > DPL are not equal to the CPL. 
  > If the SS register is being loaded
  >  and the segment pointed to is a
  > non-writable data segment.
  > If the DS, ES, FS, or GS register is being load
  > ed and the segment pointed to is not a data or 
  > readable code segment.
  > If the DS, ES, FS, or GS register is being lo
  > aded and the segment pointed to is a data or 
  > nonconforming code segment, but both the 
  > RPL and the CPL are greater than the DPL.
  - #SS(0) - If the current top of stack is not within the stack segment.
  > If a memory operand effective address is outside the SS segment limit.
  - #SS(selector) - If the SS register is being loaded and the segment pointed to is marked not present.
  - #NP - If the DS, ES, FS, or GS register is being lo
  > aded and the segment pointed to is marked not 
  > present.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is made wh
  > ile the current privilege level is 3 and alignment 
- 64-Bit Mode Exceptions
  - #GP(0) - If the memory address is in a non-canonical form.
  - #SS(0) - If the stack address is in a non-canonical form.
  - #GP(selector) - If the descriptor is outside the descriptor table limit.
  > If the FS or GS register is being loaded and th
  > e segment pointed to is not a data or readable 
  > code segment.
  > If the FS or GS register is being loaded and th
  > e segment pointed to is a data or nonconforming 
  > code segment, but both the RPL an
  > d the CPL are greater than the DPL.
  - #AC(0) - If an unaligned memory reference is
  > made while alignment checking is enabled.
  - #PF(fault-code) - If a page fault occurs.
  - #NP - If the FS or GS register is being loaded
  > and the segment pointed to is marked not present.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is
  > made while alignment checking is enabled.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same as for protected mode exceptions.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #UD - If the LOCK prefix is used.

## Operation

```C
IF StackAddrSize = 32THENIF OperandSize = 32THENDEST := SS:ESP; (* Copy a doubleword *)ESP := ESP + 4;= 16*)ELSE (* OperandSize DEST := SS:ESP; (* Copy a word *)ESP := ESP + 2;FI;ELSE IF StackAddrSize = 64THEN = 64IF OperandSizeTHENDEST := SS:RSP; (* Copy quadword *)RSP := RSP + 8;ELSE (* OperandSize = 16*)DEST := SS:RSP; (* Copy a word *)RSP := RSP + 2;FI;FI; = 16ELSE StackAddrSizeTHEN IF OperandSize= 16THENDEST := SS:SP; (* Copy a word *)SP := SP ELSE (* OperandSize = 32 *)DEST := SS:SP; (* Copy a doubleword *)SP := SP + 4;FI;FI;Loading a segment register while in protected mode results in special actions, as described in the following listing. These checks are performed on the segment selector and the segment descriptor it points to.64-BIT_MODEIF FS, or GS is loaded with non-NULL selector;THENIF segment selector index is outside descriptor table limitsOR segment is not a data or readable code segmentOR ((segment is a data or nonconforming code segment)AND ((RPL > DPL) or (CPL > DPL))THEN #GP(selector);IF segment not marked presentTHEN #NP(selector);ELSESegmentRegister := segment selector;SegmentRegister := segment descriptor;FI;FI;IF FS, or GS is loaded with a NULL selector;THENSegmentRegister := segment selector;SegmentRegister := segment descriptor;FI;PREOTECTED MODE OR COMPATIBILITY MODE;IF SS is loaded;THENIF segment selector is NULLTHEN #GP(0); FI;IF segment selector index is outside descriptor table limits  CPLor segment selector's RPL or segment is not a writable data segment or DPL CPLTHEN #GP(selector); FI;IF segment not marked present THEN #SS(selector); ELSESS := segment selector;SS := segment descriptor; FI;FI;IF DS, ES, FS, or GS is loadIF segment selector index is outside descriptor table limitsor segment is not a data or readable code segmentor ((segment is a data or nonconforming code segment)and ((RPL > DPL) or (CPL > DPL))THEN #GP(selector); FI;IF segment not marked presentTHEN #NP(selector);ELSESegmentRegister := segment selector;SegmentRegister := segment descriptor; FI;FI;IF DS, ES, FS, or GS is loaded with a NULL selectorTHENSegmentRegister := segment selector;SegmentRegister := segment descriptor;FI;
```
