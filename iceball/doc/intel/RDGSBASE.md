# RDFSBASE/RDGSBASE

Read FS/GS Segment Base

Loads the general-purpose register indicated by the ModR/M:r/m field with the FS or GS segment base address.The destination operand may be either a 32-bit or a 64-bit general-purpose register.
The REX.W prefix indicates the operand size is 64 bits.
If no REX.W prefix is used, the operand size is 32 bits; the upper 32 bits of the source base address (for FS or GS) are ignored and upper 32 bits of the destination register are cleared.
This instruction is supported only in 64-bit mode.

## Flags affected

- None.C/C++ Compiler Intrinsic EquivalentRDFSBASE unsigned int _readfsbase_u32(void );RDFSBASE unsigned __int64 _readfsbase_u64(void );RDGSBASE unsigned int _readgsbase_u32(void );RDGSBASE unsigned __int64 _readgsbase_u64(void );

## Exceptions

- Compatibility Mode Exceptions
- Protected Mode Exceptions
  - #UD - The RDFSBASE and RDGSBASE instructions
  >  are not recognized in protected mode.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - The RDFSBASE and RDGSBASE instructions
  >  are not recognized in real-address mode.
- Virtual-8086 Mode Exceptions
  - #UD - The RDFSBASE and RDGSBASE instructions
  >  are not recognized in virtual-8086 mode.

## Operation

```C
DEST := FS/GS segment base address;
```
