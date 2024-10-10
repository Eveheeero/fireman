# XACQUIRE/XRELEASE

Hardware Lock Elision Prefix Hints

The XACQUIRE prefix is a hint to start lock elision on the memory address specified by the instruction and the XRELEASE prefix is a hint to end lock elision on the memory address specified by the instruction.The XACQUIRE prefix hint can only be used with the following instructions (these instructions are also referred to as XACQUIRE-enabled when used with the XACQUIRE prefix): - Instructions with an explicit LOCK prefix (F0H) prepended to forms of the instruction where the destination operand is a memory operand: ADD, ADC, AND, BTC, BTR, BTS, CMPXCHG, CMPXCHG8B, DEC, INC, NEG, NOT, OR, SBB, SUB, XOR, XADD, and XCHG.
 - The XCHG instruction either with or without the presence of the LOCK prefix.
The XRELEASE prefix hint can only be used with the following instructions (also referred to as XRELEASE-enabled when used with the XRELEASE prefix): - Instructions with an explicit LOCK prefix (F0H) prepended to forms of the instruction where the destination operand is a memory operand: ADD, ADC, AND, BTC, BTR, BTS, CMPXCHG, CMPXCHG8B, DEC, INC, NEG, NOT, OR, SBB, SUB, XOR, XADD, and XCHG.
 - The XCHG instruction either with or without the presence of the LOCK prefix.
 - The "MOV mem, reg" (Opcode 88H/89H) and "MOV mem, imm" (Opcode C6H/C7H) instructions.
In these cases, the XRELEASE is recognized without the presence of the LOCK prefix.® 64 and IA-32 Architectures Software Developer's The lock variables must satisfy the guidelines described in IntelManual, Volume 1, Section 16.3.3, for elision to be successful, otherwise an HLE abort may be signaled.If an encoded byte sequence that meets XACQUIRE/XRELEASE requirements includes both prefixes, then the HLE semantic is determined by the prefix byte that is placed closest to the instruction opcode.
For example, an F3F2C6 will not be treated as a XRELEASE-enabled instruction since the F2H (XACQUIRE) is closest to the instruction opcode C6.
Similarly, an F2F3F0 prefixed instruction wiIntel 64 and IA-32 CompatibilityThe effect of the XACQUIRE/XRELEASE prefix hint is the same in non-64-bit modes and in 64-bit mode.For instructions that do not support the XACQUIRE hint, the presence of the F2H prefix behaves the same way as prior hardware, according to - REPNE/REPNZ semantics for string instructions, - Serve as SIMD prefix for legacy SIMD instructions operating on XMM register - Cause #UD if prepending the VEX prefix.
- Undefined for non-string instructions or other situations.For instructions that do not support the XRELEASE hint, the presence of the F3H prefix behaves the same way as in prior hardware, according to - REP/REPE/REPZ semantics for string instructions, - Serve as SIMD prefix for legacy SIMD instructions operating on XMM register - Cause #UD if prepending the VEX prefix.
- Undefined for non-string instructions or other situations.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
XACQUIREIF XACQUIRE-enabled instructionTHENIF (HLE_NEST_COUNT < MAX_HLE_NEST_COUNT) THENHLE_NEST_COUNT++IF (HLE_NEST_COUNT = 1) THENHLE_ACTIVE := 1IF 64-bit modeTHEN restartRIP := instruction pointer of the XACQUIRE-enabled instructionELSErestartEIP := instruction pointer of the XACQUIRE-enabled instructionFI;Enter HLE Execution (* record register state, start tracking memory state *)FI; (* HLE_NEST_COUNT = 1*)IF ElisionBufferAvailable THENAllocate elision bufferRecord address and data for forwarding and commit checkingPerform elisionELSE Perform lock acquire operation transactionally but without elisionFI;ELSE (* HLE_NEST_COUNT = MAX_HLE_NEST_COUNT*)GOTO HLE_ABORT_PROCESSINGFI;ELSEXRELEASEIF XRELEASE-enabled instruction THENIF (HLE_NEST_COUNT > 0) THENHLE_NEST_COUNT--IF lock address matches in elision buffer THENIF lock satisfies address and value requirements THENDeallocate elision bufferELSEGOTO HLE_ABORT_PROCESSINGFI;FI;IF (HLE_NEST_COUNT = 0) THENIF NoAllocatedElisionBuffer THENTry to commit transactional executionIF fail to commit transactional execution THENGOTO HLE_ABORT_PROCESSING;ELSE (* commit success *)HLE_ACTIVE := 0FI;ELSEGOTO HLE_ABORT_PROCESSINGFI;FI;FI; (* HLE_NEST_COUNT > 0 *)ELSE Treat instruction as non-XRELEASE F3H prefixed legacy instructionFI;(* For any HLE abort condition encountered during HLE execution *)HLE_ABORT_PROCESSING: HLE_ACTIVE := 0HLE_NEST_COUNT := 0Restore architectural register stateDiscard memory updates performed in transactionFree any allocated lock elision buffersIF 64-bit modeTHEN RIP := restartRIPELSEEIP := restartEIPFI;
```
