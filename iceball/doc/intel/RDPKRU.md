# RDPKRU

Read Protection Key Rights for User Pages

Reads the value of PKRU into EAX and clears EDX.
ECX must be 0 when RDPKRU is executed; otherwise, a general-protection exception (#GP) occurs.RDPKRU can be executed only if CR4.PKE= 1; otherwise, an invalid-opcode exception (#UD) occurs.
Software can discover the value of CR4.PKE by examining CPUID.(EAX=07H,ECX=0H):ECX.OSPKE [bit 4].On processors that support the Intel64 Architecture, the high-order 32-bits of RCX are ignored and the high-order 32-bits of RDX and RAX are cleared.

## Flags affected

- None.C/C++ Compiler Intrinsic EquivalentRDPKRU uint32_t _rdpkru_u32(void);

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  > B
  - #GP(0) - If ECX
  >  0. 
  - #UD - If the LOCK prefix is used.
  > If CR4.PKE = 0.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF (ECX = 0) THENEAX := PKRU;EDX := 0;ELSE #GP(0); FI;
```
