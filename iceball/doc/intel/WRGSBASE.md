# WRFSBASE/WRGSBASE

Write FS/GS Segment Base

Loads the FS or GS segment base address with the general-purpose register indicated by the modR/M:r/m field.The source operand may be either a 32-bit or a 64-bit general-purpose register.
The REX.W prefix indicates the operand size is 64 bits.
If no REX.W prefix is used, the operand size is 32 bits; the upper 32 bits of the source register are ignored and upper 32 bits of the base address (for FS or GS) are cleared.
This instruction is supported only in 64-bit mode.

## Flags affected

- None.C/C++ Compiler Intrinsic EquivalentWRFSBASE  void _writefsbase_u32( unsigned int );WRFSBASE  _writefsbase_u64( unsigned __int64 );WRGSBASE  void _writegsbase_u32( unsigned int );WRGSBASE  _writegsbase_u64( unsigned __int64 );

## Exceptions

- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.FSGSBASE[bit 16] = 0.
- Protected Mode Exceptions
  - #UD - The WRFSBASE and WRGSBASE instructions
  >  are not recognized in protected mode.
- Real-Address Mode Exceptions
  - #UD - The WRFSBASE and WRGSBASE instructions
  >  are not recognized in real-address mode.
- Virtual-8086 Mode Exceptions
  - #UD - The WRFSBASE and WRGSBASE instructions
  >  are not recognized in virtual-8086 mode.
- Compatibility Mode Exceptions

## Operation

```C
FS/GS segment base address := SRC;
```
