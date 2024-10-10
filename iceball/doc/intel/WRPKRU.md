# WRPKRU

Write Data to User Page Key Register

Writes the value of EAX into PKRU.
ECX and EDX must be 0 when WRPKRU is executed; otherwise, a general-protection exception (#GP) occurs.WRPKRU can be executed only if CR4.PKE= 1; otherwise, an invalid-opcode exception (#UD) occurs.
Software can discover the value of CR4.PKE by examining CPUID.(EAX=07H,ECX=0H):ECX.OSPKE [bit 4].On processors that support the Intel64 Architecture, the high-order 32-bits of RCX, RDX, and RAX are ignored.WRPKRU will never execute speculatively.
Memory accesses affected by PKRU register will not execute (even speculatively) until all prior executions of WRPKRU have completed execution and updated the PKRU register.

## Flags affected

- None.C/C++ Compiler Intrinsic EquivalentWRPKRU void _wrpkru(uint32_t);

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  > B
  - #GP(0) - If ECX
  >  0.
  > B
  > If EDX 
  >  0.
  - #UD - If the LOCK prefix is used.
  > If CR4.PKE = 0.

## Operation

```C
=IF (ECX  0 AND EDX = 0) THEN PKRU := EAX;ELSE #GP(0); FI;
```
