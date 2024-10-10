# NOP

No Operation

This instruction performs no operation.
It is a one-byte or multi-byte NOP that takes up space in the instruction stream but does not impact machine context, except for the EIPregister.The multi-byte form of NOP is available on processors with model encoding: - CPUID.01H.EAX[Bytes 11:8] = 0110B or 1111BThe multi-byte NOP instruction does not alter the content of a register and will not issue a memory operation.
The instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Flags affected

- None.

## Operation

```C
The one-byte NOP instruction is an alias mnemonic for the XCHG (E)AX, (E)AX instruction.The multi-byte NOP instruction performs no operation on supported processors and generates undefined opcode exception on processors that do not support the multi-byte NOP instruction.The memory operand form of the instruction allows software to create a byte sequence of "no operation" as one instruction. For situations where multiple-byte NOPs are needed, the recommended operations (32-bit mode and 64-bit mode) are: Table 4-12.  Recommended Multi-Byte Sequence of NOP InstructionLengthAssemblyByte Sequence2 bytes66 NOP66 90H3 bytesNOP DWORD ptr [EAX]0F 1F 00H4 bytesNOP DWORD ptr [EAX + 00H]0F 1F 40 00H5 bytesNOP DWORD ptr [EAX + EAX*1 + 00H]0F 1F 44 00 00H6 bytes66 NOP DWORD ptr [EAX + EAX*1 + 00H]66 0F 1F 44 00 00H7 bytesNOP DWORD ptr [EAX + 00000000H]0F 1F 80 00 00 00 00H8 bytesNOP DWORD ptr [EAX + EAX*1 + 00000000H]0F 1F 84 00 00 00 00 00H9 bytes66 NOP DWORD ptr [EAX + EAX*1 + 00000000H]66 0F 1F 84 00 00 00 00 00H
```
