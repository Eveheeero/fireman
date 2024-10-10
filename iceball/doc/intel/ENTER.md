# ENTER

Make Stack Frame for Procedure Parameters

Creates a stack frame (comprising of space for dynamic storage and 1-32 frame pointer storage) for a procedure.
The first operand (imm16) specifies the size of the dynamic storage in the stack frame (that is, the number of bytes of dynamically allocated on the stack for the procedure).
The second operand (imm8) gives the lexical nesting level (0 to 31) of the procedure.
The nesting level (imm8 mod 32) and the OperandSize attribute determine the size in bytes of the storage space for frame pointers.The nesting level determines the number of frame pointers that are copied into the "display area" of the new stack frame from the preceding frame.
The default size of the frame pointer is the StackAddrSize attribute, but can be overridden using the 66H prefix.
Thus, the OperandSize attribute determines the size of each frame pointer that will be copied into the stack frame and the data being transferred from SP/ESP/RSP register into the BP/EBP/RBP register.The ENTER and companion LEAVE instructions are provided to support block structured languages.
The ENTER instruction (when used) is typically the first instruction in a procedure and is used to set up a new stack frame for a procedure.
The LEAVE instruction is then used at the end of the procedure (just before the RET instruction) to release the stack frame.If the nesting level is 0, the processor pushes the frame pointer from the BP/EBP/RBP register onto the stack, copies the current stack pointer from the SP/ESP/RSP register into the BP/EBP/RBP register, and loads the SP/ESP/RSP register with the current stack-pointer value minus the value in the size operand.
For nesting levels of 1 or greater, the processor pushes additional frame pointers on the stack before adjusting the stack pointer.
These additional frame pointers provide the called procedure with access points to other nested frames on the stack.
See ® 64 and IA-32 Architectures Software "Procedure Calls for Block-Structured Languages" in Chapter 6 of the IntelDeveloper's Manual, Volume 1, for more information about the actions of the ENTER instruction.The ENTER instruction causes a page fault whenever a write using the final value of the stack pointer (within the current stack segment) would do so.In 64-bit mode, default operation size is 64 bits; 32-bit operation size cannot be encoded.
Use of 66H prefix changes frame pointer operand size to 16 bits.When the 66H prefix is used and causing the OperandSize attribute to be less than the StackAddrSize, software is responsible for the following: - The companion LEAVE instruction must also use the 66H prefix, - The value in the RBP/EBP register prior to executing "66H ENTER" must be within the same 16KByte region of the current stack pointer (RSP/ESP), such that the valu

## Flags affected

- None.

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If the stack address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs or if a write usin
  > g the final value of the stack pointer (within the current 
- Real-Address Mode Exceptions
  - #SS - If the new value of the SP or ESP register is outside the stack segment limit.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #SS(0) - If the new value of the SP or ESP re
  > gister is outside the stack segment limit.
  - #PF(fault-code) - If a page fault occurs or if a write usin
  > g the final value of the stack pointer (within the current 
  > stack segment) would cause a page fault.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #SS(0) - If the new value of the SP or ESP re
  > gister is outside the stack segment limit.
  - #PF(fault-code) - If a page fault occurs or if a write usin
  > g the final value of the stack pointer (within the current 
  > stack segment) would cause a page fault.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
AllocSize := imm16;NestingLevel := imm8 MOD 32;IF (OperandSize = 64)THEN Push(RBP); (* RSP decrements by 8 *)FrameTemp := RSP; = 32ELSE IF OperandSize THEN Push(EBP); (* (E)SP decrements by 4 *)FrameTemp := ESP; FI;= 16 *)ELSE (* OperandSize Push(BP); (* RSP or (E)SP decrements by 2 *)FrameTemp := SP; FI; = 0IF NestingLevelTHEN GOTO CONTINUE;FI;IF (NestingLevel > 1) THEN FOR i := 1 to (NestingLevel - 1)DO IF (OperandSize = 64)THENRBP := RBP - 8;Push([RBP]); (* Quadword push *)ELSE IF OperandSize = 32THENIF StackSize = 32EBP := EBP - 4;Push([EBP]); (* Doubleword push *)ELSE (* StackSize = 16 *)BP := BP - 4;Push([BP]); (* Doubleword push *)FI;FI;ELSE (* OperandSize = 16 *)IF StackSize = 64THENRBP := RBP - 2;Push([RBP]); (* Word push *)ELSE IF StackSize = 32THENEBP := EBP - 2;Push([EBP]); (* Word push *)ELSE (* StackSize = 16 *)BP := BP - 2;Push([BP]); (* Word push *)FI;FI;OD;THENPush(FrameTemp); (* Quadword push and RSP decrements by 8 *)ELSE IF OperandSize = 32THEN Push(FrameTemp); FI; (* Doubleword push and (E)SP decrements by 4 *)ELSE (* OperandSize = 16 *)Push(FrameTemp); (* Word push and RSP|ESP|SP decrements by 2 *)FI;CONTINUE:IF 64-Bit Mode (StackSize = 64)THENRBP := FrameTemp;- AllocSize;RSP := RSP   =ELSE IF OperandSize32 THENEBP := FrameTemp;-ESP := ESP  AllocSize; FI; =ELSE (* OperandSize 16 *)BP := FrameTemp[15:1]; (* Bits 16 and above of applicable RBP/EBP are unmodified *)- SP := SP AllocSize;FI;END;
```
